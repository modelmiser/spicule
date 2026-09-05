//! Tenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code drains a buffer by value with no `unsafe` and correct drop discipline —
//! and, the keystone, that **leaking the guard** (`mem::forget`, a *safe* function) stays memory-safe
//! (no double-free), losing the un-yielded tail rather than corrupting memory. The leak path is
//! testable here precisely because `mem::forget` needs no `unsafe`.

#![forbid(unsafe_code)]

use core::cell::Cell;
use core::mem;
use seam_spsc::drain::Buffer;

fn filled<const N: usize>(vals: &[u32]) -> Buffer<u32, N> {
    let mut b = Buffer::<u32, N>::new();
    for &v in vals {
        b.push(v).expect("capacity");
    }
    b
}

#[test]
fn drains_all_elements_in_order_and_empties() {
    let mut b = filled::<4>(&[1, 2, 3, 4]);
    let got: HeaplessVec = b.drain().collect();
    assert_eq!(got.as_slice(), &[1, 2, 3, 4]);
    assert!(b.is_empty());
}

#[test]
fn partial_drain_then_drop_runs_remainder_cleanup() {
    // Yield 2 of 4, then drop the Drain: the guard's Drop must clean up the un-yielded 2.
    let mut b = filled::<4>(&[10, 20, 30, 40]);
    {
        let mut d = b.drain();
        assert_eq!(d.next(), Some(10));
        assert_eq!(d.next(), Some(20));
        // `d` dropped here — its Drop drops 30 and 40.
    }
    assert!(b.is_empty());
}

/// Drop-counting payload to prove exactly-once discipline and detect double-free / leak.
struct Counted<'a> {
    drops: &'a Cell<u32>,
}
impl Drop for Counted<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn partial_drain_then_drop_actually_drops_the_remainder() {
    // M1 REGRESSION. `partial_drain_then_drop_runs_remainder_cleanup` above looks like it covers
    // `Drain::drop`'s element loop, but it uses `u32` (no drop glue) and asserts only
    // `is_empty()` — which `drain()`'s `len = 0` makes true no matter what the guard does.
    // Replacing that loop's range with an empty one passed the ENTIRE suite. This test counts
    // drops with a payload that has a destructor, so the loop has to run.
    let drops = Cell::new(0);
    {
        let mut b = Buffer::<Counted, 4>::new();
        for _ in 0..4 {
            b.push(Counted { drops: &drops }).ok().expect("capacity");
        }
        {
            let mut d = b.drain();
            drop(d.next().expect("first"));  // yielded and dropped here: 1
            drop(d.next().expect("second")); // 2
            assert_eq!(drops.get(), 2, "the two yielded elements dropped as they were consumed");
            // `d` dropped here — its Drop must drop the two un-yielded elements.
        }
        assert_eq!(
            drops.get(), 4,
            "M1: Drain::drop must drop the un-yielded remainder; got {} of 4", drops.get()
        );
        assert!(b.is_empty());
    }
    assert_eq!(drops.get(), 4, "no further drops when the emptied buffer goes out of scope");
}

#[test]
fn an_undrained_buffer_drops_its_elements() {
    // M10 REGRESSION. `Buffer::drop`'s own element loop never destroyed an observable value in
    // any test: every buffer in this file is either `u32` or has been drained to `len == 0`
    // before it drops, so gutting that loop passed the entire suite. Here the buffer is filled
    // with a destructor-bearing payload and simply dropped, never drained.
    let drops = Cell::new(0);
    {
        let mut b = Buffer::<Counted, 4>::new();
        for _ in 0..3 {
            b.push(Counted { drops: &drops }).ok().expect("capacity");
        }
        assert_eq!(drops.get(), 0, "nothing dropped while the buffer is alive");
    } // `b` dropped here, still holding 3 elements
    assert_eq!(
        drops.get(), 3,
        "M10: Buffer::drop must drop its occupied elements; got {} of 3", drops.get()
    );
}

#[test]
fn leaking_the_guard_is_sound_loses_the_tail_no_double_free() {
    let drops = Cell::new(0);
    let mut b = Buffer::<Counted, 4>::new();
    for _ in 0..4 {
        b.push(Counted { drops: &drops }).ok().expect("capacity");
    }

    // Drain 2, then LEAK the guard (mem::forget is safe — no `unsafe` needed).
    let mut d = b.drain();
    let a = d.next().expect("first");
    let c = d.next().expect("second");
    mem::forget(d); // the guard's Drop never runs

    // The two yielded values are owned here; dropping them is exactly two drops.
    drop(a);
    drop(c);
    assert_eq!(drops.get(), 2, "the two yielded elements dropped once each");

    // The buffer was pre-pessimized to len 0 by `drain`, so dropping it double-frees nothing
    // and runs no destructor for the moved-out slots. The un-yielded tail (2 elements) is LEAKED.
    drop(b);
    assert_eq!(
        drops.get(),
        2,
        "buffer drop is a no-op (len pre-set to 0); the un-yielded tail leaked, was NOT double-freed"
    );
}

/// A tiny `#![forbid(unsafe_code)]`-friendly collector so the order test needs no `Vec`/`alloc`.
struct HeaplessVec {
    data: [u32; 8],
    len: usize,
}
impl HeaplessVec {
    fn as_slice(&self) -> &[u32] {
        &self.data[..self.len]
    }
}
impl FromIterator<u32> for HeaplessVec {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        let mut v = HeaplessVec { data: [0; 8], len: 0 };
        for x in iter {
            v.data[v.len] = x;
            v.len += 1;
        }
        v
    }
}
