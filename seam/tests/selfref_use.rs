//! Eleventh seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code sets up and reads a self-referential cell through its internal pointer
//! with no `unsafe` and no proof — and that the borrow checker, via `Pin` over a `!Unpin` cell, makes
//! the dangling-self-pointer move *unconstructible* in safe code. The whole soundness story is "the
//! value cannot move after `init`," and that is a compile-time guarantee here.

#![forbid(unsafe_code)]

use core::pin::pin;
use seam_spsc::selfref::SelfRef;

#[test]
fn reads_through_the_self_pointer_after_init() {
    let mut s = pin!(SelfRef::new(42));
    s.as_mut().init();
    // `get()` dereferences the internal `*const u64`; sound because `s` is pinned and cannot move.
    assert_eq!(s.as_ref().get(), 42);
    assert_eq!(s.as_ref().value(), 42); // the direct read agrees
}

#[test]
fn the_cell_is_not_unpin() {
    // The whole guarantee rests on `SelfRef: !Unpin`. Assert it structurally: a function bounded by
    // `Unpin` must NOT accept `SelfRef`. (We can only state the positive — that ordinary `Unpin` types
    // do satisfy it — here; the negative is the CF1 compile_fail in the module docs.)
    fn requires_unpin<T: Unpin>() {}
    requires_unpin::<u64>(); // compiles
    // requires_unpin::<SelfRef>(); // would NOT compile — SelfRef is !Unpin (see CF1)
}

#[test]
fn many_cells_each_point_at_their_own_value() {
    // Each pinned cell's self-pointer is independent; pinning them separately keeps each address fixed.
    let mut a = pin!(SelfRef::new(7));
    let mut b = pin!(SelfRef::new(9));
    a.as_mut().init();
    b.as_mut().init();
    assert_eq!(a.as_ref().get(), 7);
    assert_eq!(b.as_ref().get(), 9);
    // Re-read to confirm stability across multiple accesses (no move happened between them).
    assert_eq!(a.as_ref().get() + b.as_ref().get(), 16);
}
