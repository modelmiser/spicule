//! Consumer-side proof for the thirty-fifth seam (`sync_cell`): the data-race / concurrency probe.
//!
//! The break itself — that the seam's unsound `Sync` lets safe code share `&SyncCell` across `std::thread::scope`
//! and race a non-atomic write — is the `no_run` doctest in the module (it needs `std`, which this `no_std` crate's
//! integration harness cannot spawn; the point is that it *compiles*, i.e. the type system *accepts* the racing
//! share). CF1 (also in the module) shows the floor correctly rejecting a shared `Cell`. These tests exhibit the
//! single-threaded surface and the type-level soundness hole: `SyncCell<i32>` is (unsoundly) `Sync`/`Send`, whereas
//! the floor's `Cell<i32>` is neither.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::sync_cell::SyncCell;

/// The single-threaded surface works like an ordinary cell — this is the safe API the seam vends.
#[test]
fn single_threaded_get_set_works() {
    let c = SyncCell::new(10i32);
    assert_eq!(c.get(), 10);
    c.set(42);
    assert_eq!(c.get(), 42);
}

/// THE TYPE-LEVEL SOUNDNESS HOLE: the seam's `unsafe impl` makes `SyncCell<i32>` both `Sync` and `Send` — the
/// unsound grant that CF1's `Cell<i32>` (correctly `!Sync`) is denied. This function only type-checks because the
/// grant exists; it is the compile-time witness of the break (the runtime data race needs a spawned thread — see
/// the module `no_run` doctest).
#[test]
fn synccell_is_unsoundly_sync_and_send() {
    fn assert_sync<T: Sync>() {}
    fn assert_send<T: Send>() {}
    assert_sync::<SyncCell<i32>>(); // compiles ONLY because of the seam's unsound `unsafe impl Sync`
    assert_send::<SyncCell<i32>>();
}

/// Contrast made concrete: a genuinely thread-safe cross-thread integer needs an ATOMIC (`AtomicI32: Sync` by the
/// floor, soundly). The floor already vends the sound version; the seam's contribution over it is only the *unsound*
/// grant — i.e. nothing that adds value, the finding. (This test just asserts the floor's atomic is `Sync`.)
#[test]
fn the_floor_already_vends_the_sound_version() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<core::sync::atomic::AtomicI32>(); // sound cross-thread mutable integer — from the floor, no seam
}
