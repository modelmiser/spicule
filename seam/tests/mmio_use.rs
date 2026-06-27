//! SOUND-USE side of the MMIO seam: ordinary consumer code drives a read-to-clear status
//! register with **zero `unsafe`** and **zero proofs**. One test function (the register is a
//! global singleton, so parallel test fns would collide on it).
//!
//! Seam-width note (horn 3): the consumer crosses the seam exactly **once**, at `take`.
//! Thereafter `read_status`/`configure` are ordinary safe calls — 0 crossings per access,
//! identical to SPSC. The memory-safety seam closes; the *axiom* (M1/M4) is the honest gap.

#![forbid(unsafe_code)] // consumer code contains no unsafe whatsoever

use seam_spsc::mmio::{Config, Status, StatusReg};

#[test]
fn mmio_read_to_clear_is_safe_and_singleton() {
    seam_spsc::mmio::__reset_for_test();

    // One seam crossing: claim the register. The singleton discharges M3.
    let mut reg = StatusReg::take().expect("first take succeeds");

    // A second owner is refused at run time (M3) — safe code cannot mint a second handle.
    assert!(StatusReg::take().is_none(), "register is a singleton; second take yields None");

    // Device raises status bits; consumer reads them (read-to-clear). Ordinary safe use
    // from here on — no unsafe, no proof.
    seam_spsc::mmio::__device_raise(0b0110);
    let s = reg.read_status();
    assert_eq!(s, Status(0b0110), "read returns the pending bits");

    // The read CLEARED them (read-to-clear side effect) — second read sees 0.
    assert_eq!(reg.read_status(), Status(0), "read-to-clear: bits gone after one read");

    // A configuration write is likewise a plain safe call (separate register on real HW).
    reg.configure(Config(0b1011));
}
