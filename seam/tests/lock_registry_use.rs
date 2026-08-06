//! Sixteenth seam — consumer side, with `unsafe` forbidden at compile time. (Feature `lockdep` only.)
//!
//! Proves: ordinary safe code acquires multiple locks in a *runtime-chosen* order — the dynamic idioms the
//! static fifteenth seam could NOT express (bank-transfer `min/max`, hand-over-hand) — with no `unsafe`,
//! and a lock-order inversion is *detected at run time* (a panic) on the path that executes it. The price
//! is visible in the build: this file compiles only with `--features lockdep` (the bare-metal #5 cost).
#![cfg(feature = "lockdep")]
#![forbid(unsafe_code)]

use seam_spsc::lock_registry::{Guard, OrderedMutex};

#[test]
fn bank_transfer_same_kind_locks_in_runtime_index_order() {
    // The canonical deadlock-free idiom the STATIC seam (#15) could not type: two locks of the SAME kind,
    // ordered by a RUNTIME key (account index). Here each account gets a distinct runtime rank = index.
    let accounts = [
        OrderedMutex::new(100u64, 0),
        OrderedMutex::new(50, 1),
        OrderedMutex::new(0, 2),
    ];
    // Transfer 30 from account 0 (has 100) to account 2 (has 0). The order is RUNTIME-chosen from the pair
    // {0,2}: acquire min=0 first, then max=2 (strictly increasing rank). Two same-kind locks, co-held.
    let (i, j) = (0usize, 2usize);
    let (lo, hi) = if i < j { (i, j) } else { (j, i) };
    let mut from = accounts[lo].lock(); // account 0, rank 0
    let mut to = accounts[hi].lock(); // account 2, rank 2 > 0 — OK
    *from -= 30;
    *to += 30;
    drop((from, to));
    assert_eq!(*accounts[0].lock(), 70);
    assert_eq!(*accounts[2].lock(), 30);
}

#[test]
fn hand_over_hand_increasing_ranks() {
    // Lock coupling: hold node[k] while acquiring node[k+1], ranks increasing along the list — sound, and
    // expressible here (the static seam's same-rank irreflexivity forbade it).
    let nodes = [
        OrderedMutex::new(1u32, 10),
        OrderedMutex::new(2, 20),
        OrderedMutex::new(3, 30),
    ];
    let g0 = nodes[0].lock();
    let g1 = nodes[1].lock(); // 20 > 10
    let g2 = nodes[2].lock(); // 30 > 20
    assert_eq!(*g0 + *g1 + *g2, 6);
}

#[test]
fn a_lock_order_inversion_is_DETECTED_at_runtime() {
    // The modality: the inversion *compiles* (unlike the static seam, where it was a type error) and is
    // trapped at run time as a panic on the path that executes it.
    let low = OrderedMutex::new(0u8, 1);
    let high = OrderedMutex::new(0u8, 9);

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // silence the expected panic's backtrace
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _h = high.lock(); // hold rank 9
        let _l = low.lock(); // acquire rank 1 < 9 — VIOLATION → panic
    }));
    std::panic::set_hook(prev);
    assert!(result.is_err(), "an out-of-order acquisition must be detected (panic)");
}

#[test]
fn the_soundness_hole_the_registry_is_thread_blind() {
    // The break the cold panel found (r22, horn 1): `HELD` is keyed on the OS THREAD, but the unit of
    // control in async is the TASK. A guard held across `.await` migrates to a new thread whose `HELD` is
    // empty, so a lower rank is accepted with NO panic — an undetected inversion. We reproduce the core
    // blindness deterministically with two threads standing in for one migrated task: a rank held on
    // thread T1 is INVISIBLE to the main thread, so it acquires a lower rank without a panic.
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    let m5 = Arc::new(OrderedMutex::new(0u32, 5));
    let m3 = Arc::new(OrderedMutex::new(0u32, 3));
    let (held_tx, held_rx) = channel();
    let (release_tx, release_rx) = channel();
    let m5c = m5.clone();
    let t1 = std::thread::spawn(move || {
        let _g5 = m5c.lock(); // HELD_T1 = {5}
        held_tx.send(()).unwrap();
        release_rx.recv().unwrap(); // keep rank 5 held
    });
    held_rx.recv().unwrap();
    // "Same task, migrated to the main thread": acquire rank 3 while rank 5 is held elsewhere. The
    // registry does NOT panic — it cannot see T1's held rank. If these holds were one task, this is an
    // undetected lock-order inversion (rank 3 under rank 5). That is the work-stealing soundness hole.
    let g3 = m3.lock(); // NO PANIC — thread-blind
    drop(g3);
    release_tx.send(()).unwrap();
    t1.join().unwrap();
    // The test passes by NOT panicking — which is precisely the bug: the inversion went undetected.
}

#[test]
fn the_registry_resets_per_scope_so_reordering_across_scopes_is_fine() {
    // After all guards drop, the thread's held-stack is empty, so a later, independent critical section may
    // start at any rank — the per-thread registry tracks the *current* held set, not history.
    let a = OrderedMutex::new(1u32, 5);
    let b = OrderedMutex::new(2u32, 3);
    {
        let _ga = a.lock(); // rank 5
    } // released
    {
        let _gb = b.lock(); // rank 3 — fine now; nothing is held
    }
    // Read each in its own scope (NOT co-held — co-holding 5 then 3 would itself be an inversion).
    let va = *a.lock();
    let vb = *b.lock();
    assert_eq!(va + vb, 3);
}

/// Obligation 3 (the r43 conjunct), positive half. The negative half — that a `Send + !Sync` payload's guard is
/// *not* `Sync` — is CF1 in the module doc, since a `compile_fail` cannot live in an integration test.
#[test]
fn guard_and_mutex_have_the_correct_bounds() {
    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}
    // Obligation 3: `i32: Sync` ⇒ its guard is `Sync`.
    assert_sync::<Guard<'static, i32>>();
    // Obligation 4: the guard is `Send` when `T: Send` (auto-derived, as in #37 — but see the module doc on
    // why sending one is thread-blind w.r.t. the `HELD` registry).
    assert_send::<Guard<'static, i32>>();
    // Obligations 1 & 2, unchanged: the mutex is `Sync` over a `Send` payload.
    assert_sync::<OrderedMutex<i32>>();
    // A composite `Sync` payload, matching #37's value-add case.
    assert_sync::<Guard<'static, [i64; 4]>>();
}
