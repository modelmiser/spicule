//! # Sixteenth seam — a runtime lock-order registry (lockdep-in-miniature): the FLOOR-UPGRADE probe (BREAKS).
//!
//! *(Project ordinal: the sixteenth primitive, and the **second** in the liveness/progress domain. The
//! fifteenth — `ordered_locks`, a *static* type-level order — **broke**, and the break **graded** the
//! floor-domain match: it failed on **granularity** (a re-entrant `Floor::root` mints a second per-token
//! lineage, but the precondition is per-thread-global) and **coverage** (the *dynamic* fragment —
//! runtime-ordered / same-rank locks — escapes a *static* floor). This seam asks the falsifiable question
//! the graded thesis implies: **does *upgrading the floor* lift the match — partial → full — and at what
//! price?** Same domain, same danger (deadlock by lock-order inversion); a different, stronger floor.)*
//!
//! ## The floor upgrade — a thread-local registry of runtime ranks
//!
//! Replace the static rank `L` and the affine `Floor<F>` token with a **runtime rank** (`u64`) per lock
//! and a **`thread_local!` held-stack** of the ranks a thread currently holds. [`OrderedMutex::lock`]
//! checks, *at run time*, that this lock's rank is **strictly greater than every rank held** (acquire in
//! strictly-increasing rank order); on violation it **panics** (the faithful `lockdep` / `RefCell`
//! modality — a lock-order bug is trapped loudly, not silently deadlocked). The guard registers the rank
//! on acquire and removes it (by value — order-independent, never panicking in a destructor) on drop.
//!
//! This fixes the static seam's two gaps directly:
//! - **Granularity → per-thread.** `thread_local!` gives **exactly one** held-stack per thread,
//!   *un-mintable twice*: there is no `root` to re-enter, so the fifteenth seam's fatal nested-lineage
//!   inversion **has no analog here**. The discipline is now genuinely per-thread-global.
//! - **Coverage → dynamic.** Ranks are *runtime values*, so two locks of the *same kind* take *different*
//!   ranks (e.g. `accounts[i].rank = i`), and the canonical deadlock-free idioms the static floor could
//!   not express become expressible: **bank-transfer `min(i,j)`-before-`max(i,j)`** ordering and
//!   **hand-over-hand** coupling (ranks increasing along the structure).
//!
//! ## The price — why "full" is bought, not free (the coupling to the cost-basis)
//!
//! Lifting the match spends three things, two of them coordinates the *memory-safety* basis already
//! names, the third a new dimension:
//! 1. **Bare-metal (#5).** A `thread_local!` needs OS thread-local storage, so this seam **cannot be
//!    `no_std`** — it is gated behind the `lockdep` cargo feature, which is *why* the gate exists: to make
//!    the bare-metal cost visible in the build graph. (Seams 1–6 and 15 are `no_std`; this one is not.)
//! 2. **Zero-cost (#6).** Every acquire pays a runtime check (a thread-local borrow + a compare) and a
//!    push/pop — a runtime witness that never erases, the *slab's* place in a new domain.
//! 3. **Modality — *prevention* → *detection*.** The static seam made an inversion **unconstructible**
//!    (a compile error). This seam makes it **constructible but trapped**: the inversion *type-checks and
//!    compiles*, and is caught only when that path **executes**. So soundness is now **path-coverage
//!    dependent** — an inversion on an un-run branch ships undetected (the exact caveat real `lockdep`
//!    carries: it finds only orderings some test actually exercised). This is not a memory-safety axis at
//!    all; it is a *guarantee modality*, and it is the real cost of reaching the dynamic fragment.
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_lock_registry, r22): the seam BREAKS — the floor-upgrade
//! ## is NON-MONOTONE; granularity RECURS one rung finer.
//!
//! > **ADDENDUM 2026-08-06 — a SECOND break, never adjudicated.** Outside the gauntlet, a boundary
//! > audit found this seam also carried the r43 conjunct: `Guard` had no `Sync` impl, so it
//! > auto-derived through its `&OrderedMutex<T>` field to the too-weak `T: Send`, making
//! > `Guard<'_, Cell<i32>>` `Sync` — safe code could share `&guard` across threads and race a
//! > non-atomic write. That is a **horn-1 memory-safety** break, a different domain and cause from
//! > the liveness/granularity break the panel ruled on below. Fixed the same day (`unsafe impl<T:
//! > Sync> Sync for Guard`, plus CF1 and a bounds test). It is deliberately **not** folded into the
//! > eleven-seam census: no cold panel ever ruled on it, and re-coding cold output warm is the
//! > failure mode §5.5 is about. It IS a measured false negative of the gauntlet, and it is why
//! > PAPER.md §5.5 now reads "held **for that lineage only**" — `spin_lock` #37 enumerated all four
//! > obligations and holds, but the enumeration did not generalize to this module, one over.
//!
//! The hypothesis — "upgrading the floor lifts the match toward full" — is **false**, and *how* it fails
//! is the result. The gauntlet ruled **SEAM-BREAKS [horn 1]** (FAILED), the **fourth** seam to drop a
//! horn. The floor-upgrade did lift **coverage** (runtime-*chosen-but-fixed* ranks close — bank-transfer
//! and hand-over-hand work, confirmed) and it *did* spend the predicted price (#5 bare-metal, #6 runtime
//! check, prevention→detection modality). **But it did not fix granularity — it *relocated* it one rung
//! finer, and broke.**
//!
//! The held-set is keyed on the **OS thread**, but the unit of sequential control in the dominant `std`
//! model is the **async task**, which the language has no primitive for. Two breaks follow (the work-
//! stealing one is fatal; both verified, see below and the 2-thread reproduction in the test notes):
//! 1. **Undetected inversion under work-stealing (horn 1).** A guard held across `.await` migrates to a
//!    new thread whose `HELD` is empty, so a *lower* rank is accepted with **no panic** — an executed,
//!    untrapped, non-increasing co-hold (and `HELD` on the old thread keeps a stale rank forever). The
//!    registry is **thread-blind**: a rank held on thread A is invisible to thread B (empirically
//!    confirmed — rank 3 acquires freely while rank 5 is held on another thread).
//! 2. **False panic on a single-threaded executor.** Task A holds rank 5 across `.await`; the executor
//!    polls unrelated Task B, which acquires rank 3 — a *trivially monotone* sequence — yet the registry
//!    panics, because it conflated two logical threads of control sharing one OS thread. `!Send` on the
//!    guard blocks (1) but leaves (2); you cannot close both directions.
//!
//! **So the "granularity" axis of the floor-domain match is a CONTAINMENT-SCOPE LADDER** — *token <
//! thread < task < …* — and each ambient Rust floor reaches exactly one rung: the affine token (#15, too
//! fine), thread-local (#16, matches per-thread but too coarse for tasks). Rust has **no task-local**
//! primitive (async is a library), so no ambient floor carries the per-control-unit precondition.
//! **Upgrading the floor is therefore non-monotone:** #16 fixed #15's *coverage* gap but its granularity
//! fix only walked one rung up the ladder and exposed the next. The match stays **partial**.
//!
//! Two further costs the panel recorded (real, subordinate to the break): **coverage is still only partial**
//! — runtime-*varying* orders (rotating leader, rebalancing B-tree latch-coupling) cannot be a fixed
//! `u64` and over-reject to the raw spinlock; and **prior art forecloses the design** — this is FreeBSD
//! **WITNESS** / lockdep lock-leveling, and WITNESS/lockdep/Abseil all **abandoned static ranks** for
//! dynamically-*learned* acquires-before graphs, because hand-assigned global ranks do not compose (the
//! single-thread held-set is also strictly less complete — it cannot catch cross-thread A→B / B→A
//! non-interleaving inversions a graph checker catches). The production answer is an even higher floor (a
//! learned runtime graph + cycle detection) — which fixes coverage but lives *entirely* in the detection
//! modality (path-coverage-limited soundness).
//!
//! So the deepened finding: **a partial floor-match is not lifted to full by upgrading the floor.** The
//! upgrade trades one gap for a finer one (coverage fixed, granularity recurs token→thread→task) and
//! spends cost-basis axes (#5, #6) plus a modality downgrade doing it. The graded match and the closed
//! cost-basis are coupled, *and* the granularity dimension is itself an unbounded ladder no ambient floor
//! tops out.
//!
//! ### The lifted *coverage* — this COMPILES and runs (the dynamic ordering the STATIC #15 could not type):
//! ```
//! # use seam_spsc::lock_registry::OrderedMutex;
//! let accounts = [OrderedMutex::new(100u64, 0), OrderedMutex::new(50, 1), OrderedMutex::new(0, 2)];
//! // Transfer 30 from account 0 to account 2: lock min(0,2)=0 first, then max=2 — strictly increasing.
//! let (lo, hi) = (0usize, 2usize);            // a RUNTIME-chosen order
//! let mut a = accounts[lo].lock();            // rank 0
//! let mut b = accounts[hi].lock();            // rank 2 > 0 — OK, two same-kind locks co-held in order
//! *a -= 30; *b += 30;
//! assert_eq!((*a, *b), (70, 30));
//! ```
//! (Coverage lifts — but the async granularity break above means the seam still does not *close*.)
//!
//! ```
//! # // The dynamic ordering the STATIC fifteenth seam could not express — bank transfer by index order.
//! # use seam_spsc::lock_registry::OrderedMutex;
//! let accounts = [OrderedMutex::new(100u64, 0), OrderedMutex::new(50, 1), OrderedMutex::new(0, 2)];
//! // Transfer 30 from account 2 to account 0: lock min(0,2)=0 first, then max=2 — strictly increasing.
//! let (lo, hi) = (0usize, 2usize);            // a RUNTIME-chosen order
//! let mut a = accounts[lo].lock();            // rank 0
//! let mut b = accounts[hi].lock();            // rank 2 > 0 — OK, two same-kind locks co-held in order
//! *a -= 30; *b += 30;
//! assert_eq!((*a, *b), (70, 30));
//! ```
//!
//! ### CF1 — the r43 conjunct, enforced here too: a guard over a `Send + !Sync` `T` is not `Sync`:
//! ```compile_fail
//! # use seam_spsc::lock_registry::Guard;
//! use core::cell::Cell;
//! fn assert_sync<T: Sync>() {}
//! assert_sync::<Guard<Cell<i32>>>();  // ERROR: `Cell<i32>: !Sync` ⇒ `Guard<Cell<i32>>: !Sync`
//! // Before the guard's `Sync` bound was pinned to `T: Sync`, this compiled — the auto-derivation ran
//! // through the `&OrderedMutex<T>` field and yielded the too-weak `T: Send`. Two threads could then
//! // share `&guard`, deref into aliasing `&Cell<i32>`, and race non-atomic writes with no consumer
//! // `unsafe`. That is #36's hole, in a third module; this is #37's fix, applied.
//! ```

extern crate std;

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;
use std::vec::Vec;

std::thread_local! {
    /// The per-thread set of ranks currently held. **Thread-local is the granularity fix**: exactly one
    /// per thread, un-mintable twice — so there is no second lineage to defeat the discipline with.
    static HELD: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// A mutex carrying a **runtime** lock rank. Acquiring it registers the rank on the calling thread's
/// held-stack and checks the global order at run time.
pub struct OrderedMutex<T> {
    rank: u64,
    flag: AtomicBool,
    data: UnsafeCell<T>,
}

// Mutual exclusion makes the `&mut T` sound to hand out; `T: Send` because the value crosses threads.
// (Same trusted concurrency core as SPSC / `ordered_locks`.)
unsafe impl<T: Send> Sync for OrderedMutex<T> {}

impl<T> OrderedMutex<T> {
    /// Create a mutex at runtime `rank`. Locks must be acquired in strictly-increasing rank order.
    #[inline]
    pub const fn new(value: T, rank: u64) -> Self {
        OrderedMutex { rank, flag: AtomicBool::new(false), data: UnsafeCell::new(value) }
    }

    /// Acquire, checking the runtime lock order against this thread's held-stack. **Panics** if this
    /// lock's rank is not strictly greater than every rank currently held — trapping a lock-order
    /// inversion *as a loud failure* on the path that executes it (the `lockdep`/`RefCell` modality),
    /// rather than letting it deadlock. Returns a guard that releases (and unregisters the rank) on drop.
    #[inline]
    pub fn lock(&self) -> Guard<'_, T> {
        HELD.with(|h| {
            if let Some(&top) = h.borrow().iter().max() {
                assert!(
                    self.rank > top,
                    "lock-order violation: acquiring rank {} while holding rank {} \
                     (ranks must be strictly greater than every rank held)",
                    self.rank, top
                );
            }
        });
        while self.flag.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        HELD.with(|h| h.borrow_mut().push(self.rank));
        Guard { lock: self }
    }
}

/// RAII guard: derefs to the guarded value, releases the lock and pops the rank on drop.
pub struct Guard<'a, T> {
    lock: &'a OrderedMutex<T>,
}

impl<T> Drop for Guard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // ORDER MATTERS. Release the lock FIRST, then do the registry bookkeeping.
        //
        // Until 2026-09-04 this ran in the opposite order, and the comment claimed the destructor could
        // "never panic". It could: `LocalKey::with` panics with `AccessError` if the thread's TLS has
        // already been destroyed, which happens for a `Guard` held in a `thread_local!` — and a panic in
        // a destructor during unwinding aborts the process. Worse, the flag was still `true` at that
        // point, so the lock was also stranded. Releasing first means the mutual-exclusion invariant
        // that discharges `unsafe impl Sync` (:164) is restored no matter what the bookkeeping does.
        self.lock.flag.store(false, Ordering::Release);

        // Remove *this* lock's rank from the held-set by value (ranks held are distinct, since acquisition
        // is strictly increasing), so guards may drop in any order without corrupting the registry.
        // `try_with` rather than `with`: during TLS teardown the registry is already gone, and losing a
        // rank from a set that is about to be destroyed is harmless. `swap_remove` is O(1); the set is tiny.
        let _ = HELD.try_with(|h| {
            let mut held = h.borrow_mut();
            if let Some(pos) = held.iter().position(|&r| r == self.lock.rank) {
                held.swap_remove(pos);
            }
        });
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: we hold the flag exclusively until drop, so no other reference to `data` exists.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: as `deref`, and `&mut self` proves unique access to the guard.
        unsafe { &mut *self.lock.data.get() }
    }
}

// THE r43 CONJUNCT, applied here (obligation 3). Identical in shape to `spin_lock.rs`'s fix and omitted here
// for the same reason it was omitted in #36: the guard's auto-`Sync` is derived through its `&'a OrderedMutex<T>`
// field, which yields the *too-weak* `T: Send`. A shared `&guard` hands `&T` to another thread, so the bound must
// be `T: Sync` — the flag excludes guard *creation*, never guard *sharing*. Without this line
// `Guard<'_, Cell<i32>>` is `Sync` and two threads deref it into aliasing `&Cell<i32>`, racing non-atomic writes
// from `#![forbid(unsafe_code)]` consumer code. (Obligation 4 — `Guard: Send where T: Send` — is left
// auto-derived, as in #37. Note that sending a guard is *thread-blind* w.r.t. the `HELD` registry: the rank pops
// off the receiving thread's stack and the origin thread keeps it forever. That is not new — it is the same
// per-thread-granularity hole this seam's verdict already records, enumerated in the module doc above.)
unsafe impl<T: Sync> Sync for Guard<'_, T> {}
