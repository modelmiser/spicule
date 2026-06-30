//! Nineteenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves the controlled packaging move in SAFE code. The affine floor carries the *memory-safety core*:
//! the value is owned, used through the guard, and the restore runs on every *ordinary* exit — normal
//! fall-through, early `return`, and even while a panic unwinds. `into_inner` is the sanctioned keep. But a
//! SAFE `mem::forget` / `ManuallyDrop` skips the restore — note this file `#![forbid(unsafe_code)]` and
//! still compiles. The skip is the *same* un-compellable-finalizer gap that broke #17; here it leaves a
//! logical invariant un-restored (a leased value lost) WITHOUT any undefined behaviour — the horn-3 cost,
//! not the horn-1 break, because memory safety never rested on the closure.
#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::restore::Restore;

#[test]
fn restore_runs_on_normal_scope_exit() {
    // The lease pattern: take the value out of the slot, and the guard returns it on drop.
    let slot: Cell<Option<i32>> = Cell::new(Some(7));
    {
        let v = slot.take().unwrap();
        let _g = Restore::new(v, |v| slot.set(Some(v)));
    } // dropped here — restore runs, the lease is returned
    assert_eq!(slot.get(), Some(7), "the restore must run on ordinary scope exit");
}

#[test]
fn restore_runs_on_early_return() {
    fn inner(slot: &Cell<Option<i32>>) {
        let v = slot.take().unwrap();
        let _g = Restore::new(v, |v| slot.set(Some(v)));
        return; // Drop still runs on the early-return path
    }
    let slot: Cell<Option<i32>> = Cell::new(Some(9));
    inner(&slot);
    assert_eq!(slot.get(), Some(9), "the restore runs on the early-return path too");
}

#[test]
fn restore_runs_while_unwinding_a_panic() {
    // The affine floor carries the unwind path: `Drop` runs during stack unwinding, so the invariant is
    // restored even when the protected scope panics. (Contrast the ninth-axis `take` seam, unwind-SENSITIVE;
    // this guard is unwind-safe, leak-sensitive — the same off-diagonal as #17.)
    let slot: Cell<Option<i32>> = Cell::new(Some(11));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let v = slot.take().unwrap();
        let _g = Restore::new(v, |v| slot.set(Some(v)));
        panic!("boom");
    }));
    assert!(r.is_err());
    assert_eq!(slot.get(), Some(11), "Drop runs while unwinding — the floor carries the panic path");
}

#[test]
fn the_value_is_usable_through_the_guard_then_restored() {
    // The memory-safety core: use the value through DerefMut, and it is still restored on drop.
    let slot: Cell<Option<i32>> = Cell::new(Some(1));
    {
        let v = slot.take().unwrap();
        let mut g = Restore::new(v, |v| slot.set(Some(v)));
        *g += 40; // scoped exclusive access through the guard
        assert_eq!(*g, 41);
    } // restore runs with the mutated value
    assert_eq!(slot.get(), Some(41), "the mutated value is restored to the slot");
}

#[test]
fn into_inner_is_the_sanctioned_keep_not_the_attack() {
    // The commit/keep branch: legitimately take the value back out and do NOT restore. The slot stays empty
    // BY DESIGN (we kept the resource), and this is explicit and greppable — unlike `mem::forget`. This is
    // the value-owning answer #17's `disarm` could not give (it returned the closure, not the value).
    let slot: Cell<Option<i32>> = Cell::new(Some(5));
    let v = slot.take().unwrap();
    let g = Restore::new(v, |v| slot.set(Some(v)));
    let kept = g.into_inner();
    assert_eq!(kept, 5, "into_inner hands back the protected value");
    assert_eq!(slot.get(), None, "the value was kept, not restored — an explicit, intended commit");
}

#[test]
fn the_break_mem_forget_skips_the_restore() {
    // `mem::forget` is SAFE — this file forbids `unsafe` and still compiles. It consumes the guard and leaks
    // it, so `Drop` never runs and the restore is skipped: the lease is lost. This is the SAME gap that
    // broke #17 — but here it costs an invariant, not soundness (no UB, nothing dangles).
    let slot: Cell<Option<i32>> = Cell::new(Some(3));
    let v = slot.take().unwrap();
    let g = Restore::new(v, |v| slot.set(Some(v)));
    core::mem::forget(g);
    assert_eq!(slot.get(), None, "the restore did NOT run — the invariant drifts, but memory is safe");
}

#[test]
fn the_break_manually_drop_also_skips_it() {
    use core::mem::ManuallyDrop;
    let slot: Cell<Option<i32>> = Cell::new(Some(4));
    {
        // A `ManuallyDrop` whose wrapper falls out of scope here: its `Drop` is a no-op, so the inner
        // guard's `Drop` never runs. No `unsafe`, no explicit `drop()` — a second safe path to the skip.
        let v = slot.take().unwrap();
        let _g = ManuallyDrop::new(Restore::new(v, |v| slot.set(Some(v))));
    }
    assert_eq!(slot.get(), None, "ManuallyDrop suppresses the destructor — also safe, also skips the restore");
}
