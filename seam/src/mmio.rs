//! # Second seam — an MMIO read-to-clear status register (Phase C contrast)
//!
//! Same floor, same rules as the SPSC seam. The point of this primitive is to show a seam
//! whose *memory-safety* obstruction **closes** exactly like SPSC (affine singleton handle,
//! encapsulated volatile access, O(1) seam width) — yet whose *soundness* rests on an axiom
//! that is **outside the formal system entirely**: the claim that a given address really is
//! this device register with these datasheet bit-semantics (M1/M4 in the spec). No checker,
//! decidable or not, in this language or any, can discharge that — it is asserted by fiat in
//! the one place the HAL author writes the base address.
//!
//! We cannot read real hardware in a unit test, so a static cell stands in for the physical
//! MMIO region; the **affine + volatile + read-to-clear structure** is the real demonstration.
//!
//! ## The stand-in's one infidelity: `read_status` has a lost-event window
//!
//! Real read-to-clear hardware clears the pending bits **as an atomic side effect of the
//! read** — one bus transaction. The stand-in cannot do that, so [`StatusReg::read_status`]
//! models it as two operations: a volatile read, then a volatile store of `0`. Any bits the
//! device raises *between* those two operations are destroyed by the store and never
//! observed. In this crate that window is reachable — `__device_raise` (feature
//! `test-helpers`) writes the region directly — but it is an artifact of the model, not of
//! the design being demonstrated.
//!
//! **Do not carry the read-then-write-zero shape to real hardware without checking the
//! datasheet's clear semantics.** The two common kinds want opposite things:
//!
//! - **Read-to-clear (RC):** the read alone clears. Drop the store entirely — it is
//!   redundant, and it is exactly what opens the window above.
//! - **Write-1-to-clear (W1C):** write back the value you *observed* (`v`), which clears
//!   precisely the bits you are acknowledging and leaves later arrivals pending. Storing `0`
//!   to a W1C register clears **nothing**, so the pattern below would silently never
//!   acknowledge anything.
//!
//! The seam's subject — affine handle, encapsulated volatile access, M1/M4 as a named axiom
//! — is unaffected either way. This note exists so the *access pattern* is not mistaken for
//! part of what the seam is asserting.
//!
//! Affine handle cannot be cloned (would break single-owner M3):
//! ```compile_fail
//! # use seam_spsc::mmio::StatusReg;
//! let reg = StatusReg::take().unwrap();
//! let _r2 = reg.clone(); // ERROR: StatusReg is affine — no Clone
//! ```

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// Stand-in for the physical MMIO region. On hardware this is `0x4000_4400 as *mut u32`;
/// the **M1/M4 axiom** is exactly the claim that that address is this register. Accessed by
/// a single owner (M3), so the interior mutability is never raced.
struct Mmio(UnsafeCell<u32>);
// SAFETY: a real MMIO region is a fixed address; single-owner access (M3, enforced by the
// affine singleton handle) means no data race despite the shared static.
unsafe impl Sync for Mmio {}
static REGION: Mmio = Mmio(UnsafeCell::new(0));

static TAKEN: AtomicBool = AtomicBool::new(false);

/// A status word read from the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);

/// A configuration word written to the register.
#[derive(Debug, Clone, Copy)]
pub struct Config(pub u32);

/// Affine singleton handle to the status register. No `Clone`/`Copy`; not `Sync`.
/// Holding one is the certificate that you are the unique owner of a real, datasheet-correct
/// register (M1/M3/M4) — the parts the floor cannot check are folded into `take`.
pub struct StatusReg {
    _priv: (),
}

impl StatusReg {
    /// Claim the register. Safe to call, but returns `None` if already taken — the run-time
    /// singleton that discharges **M3** (at most one owner). The `unsafe`-by-fiat content is
    /// the address baked into this HAL: by handing out a `StatusReg`, the HAL asserts M1/M4.
    pub fn take() -> Option<StatusReg> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(StatusReg { _priv: () })
        }
    }

    /// Read-to-clear: return the current status **and** clear the pending bits in hardware.
    /// A plain safe method for the consumer — the volatile access is encapsulated (M2).
    ///
    /// **Model infidelity:** on real read-to-clear hardware the clear is an atomic side
    /// effect of the read. Here it is a second operation, so bits raised between the read
    /// and the store are lost. See the module docs — the shape is not portable as written.
    pub fn read_status(&mut self) -> Status {
        // SAFETY (M2): exactly one volatile read of the register width, then the hardware
        // clears the pending bits (modelled by the store). `&mut self` + the singleton give
        // exclusive access, so this is the only accessor. M1/M4 are the trusted axiom.
        let p = REGION.0.get();
        let v = unsafe { core::ptr::read_volatile(p) };
        // NOT part of the demonstration: real RC hardware has already cleared by now, and a
        // W1C register would need `v` written back, not 0. This store only emulates the
        // side effect for the static stand-in, and it is what opens the lost-event window.
        unsafe { core::ptr::write_volatile(p, 0) } // read-to-clear side effect (modelled)
        Status(v)
    }

    /// Configure the peripheral (volatile write). Plain safe method for the consumer.
    pub fn configure(&mut self, cfg: Config) {
        // SAFETY (M2): one volatile write; exclusive access via `&mut self` + singleton.
        let p = REGION.0.get();
        unsafe { core::ptr::write_volatile(p, cfg.0) }
    }
}

/// Test/demo helper: simulate the device asserting status bits between reads.
///
/// Feature-gated (`test-helpers`): this is a SAFE `pub fn` doing a non-atomic
/// read-modify-write on `REGION` with no handle, so in the default build it would let
/// safe code race the region that `unsafe impl Sync for Mmio` declares shareable.
#[cfg(feature = "test-helpers")]
#[doc(hidden)]
pub fn __device_raise(bits: u32) {
    let p = REGION.0.get();
    // SAFETY: single-threaded test helper standing in for a hardware event.
    unsafe {
        let cur = core::ptr::read_volatile(p);
        core::ptr::write_volatile(p, cur | bits);
    }
}

/// Test/demo helper: reset the singleton between tests.
///
/// Feature-gated (`test-helpers`): clearing `TAKEN` while a `StatusReg` is live lets a
/// second `take()` succeed, producing two owners of one register. M3 is the sole
/// discharge for `unsafe impl Sync for Mmio` (:30), so in the default build this
/// function would revoke the seam's own soundness argument from safe code.
#[cfg(feature = "test-helpers")]
#[doc(hidden)]
pub fn __reset_for_test() {
    TAKEN.store(false, Ordering::Release);
    let p = REGION.0.get();
    // SAFETY: one aligned volatile write of the register width to the static region.
    // Exclusivity is NOT established here — it rests on this function being reachable
    // only from a single-threaded test build (`test-helpers`), not on M3, which the
    // store above has just revoked.
    unsafe { core::ptr::write_volatile(p, 0) }
}
