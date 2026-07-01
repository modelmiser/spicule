//! Consumer-side proof for the thirty-third seam (`divisor`): the panic-freedom / totality probe.
//!
//! The negative half — that division by a *raw* integer (an un-checked, possibly-zero divisor) does not compile —
//! is the `compile_fail` doctest CF1 in the module (it cannot live here, because it must fail to compile). These
//! tests exhibit the positive half and the conceded residue: div-by-zero is unreachable through the vended API,
//! while the `i32::MIN / -1` overflow surfaces as `Err`, not a panic — the honest boundary of what the nonzero
//! witness carries.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::divisor::{divide, remainder, DivOverflow, Divisor};

/// The zero divisor is rejected at construction — there is no `Divisor` to hand to `divide`, so the div-by-zero
/// panic is unreachable through the vended interface (the concentrated, decidable fragment that closes).
#[test]
fn zero_divisor_is_rejected_at_construction() {
    assert!(Divisor::new(0).is_none());
    assert!(Divisor::new(1).is_some());
    assert!(Divisor::new(-7).is_some());
}

/// Ordinary divisions go through with the div-by-zero source foreclosed by the type.
#[test]
fn ordinary_division_is_total_and_correct() {
    let d = Divisor::new(3).expect("3 is nonzero");
    assert_eq!(divide(20, d), Ok(6));
    assert_eq!(remainder(20, d), Ok(2));

    let neg = Divisor::new(-4).expect("-4 is nonzero");
    assert_eq!(divide(20, neg), Ok(-5));
}

/// THE RESIDUE: a *valid* nonzero divisor can be `-1`, and the dividend `i32::MIN`, so the nonzero witness does
/// NOT foreclose the second panic source. The seam is honest about it — `divide` returns `Err(DivOverflow)` here
/// instead of panicking, relocating that source to a runtime `Result` rather than carrying it in the type.
#[test]
fn min_over_neg_one_is_the_conceded_overflow_residue() {
    let neg_one = Divisor::new(-1).expect("-1 is nonzero");
    assert_eq!(divide(i32::MIN, neg_one), Err(DivOverflow));
    assert_eq!(remainder(i32::MIN, neg_one), Err(DivOverflow));

    // Everything except that one pair divides fine by -1 — the residue is exactly the MIN/-1 corner.
    assert_eq!(divide(i32::MIN + 1, neg_one), Ok(i32::MAX));
    assert_eq!(divide(100, neg_one), Ok(-100));
}
