//! WITNESS for the MMIO seam's one soundness escape hatch: `__reset_for_test` revokes M3.
//!
//! M3 (at-most-one-owner) is the SOLE discharge for `unsafe impl Sync for Mmio`
//! (`seam/src/mmio.rs:30`). `StatusReg::take()` enforces it with a `TAKEN` atomic, and
//! `mmio_use.rs` asserts the enforcement holds. This file asserts the complementary fact:
//! `__reset_for_test` clears `TAKEN` unconditionally, so a second `take()` succeeds while
//! the first handle is still live — two owners, each vending `&mut self` to one register.
//!
//! That is why the helpers are behind `test-helpers` (see `seam/Cargo.toml`): under the
//! default features this file does not compile, the helpers do not exist, and M3 is
//! genuinely undischargeable from safe code. The property is pinned here rather than left
//! as a latent surprise for whoever next reaches for the helper.
//!
//! Separate test target on purpose: `mmio_use.rs` documents that the register is a global
//! singleton and parallel test fns would collide on it. A second `#[test]` in that file
//! would race it; a second test BINARY gets its own process, so its own statics.
//!
//! We stop at the double-take. Demonstrating the actual data race needs two threads
//! writing the region concurrently — deliberately executing UB, which is not something a
//! test suite should do on every run. The structural break is the finding; the race is its
//! consequence.

#![cfg(feature = "test-helpers")]
#![forbid(unsafe_code)] // the whole point: safe code alone revokes the invariant

use seam_spsc::mmio::StatusReg;

#[test]
fn reset_for_test_revokes_m3_and_yields_two_live_owners() {
    seam_spsc::mmio::__reset_for_test();

    let first = StatusReg::take().expect("first take succeeds");

    // M3 holds while `first` is live — this is the guarantee mmio_use.rs relies on.
    assert!(
        StatusReg::take().is_none(),
        "M3: a second take must be refused while a handle is live"
    );

    // The escape hatch. No `unsafe`, no handle consumed, no ownership transferred.
    seam_spsc::mmio::__reset_for_test();

    let second = StatusReg::take();
    assert!(
        second.is_some(),
        "WITNESS: __reset_for_test revoked M3 — a second owner was minted while the first lives"
    );

    // Both handles are alive here simultaneously. Each `read_status`/`configure` call takes
    // `&mut self` and its SAFETY comment claims exclusive access to REGION; with two owners
    // that claim is false. Sequential use from one thread is not itself UB — the UB needs
    // concurrency, which `unsafe impl Sync for Mmio` would permit.
    let _both_alive = (&first, second.as_ref().unwrap());
    drop((first, second));
}
