//! Consumer-side proof for the thirty-sixth seam (`spin_mutex`): the salvage of #35.
//!
//! The negative half — that a non-`Send` payload cannot be shared through the mutex — is the `compile_fail` doctest
//! CF1 in the module (`SpinMutex<Rc<i32>>: !Sync`). These tests exhibit the positive half: the discharged
//! `unsafe impl<T: Send> Sync` makes a `&SpinMutex` genuinely shareable and race-free. The exact shape that was UB
//! for `SyncCell` #35 (two threads writing a shared cell) is here **serialized by the lock** — sound, no consumer
//! `unsafe`. This is the sound, value-adding, `core`-only concurrency seam #35 claimed could not exist.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::spin_mutex::SpinMutex;

/// Single-threaded core: `lock()` vends `&mut` through the guard, and the guard releases on drop so the next `lock()`
/// succeeds. No `Result`, no `unsafe`.
#[test]
fn lock_vends_mutable_access_and_releases_on_drop() {
    let m = SpinMutex::new(10i32);
    {
        let mut g = m.lock();
        *g += 5;
        assert_eq!(*g, 15);
    } // guard dropped here ⇒ flag released
    assert_eq!(*m.lock(), 15);
}

/// The mutual-exclusion invariant, observed directly: while a guard is live, `try_lock` returns `None`; once it is
/// dropped, `try_lock` succeeds. This is the runtime witness that DISCHARGES the `unsafe impl Sync` — at most one
/// guard exists at a time, so every `&mut` handed out is exclusive.
#[test]
fn at_most_one_guard_is_live_at_a_time() {
    let m = SpinMutex::new(0i32);
    let g = m.lock();
    assert!(m.try_lock().is_none(), "a second acquisition must fail while the first guard is live");
    drop(g);
    assert!(m.try_lock().is_some(), "after release, acquisition succeeds again");
}

/// THE SALVAGE, demonstrated: the shape that RACED in #35 (`SyncCell`: two threads writing a shared cell) is here
/// **data-race-free**, because `lock()` serializes the writes. The final value is exactly the increment count — no
/// lost updates, no UB — with zero consumer `unsafe`. `SpinMutex` is the concurrency slab the `core` floor (atomics
/// only, no mutex) does not vend directly.
#[test]
fn shared_across_threads_is_race_free() {
    let m = SpinMutex::new(0i32);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..1000 {
                    *m.lock() += 1;
                }
            });
        }
    });
    assert_eq!(*m.lock(), 8 * 1000, "mutual exclusion ⇒ no lost updates ⇒ exactly one increment per iteration");
}

/// The discharged bound is CONDITIONAL (the value-add over #35's unconditional-and-false impl): `SpinMutex<T>` is
/// `Sync`/`Send` exactly when `T: Send`. Here we confirm the *positive* half statically — an `i32` payload makes the
/// mutex shareable — which is what lets the floor vend it as a sound cross-thread cell the way it vends `AtomicI32`,
/// but for an arbitrary `Send` `T` (the value the atomic alone cannot give). The negative half is CF1.
#[test]
fn spin_mutex_over_send_payload_is_sync_and_send() {
    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}
    assert_sync::<SpinMutex<i32>>();
    assert_send::<SpinMutex<i32>>();
    // A composite Send payload the bare `core` atomics cannot hold at all:
    assert_sync::<SpinMutex<[i64; 4]>>();
}
