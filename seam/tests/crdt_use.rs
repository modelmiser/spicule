//! Twenty-first seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The fourth-domain (convergence) probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • the blessed `GCounter` (element-wise `max`) CONVERGES — merge order, grouping, and re-delivery are all
//!     irrelevant (commutative, associative, idempotent);
//!   • a non-conforming `JoinSemilattice` impl — written entirely in safe code, type-checking fine — DIVERGES
//!     replicas: a consistency violation the floor cannot prevent. Two flavours: a non-idempotent `join`
//!     (double-counts on re-delivery) and a non-commutative `join` (order-dependent final state).
//!
//! No `unsafe`, no UB: divergence is observable as replicas holding *different values* after seeing the same
//! updates — a new failure mode, distinct from memory corruption, leakage, deadlock, and timing.
#![forbid(unsafe_code)]

use seam_spsc::crdt::{GCounter, JoinSemilattice, Replica};

#[test]
fn gcounter_converges_regardless_of_merge_order() {
    // Two replicas each observe a local event, then exchange state. A correct join converges them.
    let mut a = Replica::new(GCounter::<2>::new());
    let mut b = Replica::new(GCounter::<2>::new());
    a.state_mut().increment(0);
    b.state_mut().increment(1);
    a.merge(&b);
    b.merge(&a);
    assert_eq!(a.get().value(), 2);
    assert_eq!(b.get().value(), 2, "element-wise max converges both replicas to the same total");
}

#[test]
fn gcounter_merge_is_idempotent() {
    // Re-delivering the same state must not change the result — the law that makes at-least-once delivery safe.
    let mut a = Replica::new(GCounter::<2>::new());
    let mut b = Replica::new(GCounter::<2>::new());
    a.state_mut().increment(0);
    b.state_mut().increment(1);
    a.merge(&b);
    let once = a.get().value();
    a.merge(&b); // duplicate delivery
    a.merge(&b); // and again
    assert_eq!(a.get().value(), once, "idempotent: re-merging the same state is a no-op");
}

/// A non-idempotent "merge" written in safe code: it ADDS the incoming count instead of taking the max. This
/// type-checks and satisfies the trait — but it double-counts on re-delivery, breaking convergence.
#[derive(Clone)]
struct BadSum(u64);
impl JoinSemilattice for BadSum {
    fn join(&mut self, other: &Self) {
        self.0 += other.0; // NOT idempotent: merging the same state twice counts it twice
    }
}

#[test]
fn the_break_a_non_idempotent_merge_diverges_replicas() {
    // Two replicas see the SAME single update (value 5), but one processes the delivery twice (a redelivery —
    // unavoidable over an at-least-once network). With a correct CRDT they would agree; BadSum diverges them.
    let mut a = Replica::new(BadSum(0));
    let update = Replica::new(BadSum(5));
    a.merge(&update); // a == 5
    let mut b = Replica::new(BadSum(0));
    b.merge(&update); // b == 5
    b.merge(&update); // redelivery of the SAME update → b == 10
    assert_eq!(a.get().0, 5);
    assert_eq!(b.get().0, 10);
    assert_ne!(
        a.get().0,
        b.get().0,
        "non-idempotent join: replicas that saw the same update DIVERGE — the convergence break, in safe code"
    );
}

/// A non-commutative "merge" written in safe code: last-writer-wins by overwrite. Type-checks, satisfies the
/// trait — but the final state depends on merge order, so replicas diverge.
#[derive(Clone)]
struct BadOverwrite(u64);
impl JoinSemilattice for BadOverwrite {
    fn join(&mut self, other: &Self) {
        self.0 = other.0; // NOT commutative: a.join(b) = b, b.join(a) = a
    }
}

#[test]
fn the_break_a_non_commutative_merge_diverges_replicas() {
    // Symmetric situation, opposite merge directions → opposite final states. A join must be order-independent;
    // overwrite is not, so two replicas reconciling the same pair disagree.
    let mut a = Replica::new(BadOverwrite(5));
    let b = Replica::new(BadOverwrite(3));
    a.merge(&b); // a := 3

    let a2 = Replica::new(BadOverwrite(5));
    let mut b2 = Replica::new(BadOverwrite(3));
    b2.merge(&a2); // b2 := 5

    assert_ne!(
        a.get().0,
        b2.get().0,
        "non-commutative join: order decides the outcome — replicas DIVERGE, in safe code"
    );
}
