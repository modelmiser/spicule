//! # Fifteenth seam — type-level lock ordering (a *closing* deadlock-freedom seam).
//!
//! *(Project ordinal: the fifteenth primitive, and the **first** in a third soundness domain —
//! **liveness / progress**. Seams 1–12 live in memory safety; 13–14 in information flow, where **both
//! broke** because Rust's affine floor carries none of the IFC precondition. This one tests the
//! **positive** half of the floor-domain match: does a *closing* seam reappear in a new domain once the
//! floor carries **that** domain's precondition? Deadlock-freedom is the clean test — a deadlocked
//! program is perfectly memory-safe and leaks nothing, so this is a genuinely distinct domain — and its
//! precondition is **exactly a partial order**, which Rust's trait resolver carries natively.)*
//!
//! ## The danger — a liveness hazard, not UB
//!
//! Two threads that take the same two locks in **opposite orders** deadlock: thread 1 holds `A` waiting
//! for `B`, thread 2 holds `B` waiting for `A` — a cyclic wait, forever. Nothing is memory-unsafe; no
//! byte is corrupted; the program simply never makes progress. The classic discipline that prevents it
//! is a **global lock order**: rank every lock, and require each thread to acquire ranks in strictly
//! decreasing order. No cycle can form, because a cycle needs some thread to acquire a *higher* rank
//! while holding a *lower* one. Lock hierarchies are folklore (Java `j.u.c`, the Linux kernel's runtime
//! `lockdep`, Boyapati/Lee/Rinard OOPSLA'02 ownership-types-against-deadlock). The question here is
//! whether the discipline can be made **type-structural and compile-time** on Rust's floor — i.e.
//! whether the liveness domain has a seam that *closes*.
//!
//! ## The witness — a type-level rank and an affine "floor" capability
//!
//! Each [`OrderedLock<T, L>`] carries a **type-level rank** `L` (a marker in a sealed tower
//! [`Bottom`] / [`Up`]). A scoped capability [`Floor<F>`] records *the lowest rank currently held*; you
//! begin holding nothing at the top via [`Floor::root`]. Acquiring a lock is [`OrderedLock::with`],
//! whose bound is the entire discipline:
//!
//! ```text
//! fn with<F, R>(&self, floor: &mut Floor<F>, body: impl FnOnce(&mut T, &mut Floor<L>) -> R) -> R
//! where L: Below<F>          // <-- this lock's rank is STRICTLY below everything held
//! ```
//!
//! Two facts make the held set a strictly-decreasing chain — i.e. acyclic — *by construction*:
//! 1. **`L: Below<F>`** forbids acquiring a rank ≥ the current floor (no inversion, no same-rank
//!    co-hold — `Below` is irreflexive).
//! 2. The floor is taken **`&mut`** for the closure's duration, so the outer floor is *unusable* inside
//!    the body; the only capability the body holds is the fresh `&mut Floor<L>` at the new, lower rank.
//!    Nested acquisitions therefore march strictly downward, and stack discipline (closure nesting) is
//!    the release order. An out-of-order acquisition is **unconstructible** — the proof obligation it
//!    would require (`La: Below<Lb>` with `La` outranking `Lb`) has no impl.
//!
//! So a program that *could* deadlock by lock-order inversion **does not type-check**. The consumer
//! writes no lock-order reasoning and no `unsafe`; the type carries the global order.
//!
//! ## What stays trusted (the foundation side)
//!
//! Two things, both concentrated away from the consumer:
//! - **Mutual exclusion** itself — the underlying spinlock ([`AtomicBool`] + [`UnsafeCell`]) is the one
//!   `unsafe` core, discharged once here, exactly like SPSC's P1–P3. (A real OS mutex would slot in
//!   unchanged; the spinlock keeps the crate `no_std` and the deadlock danger faithful.)
//! - **The rank *assignment*** — that the integer ranks the author hands the locks form a sound global
//!   order is a *human judgment the type system enforces consistently but does not itself verify*. This
//!   is an **MMIO-shaped TCB axiom**: the floor guarantees "you obeyed the order you declared," not
//!   "the order you declared is the right one."
//!
//! ## The hypothesis under test
//!
//! If this seam **closes** (sound + decidable + concentrated), it is the **positive instance** of the
//! floor-domain match the project's frontier asked for: memory safety closes on the *affine* floor,
//! information flow *fails to close* on it, and liveness/deadlock-freedom closes on a *type-level-order*
//! floor — three domains, each matched (or not) to a different floor. The cost it pays — and whether
//! that cost is a clean new axis, a projection onto an axis already on the map, or a trusted axiom — is
//! the cold panel's to rule. *(Builder's prediction, non-load-bearing: closes for the static
//! lock-order fragment, modulo the rank-assignment axiom, paying API-locality — `Floor` is viral — and a
//! static-order-only expressiveness limit: no data-dependent acquisition, no two same-rank locks
//! co-held. Whether "deadlock-freedom" overclaims a sub-property is itself a likely finding.)*
//!
//! ### CF1 — an out-of-order acquisition (low rank then high) does not type-check (the discipline):
//! ```compile_fail
//! # use seam_spsc::ordered_locks::{OrderedLock, Floor, L2, L5};
//! let low = OrderedLock::<u32, L2>::new(1);
//! let high = OrderedLock::<u32, L5>::new(2);
//! Floor::root(|mut top| {
//!     low.with(&mut top, |_lo, f_low| {     // hold rank 2 (f_low: Floor<L2>)
//!         high.with(f_low, |_hi, _f| {})    // ERROR[E0308]: rank L5 needs floor rank ≥ 6, got Floor<L2>
//!     })
//! });
//! ```
//! The discipline surfaces as a **type mismatch on the floor**, not an "unsatisfied `Below`" message: the
//! `L: Below<F>` bound pins `F`'s shape from the lock's rank `L`, so inference forces `f_low` to be a
//! `Floor<Up^6<_>>` and the supplied `Floor<L2>` conflicts. (Same phenomenon as `typed_nat`'s CF2.)
//!
//! ### CF2 — two locks of the *same* rank cannot be co-held (`Below` is irreflexive):
//! ```compile_fail
//! # use seam_spsc::ordered_locks::{OrderedLock, Floor, L3};
//! let a = OrderedLock::<u32, L3>::new(1);
//! let b = OrderedLock::<u32, L3>::new(2);
//! Floor::root(|mut top| {
//!     a.with(&mut top, |_a, f_a| {          // hold rank 3 (f_a: Floor<L3>)
//!         b.with(f_a, |_b, _f| {})          // ERROR[E0308]: rank L3 needs floor rank ≥ 4, got Floor<L3>
//!     })
//! });
//! ```
//! Equal ranks are not `Below` each other, so a same-rank lock demands a strictly-higher floor than the
//! one a same-rank hold provides — again a floor type mismatch.
//!
//! ### CF3 — a consumer cannot forge a [`Level`] to invent a rank outside the tower (sealed):
//! ```compile_fail
//! # use seam_spsc::ordered_locks::Level;
//! struct Rogue;
//! impl Level for Rogue {}   // ERROR: `Rogue: Sealed` is not satisfied — the tower is closed.
//! ```

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering};

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Bottom {}
    impl<L: super::Level> Sealed for super::Up<L> {}
}

/// The bottom rank of the lock tower (acquired *last*, deepest in the hold chain).
pub struct Bottom;

/// One rank above `L`. The tower `Bottom`, `Up<Bottom>`, `Up<Up<Bottom>>`, … is the rank order;
/// higher in the tower means acquired *earlier*.
pub struct Up<L>(PhantomData<L>);

/// A type-level lock rank. **Sealed** — only [`Bottom`] and [`Up<L>`] implement it, so a consumer
/// cannot introduce a forged rank that sidesteps the [`Below`] order (CF3).
pub trait Level: sealed::Sealed {}
impl Level for Bottom {}
impl<L: Level> Level for Up<L> {}

/// A compile-time proof that `Self` ranks **strictly below** `H` in the tower. Carries no data and
/// erases: a `Self: Below<H>` bound is discharged by trait resolution. The relation is the global lock
/// order; it is **irreflexive** (no `Below<Bottom>` impl, and `Up<L>: Below<Up<H>>` only via `L:
/// Below<H>`), which is exactly why two equal ranks cannot be co-held (CF2).
///
/// Public so it can appear in `where` clauses; it cannot be abused, because a consumer impl is only
/// legal for a *local* type (orphan rule), which can never be the sealed [`Level`] that [`with`] also
/// requires.
pub trait Below<H> {}
// Bottom is below anything that sits above the bottom.
impl<H: Level> Below<Up<H>> for Bottom {}
// Up<L> < Up<H>  iff  L < H. (Nothing is below Bottom — the chain terminates.)
impl<L, H> Below<Up<H>> for Up<L> where L: Below<H> {}

/// A scoped capability recording **the lowest rank currently held** is `F` — so the next lock acquired
/// must rank strictly below `F`. Obtained at the top (holding nothing) from [`Floor::root`], and handed
/// down, one rank at a time, by [`OrderedLock::with`]. Zero-sized; it is a *typestate token*, not data.
pub struct Floor<F>(PhantomData<F>);

impl Floor<TopRank> {
    /// Begin a lock-acquiring scope holding nothing. Hands the body a top [`Floor`] from which any lock
    /// in the tower (ranks below [`TopRank`]) may be acquired. The closure scoping is what makes the
    /// held set obey stack discipline; there is no way to obtain a `Floor` except here and inside
    /// [`OrderedLock::with`].
    #[inline]
    pub fn root<R>(body: impl FnOnce(Floor<TopRank>) -> R) -> R {
        body(Floor(PhantomData))
    }
}

/// A lock whose rank `L` is fixed in its type. The rank places it in the global order; acquiring it via
/// [`OrderedLock::with`] is permitted only while holding a strictly higher floor.
pub struct OrderedLock<T, L> {
    flag: AtomicBool,
    data: UnsafeCell<T>,
    _level: PhantomData<L>,
}

// The lock provides the mutual exclusion that makes `&mut T` sound to hand out; `T: Send` because the
// guarded value crosses threads. This is the foundation's one trusted concurrency claim (like SPSC).
unsafe impl<T: Send, L> Sync for OrderedLock<T, L> {}

impl<T, L: Level> OrderedLock<T, L> {
    /// Create a lock at rank `L`, holding `value`.
    #[inline]
    pub const fn new(value: T) -> Self {
        OrderedLock {
            flag: AtomicBool::new(false),
            data: UnsafeCell::new(value),
            _level: PhantomData,
        }
    }

    /// Acquire under the ordering discipline. The bound `L: Below<F>` is the whole seam: this lock's
    /// rank must be strictly below the lowest rank currently held (`F`). The body receives exclusive
    /// `&mut T` for the critical section and a fresh `&mut Floor<L>` from which to acquire still-lower
    /// locks. Because the caller's `floor` is borrowed `&mut` for the duration, the held set marches
    /// strictly downward — acyclic by construction — and the lock is released (the spin flag cleared)
    /// when the body returns.
    #[inline]
    pub fn with<F, R>(&self, floor: &mut Floor<F>, body: impl FnOnce(&mut T, &mut Floor<L>) -> R) -> R
    where
        L: Below<F>,
    {
        // The floor token is consumed-by-borrow for the scope: its job is purely to witness `L < F`.
        let _ = floor;
        // Acquire the spinlock (trusted mutual exclusion — the foundation's `unsafe` core).
        while self.flag.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        let mut inner = Floor(PhantomData);
        // SAFETY: we hold `flag` exclusively until the Release store below, so no other reference to
        // `data` exists for the duration of `body`. The `&mut` we hand out cannot outlive the borrow of
        // `self` (its lifetime is tied to `&self`), so it cannot escape the critical section.
        let r = body(unsafe { &mut *self.data.get() }, &mut inner);
        self.flag.store(false, Ordering::Release);
        r
    }
}

/// The rank handed to [`Floor::root`] — above every lock rank an ordinary program uses, so any lock in
/// the tower can be the first acquired. Eight ranks ([`L0`]–[`L7`]) sit below it; that bound is
/// arbitrary and only keeps the readable aliases finite.
pub type TopRank = Up<Up<Up<Up<Up<Up<Up<Up<Up<Bottom>>>>>>>>>;

/// Readable rank aliases (`L0` lowest). Higher number = higher rank = acquired earlier.
pub type L0 = Bottom;
/// Rank 1.
pub type L1 = Up<L0>;
/// Rank 2.
pub type L2 = Up<L1>;
/// Rank 3.
pub type L3 = Up<L2>;
/// Rank 4.
pub type L4 = Up<L3>;
/// Rank 5.
pub type L5 = Up<L4>;
/// Rank 6.
pub type L6 = Up<L5>;
/// Rank 7.
pub type L7 = Up<L6>;
