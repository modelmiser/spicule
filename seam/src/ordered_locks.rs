//! # Fifteenth seam — type-level lock ordering (a *partially-matched* deadlock-freedom seam — it BREAKS).
//!
//! *(Project ordinal: the fifteenth primitive, and the **first** in a third soundness domain —
//! **liveness / progress**. Seams 1–12 live in memory safety; 13–14 in information flow, where **both
//! broke** because Rust's affine floor carries none of the IFC precondition. This one tested the
//! **positive** half of the floor-domain match: does a *closing* seam reappear in a new domain once the
//! floor carries **that** domain's precondition? Deadlock-freedom is the clean test — a deadlocked
//! program is perfectly memory-safe and leaks nothing, so this is a genuinely distinct domain — and its
//! precondition looks like a partial order, which Rust's trait resolver carries natively. The answer came
//! back **not "yes" but "partly"**: the seam **breaks** (see the verdict section below), and the break
//! grades the floor-domain match into **full / partial / none**. The construction is preserved as the
//! attacked artifact; read the module-doc top-down as the *hypothesis*, then the verdict section for what
//! the cold panel actually found.)*
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
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_ordered_locks, r21): the seam BREAKS — and the break
//! ## grades the floor-domain match.
//!
//! The hypothesis this module was built to test — "liveness/deadlock-freedom *closes* on a
//! type-level-order floor, the **positive** instance of the floor-domain match" — is **false**, and *how*
//! it fails is the deepest result of the phase. The gauntlet ruled **SEAM-BREAKS [horn 1]** (FAILED,
//! 3/5 attackers broke it, one fatal), the **third** seam to drop a horn — and it breaks along *two*
//! independent dimensions, both forms of "the floor under-carries the precondition":
//!
//! 1. **Fatal soundness break — wrong *granularity* (per-token, not per-thread).** The discipline enforces
//!    descent *within one `Floor` token lineage*. But [`Floor::root`] is an ordinary safe call, hence
//!    **re-entrant**: two sibling `root` calls mint two independent `Floor<TopRank>` tokens, and safe code
//!    acquires a *low* rank under one and a *high* rank under the other — a lock-order **inversion** in
//!    which every `L: Below<F>` obligation still discharges (it compiles; see the demonstration below).
//!    The §2 precondition is *per-thread-global* (the whole held set is a chain); affinity on a *value*
//!    (the token) gives only a *per-token* fact. Rust's floor has **no ambient per-thread one-shot
//!    resource** to mint `root` from, so even the *static* fragment does not close without bolting on a
//!    thread-local consumed capability the language does not provide. The "inversion is unconstructible"
//!    claim (§1/§4) is **false as written**.
//! 2. **Partial floor-match — wrong *coverage* (static, not dynamic).** Even *granting* a `root` fix, what
//!    closes is **lock-order-*inversion* freedom**, **not** deadlock-freedom: the canonical deadlock-free
//!    idioms — bank-transfer `min(i,j)` ordering, hand-over-hand list/tree coupling, address-ordered
//!    bucket locks, IRQ/cross-stack ordering — are **inexpressible** (same static rank + irreflexive
//!    `Below` + compile-time-only order). The **dynamic-acyclicity** coordinate (what `lockdep` checks at
//!    runtime) **escapes the static floor** *exactly as IFC's quantitative coordinate escaped the affine
//!    floor*. Prior art forces this: it is **Boyapati–Lee–Rinard (OOPSLA'02) minus their dynamic
//!    tree-order extension**, and **Kobayashi's natural-number levels** — both abandoned pure static
//!    levels *because* the data-dependent fragment forced them off the wall.
//!
//! **So the floor-domain match is GRADED, not binary.** Memory safety on the affine floor: a **full**
//! match (basis closed). Information flow on the affine floor: **no** match (both seams break). Liveness
//! on the static-type-order floor: a **partial** match — the static, distinct-rank, single-stack fragment
//! is carried; the per-thread (granularity) and dynamic (coverage) fragments escape. A floor can fail to
//! carry a domain's precondition along (at least) two axes — *granularity* and *static-vs-dynamic* — and
//! this seam fails on both.
//!
//! Corrections the panel recorded: (a) the right name is **lock-order-inversion freedom**, never
//! "deadlock-freedom" (condvar/channel/join cycles remain — BLR over-broadened the name too); (b) the
//! claim that Rust's *affine* floor is what enables closure is **overstated** — the order-carrying floor
//! predates affinity by two decades (BLR/Java, Kobayashi/π-calculus, neither affine); affinity buys only
//! token non-duplication, and per break 1 it does not even buy *that*. One panel sub-claim is itself
//! refuted by the compiler: the judge asserts CF1/CF2 surface as **E0277**; standalone `rustc` emits
//! **E0308** (the recursive `Below` impls shape `F` from the obligation *before* unifying the floor
//! argument), so the doctest labels above stand — the panel adjudicates the *verdict*, but a checkable
//! diagnostic claim is settled by `rustc`, not by source-level reasoning.
//!
//! ### The break, demonstrated — this COMPILES and runs, though it types an inversion:
//! ```
//! use seam_spsc::ordered_locks::{OrderedLock, Floor, L2, L5};
//! let low = OrderedLock::<u32, L2>::new(1);
//! let high = OrderedLock::<u32, L5>::new(2);
//! Floor::root(|mut top_a| {                          // lineage A
//!     Floor::root(|mut top_b| {                      // lineage B — a SECOND independent top token
//!         low.with(&mut top_a, |_l, _fl| {           // hold rank 2 (low) under lineage A
//!             high.with(&mut top_b, |_h, _fh| {})    // acquire rank 5 (HIGH) under lineage B — INVERSION
//!         })                                          // every `L: Below<TopRank>` discharges; it compiles.
//!     })                                              // two symmetric threads would deadlock here.
//! });
//! ```
//! (Single-threaded it merely runs — two different locks, no self-deadlock — but the *type* accepted a
//! non-decreasing held set, which is the hole. A real fix needs `root` to consume a thread-local one-shot
//! capability, or locks branded to a root lineage; neither is on Rust's ambient floor.)
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
/// Public so it can appear in `where` clauses; it cannot be abused — but **not for the reason this
/// doc used to give** (corrected 2026-09-04). It said a consumer impl "is only legal for a *local*
/// type (orphan rule)". That is false: `impl Below<MyLocal> for L2 {}` compiles downstream, with a
/// foreign `Self` and a local type parameter (RFC 2451).
///
/// What actually closes it is twofold. The dangerous direction — `impl Below<L2> for L5 {}`, which
/// would launder a rank inversion — is all-foreign and rejected by E0117. And a `Below<Rogue>` impl
/// for a local `Rogue` is inert because [`with`](OrderedLock::with) additionally requires the sealed [`Level`], and
/// because `Floor<F>` has a private field and is only ever minted at `TopRank` or at a sealed `L`.
/// The private-field half is the load-bearing part and was never stated.
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
        // Release via RAII, not a straight-line store. `body` is arbitrary caller code and may panic;
        // a bare store after the call is skipped by the unwind, stranding `flag` at `true` forever and
        // hanging every later acquirer in a hot spin loop. The three sibling lock seams (`spin_lock`,
        // `spin_mutex`, `lock_registry`) all release from `Drop` and so survive unwinding; this one did
        // not until 2026-09-04. See `tests/ordered_locks_use.rs::panic_in_body_releases_the_lock`.
        struct Release<'f>(&'f AtomicBool);
        impl Drop for Release<'_> {
            #[inline]
            fn drop(&mut self) {
                self.0.store(false, Ordering::Release);
            }
        }
        let _release = Release(&self.flag);

        // SAFETY: we hold `flag` exclusively for the whole of `body` — the guard above releases it on
        // every exit path, normal or unwinding — so no other reference to `data` exists for the
        // duration. The `&mut` we hand out cannot outlive the borrow of `self` (its lifetime is tied to
        // `&self`), so it cannot escape the critical section.
        body(unsafe { &mut *self.data.get() }, &mut inner)
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
