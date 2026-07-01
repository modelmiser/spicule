//! Consumer-side proof for the thirty-fourth seam (`total_div`): the salvage of #33.
//!
//! The negative half — that division by a *raw* integer does not compile — is the `compile_fail` doctest CF1 in
//! the module. These tests exhibit the positive half: a `TotalDivisor` (`∉ {0, -1}`) makes `total_divide`/
//! `total_remainder` return a plain `-> i32` that NEVER panics — including on `i32::MIN`, the dividend that was
//! #33's conceded residue. Both of division's panic sources are closed at construction; there is no `Result`, no
//! `.unwrap()`, no panic path left to write.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::total_div::{total_divide, total_remainder, TotalDivisor};

/// The construction check rejects BOTH panic sources' divisor values — `0` (div-by-zero) and `-1` (the overflow's
/// divisor conjunct) — and admits everything else.
#[test]
fn zero_and_minus_one_are_both_rejected_at_construction() {
    assert!(TotalDivisor::new(0).is_none()); // div-by-zero source
    assert!(TotalDivisor::new(-1).is_none()); // i32::MIN / -1 overflow source
    assert!(TotalDivisor::new(1).is_some());
    assert!(TotalDivisor::new(-2).is_some());
    assert!(TotalDivisor::new(i32::MAX).is_some());
    assert!(TotalDivisor::new(i32::MIN).is_some()); // MIN is a fine *divisor*; only -1 is the hazard
}

/// Ordinary total division is correct and returns a plain `i32` — no `Result`, no `.unwrap()`.
#[test]
fn total_division_is_plain_i32_and_correct() {
    let d = TotalDivisor::new(3).expect("3 ∉ {0,-1}");
    let q: i32 = total_divide(20, d); // note: plain i32, not Result
    assert_eq!(q, 6);
    assert_eq!(total_remainder(20, d), 2);

    let neg = TotalDivisor::new(-4).expect("-4 ∉ {0,-1}");
    assert_eq!(total_divide(20, neg), -5);
}

/// THE SALVAGE, demonstrated: `i32::MIN` — #33's conceded overflow residue — divides totally by EVERY admissible
/// divisor, with no panic and no `Err`. Because `-1` is excluded, the one overflowing pair can never be formed.
#[test]
fn min_dividend_is_total_for_every_admissible_divisor() {
    // The pair that overflowed #33's `divide` (MIN / -1) is unconstructible: -1 is not a TotalDivisor.
    // Every *other* divisor divides MIN totally, returning a plain i32.
    for raw in [1, 2, -2, 3, -3, 7, i32::MAX, i32::MIN] {
        let d = TotalDivisor::new(raw).expect("∉ {0,-1}");
        let q: i32 = total_divide(i32::MIN, d); // never panics
        assert_eq!(q, i32::MIN / raw); // matches the primitive where the primitive is itself total
        let _r: i32 = total_remainder(i32::MIN, d);
    }
}

/// Excluding `-1` costs nothing safe: `a / -1 == -a`, and the only `a` where `a / -1` overflows (`i32::MIN`) is
/// exactly where `-a` overflows too. So negation covers the former `-1` divisor for every dividend that does not
/// already overflow — the witness drops the hazard, not a capability.
#[test]
fn dividing_by_minus_one_was_never_safely_more_than_negation() {
    for a in [0, 1, -1, 100, -100, i32::MAX, i32::MIN + 1] {
        // a / -1 (had it been allowed) == -a, which is safe here because a != i32::MIN.
        assert_eq!(a.checked_div(-1), Some(-a));
    }
    // The lone exception coincides exactly with negation's own overflow — nothing safe is lost:
    assert_eq!(i32::MIN.checked_div(-1), None);
    assert_eq!(i32::MIN.checked_neg(), None);
}
