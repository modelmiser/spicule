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
fn forget_is_a_leak_not_a_double_free() {
    // `mem::forget` skips Drop, so `owned` is leaked and never dropped; the slot keeps its ORIGINAL bits and
    // is dropped exactly once. The mutation is lost (a leak), but there is NO double-free. This is the key
    // contrast with #17: the same un-compellable-Drop gap is, for THIS packaging, merely a horn-3 leak.
    let drops = Cell::new(0);
    {
        let mut slot = DropCount(&drops);
        {
            let mut g = Reseat::new(&mut slot);
            let _ = &mut *g; // touch the owned value
            core::mem::forget(g); // skip Drop: owned leaks, slot keeps original bits
        }
        // slot still in scope, holds a valid (original) value
    } // slot drops exactly once
    assert_eq!(drops.get(), 1, "forget leaks the owned copy but never double-frees — a leak, not the break");
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
