//! REGRESSION 2026-09-04 — `Guard::drop` must not touch TLS before releasing the lock.
//!
//! `Guard::drop` used to run `HELD.with(...)` (the rank bookkeeping) BEFORE the release store, and its
//! comment claimed the destructor could "never panic". It could: `LocalKey::with` panics with
//! `AccessError` if the thread's thread-local storage has already been destroyed — which is exactly the
//! situation for a `Guard` that itself lives in a `thread_local!`. A panic in a destructor during
//! unwinding aborts the process, and the flag was still `true` at that point, so the lock was stranded
//! as well.
//!
//! Without the fix this test does not fail — it ABORTS the test binary
//! (`fatal runtime error: thread local panicked on drop`), taking the whole suite with it. That is the
//! honest shape of the defect and why it is worth pinning.
//!
//! `--features lockdep` only, like every other consumer of this seam.

#![cfg(feature = "lockdep")]
#![forbid(unsafe_code)]

use seam_spsc::lock_registry::{Guard, OrderedMutex};

thread_local! {
    /// A guard parked in TLS. Its destructor runs during thread teardown, at which point the
    /// registry's own thread_local may already be gone.
    static PARKED: std::cell::RefCell<Option<Guard<'static, u64>>> =
        const { std::cell::RefCell::new(None) };
}

#[test]
fn guard_dropped_during_tls_teardown_does_not_abort() {
    // `Box::leak` gives the 'static lock the TLS slot needs. Safe code; no unsafe in this file.
    let lock: &'static OrderedMutex<u64> = Box::leak(Box::new(OrderedMutex::new(42u64, 0)));

    let handle = std::thread::spawn(move || {
        // REGISTRATION ORDER IS THE WHOLE TEST. TLS destructors run in reverse registration order, so
        // to have the registry (`HELD`) torn down BEFORE the parked guard, `PARKED` must be registered
        // FIRST. Touching it here does that. If instead we called `lock()` first — which registers
        // `HELD` — then `HELD` would be registered first, destroyed last, and the guard's destructor
        // would find it perfectly alive. A version of this test written that way passes against the
        // unfixed code and proves nothing; that is how it was written on the first attempt.
        PARKED.with(|p| *p.borrow_mut() = None);
        let g = lock.lock(); // registers HELD, second — so HELD is destroyed FIRST
        PARKED.with(|p| *p.borrow_mut() = Some(g));
        // Thread ends: HELD is destroyed, then PARKED's guard drops into a dead registry.
    });
    handle.join().expect("thread must exit cleanly, not abort");

    // The lock must be free afterwards: releasing happens before the bookkeeping, so it happens even
    // when the bookkeeping cannot. A stranded flag would hang this line forever.
    let g = lock.lock();
    assert_eq!(*g, 42, "lock reacquired after the parked guard was dropped at teardown");
}
