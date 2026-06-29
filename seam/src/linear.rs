//! # Seventeenth seam — a must-run cleanup guard: the LINEARITY probe.
//!
//! *(Project ordinal: the seventeenth primitive. Built **breadth-wise** rather than depth-wise: where
//! the sixteenth (`lock_registry`) climbed the *granularity ladder* within one domain, this one takes a
//! new **precondition** — "this finalizer MUST run" — and asks the cold panel to **grade** Rust's
//! ambient **affine** floor against it. It is the *inverse* of the tenth seam (`drain`): `drain`
//! **closed** by making a skipped `Drop` *harmless* (leak-**tolerance**); this seam needs a skipped
//! `Drop` to be *impossible* (leak-**intolerance**) — the same axis-9 danger from the opposite side.)*
//!
//! ## The primitive — a RAII scope-exit guard whose cleanup is the point
//!
//! [`MustRun`] arms a one-shot closure that runs when the guard is dropped: the canonical
//! commit-or-rollback / release-the-resource / undo-on-failure idiom. Its **re-import promise** is the
//! load-bearing claim under attack: *"the cleanup runs when this scope ends."*
//!
//! ## Why the floor is suspected to under-carry it — affine ≠ linear
//!
//! "The cleanup runs exactly once" is a **linearity** requirement: the guard must be *consumed* (used),
//! not merely *droppable*. Rust's ownership floor is **affine** — *use at most once* — so it lets you
//! **drop or `mem::forget` a value without using it**. `Drop` is therefore **not guaranteed to run**:
//! `core::mem::forget`, `ManuallyDrop`, `Box::leak`, and reference cycles all skip it, and **every one
//! is a safe operation**. This is the canonical *leakpocalypse* (RFC 1066; the removal of
//! `thread::scoped`/`JoinGuard`, RFC 1084). So a seam promising "this finalizer WILL run" appears
//! unenforceable on the affine floor — and the open question is the exact **grade**.
//!
//! ## What the floor DOES carry (the partial fragment — and it is a *lint*, not a type rule)
//!
//! The affine floor catches the *commonest accidental* skip — a guard created and dropped at statement
//! position — but only through the **`#[must_use]` lint**, escalatable to an error with
//! `#![deny(unused_must_use)]`. That is a real fragment, but it is advisory (a *lint*, silenced by
//! `let _ =`), and it does **not** fire for `mem::forget`/`ManuallyDrop`/leak (those *use* the value).
//! The doctests below pin both halves precisely.
//!
//! ### The fragment the floor carries — caught only when the lint is escalated to deny:
//! ```compile_fail
//! #![deny(unused_must_use)]
//! # use seam_spsc::linear::MustRun;
//! MustRun::new(|| {}); // ERROR (denied): unused `MustRun` that must be used — the accidental skip
//! ```
//!
//! ### The fragment it does NOT carry — `mem::forget` is safe, *uses* the value (no lint), skips Drop:
//! ```
//! #![deny(unused_must_use)]
//! # use seam_spsc::linear::MustRun;
//! use core::cell::Cell;
//! let ran = Cell::new(false);
//! let guard = MustRun::new(|| ran.set(true));
//! core::mem::forget(guard);          // SAFE — and the cleanup is now unreachable
//! assert_eq!(ran.get(), false);      // the "must-run" cleanup did NOT run
//! ```
//! (It compiles, the lint is silent, and the guarantee is gone — even with warnings denied.)
//!
//! ## The deliverable the spec poses (verdict PENDING the cold panel — `SEAM_SPEC_linear.md`)
//!
//! The break (a safe `forget` skips the finalizer) is **compiler-settled**, not in dispute. What the
//! cold gauntlet adjudicates is the **interpretation**, and the spec poses it as four rulings:
//! 1. **Grade.** Is the affine floor a *none* match for must-run-cleanup, or a *partial* one — and if
//!    partial, is the `#[must_use]` lint "the floor carrying a fragment" or "not the floor at all"
//!    (a non-type, advisory layer)? This would extend Phase N/O's full/partial/none grading with a
//!    new texture: *partial-at-the-lint-level, none-at-the-type-level*.
//! 2. **Unit of the match.** Deadlock-freedom (#15/#16) is *also* a liveness precondition and graded
//!    *partial*. Is must-run-cleanup the **same** precondition getting the same grade, or does the
//!    liveness domain host **multiple preconditions of different grades** — sharpening the unit of the
//!    floor-match from *domain* to *precondition*?
//! 3. **Axis bookkeeping.** This is **not** a new axis: it is the *enforcement* direction of the
//!    ninth axis (leak-freedom), where `drain` populated the *tolerance* direction. Does the panel
//!    agree the off-diagonal (drain closes leak-tolerant / this breaks leak-intolerant) is the
//!    contribution, with **N unchanged**?
//! 4. **Verdict mode.** Does this land as `floor-mismatch / graded-match` (the Phase-N mode), or is
//!    linearity a *substructural type-theory* gap that is **out-of-frame** in a new way (the affine
//!    floor is the wrong *structural logic*, not merely the wrong *domain*)?
//!
//! And the sound use *does* run — see `tests/linear_use.rs` (normal exit, early return, and
//! panic-unwind all run the cleanup; only `forget`/`ManuallyDrop` skip it).

/// A scope-exit guard that runs a one-shot `cleanup` closure when it is dropped — the RAII
/// "this finalizer runs when the scope ends" idiom (transaction commit/rollback, resource release,
/// undo-on-failure). The `#[must_use]` carries the *one* fragment the affine floor can enforce: an
/// accidental statement-position drop is linted (and, under `deny`, errored). Everything else —
/// `mem::forget`, `ManuallyDrop`, leaking into a cycle — skips the cleanup, safely.
#[must_use = "a MustRun guard does nothing useful if it is forgotten or leaked; bind it so it drops \
              at end of scope, or call `run_now()` to run the cleanup explicitly"]
pub struct MustRun<F: FnOnce()> {
    /// `Some` until the cleanup has been taken (by `Drop` or `run_now`). The `Option` makes the
    /// one-shot discipline structural: the closure is run at most once, whichever path consumes it.
    cleanup: Option<F>,
}

impl<F: FnOnce()> MustRun<F> {
    /// Arm the guard. The cleanup runs when the returned guard is dropped — *if* it is dropped.
    #[inline]
    pub fn new(cleanup: F) -> Self {
        MustRun { cleanup: Some(cleanup) }
    }

    /// The *recovered* linear discipline: consume the guard and run the cleanup now, explicitly.
    /// This is the sound use — but the type system cannot **force** a caller to reach it instead of
    /// `mem::forget`-ing or leaking the guard. That gap (an offered-but-uncompellable use) is the seam.
    #[inline]
    pub fn run_now(mut self) {
        if let Some(f) = self.cleanup.take() {
            f();
        }
        // `self` now drops with `cleanup == None`: the closure runs at most once.
    }
}

impl<F: FnOnce()> Drop for MustRun<F> {
    #[inline]
    fn drop(&mut self) {
        if let Some(f) = self.cleanup.take() {
            f();
        }
    }
}
