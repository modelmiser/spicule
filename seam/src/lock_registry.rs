//! # Sixteenth seam — a runtime lock-order registry (lockdep-in-miniature): the FLOOR-UPGRADE probe.
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
//! So the honest hypothesis: upgrading the floor lifts **coverage** and **granularity** (the seam may now
//! *close*, dynamically, like the slab) — but it pays #5 + #6 + a prevention→detection modality shift.
//! **The graded floor-match and the closed cost-basis are coupled:** you do not get a fuller match for
//! free by "picking a better floor"; you buy it with axes the basis already enumerated, in a weaker
//! modality. *(Builder's prediction, non-load-bearing — the cold panel rules whether dynamic-detection
//! "closes" the liveness seam or is a distinct, weaker modality that makes the match "full-but-dynamic"
//! rather than "full-and-static," and whether the thread-local registry is genuinely sound.)*
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
        // Remove *this* lock's rank from the held-set by value (ranks held are distinct, since acquisition
        // is strictly increasing), so guards may drop in any order without corrupting the registry — never
        // panic in a destructor. `swap_remove` is O(1); the held-set is tiny.
        HELD.with(|h| {
            let mut held = h.borrow_mut();
            if let Some(pos) = held.iter().position(|&r| r == self.lock.rank) {
                held.swap_remove(pos);
            }
        });
        self.lock.flag.store(false, Ordering::Release);
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
