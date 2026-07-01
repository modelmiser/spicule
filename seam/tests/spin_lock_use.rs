//! Consumer-side proof for the thirty-seventh seam (`spin_lock`): the salvage of #36.
//!
//! The negative half — that a guard over a `Send + !Sync` payload is NOT `Sync` (so the r43 laundering path no
//! longer compiles) — is the `compile_fail` doctest CF1 in the module. These tests exhibit the positive half: the
//! correctly-bounded guard still vends race-free shared mutation for a `Sync` payload, and the mutex composes with
//! no consumer `unsafe`. This is the sound, value-adding, `core`-only concurrency seam the #35→#36→#37 chain
//! settles on.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::spin_lock::{SpinLock, SpinLockGuard};

/// Single-threaded core: `lock()` vends `&mut` through the guard, released on drop.
#[test]
fn lock_vends_mutable_access_and_releases_on_drop() {
    let m = SpinLock::new(10i32);
    {
        let mut g = m.lock();
        *g += 5;
        assert_eq!(*g, 15);
    } // guard dropped ⇒ flag released
    assert_eq!(*m.lock(), 15);
}

/// The mutual-exclusion invariant, observed directly: while a guard is live, `try_lock` returns `None`.
#[test]
fn at_most_one_guard_is_live_at_a_time() {
    let m = SpinLock::new(0i32);
    let g = m.lock();
    assert!(m.try_lock().is_none(), "a second acquisition must fail while the first guard is live");
    drop(g);
    assert!(m.try_lock().is_some(), "after release, acquisition succeeds again");
}

/// The sound witness: sharing `&SpinLock` across threads and mutating through `lock()` is data-race-free (the writes
/// are serialized). The final value is exactly the increment count — no lost updates, zero consumer `unsafe`.
#[test]
fn shared_across_threads_is_race_free() {
    let m = SpinLock::new(0i32);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..1000 {
                    *m.lock() += 1;
                }
            });
        }
    });
    assert_eq!(*m.lock(), 8 * 1000, "mutual exclusion ⇒ no lost updates");
}

/// The r43 FIX, stated at the type level (positive half; the negative half is CF1). Obligation 3: the guard is
/// `Sync` when `T: Sync` — so a `Sync` payload's guard IS shareable (this is what makes cross-thread `&guard` use
/// sound), while a `Send + !Sync` payload's guard is NOT (CF1). Obligation 4: the guard is `Send` when `T: Send`
/// (sound for a spinlock). And the mutex itself is `Sync`/`Send` when `T: Send` (obligations 1 & 2, unchanged).
#[test]
fn guard_and_lock_have_the_correct_bounds() {
    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}
    // Obligation 3 (the fix), positive half: `i32: Sync` ⇒ its guard is `Sync`.
    assert_sync::<SpinLockGuard<'static, i32>>();
    // Obligation 4: `i32: Send` ⇒ its guard is `Send` (sound for a spinlock, unlike `std::MutexGuard`).
    assert_send::<SpinLockGuard<'static, i32>>();
    // Obligations 1 & 2 (unchanged from #36): the lock is `Sync`/`Send` over a `Send` payload.
    assert_sync::<SpinLock<i32>>();
    assert_send::<SpinLock<i32>>();
    // A composite `Sync` payload the bare `core` atomics cannot hold at all — the value-add:
    assert_sync::<SpinLock<[i64; 4]>>();
    assert_sync::<SpinLockGuard<'static, [i64; 4]>>();
}
