//! Thirteenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code compares and selects secrets through the constant-time interface with no
//! `unsafe` and no data-dependent branch on the secret — the canonical use being a **constant-time MAC
//! check**. The functional results are correct; the *timing* property is argued (and is a TCB axiom — the
//! compiler does not guarantee it), not tested here.

#![forbid(unsafe_code)]

use seam_spsc::secret::Secret;

#[test]
fn ct_eq_is_functionally_correct() {
    let key = Secret::new([0xDEu8, 0xAD, 0xBE, 0xEF]);
    let same = Secret::new([0xDEu8, 0xAD, 0xBE, 0xEF]);
    let diff_last = Secret::new([0xDEu8, 0xAD, 0xBE, 0x00]);
    let diff_first = Secret::new([0x00u8, 0xAD, 0xBE, 0xEF]);

    assert!(key.ct_eq(&same).expose_bool());
    assert!(!key.ct_eq(&diff_last).expose_bool());
    assert!(!key.ct_eq(&diff_first).expose_bool()); // a first-byte mismatch is rejected like any other
}

#[test]
fn the_canonical_use_a_constant_time_mac_check() {
    // The real reason this seam exists: verify a received tag against the expected one without leaking,
    // via timing, *how many leading bytes matched* (which a naive `==` would leak, enabling forgery).
    let expected = Secret::new([1u8, 2, 3, 4, 5, 6, 7, 8]);
    let received_good = Secret::new([1u8, 2, 3, 4, 5, 6, 7, 8]);
    let received_bad = Secret::new([1u8, 2, 3, 4, 5, 6, 7, 9]); // matches 7 of 8 bytes

    // The accept/reject bit is the ONE sanctioned exit from constant-time land.
    assert!(expected.ct_eq(&received_good).expose_bool(), "valid tag accepted");
    assert!(!expected.ct_eq(&received_bad).expose_bool(), "near-miss tag rejected, no prefix leak");
}

#[test]
fn conditional_select_is_branchless_and_correct() {
    let a = Secret::new([10u8, 20, 30, 40]);
    let b = Secret::new([50u8, 60, 70, 80]);

    let pick_a = Secret::<4>::conditional_select(&a, &b, a.ct_eq(&a)); // choice = true → a
    let pick_b = Secret::<4>::conditional_select(&a, &b, a.ct_eq(&b)); // choice = false → b

    // Confirm via constant-time equality (we never expose the bytes directly).
    assert!(pick_a.ct_eq(&a).expose_bool());
    assert!(pick_b.ct_eq(&b).expose_bool());
}
