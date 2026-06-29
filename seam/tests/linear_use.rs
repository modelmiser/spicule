//! Seventeenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves both halves of the linearity break in SAFE code. The affine floor DOES carry the *ordinary*
//! exits: the cleanup runs on normal fall-through, on early `return`, and even while a panic unwinds.
//! But a SAFE `mem::forget` / `ManuallyDrop` skips it entirely — note this file `#![forbid(unsafe_code)]`
//! and still compiles, because forgetting a value is a *safe* operation. The must-run guarantee is
//! therefore unenforceable on Rust's affine floor (the leakpocalypse).
#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::linear::MustRun;

#[test]
fn cleanup_runs_on_normal_scope_exit() {
    let ran = Cell::new(false);
    {
        let _g = MustRun::new(|| ran.set(true));
    } // dropped here — cleanup runs
    assert!(ran.get(), "the cleanup must run on ordinary scope exit");
}

#[test]
fn cleanup_runs_on_early_return() {
    fn inner(ran: &Cell<bool>) {
        let _g = MustRun::new(|| ran.set(true));
        return; // Drop still runs on the early-return path
    }
    let ran = Cell::new(false);
    inner(&ran);
    assert!(ran.get());
}

#[test]
fn cleanup_runs_while_unwinding_a_panic() {
    // The affine floor DOES carry the unwind path: `Drop` runs during stack unwinding. (Contrast the
    // ninth-axis `take` seam, which was unwind-SENSITIVE; this guard is unwind-safe, leak-sensitive —
    // the off-diagonal codified in Rust as "Exception Safety" vs "Leaking".)
    let ran = Cell::new(false);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _g = MustRun::new(|| ran.set(true));
        panic!("boom");
    }));
    assert!(r.is_err());
    assert!(ran.get(), "Drop runs while unwinding — the floor carries the panic path");
}

#[test]
fn run_now_is_the_recovered_but_unforced_discipline() {
    // The sound use the type OFFERS but cannot COMPEL: explicitly consume-and-run.
    let ran = Cell::new(false);
    let g = MustRun::new(|| ran.set(true));
    g.run_now();
    assert!(ran.get());
}

#[test]
fn disarm_is_the_sanctioned_ownership_handoff_cancel_not_the_attack() {
    // The case `disarm` actually serves (NOT commit-or-rollback — the r24 panel showed it returns the
    // closure, not the protected value): usually run the finalizer on drop, but in this branch the
    // resource was handed elsewhere, so cancel it — explicitly and greppably, *not* by `mem::forget`.
    let released = Cell::new(false);
    let release_on_drop = MustRun::new(|| released.set(true));
    // ... we handed the resource off, so cancel the release:
    let cancelled = release_on_drop.disarm();
    assert!(cancelled.is_some(), "disarm hands back the un-run cleanup closure");
    assert!(!released.get(), "the finalizer did NOT run — an explicit, visible cancel (unlike forget)");
}

#[test]
fn the_break_mem_forget_skips_the_must_run_cleanup() {
    // `mem::forget` is SAFE — this file forbids `unsafe` and still compiles. It consumes the guard and
    // leaks it, so `Drop` never runs and the "must-run" cleanup is skipped. Affine ownership cannot make
    // a destructor mandatory; that would need linear types (use exactly once).
    let ran = Cell::new(false);
    let g = MustRun::new(|| ran.set(true));
    core::mem::forget(g);
    assert!(!ran.get(), "the must-run cleanup did NOT run — the seam's guarantee is unenforceable");
}

#[test]
fn the_break_manually_drop_also_skips_it() {
    use core::mem::ManuallyDrop;
    let ran = Cell::new(false);
    {
        // A `ManuallyDrop` whose wrapper falls out of scope here: its `Drop` is a no-op, so the inner
        // guard's `Drop` never runs. No `unsafe`, no explicit `drop()` — the suppression is structural,
        // a second safe path to the same skip. (`drop(manually_drop)` is itself a deny-by-default lint
        // precisely because it does nothing — the language *documents* that this skips the destructor.)
        let _g = ManuallyDrop::new(MustRun::new(|| ran.set(true)));
    }
    assert!(!ran.get(), "ManuallyDrop suppresses the destructor — also safe, also skips the cleanup");
}
