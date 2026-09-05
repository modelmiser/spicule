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

#[test]
fn panic_in_body_releases_the_lock() {
    // REGRESSION 2026-09-04. `with` used to release the flag with a straight-line store AFTER calling
    // `body`. `body` is arbitrary caller code; if it panicked, the unwind skipped the store and the flag
    // stayed `true` forever, hanging every later acquirer in a hot spin loop. This module was the only
    // one of the four lock seams whose release was not RAII — `spin_lock`, `spin_mutex` and
    // `lock_registry` all release from `Drop` and so always survived unwinding.
    //
    // Without the fix this test HANGS rather than failing, which is the honest shape of the defect:
    // a stranded spinlock has no timeout.
    let lock = OrderedLock::<u32, L4>::new(7);

    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Floor::root(|mut top| {
            lock.with(&mut top, |_v, _f| {
                panic!("boom inside the critical section");
            })
        })
    }));
    assert!(panicked.is_err(), "the panic must propagate out of `with`");

    // The lock must be acquirable again. Before the fix this never returns.
    let seen = Floor::root(|mut top| lock.with(&mut top, |v, _f| *v));
    assert_eq!(seen, 7, "lock was released despite the panic, and the value is intact");
}

#[test]
fn panic_in_a_nested_body_releases_both_locks() {
    // The same hazard one level down: an inner panic must unwind through two live critical sections
    // and leave both flags clear.
    let outer = OrderedLock::<u32, L6>::new(1);
    let inner = OrderedLock::<u32, L4>::new(2);

    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Floor::root(|mut top| {
            outer.with(&mut top, |_o, f_outer| {
                inner.with(f_outer, |_i, _f| panic!("boom, nested"))
            })
        })
    }));
    assert!(r.is_err());

    let both = Floor::root(|mut top| {
        outer.with(&mut top, |o, f| inner.with(f, |i, _| *o + *i))
    });
    assert_eq!(both, 3, "both locks released on the unwind path");
}
