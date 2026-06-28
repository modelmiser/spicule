//! Ninth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code moves a value out of a `&mut T` and back through `take`, with no
//! `unsafe`, no proof, and correct drop discipline on the happy path. The *panic* path is deliberately
//! NOT exercised: a mid-take panic aborts the whole process (that abort is the unwind-safety cost being
//! paid), so a test of it would kill the runner — its soundness is argued in the module/spec instead.

#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::take::take;

#[test]
fn takes_transforms_and_replaces() {
    let mut s = String::from("hello");
    take(&mut s, |old| (old + " world", ()));
    assert_eq!(s, "hello world");
}

#[test]
fn returns_the_extra_output() {
    let mut n = 10u32;
    let doubled = take(&mut n, |old| (old * 2, old)); // store old*2, return the previous value
    assert_eq!(n, 20);
    assert_eq!(doubled, 10);
}

#[test]
fn moves_a_non_copy_value_through_by_value() {
    // The closure receives the value *by move* (owned `T`), not a borrow — that is the whole point:
    // `take` is for transforms that must consume and reconstruct, e.g. swapping an enum variant.
    let mut v = vec![1, 2, 3];
    take(&mut v, |mut old| {
        old.push(4);
        (old, ())
    });
    assert_eq!(v, vec![1, 2, 3, 4]);
}

/// A payload that records every drop, so we can prove the happy path drops the old value exactly once
/// (no leak, no double-drop) — the soundness property the abort-bomb protects on the panic path.
struct Counted<'a> {
    tag: u32,
    drops: &'a Cell<u32>,
}
impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn happy_path_drops_the_old_value_exactly_once() {
    let drops = Cell::new(0);
    let mut slot = Counted { tag: 1, drops: &drops };

    // Replace with a fresh value; the OLD payload is consumed inside the closure (dropped once there),
    // the NEW one lives in `slot` afterward. Net: exactly one drop so far (the old one).
    take(&mut slot, |old| {
        assert_eq!(old.tag, 1);
        // `old` is dropped here, at end of closure: drops -> 1.
        (Counted { tag: 2, drops: old.drops }, ())
    });
    assert_eq!(drops.get(), 1, "old value dropped exactly once, not zero (leak) or twice (double-free)");
    assert_eq!(slot.tag, 2);

    // Dropping `slot` (the new value) at scope end brings the total to 2 — also exactly once each.
    drop(slot);
    assert_eq!(drops.get(), 2);
}
