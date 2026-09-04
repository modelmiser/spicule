//! # Thirty-sixth seam — A CORE-ONLY SPINLOCK MUTEX: the salvage of #35 (the sound concurrency seam).
//!
//! *(Project ordinal: the thirty-sixth primitive, and the **salvage of `sync_cell` #35** — mirroring #26→#27,
//! #29→#30, #31→#32, #33→#34. #35 (`sync_cell`) was a deliberately-unsound strawman (`unsafe impl<T> Sync` with the
//! obligation simply **deleted**); it BROKE (tally 8→9) but its *contribution* FAILED: I staked "**no value-adding
//! concurrency seam exists / data-race freedom is floor-closed / a re-import is categorically an unchecked axiom**,"
//! and the r42 panel **refuted** it with one obvious omitted seam — a sound, `core`-only `SpinMutex<T>` whose
//! `unsafe impl<T: Send> Sync` is **DISCHARGED by a lock invariant**, not asserted false. This seam builds exactly
//! that. It is the concurrency **slab**: the sound value-adding seam that proves data-race freedom is a
//! *concentrate-behind-a-seam* domain, the concurrency analog of SPSC #1 / the slab #4.)*
//!
//! ## The primitive — re-exporting SHARED MUTABLE STATE ACROSS THREADS, this time SOUNDLY
//!
//! [`SpinMutex`] wraps an [`core::cell::UnsafeCell`] beside an [`core::sync::atomic::AtomicBool`] lock flag.
//! [`SpinMutex::lock`] spins on a `compare_exchange` (`false → true`, `Acquire`) until it wins the flag, then hands
//! back a [`SpinGuard`]. The guard vends `&T`/`&mut T` through `Deref`/`DerefMut`, and its `Drop` releases the flag
//! (`Release`). Safe consumer code composes `new`/`lock`/`*guard` with **no `unsafe`**. The dangerous re-import is
//! the pair of **`unsafe impl<T: Send> Sync`/`Send`** — but here, unlike #35, the assertion is *earned*.
//!
//! ## THE KEY OBSERVATION — this `unsafe impl` is DISCHARGED, not FALSE (the #35 correction, made concrete)
//!
//! - **`SyncCell` #35 (FALSE):** `unsafe impl<T> Sync` is **unconditional**; nothing in the type enforces the
//!   obligation *"no two threads concurrently touch the non-atomic storage with ≥1 write."* The obligation is simply
//!   **deleted** — an undischarged TCB axiom — so a safe `thread::scope` races it (UB). The seam **breaks**.
//! - **`SpinMutex` #36 (DISCHARGED):** the same-shaped `unsafe impl<T: Send> Sync` carries the **same** obligation —
//!   but the `AtomicBool` CAS is a **runtime witness** that at most one `SpinGuard` exists at a time, so every `&T`/
//!   `&mut T` handed out is **exclusive**. The obligation is *met*, not asserted. The `Acquire`/`Release` pairing on
//!   the flag publishes a happens-before edge so the critical sections are properly ordered. So safe code sharing a
//!   `&SpinMutex<i32>` across threads and mutating through `lock()` is **data-race-free** — the exact shape that was
//!   UB in #35 is sound here. The `T: Send` **bound** is load-bearing (it is #35's missing conjunct): the lock lets
//!   a `T` be accessed from whichever thread holds it, so `Sync` requires `T: Send` — and CF1 shows the floor
//!   correctly rejects a non-`Send` payload (`SpinMutex<Rc<i32>>: !Sync`).
//!
//! So data-race freedom is **"floor-PROPAGATES-but-does-not-close"** (the corrected #35 classification): the
//! auto-trait *propagates* `Send`/`Sync` structurally (like borrowck propagates ownership) but does **not close**
//! DRF — **closure comes from a concentrating seam**, and `SpinMutex` is that seam. Concurrency sits in the **same
//! bucket as memory-aliasing** (floor propagates + concentrating seams close), not a special "floor-closed" bucket.
//!
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_spin_mutex.md, r43): SEAM-BREAKS [horn1-sound-reimport] — THE SALVAGE
//! ## ITSELF BROKE (a DISCOVERED break, tally 9→10): I discharged the LOCK obligation but OMITTED the GUARD's.
//!
//! Horn-classifier: **SEAM-BREAKS [horn1-sound-reimport]**, JUDGE → **FAILED** (attacker-soundness GENUINE-FLAW,
//! judge-confirmed fatal). My discharge proved **single-guard-liveness** (the CAS admits one winner until `Drop`) —
//! but that is *the wrong lemma*. Soundness of a shared `&SpinMutex` also requires the **guard** not leak `&T` into
//! two threads, governed by the **guard's `Sync` bound**, which I never stated. With no manual impl, `SpinGuard`
//! auto-derives `Sync` via my own `unsafe impl<T: Send> Sync for SpinMutex`: `SpinGuard: Sync ⟺ SpinMutex<T>: Sync
//! ⟺ T: Send`. So `SpinGuard: Sync` for every `T: Send` ⇒ `&SpinGuard: Send`. Pick `T = Cell<i32>` (`Send + !Sync`,
//! which slips past CF1's `Rc: !Send` filter): `let g = m.lock();` once (uncontended), share `&g` across a
//! `thread::scope`, and two threads deref `&SpinGuard → &Cell<i32>` racing a non-atomic write against a read —
//! **UB from safe code, zero consumer `unsafe`, the lock never even contended** (VERIFIED: the path compiles under
//! `#![forbid(unsafe_code)]`). This is exactly why `std` writes the stricter `unsafe impl<T: Sync> Sync for
//! MutexGuard` (+ `MutexGuard: !Send`). A concurrency `unsafe impl` has **TWO** conjuncts — cross-thread exclusion
//! (the lock, discharged) AND the guard's no-`&T`-leak bound (`T: Sync`, OMITTED); I discharged one and dropped one.
//!
//! **What SURVIVES the break (the judge is explicit).** The #35-refuting **existential** — *"a sound, value-adding,
//! `core`-only concurrency seam exists"* — **stands on its own**: a *correctly-bounded* SpinMutex IS sound
//! (`spin::Mutex`, RustBelt-proven). So the classification contribution (DRF is **concentrate-behind-a-seam**, the
//! memory-aliasing bucket; a **discharged** `unsafe impl` distinct from a **false** one) is NOT refuted by this
//! break — my *artifact* had a hole, the *thesis* holds. Three tradeoffs ride along to the fix (#37): (1) §5's
//! liveness residue must also name **IRQ/interrupt-context self-deadlock** (`irqsave`) and **unbounded priority
//! inversion** — the acute footguns of the bare-metal use case; (2) deliverable 2 owes the **static-vs-dynamic
//! discharge** asymmetry — SPSC/slab discharge statically (compiler-verified), SpinMutex dynamically (a runtime
//! witness leaving a compiler-invisible lock-ordering obligation); "same bucket as memory-aliasing" holds only with
//! the caveat that that bucket ALSO spans static (split-borrow) and dynamic (`RefCell`: runtime, panic-on-violation);
//! (3) discharged-vs-false is **RustBelt's semantic type soundness** (POPL 2018) applied to `Mutex` — cite it, state
//! what "seam" adds beyond semantic typing.
//!
//! **META — the tic recurred INSIDE the salvage meant to prove I'd fixed it.** Same "omit a conjunct" shape as #35
//! (deleted the exclusion obligation) and #33 (omitted `checked_div`) — but here on the *repair* of a concurrency
//! seam, I omitted the guard's `Sync` bound. Calibration is NOT converging on the concurrency domain: the
//! `unsafe impl`'s full obligation set (exclusion + guard-`Sync`) only got mapped under the gauntlet. Tally 9→10 (a
//! DISCOVERED break — the 9th discovered, distinct from #35's 1 conceded folklore); N stays 9; rejection-mode count
//! STAYS 8 (a break, not a sound-seam rejection). The #35→#36 pair does NOT settle — the salvage chain **extends to
//! #37** (the correctly-bounded `SpinGuard<T: Sync>` + `!Send`, folding in the three tradeoffs). Lean
//! `spin_mutex_guard_sync_factors` (a `..._factors` — it breaks; the classification-survives-the-break lives in the
//! prose).
//!
//! NOTE: the code below is preserved **as-attacked** (the guard is deliberately left with its unsound auto-`Sync`),
//! a museum exhibit like `sync_cell` #35; the fix is the #37 salvage, not an in-place patch.
//!
//! ### CF1 — the DISCHARGED bound is CONDITIONAL: the floor rejects a mutex over a non-`Send` payload:
//! ```compile_fail
//! # use seam_spsc::spin_mutex::SpinMutex;
//! use std::rc::Rc;
//! fn assert_sync<T: Sync>() {}
//! assert_sync::<SpinMutex<Rc<i32>>>();  // ERROR: `Rc<i32>: !Send` ⇒ `SpinMutex<Rc<i32>>: !Sync`
//! // Unlike SyncCell's UNCONDITIONAL `unsafe impl<T> Sync`, this one is bounded `T: Send` — so a non-Send
//! // interior cannot be smuggled across threads through the mutex. The bound is #35's missing conjunct.
//! ```
//!
//! ### The SOUND witness — safe code shares `&SpinMutex` across threads and mutates race-free (compiles; `no_run`):
//! ```no_run
//! # use seam_spsc::spin_mutex::SpinMutex;
//! // The exact shape that was a DATA RACE (UB) for SyncCell #35 — here it is data-race-FREE, no consumer `unsafe`.
//! let m = SpinMutex::new(0i32);
//! std::thread::scope(|s| {
//!     s.spawn(|| { *m.lock() += 1; });   // each thread locks (mutual exclusion), so the two
//!     s.spawn(|| { *m.lock() += 1; });   // non-atomic writes are SERIALIZED, not raced.
//! });
//! assert_eq!(*m.lock(), 2);
//! ```

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A minimal spinlock mutex: an [`AtomicBool`] flag guarding an [`UnsafeCell`]. Sharing a `&SpinMutex` across threads
/// is sound because [`SpinMutex::lock`] hands out access only through a [`SpinGuard`], and the flag guarantees at
/// most one guard is live at a time. The `unsafe impl`s below are the concentrated `unsafe`; the consumer touches
/// none. Contrast `sync_cell::SyncCell` (#35), whose `unsafe impl Sync` was **unconditional and false**; this one is
/// **bounded `T: Send` and discharged** by the mutual-exclusion invariant.
pub struct SpinMutex<T> {
    /// `false` = unlocked, `true` = held. The single sound cross-thread cell (`core` atomic).
    locked: AtomicBool,
    /// The protected value. Reached only through a [`SpinGuard`], i.e. only while `locked == true` for this thread.
    data: UnsafeCell<T>,
}

impl<T> SpinMutex<T> {
    /// Wrap a value in an unlocked mutex.
    pub const fn new(v: T) -> Self {
        SpinMutex { locked: AtomicBool::new(false), data: UnsafeCell::new(v) }
    }

    /// Acquire the lock, spinning until it is free, and return a [`SpinGuard`]. `Acquire` on success pairs with the
    /// releasing `Release` in [`SpinGuard`]'s `Drop` to publish a happens-before edge between critical sections.
    pub fn lock(&self) -> SpinGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Spin read-only (relaxed) until the flag looks free, then retry the CAS — the standard test-and-CAS
            // spin that keeps the exclusive CAS off the bus while contended.
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        SpinGuard { mutex: self }
    }

    /// Try to acquire without spinning: `Some(guard)` if the lock was free, `None` if already held. Demonstrates the
    /// mutual-exclusion invariant directly (a second `try_lock` while a guard is live returns `None`).
    pub fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
        if self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinGuard { mutex: self })
        } else {
            None
        }
    }
}

/// An RAII lock guard: while it is live, this thread holds the flag, so it has **exclusive** access to the cell. The
/// guard is the seam's checked interface — it vends `&T`/`&mut T` with no consumer `unsafe`, and releases on `Drop`.
///
/// ⚠️ **THE r43 BREAK LIVES HERE (preserved as-attacked).** This guard has **no manual `Sync` impl**, so it
/// auto-derives `SpinGuard: Sync ⟺ SpinMutex<T>: Sync ⟺ T: Send` — too weak. Sharing one `&guard` across threads
/// then races `&T` for a `Send + !Sync` `T` (e.g. `Cell<i32>`), UB from safe code. The #37 fix is `std`'s bound:
/// `unsafe impl<T: Sync> Sync for SpinGuard` (+ `!Send`). See the module VERDICT.
pub struct SpinGuard<'a, T> {
    mutex: &'a SpinMutex<T>,
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: the guard's existence is the runtime witness that this thread holds the lock (won the CAS, not yet
        // dropped), so no other thread has a live reference into `data`. Exclusive shared access is DISCHARGED by the
        // mutual-exclusion invariant — this is exactly the obligation SyncCell #35 deleted.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: as `deref`, plus `&mut self` on the guard ⇒ this is the unique guard reference, so the `&mut T` is
        // unique. The lock invariant supplies cross-thread exclusion; the borrow checker supplies intra-thread.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        // Release the flag; `Release` pairs with the next acquirer's `Acquire` to publish this critical section's
        // writes. NOTE (the liveness residue): if the guard is `mem::forget`-ten, this never runs and the lock is
        // held forever — a DEADLOCK (liveness), not UB. That is the honest horn-3 cost, not a soundness break.
        self.mutex.locked.store(false, Ordering::Release);
    }
}

// THE DISCHARGED RE-IMPORT. Same shape as SyncCell #35's unconditional `unsafe impl Sync` — but **bounded `T: Send`** and
// **earned**: the obligation (no concurrent non-exclusive access) is MET by the lock flag, not deleted. `Sync`
// requires `T: Send` because the guard lets a `T` be touched from whichever thread holds the lock (the value
// effectively migrates), so a non-`Send` `T` must not be shareable — CF1 confirms the floor enforces this.
unsafe impl<T: Send> Sync for SpinMutex<T> {}
unsafe impl<T: Send> Send for SpinMutex<T> {}
