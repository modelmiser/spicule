//! # Nineteenth seam — a value-owning scope-restore guard: the CONTROLLED PACKAGING probe.
//!
//! *(Project ordinal: the nineteenth primitive. Built not to hunt a new axis or a new domain but to turn
//! Phase Q's headline — "which horn a danger threatens is a function of how the re-import rule **packages**
//! it" — from an **observation** into a **controlled experiment**. Phase Q noticed that #17's must-run gap
//! and #18's completion gap, though the *same* `mem::forget` mechanism, landed on different horns; but they
//! were different primitives, so the move could be incidental. This seam holds the danger **identical to
//! #17** and changes **only** the packaging.)*
//!
//! ## The primitive — the value-owning guard `#17` deliberately did NOT provide
//!
//! [`Restore`] owns a value `T` and a one-shot closure `F: FnOnce(T)`; it `Deref`/`DerefMut`s to the `T`
//! (you *use* the value through the guard), and on `Drop` it runs `restore(value)` — the
//! **restore-the-invariant** finalizer (return a leased resource to its pool, write a saved global back,
//! reset a borrowed flag). [`Restore::into_inner`] is the *sanctioned commit/keep* path: take the value back
//! out **without** restoring (the branch that legitimately keeps it). This is the value-owning
//! `scopeguard::guard(value, f)` + `into_inner` shape that the seventeenth seam ([`crate::linear::MustRun`])
//! **explicitly declined to host** (its `disarm` returns the *closure*, not the protected *value*).
//!
//! ## The SAME danger as #17 — an un-compellable finalizer
//!
//! The restore closure runs on `Drop` — *if* `Drop` runs. Rust's floor is **affine** (use *at most* once),
//! so `core::mem::forget`, `ManuallyDrop`, `Box::leak`, and reference cycles all skip the destructor, and
//! **every one is a safe operation** (the *leakpocalypse*, RFC 1066; the `thread::scoped` removal, RFC
//! 1084). So the restore is *exactly* as un-compellable here as the finalizer was in #17 — bit-for-bit the
//! same gap. Nothing about the danger has changed.
//!
//! ## What the experiment changes — the packaging (with a co-variation the panel flagged, see the verdict)
//!
//! *(The intended single variable is the packaging; but the cold panel ruled the move **confounded** — to
//! package the danger as a side-obligation, #19 also introduces a value-ownership core that #17 lacked, so
//! the value-ownership model co-varies. The honest claim is therefore "a better-controlled probe than the
//! incidental #17-vs-#18 pair," not "a clean single-variable manipulation." See the verdict below.)*
//!
//! In #17 (`MustRun`), the re-import **promise** was *"the finalizer runs"*: the running of the closure
//! **was** the seam's entire soundness claim, so a skipped `Drop` **drops the claim outright** — a horn-1
//! break. Here the re-import promise is split:
//! - **A sound memory-safety CORE the floor carries.** `Restore` *owns* the `T` (affine: `!Copy`/`!Clone`),
//!   vends scoped exclusive access through `DerefMut`, and `into_inner` consumes the guard so the value is
//!   handed out *or* restored but never both. No `unsafe`, no UAF, no double-anything — this part is sound,
//!   decidable, and concentrated regardless of whether the closure ever runs.
//! - **A restore obligation packaged as a side-effect *beside* that core.** Skipping the closure
//!   (`mem::forget`) leaves a **logical invariant** un-restored — a leased resource never returned, a saved
//!   global never written back — which is an *accounting/correctness* fault, **never** undefined behaviour.
//!
//! So the hypothesis the spec (`design_seam/SEAM_SPEC_restore.md`) puts to the cold panel is the controlled
//! claim: **the identical `mem::forget` danger that BROKE #17's horn 1 here only COSTS horn 3** — an
//! undischargeable "don't abandon the guard" obligation that recurs at every `?`/panic/early-return, exactly
//! #18's completion cost — *purely because* the re-import rule packages it as a side-obligation beside a
//! sound ownership core rather than as the whole soundness promise. The verdict is **predicted**
//! SEAM-COSTS [horn 3] (a hold-with-cost, the #18 shape carrying the #17 danger). The falsifier the panel is
//! invited to find: a way the danger forces a *fixed* horn regardless of packaging (which would bound the
//! thesis), or a demonstration that the probe is trivial (merely *renaming* the must-run obligation
//! non-load-bearing) rather than a genuine controlled move.
//!
//! ### The memory-safety core IS carried — these are compile errors (the part that CLOSES):
//! ```compile_fail
//! # use seam_spsc::restore::Restore;
//! # use core::cell::Cell;
//! let slot = Cell::new(Some(1i32));
//! let v = slot.take().unwrap();
//! let g = Restore::new(v, |v| slot.set(Some(v)));
//! let g2 = g;                  // the guard is affine: this MOVES it
//! let _kept = g.into_inner();  // ERROR: use of moved value `g` — the guard cannot be duplicated
//! # let _ = g2.into_inner();
//! ```
//! ```compile_fail
//! # use seam_spsc::restore::Restore;
//! # use core::cell::Cell;
//! let slot = Cell::new(Some(1i32));
//! let v = slot.take().unwrap();
//! let g = Restore::new(v, |v| slot.set(Some(v)));
//! let _kept = g.into_inner(); // commit: keep the value, do not restore
//! let _ = &*g;               // ERROR: use of moved value `g` — into_inner consumed it
//! ```
//! ```compile_fail
//! # use seam_spsc::restore::Restore;
//! # use core::cell::Cell;
//! let slot = Cell::new(Some(1i32));
//! let v = slot.take().unwrap();
//! let mut g = Restore::new(v, |v| slot.set(Some(v)));
//! let r: &mut i32 = &mut *g;   // borrow the value through the guard
//! let _kept = g.into_inner();  // ERROR: cannot move `g` — it is borrowed by `r` (scoped access is sound)
//! *r += 1;
//! ```
//!
//! ### The restore is NOT compellable — a safe `mem::forget` skips it (it compiles and runs):
//! ```
//! # use seam_spsc::restore::Restore;
//! # use core::cell::Cell;
//! let slot: Cell<Option<i32>> = Cell::new(Some(42));
//! let leased = slot.take().expect("the slot was full");   // lease the value out
//! let g = Restore::new(leased, |v| slot.set(Some(v)));     // would return it to the slot on drop
//! core::mem::forget(g);                                    // SAFE — skip the restore
//! assert_eq!(slot.get(), None);                            // the lease was LOST — invariant violated, but no UB
//! ```
//! (It compiles under `#![forbid(unsafe_code)]`, the value is leaked, and the "every lease is returned"
//! invariant is broken — yet memory safety is intact: nothing is freed twice, nothing dangles.)
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_restore, r26): SEAM-COSTS [horn 3] — it HOLDS with a cost,
//! ## confirming the packaging dial; but the panel **bounded** the thesis three ways, all accepted.
//!
//! The seam **survived** (TRADEOFF, 1/1). Three attackers (ergonomics, soundness, implementation) conceded
//! the **memory-safety core is sound** — no safe double-free/alias/double-consume (`mem::swap` strands a
//! value but yields no UB), checking is trivially decidable, and no nontrivial compiler pass is implied
//! (the seam *concedes* restore-liveness up front rather than asking the compiler to prove the closure runs,
//! so it never hands any pass an undecidable query). The predicted **horn-3 cost is correct**: the
//! identical #17 `Drop`-gap here strands only the restore *invariant* — an undischargeable "don't forget the
//! guard" obligation recurring at every `?`/panic/return — never soundness. **The core packaging claim
//! holds: the same `mem::forget` danger that BROKE #17's horn 1 only COSTS horn 3 here.**
//!
//! But two attackers ruled BREAKS, folded by the judge into the recorded costs below — and they sharpen the
//! result past my prediction:
//! 1. **The move is CONFOUNDED, not a clean single-variable experiment (expressiveness).** "Only the
//!    packaging changed" is false: #17→#19 also added a **load-bearing affine-ownership core**, so the
//!    *value-ownership model* co-varied with the packaging. This is structural, not sloppy — to package a
//!    danger as a *side*-obligation you must introduce a sound core for it to sit *beside*, so "core" and
//!    "packaging" are not independently dialable. The horn-move is **real but co-varied** — a better-
//!    controlled probe than #17-vs-#18's incidental pair, not a single-variable manipulation.
//! 2. **The thesis is BOUNDED, not universal — the horn IS forceable (expressiveness + prior-art).** The
//!    *same* skippable-`Drop` danger in a **borrowed-slot / `take_mut` packaging** forces **horn-1 UB** (a
//!    mid-restore panic double-drops the borrowed slot — the ninth-axis `take` danger), and **`Pin`'s drop
//!    guarantee** and the post-leakpocalypse **`thread::scope` restructure** (the closure shape that *forces*
//!    the join) exhibit horns that **cannot** be repackaged away. So "which horn" depends on **where safety
//!    rests**, which is *partly danger-intrinsic*: packaging moves the horn only **within an envelope** set
//!    by whether soundness can be decoupled from the un-compellable closure. My spec scoped the borrowed-slot
//!    case out (§2/§5), so the forcing point **marks the price** rather than breaking this seam — but it is
//!    the real boundary of the thesis, and the honest answer to the deliverable's "can the horn be forced?"
//!    is **yes, partly**.
//! 3. **Not novel — a confirm-by-restaging, no new verdict mode (prior-art).** This exact dial was turned in
//!    2015: `JoinGuard`/`thread::scoped` broke horn 1, `scopeguard` costs horn 3, and the matter concluded in
//!    **RFC 1066 (the leakpocalypse)**. The contribution is **the method** (turning Phase Q's observation
//!    into a *deliberately* controlled probe) and **locating the dial** on the realizability lattice — not a
//!    new structure. N stays 9 verdict modes; the 5 broken seams are unchanged (this one holds).
//!
//! **The refined finding (the real yield).** The packaging dial is **real but bounded**: *which horn a
//! danger threatens is a design choice within a structural envelope determined by whether soundness can be
//! decoupled from the un-compellable closure.* Decouplable (an **owned** value → the restore is a
//! side-obligation beside a sound core) ⇒ the danger **costs** (horn 3, this seam, `scopeguard`).
//! Not decouplable (a **borrowed** value restored across unwind, `Pin`'s load-bearing address, `thread::
//! scope`'s borrowed frames) ⇒ the danger **forces** horn 1. Phase Q said the horn moves; Phase R says it
//! moves *within a fence*, and names the fence.
//!
//! And the sound use *does* restore on every ordinary path — see `tests/restore_use.rs` (normal exit, early
//! return, and panic-unwind all run the restore; `into_inner` is the sanctioned keep; only `forget`/
//! `ManuallyDrop` skip it).

use core::ops::{Deref, DerefMut};

/// A value-owning scope guard: it owns a `T`, lets you use it through `Deref`/`DerefMut`, and runs a
/// one-shot `restore` closure (`FnOnce(T)`) when it is dropped — the RAII "restore the invariant when the
/// scope ends" idiom (return a leased resource, write a saved global back, reset a borrowed flag). The
/// closure is **not compellable**: `mem::forget`, `ManuallyDrop`, and leaking all skip it (safely). The
/// `#[must_use]` catches the *accidental* statement-position drop, which would restore the value before you
/// ever used it — the one fragment the affine floor can flag.
#[must_use = "a Restore guard restores its value when dropped; bind it for the scope you want to protect, \
              or call `into_inner()` to take the value without restoring"]
pub struct Restore<T, F: FnOnce(T)> {
    /// `Some((value, restore))` until consumed by `Drop` or `into_inner`. The `Option` makes the one-shot
    /// discipline structural: the value is handed out *or* restored, never both, whichever path consumes it.
    cell: Option<(T, F)>,
}

impl<T, F: FnOnce(T)> Restore<T, F> {
    /// Arm the guard over `value` with the `restore` finalizer. The finalizer runs when the returned guard
    /// is dropped — *if* it is dropped.
    #[inline]
    pub fn new(value: T, restore: F) -> Self {
        Restore { cell: Some((value, restore)) }
    }

    /// The *sanctioned commit/keep* path: consume the guard and take the value back out **without** running
    /// the restore. This is the branch that legitimately keeps the resource (the commit, in transaction
    /// terms). It is the value-owning answer the seventeenth seam could not give — and, like that seam's
    /// `disarm`, it is the *intended, greppable* cancel, distinct from the *unintended, silent* `mem::forget`
    /// skip the affine floor cannot prevent.
    #[inline]
    pub fn into_inner(mut self) -> T {
        let (value, _restore) = self.cell.take().expect("Restore holds its value until consumed");
        value
        // `self` now drops with `cell == None`: the restore does not run (the value was committed).
    }
}

impl<T, F: FnOnce(T)> Deref for Restore<T, F> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        // `cell` is `Some` for the whole usable life of the guard (only `Drop`/`into_inner` clear it, and
        // both consume `self`), so this never panics.
        &self.cell.as_ref().expect("Restore holds its value until consumed").0
    }
}

impl<T, F: FnOnce(T)> DerefMut for Restore<T, F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.cell.as_mut().expect("Restore holds its value until consumed").0
    }
}

impl<T, F: FnOnce(T)> Drop for Restore<T, F> {
    #[inline]
    fn drop(&mut self) {
        if let Some((value, restore)) = self.cell.take() {
            restore(value);
        }
    }
}
