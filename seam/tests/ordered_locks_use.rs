//! Fifteenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code acquires multiple ranked locks, in a strictly-decreasing order, with no
//! `unsafe` and no lock-order reasoning written by hand — the type-level rank carries the global order.
//! The *danger* (a lock-order inversion that could deadlock across threads) is foreclosed at compile time;
//! the laundering attempts that would express an inversion are the `compile_fail` doctests on the module.

#![forbid(unsafe_code)]

use seam_spsc::ordered_locks::{Floor, OrderedLock, L2, L4, L6};

#[test]
fn nested_acquisition_in_rank_order_type_checks_and_runs() {
    // Three locks at descending ranks. Acquired high → low, the chain is strictly decreasing.
    let high = OrderedLock::<u32, L6>::new(100);
    let mid = OrderedLock::<u32, L4>::new(20);
    let low = OrderedLock::<u32, L2>::new(3);

    let total = Floor::root(|mut top| {
        high.with(&mut top, |h, f_high| {
            mid.with(f_high, |m, f_mid| {
                low.with(f_mid, |l, _f_low| *h + *m + *l)
            })
        })
    });
    assert_eq!(total, 123);
}

#[test]
fn a_single_lock_mutates_under_exclusion() {
    let lock = OrderedLock::<u32, L4>::new(0);
    Floor::root(|mut top| {
        lock.with(&mut top, |v, _f| *v += 41);
    });
    Floor::root(|mut top| {
        lock.with(&mut top, |v, _f| {
            *v += 1;
            assert_eq!(*v, 42);
        });
    });
}

#[test]
fn siblings_at_the_same_rank_are_fine_when_held_one_at_a_time() {
    // Same-rank locks cannot be *co-held* (that is CF2), but acquiring them sequentially — each in its own
    // scope, neither held while the other is taken — is exactly what the discipline permits.
    let a = OrderedLock::<u32, L4>::new(10);
    let b = OrderedLock::<u32, L4>::new(20);

    let x = Floor::root(|mut top| a.with(&mut top, |v, _f| *v));
    let y = Floor::root(|mut top| b.with(&mut top, |v, _f| *v));
    assert_eq!(x + y, 30);
}

#[test]
fn skipping_ranks_is_allowed_only_strictly_below_matters() {
    // The discipline requires *strictly below*, not *adjacent*: holding rank 6 you may jump straight to
    // rank 2, skipping 5/4/3. Acyclicity needs a strict decrease, not a dense one.
    let high = OrderedLock::<u32, L6>::new(7);
    let low = OrderedLock::<u32, L2>::new(5);
    let prod = Floor::root(|mut top| {
        high.with(&mut top, |h, f_high| low.with(f_high, |l, _f| *h * *l))
    });
    assert_eq!(prod, 35);
}
