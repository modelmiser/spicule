//! Provenance probe for the `selfref` seam — the sequence the existing tests never perform.
//!
//! `init` mints `ptr` from `&this.value`, a SHARED reborrow of the `&mut Self` that existed at that
//! moment. The module's SAFETY comment argues only about WHERE the pointer points (address
//! stability, which `Pin` + `!Unpin` genuinely deliver). It never argues what BORROW the pointer
//! derives from. Under Stacked Borrows a later mutable reborrow of the pinned cell performs a write
//! access that pops that tag, after which reading through it is UB.
//!
//! Every existing test does `as_mut().init()` and then goes straight to `as_ref().get()`; a read
//! access does not pop a `SharedReadOnly` item, so the sequence below — an intervening `as_mut()` —
//! is untested. This file exists so Miri has something to judge. `cargo test` alone cannot answer
//! the question; only `cargo miri test` can.
//!
//! If this passes under Miri, the seam is fine under the model Miri implements, which is worth
//! recording. It is NOT a proof for all time: Rust has no ratified aliasing model, and Tree Borrows
//! and Stacked Borrows disagree about exactly this shape.

#![forbid(unsafe_code)]

use core::pin::pin;
use seam_spsc::selfref::SelfRef;

#[test]
fn read_through_self_pointer_after_an_intervening_mutable_reborrow() {
    let mut cell = pin!(SelfRef::new(42));
    cell.as_mut().init();

    // The intervening mutable reborrow. Ordinary safe code; no test in the crate does this.
    let _reborrow = cell.as_mut();

    // Now read through the pointer minted before that reborrow existed.
    assert_eq!(cell.as_ref().get(), 42, "value read through the self-pointer");
}

#[test]
fn re_init_writes_through_a_fresh_mutable_borrow_then_reads() {
    // Re-running `init` re-mints `ptr` from a fresh borrow. [CORRECTED 2026-11-14: this comment
    // used to claim `init` is the ONLY safe route to a `Pin<&mut Self>` and that the hazard is
    // unreachable for that reason. `as_mut` is another route — the first test in this file uses
    // it — and that test passes under Miri: under Stacked Borrows because a `&mut` to a `!Unpin`
    // type is not treated as unique, under Tree Borrows because an unwritten reborrow does not
    // invalidate a read-only pointer. See the SAFETY comment on `SelfRef::get`.]
    let mut cell = pin!(SelfRef::new(99));
    cell.as_mut().init();
    let first = cell.as_ref().get();
    cell.as_mut().init(); // fresh &mut, writes `ptr`, re-mints provenance
    assert_eq!(cell.as_ref().get(), first, "value unchanged across re-init");
    assert_eq!(cell.as_ref().get(), 99);
}

#[test]
fn repeated_reborrow_and_read_cycles() {
    let mut cell = pin!(SelfRef::new(7));
    cell.as_mut().init();
    for _ in 0..3 {
        let _reborrow = cell.as_mut();
        assert_eq!(cell.as_ref().get(), 7);
    }
}
