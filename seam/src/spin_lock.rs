//! # Thirty-seventh seam — THE CORRECTLY-BOUNDED SPINLOCK: the salvage of #36 (`spin_mutex`).
//!
//! *(Project ordinal: the thirty-seventh primitive, and the **salvage of `spin_mutex` #36** — mirroring #26→#27,
//! #29→#30, #31→#32, #33→#34. #36 was itself the salvage of #35, but it **broke** (r43, tally 9→10): I discharged
//! the LOCK obligation (the CAS witnesses one live guard) but **omitted the GUARD's** — with no manual guard impl,
//! `SpinGuard` auto-derived `Sync ⟺ SpinMutex<T>: Sync ⟺ T: Send`, too weak, so sharing one `&guard` across threads
//! raced the interior of a `Send + !Sync` `T` (`Cell<i32>`). This seam adds the one missing bound. It is the
//! sound, value-adding, `core`-only concurrency seam the #35→#36→#37 chain finally settles on — the concurrency
//! **slab**, proving data-race freedom is a *concentrate-behind-a-seam* domain.)*
//!
//! ## The primitive — identical mechanics to #36; ONE bound added
//!
//! [`SpinLock`] is `spin_mutex::SpinMutex` **with the guard correctly bounded**: an `AtomicBool` flag guarding an
//! [`core::cell::UnsafeCell`]; [`SpinLock::lock`] spins on `compare_exchange` (`Acquire`) and hands back a
//! [`SpinLockGuard`] vending `&T`/`&mut T` (`Deref`/`DerefMut`) and releasing (`Release`) on `Drop`. Safe consumer
//! code composes `new`/`lock`/`*guard` with **no `unsafe`**.
//!
//! ## THE FIX — the complete auto-trait obligation set of a mutex, enumerated (the discipline #36 lacked)
//!
//! A mutex over `UnsafeCell` has **four** `unsafe`-trait obligations; #36 discharged three and dropped the fourth.
//! 1. `SpinLock<T>: Send where T: Send` — auto-derived, sound (moving the whole lock moves the `T`).
//! 2. `SpinLock<T>: Sync where T: Send` — the mutex is shareable when `T` is **sendable**, because `lock()` gives
//!    *exclusive* access, so the `T` only ever **migrates** between threads (never shared `&T`) — hence `T: Send`,
//!    not `T: Sync`. (Correct in #36; matches `std::Mutex<T>: Sync where T: Send`.)
//! 3. **`SpinLockGuard<T>: Sync where T: Sync`** — a *shared* `&guard` derefs to `&T` reachable from many threads,
//!    so the guard is shareable only when `T: Sync`. **This is the bound #36 OMITTED** (its guard auto-derived
//!    `Sync ⟺ T: Send`, admitting `Cell<i32>` and racing it). The explicit `unsafe impl<T: Sync> Sync for
//!    SpinLockGuard` below opts the guard **out** of the too-weak auto-`Sync` and pins it to `T: Sync`.
//! 4. `SpinLockGuard<T>: Send where T: Send` — auto-derived. Moving a guard *migrates* exclusive access, sound for
//!    a spinlock (`spin::MutexGuard` is `Send`). This is **deliberately NOT `!Send`**: `std::MutexGuard` is `!Send`
//!    because an OS futex must be *unlocked on the acquiring thread*, but a spinlock's `Release` store is valid from
//!    any thread — so `!Send` is std-specific conservatism, not a soundness requirement here. (See §5 residue.)
//!
//! With (3) added, the r43 laundering path **fails to compile**: `SpinLockGuard<Cell<i32>>: !Sync` (because
//! `Cell<i32>: !Sync`), so `&guard` is not `Send` and cannot cross a `thread::scope` — CF1 witnesses this. The lock
//! invariant (the CAS) discharges cross-thread *exclusion*; the `T: Sync` bound discharges the guard's *sharing*;
//! together they discharge the `unsafe impl` in full. This is the **discharged** re-import #35 claimed impossible.
//!
//! ## HYPOTHESIS (to be put through the cold gauntlet, REPORT_SEAM_SPEC_spin_lock.md):
//! ## SEAM-COSTS [horn3-concentration] — SOUND (the salvage HOLDS): the correctly-bounded guard closes the r43
//! ## break; both conjuncts (exclusion + guard-`Sync`) are discharged; the honest price is a **liveness residue**.
//!
//! Staked contribution: this **settles the #35→#36→#37 chain** — a sound, value-adding, `core`-only concurrency
//! seam EXISTS (the thing #35 said could not), confirming data-race freedom is **concentrate-behind-a-seam** (locks
//! are its seams; the memory-aliasing bucket; a **discharged** `unsafe impl` categorically distinct from a **false**
//! one). NOT a fourth soundness domain (a data race is still UB). Tally STAYS 10 (this HOLDS); N stays 9.
//!
//! Pre-conceded (the #34/#32 discipline — claim only the carriable half):
//! - **No novel mechanism.** This is `spin::Mutex` / a textbook spinlock; the contribution is the **classification**
//!   + closing the #36 break, not the lock. Discharged-vs-false is **RustBelt's semantic type soundness** (Jung et
//!   al., POPL 2018) applied to `Mutex` — the guard inhabits the logical relation *iff* the `T: Sync` invariant is
//!   provable; the "seam" framing adds only the *floor-relative* reading (what the NLL floor carries vs. concentrates).
//! - **The liveness residue is real and named (§5):** a `mem::forget`-ten guard never releases ⇒ permanent
//!   **deadlock**; a same-thread re-`lock` **self-deadlocks**; in **IRQ/interrupt context** a lock held across an
//!   interrupt that re-enters self-deadlocks (needs an `irqsave` variant); contended, it can suffer **unbounded
//!   priority inversion**; and it **busy-waits** (burns CPU). None is UB — all are liveness/performance (the
//!   #10/#17/#29 leak/completion coordinate on the lock face). Conservation of hardness: the concurrency danger
//!   **relocates** from *data race* to *deadlock/inversion*, and I make the relocation visible rather than hide it.
//! - **The bucket-merge is textured, not flat:** SPSC #1 / slab #4 discharge **statically** (compiler-verified,
//!   total, composable); `SpinLock` discharges **dynamically** (a runtime CAS witness, leaving a compiler-invisible
//!   global lock-ordering obligation — cf. `ordered_locks` #15/#16). "Same bucket as memory-aliasing" holds *with
//!   the caveat* that that bucket itself spans static (split-borrow) and dynamic (`RefCell`: runtime, panic-on-
//!   violation) discharge — `SpinLock` is the dynamic end, the concurrency analog of `RefCell`.
//!
//! ### CF1 — the r43 BREAK is now REJECTED: a guard over a `Send + !Sync` `T` is not `Sync`, so it cannot be shared:
//! ```compile_fail
//! # use seam_spsc::spin_lock::SpinLockGuard;
//! use core::cell::Cell;
//! fn assert_sync<T: Sync>() {}
//! assert_sync::<SpinLockGuard<Cell<i32>>>();  // ERROR: `Cell<i32>: !Sync` ⇒ `SpinLockGuard<Cell<i32>>: !Sync`
//! // This is EXACTLY #36's hole, now closed: the guard is shareable only when `T: Sync`, so the r43 witness
//! // (share `&guard` across `thread::scope`, race `&Cell<i32>`) no longer type-checks.
//! ```
//!
//! ### The SOUND witness — safe code shares `&SpinLock` across threads and mutates race-free (compiles; `no_run`):
//! ```no_run
//! # use seam_spsc::spin_lock::SpinLock;
//! let m = SpinLock::new(0i32);
//! std::thread::scope(|s| {
//!     s.spawn(|| { *m.lock() += 1; });   // mutual exclusion serializes the two non-atomic writes —
//!     s.spawn(|| { *m.lock() += 1; });   // data-race-free, no consumer `unsafe`.
//! });
//! assert_eq!(*m.lock(), 2);
//! ```

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A minimal spinlock mutex — `spin_mutex::SpinMutex` with the guard's `Sync` bound corrected (the r43 fix). An
/// [`AtomicBool`] flag guards an [`UnsafeCell`]; access is vended only through a [`SpinLockGuard`], and the flag
/// guarantees at most one guard is live. The `unsafe impl`s are the concentrated `unsafe`; the consumer touches none.
pub struct SpinLock<T> {
    /// `false` = unlocked, `true` = held. The single sound cross-thread cell (`core` atomic).
    locked: AtomicBool,
    /// The protected value. Reached only through a [`SpinLockGuard`], i.e. only while this thread holds the flag.
    data: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    /// Wrap a value in an unlocked lock.
    pub const fn new(v: T) -> Self {
        SpinLock { locked: AtomicBool::new(false), data: UnsafeCell::new(v) }
    }

    /// Acquire the lock, spinning until free, and return a [`SpinLockGuard`]. `Acquire` on success pairs with the
    /// releasing `Release` in the guard's `Drop` to publish a happens-before edge between critical sections.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Spin read-only (relaxed) until the flag looks free, then retry the exclusive CAS.
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        SpinLockGuard { lock: self }
    }

    /// Try to acquire without spinning: `Some(guard)` if free, `None` if held. Witnesses the mutual-exclusion
    /// invariant directly (a second `try_lock` while a guard is live returns `None`).
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinLockGuard { lock: self })
        } else {
            None
        }
    }
}

/// An RAII lock guard: while it is live, this thread holds the flag, so it has **exclusive** access to the cell. It
/// vends `&T`/`&mut T` with no consumer `unsafe` and releases on `Drop`. Correctly bounded (the r43 fix): it is
/// `Sync` only when `T: Sync` (see the `unsafe impl` below), so it cannot be shared across threads to race a
/// `Send + !Sync` interior — the exact hole `spin_mutex::SpinGuard` #36 left open.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: the guard's existence witnesses that this thread holds the lock, so no other thread has a live
        // reference into `data`. Cross-thread exclusion is discharged by the lock invariant; cross-thread *sharing*
        // of the guard is bounded by the `T: Sync` impl below (the r43 fix).
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: as `deref`, plus `&mut self` ⇒ this is the unique guard reference, so the `&mut T` is unique.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        // Release the flag; `Release` pairs with the next acquirer's `Acquire`. NOTE (the liveness residue, §5): a
        // `mem::forget`-ten guard never runs this ⇒ permanent DEADLOCK (liveness), not UB.
        self.lock.locked.store(false, Ordering::Release);
    }
}

// THE DISCHARGED RE-IMPORT (obligations 1 & 2). Same as #36's SpinMutex — bounded `T: Send`, discharged by the lock
// flag. `Sync` requires `T: Send` because `lock()` gives exclusive access, so the `T` only migrates between threads.
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

// THE r43 FIX (obligation 3). Opt the guard OUT of the too-weak auto-`Sync` (which would be `T: Send`, via the
// `&SpinLock<T>` field) and pin it to `T: Sync`: a shared `&guard` hands `&T` to other threads, sound only when
// `&T` is `Sync`. This is the single bound #36 omitted; with it, the r43 laundering path fails to compile (CF1).
// (Obligation 4 — `SpinLockGuard: Send where T: Send` — is left auto-derived: sound for a spinlock, see the module
// doc's enumeration of why this differs from `std::MutexGuard`'s `!Send`.)
unsafe impl<T: Sync> Sync for SpinLockGuard<'_, T> {}
