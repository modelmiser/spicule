//! Consumer of the sixth seam (refinement-typed bounds). `#![forbid(unsafe_code)]` again proves the
//! unchecked access is reached with zero consumer `unsafe`. The distinctive tests are the ones that
//! would be *compile errors* for the branded index: a `Refined` escapes its constructor, is stored,
//! and is returned across function boundaries — the concrete witness that this primitive is
//! **brand-free (#5 aced)** where `Idx<'id>` is not.

#![forbid(unsafe_code)]

use seam_spsc::refined::{at, refine, Refined};

#[test]
fn refine_then_index_with_no_consumer_unsafe() {
    let data = [10u32, 20, 30, 40, 50];
    let r = refine(2, data.len()).expect("2 < 5");
    assert_eq!(*at(&data, r), 30); // compiles to get_unchecked in the foundation; no `unsafe` here
}

#[test]
fn out_of_range_refine_is_none_not_ub() {
    assert!(refine(0, 3).is_some());
    assert!(refine(2, 3).is_some());
    assert!(refine(3, 3).is_none()); // value == bound → predicate false → no Refined minted
    assert!(refine(9999, 3).is_none());
}

/// THE keystone test: a `Refined` **escapes the function that built it**. For the branded index this
/// is `branded`'s CF3 (a compile error — `'id` cannot leave its scope). Here it compiles and runs:
/// brand-freedom (#5) is real, not asserted.
#[test]
fn a_refined_index_escapes_its_constructor() {
    fn midpoint_of(len: usize) -> Option<Refined> {
        refine(len / 2, len) // returned out of the function — impossible for Idx<'id>
    }
    let data = [1i64, 2, 3, 4, 5, 6, 7];
    let mid = midpoint_of(data.len()).unwrap();
    assert_eq!(*at(&data, mid), 4);

    // ...and is freely storable in a container, again unlike a branded index.
    let proofs: Vec<Refined> = (0..data.len()).filter_map(|i| refine(i, data.len())).collect();
    let sum: i64 = proofs.iter().map(|&r| *at(&data, r)).sum();
    assert_eq!(sum, 1 + 2 + 3 + 4 + 5 + 6 + 7);
}

/// The brand-free *trade*: a `Refined` binds to a length, not a slice identity, so it is valid
/// against any same-length slice (memory-safe; the identity-binding the brand would have given up).
#[test]
fn refinement_binds_length_not_identity() {
    let a = [10u8, 11, 12];
    let b = [20u8, 21, 22]; // different data, same length
    let r = refine(1, 3).unwrap();
    assert_eq!(*at(&a, r), 11);
    assert_eq!(*at(&b, r), 21); // sound on both — bound matches len; "wrong slice" is logic, not UB
}

/// Using a refined index on a *different-length* slice is a caught logic error (panic), never UB.
#[test]
#[should_panic(expected = "refined index proven against length 5 used on a slice of length 3")]
fn foreign_length_slice_panics_not_ub() {
    let short = [1u32, 2, 3];
    let r = refine(4, 5).unwrap(); // proven < 5
    let _ = at(&short, r); // bound (5) != len (3) → assert fires before any unchecked access
}
