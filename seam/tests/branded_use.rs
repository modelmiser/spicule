//! Consumer of the fifth seam (branded indices). `#![forbid(unsafe_code)]` is the load-bearing
//! line: it proves the bounds-check elision is reached with **zero** consumer `unsafe`. Every
//! index access below compiles to `get_unchecked` in the foundation, yet nothing here may write
//! `unsafe`. That is the zero-cost axis (#6) aced. The fifth-axis *cost* shows up structurally:
//! all of this work must happen inside a `scope` closure, and no `Idx` can be returned from one.

#![forbid(unsafe_code)]

use seam_spsc::branded::scope;

#[test]
fn checked_once_then_indexed_with_no_consumer_unsafe() {
    let data = [10u32, 20, 30, 40, 50];
    let total = scope(&data, |view| {
        let mut sum = 0u32;
        // One bounds check per `try_index`; the indexing itself is unchecked in the foundation.
        for i in 0..view.len() {
            let idx = view.try_index(i).expect("i < len by construction");
            sum += view[idx]; // <- compiles to get_unchecked; no `unsafe` permitted in this file
        }
        sum
    });
    assert_eq!(total, 150);
}

#[test]
fn out_of_range_try_index_is_none_not_ub() {
    let data = [1u8, 2, 3];
    scope(&data, |view| {
        assert!(view.try_index(0).is_some());
        assert!(view.try_index(2).is_some());
        assert!(view.try_index(3).is_none()); // == len → rejected at the check, never indexed
        assert!(view.try_index(9999).is_none());
    });
}

#[test]
fn an_index_is_reusable_copy_within_its_scope() {
    let data = [7i64, 8, 9];
    scope(&data, |view| {
        let i = view.try_index(1).unwrap();
        // `Idx` is `Copy`: the same proof indexes repeatedly with no re-check (the hot-loop case).
        let mut acc = 0i64;
        for _ in 0..1000 {
            acc += view[i];
        }
        assert_eq!(acc, 8000);
    });
}

#[test]
fn two_indices_into_the_same_branded_slice() {
    let data = ['a', 'b', 'c', 'd'];
    scope(&data, |view| {
        let first = view.try_index(0).unwrap();
        let last = view.try_index(view.len() - 1).unwrap();
        assert_eq!(view[first], 'a');
        assert_eq!(view[last], 'd');
    });
}

#[test]
fn empty_slice_mints_no_indices() {
    let data: [u32; 0] = [];
    scope(&data, |view| {
        assert!(view.is_empty());
        assert!(view.try_index(0).is_none());
    });
}
