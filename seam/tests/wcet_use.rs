//! Twenty-second seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The WCET / real-time fourth-domain probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • the static op-count accounting discharges for in-budget straight-line code (the part the floor carries);
//!   • the op-count is NOT real cost — a single `step` charged as one unit does a MILLION units of real work,
//!     so the type-level "deadline met" proof is sound for the MODEL but a fiction for real time.
//!
//! No `unsafe`, no wall-clock timing needed: the break is observable as a counter (real work done) wildly
//! exceeding the type-level cost — a new failure mode (deadline miss), distinct from UB, leakage, deadlock,
//! divergence, and timing side-channels.
#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::typed_nat::{S, Z};
use seam_spsc::wcet::pure;

type N1 = S<Z>;
type N3 = S<S<S<Z>>>;

#[test]
fn straight_line_cost_accounts_and_discharges_in_budget() {
    // Two steps, deadline three: the static cost (2) is within budget (3), so `within` discharges.
    let r = pure(2i32).step(|x| x + 1).step(|x| x * 10).within::<N3>();
    assert_eq!(r, 30, "in-budget straight-line code discharges its deadline");
}

#[test]
fn cost_reflects_the_op_count() {
    let c = pure(()).step(|_| ()).step(|_| ());
    assert_eq!(c.cost(), 2, "the type-level cost is the number of steps");
}

#[test]
fn the_break_op_count_is_not_real_cost() {
    // ONE `step`, charged as ONE unit — but its body does a MILLION units of real work. The type-level cost
    // model cannot see inside the closure, so it accounts `1` and "proves" a one-unit deadline is met.
    let work = Cell::new(0u64);
    let c = pure(()).step(|_| {
        for _ in 0..1_000_000 {
            work.set(work.get() + 1);
        }
    });
    assert_eq!(c.cost(), 1, "the model charges one unit for the step");
    c.within::<N1>(); // type-checks: it "proves" the computation meets a 1-unit deadline
    assert_eq!(
        work.get(),
        1_000_000,
        "but it did a MILLION units of real work — the op-count is a fiction for real time (the break)"
    );
}

#[test]
fn the_break_a_single_step_can_hide_unbounded_work_relative_to_its_cost() {
    // The same point, sharpened: two computations with the SAME static cost (1) but vastly different real
    // cost. The type system cannot tell them apart — it sees only the op-count, never the elapsed time.
    let cheap_work = Cell::new(0u64);
    let cheap = pure(()).step(|_| cheap_work.set(1));
    let dear_work = Cell::new(0u64);
    let dear = pure(()).step(|_| {
        for _ in 0..500_000 {
            dear_work.set(dear_work.get() + 1);
        }
    });
    assert_eq!(cheap.cost(), dear.cost(), "identical type-level cost…");
    assert_ne!(cheap_work.get(), dear_work.get(), "…but wildly different real work — the model is blind to it");
    cheap.within::<N1>(); // both "meet" the same 1-unit deadline despite the real-cost gap
    dear.within::<N1>();
}
