//! Eighth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code indexes a `Bound` buffer by a type-level-proven index with no `unsafe`,
//! no runtime bounds check, and no proof obligation written by the consumer — and that the proof is a
//! pure compile-time artifact (it erases to a plain `get_unchecked`). Every in-bounds access here is
//! checked by trait resolution at compile time; the only thing the consumer pays is checker fuel.

#![forbid(unsafe_code)]

use seam_spsc::typed_nat::{get, Bound, N0, N2, N3, N6, N7};

#[test]
fn proven_indices_read_the_right_elements() {
    let v = [10u32, 11, 12, 13, 14, 15, 16];
    let b = Bound::<u32, N7>::new(&v).expect("length 7 binds to N7");
    // Each access is sound by a type-level proof `Ni: Lt<N7>` — no runtime bounds check.
    assert_eq!(*get::<N0, _, _>(&b), 10);
    assert_eq!(*get::<N2, _, _>(&b), 12);
    assert_eq!(*get::<N3, _, _>(&b), 13);
}

#[test]
fn new_rejects_a_length_mismatch() {
    // The one disclosed runtime fact: the slice's length must equal the type-level numeral.
    let v = [0u32; 5];
    assert!(Bound::<u32, N7>::new(&v).is_none()); // 5 != 7
    let ok = [0u32; 7];
    assert!(Bound::<u32, N7>::new(&ok).is_some());
}

#[test]
fn the_witness_is_zero_sized_it_erases() {
    // The whole index proof carries no runtime data: `Bound` is exactly a `&[T]` in size, and the
    // type-level index types are zero-sized. The witness is the *constraint*, not a value.
    use core::mem::size_of;
    assert_eq!(size_of::<Bound<u32, N7>>(), size_of::<&[u32]>());
    assert_eq!(size_of::<seam_spsc::typed_nat::N3>(), 0);
}

#[test]
fn a_hot_loop_indexes_with_no_per_access_check() {
    // Index the same buffer many times by proven indices; the only cost per access is the (already
    // discharged) compile-time proof. Mirrors the branded index's hot-loop keystone, but the witness
    // here is a *type-level proof*, not a runtime `Idx`.
    let v = [1u64, 2, 3, 4, 5, 6, 7];
    let b = Bound::<u64, N7>::new(&v).expect("length 7 binds to N7");
    let mut acc = 0u64;
    for _ in 0..100_000 {
        acc += *get::<N0, _, _>(&b); // 1
        acc += *get::<N3, _, _>(&b); // 4
        acc += *get::<N6, _, _>(&b); // 7 (the last element of a length-7 buffer)
    }
    assert_eq!(acc, (1 + 4 + 7) * 100_000);
}
