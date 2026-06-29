//! Fourteenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code computes on secrets and declassifies them through capability-gated exits
//! with no `unsafe` — and that the declassification budget is *type-structural* (you cannot read a secret
//! without spending a `Cap`, and each `Cap` spends once). This is the IFC-domain analog of "concentrated,
//! checked `unsafe`": declassification is bounded and auditable, not free etiquette.

#![forbid(unsafe_code)]

use seam_spsc::labeled::{Cap, Secret};

#[test]
fn a_secret_declassifies_once_at_the_trust_boundary() {
    let key = Secret::new(0xABu8);
    // The audited exit: spend one capability to observe the value.
    let revealed = key.declassify(Cap::issue());
    assert_eq!(revealed, 0xAB);
}

#[test]
fn computation_on_secrets_stays_secret_no_cap_spent() {
    // Combine and transform without declassifying — the result is still a `Secret`, no capability used.
    let a = Secret::new(20u32);
    let b = Secret::new(22u32);
    let sum: Secret<u32> = a.zip(b, |x, y| x + y);
    let doubled: Secret<u32> = sum.map(|s| s * 2);
    // Only at the very end do we spend a capability to see the answer.
    assert_eq!(doubled.declassify(Cap::issue()), 84);
}

#[test]
fn the_declassification_budget_is_explicit() {
    // The trust authority issues exactly the budget it intends; each cap declassifies one value. A
    // reviewer can count `Cap::issue()` to bound how much ever leaves the secret world.
    let secrets = [Secret::new(1u8), Secret::new(2u8), Secret::new(3u8)];
    let mut revealed = [0u8; 3];
    for (i, s) in secrets.into_iter().enumerate() {
        revealed[i] = s.declassify(Cap::issue()); // one issue() per declassification — greppable budget
    }
    assert_eq!(revealed, [1, 2, 3]);
}

#[test]
fn a_constant_time_style_equality_stays_secret_until_one_declassify() {
    // The canonical IFC use, but with the declassification *gated*: compare two secrets (the comparison
    // result is itself secret), then spend a single capability to learn accept/reject — the one
    // legitimate observable. Unlike the secret seam, the bool cannot reach `if` without that capability.
    let expected = Secret::new([1u8, 2, 3, 4]);
    let received = Secret::new([1u8, 2, 3, 4]);
    let matches: Secret<bool> = expected.zip(received, |e, r| {
        let mut diff = 0u8;
        for i in 0..4 {
            diff |= e[i] ^ r[i];
        }
        diff == 0
    });
    let accept = matches.declassify(Cap::issue()); // the single, budgeted exit
    assert!(accept);
}
