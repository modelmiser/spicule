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
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_sync_cell.md, r42): SEAM-BREAKS [horn1] — but the CONTRIBUTION FAILED:
//! ## my "floor-closed / no value-adding seam" classification is REFUTED by `SpinMutex` (the calibration tic again).
//!
//! Horn-classifier: **SEAM-BREAKS [horn1-sound-reimport]** — *"the unconditional `unsafe impl Sync` is an
//! undischarged axiom; safe `thread::scope` races the non-atomic `set` — real UB."* The break survives every
//! attacker (`&SyncCell<i32>` shared across `thread::scope`, two racing non-atomic writes, **zero consumer
//! `unsafe`**). By the mechanical convention the broken-seam tally rises **8→9** — the first concurrency-domain
//! break — **but this break was pre-CONCEDED as folklore** (a deliberately-unsound strawman built as the probe's
//! vehicle), qualitatively unlike the 8 *discovered* breaks. The break is not the point.
//!
//! **JUDGE → FAILED, and the point is the refuted contribution.** My staked §6 deliverable — *"no value-adding
//! concurrency seam exists; data-race freedom is **floor-closed**; a re-import is **categorically** an unchecked
//! axiom"* — is **genuinely refuted**. The counterexample is one obvious seam I omitted: a sound, non-vacuous,
//! `core`-only **`SpinMutex<T>`** (or a lock-based channel) whose `unsafe impl<T: Send> Sync` is **DISCHARGED by the
//! lock's mutual-exclusion invariant** — not asserted false like `SyncCell`'s. That is *exactly* the seam I claimed
//! cannot exist. `SpinMutex` is the concurrency **slab**.
//!
//! **THE CORRECTION (the honest classification).** Distinguish a **discharged** `unsafe impl` (SpinMutex — sound,
//! the obligation met by a lock invariant) from a **false** one (SyncCell — the obligation simply deleted). Then
//! data-race freedom (DRF) is **"floor-PROPAGATES-but-does-not-close,"** *analogous to borrowck / memory-aliasing*:
//! the auto-trait **propagates** `Send`/`Sync` structurally (like borrowck propagates ownership) but does **not
//! close** DRF — **closure comes from concentrating seams** (locks, channels, atomics). So my trichotomy (IFC
//! floor-closes-nothing / memory concentrates / concurrency *floor-closed-special*) **collapses to a dichotomy**:
//! concurrency is in the **same bucket as memory-aliasing** (floor propagates + concentrating seams close), not a
//! special third bucket.
//!
//! **What SURVIVES:** DRF is **not a fourth soundness domain** — a data race *is* UB (memory-safety); the judge did
//! not dispute this. So the **5th fourth-domain probe also fails to open a fourth domain** (after `crdt`/`wcet`/
//! `divisor`) — the 3-domain map + application-correctness basis holds — but the *reason* I gave (floor-closed) was
//! wrong; the right reason is that concurrency is a *concentrate-behind-a-seam* domain like memory-aliasing.
//!
//! **META — the same calibration tic, at full strength on fresh ground.** I over-claimed the **cleaner framing** (a
//! neat 3-way trichotomy with concurrency as a special floor-closed bucket) when the truth is the messier middle
//! (concurrency = the memory-aliasing bucket; locks are its seams). Refuted by the **obvious omitted seam**
//! (`SpinMutex`, standard ecosystem — I cited `SendWrapper`/`fragile` but missed the sound lock), the **same shape
//! as #33** (omitted `checked_div`). Unlike #32/#34 (tics trimmed below the horn-deciding threshold), this one was a
//! **horn-deciding failure** — moving to a fresh domain, my over-confidence about the classification reasserted the
//! tic at full strength. The **salvage #36** (mirroring #26→#27): build the `SpinMutex<T>` — the sound,
//! value-adding, `core`-only concurrency seam with a *discharged* `unsafe impl` — proving DRF is a
//! concentrate-behind-a-seam domain and closing the over-claim. Tally 8→9 (conceded-folklore break); N stays 9;
//! rejection-mode count STAYS 8 (this is a break, not a sound-seam rejection). Lean `sync_cell_datarace_factors`
//! (a `..._factors`: the seam breaks/factors; the refuted classification and the `SpinMutex` correction live in the
//! prose).
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
