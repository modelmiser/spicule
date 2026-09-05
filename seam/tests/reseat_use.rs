//! Twentieth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The controlled counterpart to #19 (`restore`): identical packaging, only the value-ownership model flips
//! (owned value → borrowed slot). Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • the sound core — mutate-in-place then reseat, on normal exit AND on panic-unwind (unwind-safe);
//!   • `mem::forget` is a LEAK, not a double-free (the slot keeps valid original bits) — so the #17 gap is,
//!     for this packaging, a horn-3-flavoured leak, not the break;
//!   • `into_inner` — the API that was SOUND for #19's owned value — DOUBLE-DROPS over a borrowed slot: the
//!     horn-1 break, laundered through the safe seam.
//!
//! The double-drop is made observable (and safe to run) with `DropCount`, which owns no memory — its `Drop`
//! only bumps a counter, so "dropping twice" reaches count 2 without real undefined behaviour. For a `T` that
//! owns a resource (a `Box`, a file), the identical structure is a genuine double-free.
#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::reseat::Reseat;

/// A drop-counter standing in for a resource-owning `T`. `Drop` bumps the shared counter; it owns no heap, so
/// a double-drop is observable as `count == 2` rather than as memory corruption. (`Clone` is NOT derived —
/// the duplication under test comes from the seam's `ptr::read`, not from safe code.)
struct DropCount<'a>(&'a Cell<u32>);
impl Drop for DropCount<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn mutate_in_place_then_reseat_on_normal_exit() {
    let mut n = 100i32;
    {
        let mut g = Reseat::new(&mut n);
        *g += 23;
    } // Drop reseats
    assert_eq!(n, 123, "the in-place mutation is reseated on ordinary scope exit");
}

#[test]
fn reseats_while_unwinding_a_panic() {
    // The guard form is unwind-safe WITHOUT an abort-bomb (contrast the eager ninth seam `take`): Drop runs
    // during unwinding and reseats a valid value, so the caller never observes the duplicate.
    let drops = Cell::new(0);
    let mut slot = DropCount(&drops);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _g = Reseat::new(&mut slot);
        panic!("boom"); // unwinds; _g's Drop reseats the value before we leave the scope
    }));
    assert!(r.is_err());
    drop(slot); // the slot holds a valid, reseated value — dropped exactly once
    assert_eq!(drops.get(), 1, "unwind-safe: the value is reseated and dropped exactly once");
}

#[test]
fn forget_after_a_replacing_mutation_double_drops() {
    // REVISED 2026-09-04. This test used to be `forget_is_a_leak_not_a_double_free` and asserted
    // the opposite. It could not fail: `DropCount` owns no heap, and its "mutation" was
    // `let _ = &mut *g;`, which invalidates nothing — so the assertion held whether or not the
    // property did.
    //
    // The property does not hold. `new` does `ptr::read(slot)`, leaving a bitwise duplicate in the
    // slot. `DerefMut` lets safe code destroy what those bits name — here by assigning a new value,
    // which drops the original. `mem::forget` then skips the `Drop` that would overwrite the stale
    // bits, so the slot's owner drops the SAME value a second time.
    //
    // No `unsafe`, no `into_inner`: `new` / `DerefMut` / `forget` alone. That is the subset the
    // module used to call sound-but-degenerate.
    let drops = Cell::new(0);
    {
        let mut slot = DropCount(&drops);
        {
            let mut g = Reseat::new(&mut slot);
            *g = DropCount(&drops); // drops the original: count 1. The slot's bits now name it.
            assert_eq!(drops.get(), 1, "the replaced original dropped through the guard");
            core::mem::forget(g); // skip the reseat; the slot keeps the stale bits
        }
    } // the slot's owner drops those stale bits: count 2 — the same value, twice
    assert_eq!(
        drops.get(), 2,
        "DOUBLE DROP: forget after a replacing mutation drops one value twice (was documented as a leak)"
    );
}

#[test]
fn forget_without_a_replacing_mutation_is_only_a_leak() {
    // The narrower claim that IS true, and the one the old test actually checked: if the guard is
    // forgotten without invalidating what the slot's duplicate names, the slot's bits are still a
    // valid `T` and are dropped exactly once. The mutation is lost — a leak. This is the boundary:
    // the leak-vs-double-free line is drawn by whether `DerefMut` was used destructively, not by
    // `mem::forget` itself.
    let drops = Cell::new(0);
    {
        let mut slot = DropCount(&drops);
        {
            let g = Reseat::new(&mut slot);
            core::mem::forget(g); // never touched the value
        }
    }
    assert_eq!(drops.get(), 1, "untouched: forget leaks the owned copy, slot dropped exactly once");
}

#[test]
fn the_break_into_inner_double_drops_over_a_borrowed_slot() {
    // THE HORN-1 BREAK. `into_inner` hands back the owned value WITHOUT reseating, so the slot keeps the read
    // duplicate. Two owners → two drops of one logical value. For a resource-owning `T` this is a double-free,
    // reached entirely from safe code (this file forbids `unsafe`). The identical API was SOUND for #19's
    // owned value (one owner); the borrowed slot flips it to a break.
    let drops = Cell::new(0);
    {
        let mut slot = DropCount(&drops);
        {
            let g = Reseat::new(&mut slot); // ptr::read: owned == slot (bitwise duplicate)
            let owned = g.into_inner(); // owned handed out; slot keeps the duplicate; the borrow ends
            drop(owned); // first drop  → count 1
        }
        // slot still in scope, holds the DUPLICATE bits
    } // slot drops the duplicate → second drop → count 2
    assert_eq!(
        drops.get(),
        2,
        "into_inner over a borrowed slot double-drops: the horn-1 break the borrowed slot forces"
    );
}
