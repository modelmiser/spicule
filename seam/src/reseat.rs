//! # Twentieth seam — a take-and-reseat guard over a BORROWED slot: forcing the horn (the Phase-R fence).
//!
//! *(Project ordinal: the twentieth primitive. Built as the **controlled counterpart** to the nineteenth
//! (`restore`). Phase R found the packaging dial **real but bounded** — the cold panel ruled the same
//! `mem::forget` danger that broke #17's horn 1 only *costs* horn 3 when packaged as a side-obligation beside
//! a sound **owned**-value core — but flagged the move as **confounded** (#17→#19 co-varied the packaging AND
//! the value-ownership model) and named a **forcing point**: the *same* danger over a **borrowed slot**
//! (`take_mut`) would force horn-1 UB regardless of packaging. This seam runs the clean single-variable
//! experiment that confound forbade: it keeps #19's **packaging identical** — a guard with `Deref`/
//! `DerefMut`/`into_inner`/Drop-restore — and changes **only** the decoupling property, owned value →
//! borrowed slot.)*
//!
//! ## The primitive — `take_mut` as a guard
//!
//! [`Reseat`] borrows `&'a mut T`, reads the value out with `ptr::read` (the foundation's sole `unsafe`),
//! hands it back through `Deref`/`DerefMut` so you mutate it in place, and on `Drop` writes it back with
//! `ptr::write` — the take-and-reseat idiom (`take_mut`/`replace_with`), here in *guard* form. Between the
//! read and the write-back the **slot holds a bitwise duplicate** of the value: a `T` that must not be
//! dropped or read until reseated. [`Reseat::into_inner`] is the #19-matching *commit/keep* path — and the
//! crux of the experiment.
//!
//! ## What the floor DOES carry here — the guard is unwind-safe and forget-safe
//!
//! Unlike the ninth seam's *eager* `take` (`take.rs`, which needed an abort-bomb because a panic in the
//! user closure unwinds *through* the gap), the guard form survives both un-compellable-`Drop` paths:
//! - **Unwind:** `Drop` runs *during* stack unwinding, so a panic in the user's mutation still reseats a
//!   valid `T` into the slot before the caller observes it. **No abort-bomb needed.**
//! - **`mem::forget` / `ManuallyDrop` / leak:** `Drop` is skipped, so `owned` is never written back.
//!   **CORRECTED 2026-09-04 — this was documented as a leak and it is a DOUBLE-FREE.** The claim held
//!   only for a guard whose value was never destructively touched. `DerefMut` hands out `&mut T`, so
//!   safe code can destroy what the slot's duplicate bits name — `*g = new_value` drops the original,
//!   and any reallocating mutation (`String::push_str`) frees the buffer those bits point at. `forget`
//!   then leaves the slot owning a dangling `T`, which its owner drops. Witnessed with no `unsafe` and
//!   no `into_inner` in `tests/reseat_use.rs::forget_after_a_replacing_mutation_double_drops`; the
//!   `String` form is a real heap double-free that glibc does not trap. The true boundary is
//!   `forget_without_a_replacing_mutation_is_only_a_leak`: leak vs double-free is decided by whether
//!   `DerefMut` was used destructively, NOT by `mem::forget`.
//!
//! ### The sound core IS carried — these are compile errors (no aliasing the gap, no escape, affine):
//! ```compile_fail
//! # use seam_spsc::reseat::Reseat;
//! let mut s = String::from("hi");
//! let g = Reseat::new(&mut s);
//! let _alias = &mut s; // ERROR: `s` is already mutably borrowed by the guard — the gap cannot be observed
//! let _ = g.len();
//! ```
//! ```compile_fail
//! # use seam_spsc::reseat::Reseat;
//! fn escape() -> Reseat<'static, i32> {
//!     let mut local = 0i32;
//!     Reseat::new(&mut local) // ERROR: `local` does not live long enough — the guard cannot outlive the slot
//! }
//! ```
//! ```compile_fail
//! # use seam_spsc::reseat::Reseat;
//! let mut n = 1i32;
//! let g = Reseat::new(&mut n);
//! let g2 = g;                  // the guard is affine: this MOVES it
//! let _ = g.into_inner();      // ERROR: use of moved value `g`
//! # let _ = g2.into_inner();
//! ```
//!
//! ### The sound use — mutate in place, and `Drop` reseats the value (it compiles and runs):
//! ```
//! # use seam_spsc::reseat::Reseat;
//! let mut n = 100i32;
//! {
//!     let mut g = Reseat::new(&mut n); // n's value read out; slot holds a duplicate
//!     *g += 23;                        // mutate the owned value through the guard
//! }                                    // Drop reseats 123 into the slot
//! assert_eq!(n, 123);                  // the mutation landed — sound, unwind-safe, no abort-bomb
//! ```
//!
//! ## Where it breaks — `into_inner`, the API that was SOUND for #19
//!
//! The horn-1 break is **isolated to `into_inner`**. For #19's *owned* value, `into_inner(self) -> T` was
//! sound: it handed back the single owner. Over a *borrowed slot* the identical API is a **double-free**: it
//! returns `owned` while the slot — its borrow ending as the guard is consumed — still holds the read
//! duplicate, so **two owners** of one value drop it twice. The packaging is byte-for-byte #19's; only the
//! decoupling property flipped; and the horn flipped with it, cost (3) → break (1). (Demonstrated, double-
//! drop made observable via a drop-counter, in `tests/reseat_use.rs`.)
//!
//! ## The deliverable the spec (`design_seam/SEAM_SPEC_reseat.md`) puts to the cold panel
//!
//! Is the borrowed-slot horn **forced** — does offering #19's commit-API over a borrowed slot *necessarily*
//! break horn 1, confirming Phase R's fence at the owned-vs-borrowed line — or is it merely **movable to a
//! different API**: the `into_inner`-**less** subset (just `new`/`Deref`/`DerefMut`/`Drop`) appears to
//! *hold* with a horn-3 leak cost (unwind-safe, forget = leak) — **that appearance was FALSE; see the
//! 2026-09-04 correction below** — which would push the fence inward, from
//! *owned-vs-borrowed* to ***soundness-load-bearing-restore vs leak-tolerant-restore***. The prediction:
//! **SEAM-BREAKS [horn 1]** for the full #19-parallel packaging, with the standing question of whether the
//! `into_inner`-less subset is a sound repackaging that *moves* the fence rather than confirming it.
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_reseat, r27): SEAM-BREAKS [horn 1], FAILED — the SIXTH
//! ## broken seam. The horn flip is a CLEAN single variable; and the fence is FORCED, not movable.
//!
//! The seam **broke** (FAILED, 0/1). Two findings, both decisive:
//!
//! 1. **The single-variable horn flip is confirmed CLEAN — Phase R's confound is repaired.** All five
//!    attackers agreed: #19 (owned value + `into_inner`) *cost* horn 3; this seam (borrowed slot +
//!    `into_inner`), with the packaging **byte-identical**, *breaks* horn 1. The soundness attacker landed a
//!    4-line safe witness — `let mut s = String::from("hello"); let mut g = Reseat::new(&mut s);
//!    g.push_str(" world"); g.into_inner()` — where `into_inner` returns the owned `String` while the slot
//!    `s` keeps the `ptr::read` duplicate (`E0509` forces the `mem::forget(self)` that suppresses the
//!    reseat), so both `s` and the returned value own overlapping/realloc'd heap and **double-free on drop,
//!    with no `unsafe` in user code**. The implementation attacker confirmed the UB is plain decidable Rust.
//!    Where #17→#19 co-varied packaging *and* ownership model, #19→#20 co-varies **neither** — so this is the
//!    clean controlled experiment, and the cost-3 → break-1 flip is attributable to the **decoupling
//!    property alone**.
//!
//! 2. **The fence is FORCED, not movable — my spec's open hedge was the defect that FAILED it.** I posed
//!    forced-vs-movable and leaned *movable*: that the `into_inner`-less subset (`new`/`Deref`/`DerefMut`/
//!    `Drop`) holds (unwind-safe, `forget` = leak), pushing the fence inward to *soundness-load-bearing vs
//!    leak-tolerant*. The ergonomics and expressiveness attackers **rejected** this: the subset they were
//!    handed is **degenerate** — it deletes the *by-value move-out* that *defines* take-and-reseat (the `T → T`-through-
//!    a-non-`T` transform `take_mut`/`replace_with` exist for), leaving a primitive **strictly dominated by
//!    plain `&mut T`**. So every *usable* packaging of the borrowed-slot take breaks horn 1: the fence is
//!    **FORCED across all useful packagings**, sitting exactly at Phase R's **owned-vs-borrowed** line — not
//!    pushed inward.
//!
//!    **CORRECTION 2026-09-04 — the verdict SURVIVES and is STRENGTHENED; the argument for it was weaker
//!    than it needed to be.** The panel took the subset to be *sound* and rejected it on ergonomics. It is
//!    not sound: `new` / `DerefMut` / `forget`, with no `into_inner`, double-frees (witness above). So the
//!    fence needs no degeneracy judgement at all — **there is no sound repackaging to move it to**. The
//!    horn-1 break is present in the *minimal* subset, so the fence is forced by **soundness**, not by
//!    usefulness. That removes the one place this result rested on a taste call about what counts as "the
//!    primitive" — a materially stronger position than the one r27 recorded. The parenthetical it replaces
//!    ("`mem::forget` being a mere leak here is real") was itself the false premise.
//!
//! **Deflation (pre-registered, a flag not a kill).** No novel mechanism: `take_mut`'s `ptr::read`/`write` +
//! `scopeguard`'s `into_inner` defuse + RFC 1066's forget-skips-`Drop`, reassembled. The contribution is
//! **analytical** — the clean controlled pair (#19/#20) and **locating** the fence.
//!
//! **The refined finding (closing the packaging trilogy).** *The horn a danger threatens is movable iff the
//! value-ownership model lets soundness DECOUPLE from the un-compellable closure.* An **owned** value can
//! (#19: the restore is a side-obligation beside a sound core ⇒ horn-3 cost). A **borrowed slot** cannot, for
//! any packaging at all (#20: single ownership across the read-duplicate is load-bearing, and the
//! `into_inner`-free repackaging is not merely degenerate but *itself unsound* ⇒ forced horn-1 break). Phase Q: the horn moves. Phase R: it
//! moves within a fence (but confounded). Phase S: the fence is real, FORCED, and sits at owned-vs-borrowed.
//!
//! The sound uses *do* reseat — see `tests/reseat_use.rs` (mutate-in-place then reseat on normal exit and on
//! panic-unwind). `mem::forget` leaks the mutation only when the value was not destructively touched;
//! after a replacing mutation it double-drops, as does `into_inner`.

use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::ptr;

/// A take-and-reseat guard over a borrowed slot: it reads the value out of `&'a mut T`, lends it through
/// `Deref`/`DerefMut`, and writes it back on `Drop`. While the guard is alive the slot holds a **bitwise
/// duplicate** that the guard's `unsafe` foundation keeps walled off; the borrow checker forbids any other
/// access to the slot (no aliasing). Unwind-safe (Drop reseats during unwinding). **NOT forget-safe:**
/// `mem::forget` after a destructive `DerefMut` leaves the slot owning a value the guard already
/// destroyed — a double-free in safe code. See [`Reseat::into_inner`] and the module correction.
pub struct Reseat<'a, T> {
    /// The borrowed slot. Holds a bitwise duplicate of the value for the guard's lifetime; reseated to a
    /// valid `T` by `Drop`, or left holding the original bits if the guard is `mem::forget`-ten (a leak).
    slot: &'a mut T,
    /// The value, read out of the slot. `ManuallyDrop` because its storage is governed by the take/reseat
    /// dance, not by ordinary drop glue.
    owned: ManuallyDrop<T>,
}

impl<'a, T> Reseat<'a, T> {
    /// Take the value out of `slot` and arm the guard. The value is reseated when the guard drops.
    #[inline]
    pub fn new(slot: &'a mut T) -> Self {
        // SAFETY: `ptr::read` duplicates the value into `owned`; `slot` now holds a bitwise copy that must
        // not be dropped or read until `Drop` writes a valid `T` back. The `&'a mut T` proves exclusive
        // access, so nothing else can observe `slot` during the gap (no aliasing — see CF1).
        //
        // WHAT THIS DOES NOT ESTABLISH (corrected 2026-09-04): the exactly-once obligation on the
        // duplicate is discharged ONLY by `Drop`, and on the `mem::forget` path nothing discharges it.
        // This comment used to claim that path was "a leak, not a double-free"; false whenever `DerefMut`
        // was used to destroy what the duplicate names.
        let owned = ManuallyDrop::new(unsafe { ptr::read(slot) });
        Reseat { slot, owned }
    }

    /// The #19-matching *commit/keep* path — and the seam's horn-1 break. It hands back the owned value
    /// **without reseating**, so the slot (its borrow ending as the guard is consumed) keeps the read
    /// **duplicate**: two owners of one value, a double-free when both drop. For #19's *owned* value the
    /// identical API was sound (one owner); over a *borrowed slot* it is unsound. Kept here, deliberately,
    /// because the experiment is to package the borrowed slot *exactly* as #19 packaged the owned value.
    #[inline]
    pub fn into_inner(mut self) -> T {
        // SAFETY: take `owned` out exactly once; then `mem::forget(self)` so `Drop` does not run — running
        // it would `ManuallyDrop::take` a second time (reading moved-from memory) and reseat a moved-from
        // value. (The *unsoundness* is not here — it is that `slot` keeps the duplicate; this block only
        // avoids a second, local UB.)
        let owned = unsafe { ManuallyDrop::take(&mut self.owned) };
        mem::forget(self);
        owned
    }
}

impl<'a, T> Deref for Reseat<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        &self.owned
    }
}

impl<'a, T> DerefMut for Reseat<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.owned
    }
}

impl<'a, T> Drop for Reseat<'a, T> {
    #[inline]
    fn drop(&mut self) {
        // Reseat: move `owned` back into `slot`, overwriting the duplicate bits **without dropping them**
        // (they are a bitwise duplicate; dropping them would double-free). Runs on normal exit AND during a
        // panic unwind, so the guard is unwind-safe. `mem::forget(guard)` skips this entirely → `owned`
        // leaks and `slot` keeps its original bits: a leak, never a double-free.
        // SAFETY: `owned` is live here on every path that reaches `Drop` (only `into_inner` takes it, and it
        // forgets `self` to skip this); `ptr::write` does not drop the duplicate already in `slot`.
        let owned = unsafe { ManuallyDrop::take(&mut self.owned) };
        unsafe { ptr::write(self.slot, owned) };
    }
}
