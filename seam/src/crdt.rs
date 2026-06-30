//! # Twenty-first seam — a state-based CRDT merge: the FOURTH-domain (convergence) probe.
//!
//! *(Project ordinal: the twenty-first primitive, and the **first probe deliberately aimed at a fourth
//! soundness domain**. The arc has mapped three: **memory safety** (full floor-domain match — many seams
//! close), **information flow** (mismatch — no seam closes, the thirteenth/fourteenth seams), and
//! **liveness/progress** (partial/graded — the fifteenth/sixteenth). This one asks whether the floor-domain-
//! match thesis is genuinely domain-general by testing a fourth: **distributed/replicated convergence**
//! (eventual consistency). The danger is **replica divergence** — two replicas that have observed the same
//! set of updates disagree — which is neither UB, nor a leak, nor a deadlock, nor a timing channel: a new
//! failure mode with its own basis.)*
//!
//! Same floor, same rules — and, as with the constant-time `secret` seam, there is **no `unsafe`** at all:
//! the danger is not memory corruption but a **semantic** property the type system does not enforce.
//!
//! ## The primitive — a replicated value merged by a join
//!
//! A [`Replica<T>`] holds a `T` and merges incoming replica state with [`Replica::merge`], which delegates to
//! `T`'s [`JoinSemilattice::join`]. Convergence — *every* replica that has merged the same set of updates,
//! in any order, with any grouping, any number of times, reaches the **same** state — holds **iff** `join`
//! is the least-upper-bound of a join-semilattice: **commutative, associative, and idempotent**. Those three
//! laws are the **convergence precondition**, and they are the object under test.
//!
//! ## Why the floor is suspected to MISMATCH this domain — the laws are unenforceable
//!
//! Rust's floor carries the **type** of a merge: [`Replica::merge`] requires `T: JoinSemilattice` and only
//! merges a `Replica<T>` with another `Replica<T>` (a `Replica<GCounter<3>>` cannot be merged with a
//! `Replica<GCounter<4>>` — see the doctests). But the floor has **no term for commutativity / associativity
//! / idempotence**: a *safe* `impl JoinSemilattice` whose `join` overwrites (non-commutative) or accumulates
//! (non-idempotent) **type-checks**, and two replicas using it **diverge** — a consistency violation written
//! entirely in safe code. This is the exact shape that made information-flow mismatch (the floor enforces the
//! *syntactic proxy* — privacy, the trait bound — not the *semantic* property — non-interference, the
//! semilattice laws). The hypothesis the spec (`design_seam/SEAM_SPEC_crdt.md`) puts to the panel:
//! convergence is a **fourth domain whose precondition Rust's affine/ownership floor does not carry**, so it
//! has **no closing seam** — **SEAM-BREAKS [floor-mismatch]**, the information-flow result in a new domain.
//!
//! The blessed [`GCounter`] (a grow-only counter, `join` = element-wise `max`) is a *correct* CRDT the
//! foundation author could provide — which raises the deliverable's sharper question: is the convergence
//! domain **floor-mismatched** (no closing seam, because safe code supplies the `join`), or **TCB-axiom-
//! costed** (it *holds* if a blessed library of verified CRDTs discharges the laws once, trusting them like
//! `mmio`'s hardware axiom)? And is convergence a genuine fourth domain, or does it **deflate** to the
//! application-correctness invariant the nineteenth seam (`restore`) already located?
//!
//! ### The floor carries the TYPE of a merge — these are compile errors:
//! ```compile_fail
//! # use seam_spsc::crdt::{Replica, GCounter};
//! let mut a = Replica::new(GCounter::<3>::new());
//! let b = Replica::new(GCounter::<4>::new());
//! a.merge(&b); // ERROR: mismatched const generic — a GCounter<3> replica cannot merge a GCounter<4>
//! ```
//! ```compile_fail
//! # use seam_spsc::crdt::Replica;
//! let mut a = Replica::new(0i32);
//! let b = Replica::new(0i32);
//! a.merge(&b); // ERROR: `i32: JoinSemilattice` is not satisfied — merge requires the trait
//! ```
//! ```compile_fail
//! # use seam_spsc::crdt::{Replica, GCounter};
//! let a = Replica::new(GCounter::<3>::new());
//! let _n: u64 = a.into_state(); // ERROR: no such method — `Replica` does not vend its raw state by value
//! ```
//!
//! ### The blessed `GCounter` converges — element-wise `max` is a genuine join (it compiles and runs):
//! ```
//! # use seam_spsc::crdt::{Replica, GCounter};
//! let mut a = Replica::new(GCounter::<2>::new());
//! let mut b = Replica::new(GCounter::<2>::new());
//! a.state_mut().increment(0); // replica 0 counts an event
//! b.state_mut().increment(1); // replica 1 counts an event
//! a.merge(&b);
//! b.merge(&a);
//! assert_eq!(a.get().value(), 2);
//! assert_eq!(b.get().value(), 2); // both replicas converge to the same total
//! ```
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_crdt): _pending — filled in after the gauntlet._
//!
//! The sound CRDT *does* converge, and a non-conforming `join` *does* diverge — see `tests/crdt_use.rs`.

/// A **join-semilattice**: `join` merges `other` into `self` as a least-upper-bound. For a [`Replica`] to
/// converge, `join` MUST be **commutative**, **associative**, and **idempotent** — the convergence laws.
/// **The seam cannot enforce them**: a safe impl that violates any law type-checks and diverges replicas.
/// (That unenforceability is the whole experiment — see the module docs.)
pub trait JoinSemilattice {
    /// Merge `other` into `self`. Must be the semilattice join (commutative / associative / idempotent).
    fn join(&mut self, other: &Self);
}

/// A replicated value: a local `T` merged with remote replica state via [`Replica::merge`]. Convergence
/// holds iff `T`'s [`JoinSemilattice`] obeys the laws.
#[derive(Clone)]
pub struct Replica<T> {
    state: T,
}

impl<T> Replica<T> {
    /// Create a replica seeded with `initial`.
    pub fn new(initial: T) -> Self {
        Replica { state: initial }
    }

    /// Read the converged state.
    pub fn get(&self) -> &T {
        &self.state
    }

    /// Mutate the local state (apply a local update). Convergence is a property of `merge`, not of local
    /// updates, so this is unrestricted.
    pub fn state_mut(&mut self) -> &mut T {
        &mut self.state
    }
}

impl<T: JoinSemilattice> Replica<T> {
    /// Merge `incoming` replica state into this one via the semilattice join. Convergent iff `join` obeys
    /// the laws — which the floor does not check.
    pub fn merge(&mut self, incoming: &Replica<T>) {
        self.state.join(&incoming.state);
    }
}

/// A grow-only counter (the canonical state-based CRDT): one monotone per-replica tally in `counts[i]`,
/// merged by element-wise `max` (a genuine join — commutative, associative, idempotent). The total
/// [`GCounter::value`] is the sum. This is a *correct* CRDT the foundation author can bless; the seam's
/// question is whether safe code can be *trusted* to supply such a `join`, or whether the laws must be a
/// TCB axiom.
#[derive(Clone)]
pub struct GCounter<const N: usize> {
    counts: [u64; N],
}

impl<const N: usize> GCounter<N> {
    /// A fresh counter, all tallies zero.
    pub fn new() -> Self {
        GCounter { counts: [0; N] }
    }

    /// Replica `id` observes one event (a monotone local increment).
    pub fn increment(&mut self, id: usize) {
        self.counts[id] += 1;
    }

    /// The total count across all replicas.
    pub fn value(&self) -> u64 {
        self.counts.iter().copied().sum()
    }
}

impl<const N: usize> Default for GCounter<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> JoinSemilattice for GCounter<N> {
    /// Element-wise `max` — the join on the product lattice of monotone counters. Commutative
    /// (`max(a,b) == max(b,a)`), associative, and idempotent (`max(a,a) == a`), so replicas converge.
    fn join(&mut self, other: &Self) {
        for i in 0..N {
            if other.counts[i] > self.counts[i] {
                self.counts[i] = other.counts[i];
            }
        }
    }
}
