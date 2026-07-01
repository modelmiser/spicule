//! # Thirty-fifth seam — SHARED MUTABILITY ACROSS THREADS: the data-race / concurrency probe.
//!
//! *(Project ordinal: the thirty-fifth primitive, and the **fifth fourth-domain probe** — after `crdt` #21
//! (convergence → folded into application-correctness), `wcet` #22 (real-time → broke unsound), and `divisor` #33
//! (panic-freedom → folded into application-correctness). This one probes **data-race freedom / concurrency** — the
//! property Rust's `Send`/`Sync` auto-traits enforce ("fearless concurrency"). The `secret` #13 panel explicitly
//! named "concurrency channels" as a distinct axis with no memory-safety analog; this asks whether data-race
//! freedom is a genuine **fourth soundness domain**, or memory-safety — a data race **is** UB — closed by a
//! *different mechanism*. It is a deliberate stress-test of the "3-domain map (memory / IFC / liveness) is complete"
//! claim the three prior deflations left standing.)*
//!
//! ## The primitive — re-exporting the capability to SHARE MUTABLE STATE ACROSS THREADS
//!
//! [`SyncCell`] wraps an [`core::cell::UnsafeCell`] with a **non-atomic `&self` get/set** (like [`core::cell::Cell`],
//! whose whole design is *single-threaded* interior mutability). The dangerous re-import is the pair of
//! **`unsafe impl Sync`/`Send`** — the foundation author's *assertion* that a `&SyncCell` is safe to share across
//! threads. Safe consumer code then uses `new`/`get`/`set` with **no `unsafe`**.
//!
//! ## The KEY OBSERVATION — data-race freedom sits at a *different structural place* than every prior domain
//!
//! - **IFC (`secret` #13):** the floor checks **nothing** (no non-interference pass), so any concentrating seam
//!   **breaks by omission** — the property is never actually enforced.
//! - **Memory-aliasing (the first 12 seams):** the floor checks via the **borrow checker**, so seams **concentrate**
//!   `unsafe` behind a checked interface (SPSC, the slab).
//! - **Data-race freedom (this seam):** the floor **already closes it structurally** via the *auto-derived*
//!   `Send`/`Sync` traits (Rustbelt-verified). A `Cell<i32>` is `!Sync` **precisely** so safe code cannot share it
//!   across threads (CF1). So a re-import seam has *nothing to add*: the only **sound** interface bounds `T: Sync`
//!   (or uses atomics) — which **is** the auto-trait, i.e. vacuously the floor — and the only **non-trivial**
//!   re-import is the `unsafe impl` escape, which is an **unchecked TCB axiom** (MMIO-shaped), not a checkable rule.
//!
//! ## HYPOTHESIS (this is the artifact commit; the cold panel adjudicates, not the builder)
//!
//! Predicted **SEAM-BREAKS [horn1-sound-reimport]** — the unconditional `unsafe impl Sync` lets safe code share
//! `&SyncCell<i32>` across threads and race the non-atomic `set` (a data race = UB), with **no consumer `unsafe`**;
//! the `no_run` doctest below shows `std::thread::scope` *accepting* the racing shares (the type-level soundness
//! hole), while CF1 shows the floor correctly **rejecting** a shared `Cell`. If accepted as a soundness break, the
//! broken-seam tally rises 8→9 — the **first concurrency-domain break**. THE FINDING (the point of the probe):
//! data-race freedom is **NOT a fourth *seamable* domain** — it is **memory-safety (UB) closed by the floor's
//! auto-trait mechanism**, so a re-import can only restate the floor (vacuous, `T: Sync`) or break (the `unsafe
//! impl` axiom). So the 3-domain map **survives** the stress test: concurrency folds into memory safety, closed
//! structurally rather than by a new coordinate. Pre-registered deflations the panel may sharpen: (a) the `unsafe
//! impl Send/Sync` footgun is textbook (the Rustonomicon) — the contribution is the **domain classification**
//! (floor-closed, unlike IFC), not the break; (b) the witness is a **type-level** soundness hole (the unsound
//! `Sync` grant), degenerate on the `no_std` floor — real UB needs a spawned thread, shown *compiling* via a
//! `no_run` doctest, not run (the same single-threaded-witness limit the session seams carried); (c) it overlaps
//! `mmio` (an unchecked TCB axiom) and `secret` #13 (a horn-1 break) — the novelty is the *floor-closed*
//! classification that distinguishes concurrency from both.
//!
//! ### CF1 — the FLOOR closes data-race freedom: a shared `Cell` is rejected (`Cell<i32>: !Sync`):
//! ```compile_fail
//! # use core::cell::Cell;
//! fn assert_sync<T: Sync>() {}
//! assert_sync::<Cell<i32>>();   // ERROR: `Cell<i32>` is not `Sync` — the floor forbids cross-thread sharing
//! ```
//!
//! ### The BREAK — the seam's `unsafe impl Sync` lets safe code race across threads (compiles; `no_run`):
//! ```no_run
//! # use seam_spsc::sync_cell::SyncCell;
//! // A separate (std) consumer crate; the SyncCell lib is `no_std`, but `Sync` is a cross-crate promise.
//! let cell = SyncCell::new(0i32);
//! std::thread::scope(|s| {
//!     s.spawn(|| cell.set(1));      // two threads, &SyncCell shared (allowed by the unsound `Sync`),
//!     s.spawn(|| cell.set(2));      // racing a NON-ATOMIC write — a data race, i.e. UB, in SAFE code.
//! });
//! ```

use core::cell::UnsafeCell;

/// A cell with **non-atomic** interior mutability (`&self` get/set), like [`core::cell::Cell`] — but with the
/// dangerous addition of `unsafe impl Sync`/`Send`, re-exporting the *capability to share it across threads*. The
/// `unsafe impl`s are the concentrated `unsafe`; the consumer touches none. That the impls are **unconditional**
/// (no `T: Sync` bound, no atomics) is exactly the unchecked axiom the seam re-imports.
pub struct SyncCell<T> {
    inner: UnsafeCell<T>,
}

impl<T: Copy> SyncCell<T> {
    /// Wrap a value.
    pub const fn new(v: T) -> Self {
        SyncCell { inner: UnsafeCell::new(v) }
    }

    /// Read the value (non-atomic).
    pub fn get(&self) -> T {
        // SAFETY (single-threaded): reading the cell is fine with no *concurrent* writer. The seam's whole point
        // is that its `unsafe impl Sync` (below) FALSELY promises this holds across threads — see the module BREAK.
        unsafe { *self.inner.get() }
    }

    /// Write the value through a shared `&self` (non-atomic) — the racing operation under the unsound `Sync`.
    pub fn set(&self, v: T) {
        // SAFETY (single-threaded only): exclusive in a single thread; NOT across threads (the false promise).
        unsafe {
            *self.inner.get() = v;
        }
    }
}

// THE DANGEROUS RE-IMPORT — the foundation author's UNCHECKED AXIOM that `&SyncCell` is safe to share and to send
// across threads. Unconditional (no `T: Sync`/`T: Send` bound): this is what makes it a re-import of an axiom rather
// than a restatement of the floor's auto-trait. It is UNSOUND for any non-atomic `T` (the module BREAK races it).
unsafe impl<T> Sync for SyncCell<T> {}
unsafe impl<T> Send for SyncCell<T> {}
