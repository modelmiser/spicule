//! # Ninth seam — take-and-replace across a possible unwind (the unwind-safety axis).
//!
//! *(Project ordinal: the eighth distinct primitive, and the second eighth-**axis** probe — the first,
//! `typed_nat`, was sound but its new-axis claim was rejected. This one targets a different axis.)*
//!
//! Same floor, same rules. This seam re-exports a different danger: **temporarily moving a value out
//! of a `&mut T` and putting one back**. Between the move-out and the move-back, the slot holds a
//! bitwise *duplicate* of the old value — a `T` that must not be dropped or read. The danger is not
//! aliasing or bounds; it is **time**: if anything observes the slot *during the gap*, you get a
//! double-drop or a read of a logically-moved value (UB). And the one thing that can run arbitrary
//! code during the gap, on the stipulated NLL floor, is **unwinding**: if the user's closure panics,
//! the stack unwinds *through the gap*, and the duplicate in the slot is dropped by the caller — a
//! double free.
//!
//! This is the exact UB of a naive `take_mut`, and it isolates an axis none of the first seven touch:
//! **unwind-safety** (a.k.a. exception safety — Abrahams' basic/strong/nothrow guarantees; Rust's
//! `UnwindSafe`/`catch_unwind`). The NLL floor's vocabulary (affinity, lifetimes, `Send`/`Sync`,
//! decidable bounds) has **no term for "this invariant is transiently broken across a potential
//! unwind."** So a seam over this danger cannot get the floor to carry it; it must pay.
//!
//! ## The witness: an abort-bomb (the sound `take_mut` pattern)
//!
//! [`take`] reads the old value out with `ptr::read` (creating the duplicate), runs the closure, and
//! writes the result back with `ptr::write`. Guarding the gap is a **`Bomb`**: a zero-sized local whose
//! `Drop` panics. If the closure panics, unwinding drops the bomb *first* (it is a live local of
//! `take`, dropped before the unwind reaches the caller) — and a panic *during* unwinding is a
//! **double-panic, which aborts the process**. The abort happens before control returns to the caller,
//! so the caller never drops the duplicate: the broken slot is *never observed*. On the happy path the
//! bomb is `mem::forget`-ten and nothing residual runs.
//!
//! ## The cost that isolates the axis
//!
//! Soundness here is bought with **process abort on a mid-take panic**: a panic in the closure cannot
//! be `catch_unwind`-ed — it takes the whole process down. That is a real, consumer-visible behavioral
//! cost (you lose unwinding/recovery for that window), and it is *the* price of soundly re-importing a
//! transient-invariant-breaking operation on an unwinding floor. It is **not** any of the seven prior
//! axes: the seam is sound, decidable & proof-free, O(1)-concentrated, brand-free, `core`-only, and
//! **zero-cost on the happy path** (the bomb erases) — it aces all seven and pays only here. That makes
//! it the off-diagonal: identical to SPSC on every named axis, split on unwind-safety alone.
//!
//! The alternative designs pay elsewhere, which is what makes "unwind-safety" the right name for the
//! shared obligation rather than any one mechanism: a `T: Default` sentinel (`replace_with`-style)
//! fills the gap with a valid value and stays unwind-recoverable — but pays **expressiveness** (needs a
//! `Default`); a naive move-out/move-back with no guard is **outright unsound** under `panic=unwind`.
//!
//! ### CF1 — `take` does not hand the closure a second `&mut` to the slot (no aliasing the gap):
//! ```compile_fail
//! # use seam_spsc::take::take;
//! let mut s = String::from("hi");
//! take(&mut s, |old| {
//!     let _alias = &mut s; // ERROR: `s` is already mutably borrowed by `take`
//!     (old, ())
//! });
//! ```

use core::mem;
use core::ptr;

/// A guard whose `Drop` panics. Armed across the "gap" in [`take`]; on a closure panic, unwinding
/// drops it and the resulting double-panic aborts the process **before** the broken slot is observed.
/// On the happy path it is `mem::forget`-ten, so its `Drop` never runs and it leaves no residue.
struct Bomb;

impl Drop for Bomb {
    #[inline(never)]
    #[cold]
    fn drop(&mut self) {
        // Reached only while already unwinding from the closure's panic → double-panic → abort.
        // This is the unwind-safety cost made mechanical: a mid-take panic becomes a hard abort.
        panic!("take: closure panicked mid-take; aborting so the broken slot is never observed");
    }
}

/// Temporarily move the value out of `slot`, transform it with `f`, and store the result back,
/// returning `f`'s extra output `R`. Sound across a possible unwind: if `f` panics, the process
/// **aborts** (via the abort-bomb) rather than letting the caller observe — and drop — the duplicated
/// value left in `slot` during the gap. On the happy path there is no runtime residue.
///
/// This is the sound `take_mut::take` pattern, generalized to return a value. The abort-on-panic is the
/// **unwind-safety** cost: the gap cannot be made `catch_unwind`-recoverable without either a sentinel
/// (`T: Default`, paying expressiveness) or leaving the slot observably broken (unsound).
#[inline]
pub fn take<T, R, F>(slot: &mut T, f: F) -> R
where
    F: FnOnce(T) -> (T, R),
{
    let bomb = Bomb;
    // SAFETY: `ptr::read` produces a duplicate `old`; for the duration of `f`, `slot` holds a bitwise
    // copy that must not be dropped or read. The `&mut T` we hold proves no *other* code can touch
    // `slot` (no aliasing — see CF1), so the only hazard is unwinding, and `bomb` converts a panic in
    // `f` into an abort before the caller can drop the duplicate. On success, `ptr::write` restores a
    // valid `T` and the bomb is defused; `slot` is well-formed again for the caller.
    unsafe {
        let old = ptr::read(slot);
        let (new, r) = f(old);
        ptr::write(slot, new);
        mem::forget(bomb);
        r
    }
}
