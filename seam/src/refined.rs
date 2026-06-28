//! # Sixth seam — refinement-typed bounds (Flux / Liquid Haskell style): closing the #5⟂#7 cell.
//!
//! Same floor, same rules. This primitive exists to settle one open claim from the branded-index
//! phase: is **AI-legibility (#7)** a real axis, *independent* of **API-locality / brand-freedom
//! (#5)**? The branded index pays both at once (a brand `'id` is viral *and* a nominal token), so it
//! cannot separate them. Independence needs an **off-diagonal** witness: a primitive that agrees
//! with another on #5 but differs on #7. This is that witness — the **(brand-free, legible)**
//! corner — and paired against the slab (brand-free, *illegible*) it holds #5 fixed and flips #7.
//!
//! ## What a refinement type is, and why it is the opposite corner from the brand
//!
//! A refinement type attaches a **decidable logical predicate** to an ordinary value:
//! `i: usize{i < len}` is "a `usize` the checker has proven less than `len`." The proof is
//! discharged by an SMT solver (Z3) over a decidable arithmetic fragment (QF-LIA / UTVPI), at
//! compile time — so a real refinement floor (Flux, Liquid Haskell, F\*, Dafny) erases the bounds
//! check entirely (#6 zero-cost ✓) with *no brand and no scope closure*.
//!
//! Contrast the two witnesses point by point — this *is* the dissociation:
//!
//! | | branded index (`Idx<'id>`) | refined index (`Refined`) |
//! |---|---|---|
//! | **#5 API-locality** | **viral** — `'id` infects every signature; an `Idx` *cannot escape* its `scope` closure (see `branded`'s CF3) | **brand-free** — a `Refined` is a plain value; it can be **stored, returned, put in a `Vec`** (proven by `refined_use::a_refined_index_escapes_its_constructor`) |
//! | **#7 legibility** | **illegible** — `'id` is a nominal token; the in-bounds fact is reconstructible from *nothing* in the type | **legible** — the predicate `value < bound` *is* the type's contract; an outside reasoner reconstructs the invariant from the signature alone |
//!
//! Same job (sound unchecked access), opposite corners on (#5, #7). That is what makes them
//! independent axes rather than one "the witness leaks" axis.
//!
//! ## The honest floor gap (the same modeling-dictionary caveat the slab's Lean carries)
//!
//! The stipulated floor is **NLL Rust exactly as shipped — which has no SMT solver.** So this module
//! cannot *statically* discharge `value < bound`; `refine` checks it at **run time**, standing in for
//! what Flux proves at compile time. Two consequences, both disclosed rather than hidden:
//!
//! 1. **The witness *form* is faithful and real** — `Refined` is genuinely brand-free (it escapes)
//!    and genuinely legible (the predicate is its documented contract). #5 and #7 are demonstrated,
//!    not asserted.
//! 2. **The static erasure (#6) is *not* reproduced here** — a real Flux floor erases the check; this
//!    model pays a residual runtime compare (`bound == len`) per `at`. So *this artifact* spends #6,
//!    while a *real refinement floor* would instead spend the **external-solver / one-toolchain** cost
//!    (an SMT solver in the trusted compilation loop — the price that distinguishes "decidable" from
//!    "decidable *and* proof-free, one toolchain", and the reason this primitive un-bundles horn 2).
//!
//! ## The brand-free trade, made concrete
//!
//! Brand-freedom is not free. A `Refined` binds to a **length**, not to a slice *identity* — its
//! predicate is `value < bound`, and `bound` is plain data. So a `Refined{value:2, bound:5}` is
//! valid against **any** length-5 slice. That is **memory-safe** (the access is in bounds for every
//! length-5 slice) but admits *logically* using it on the wrong same-length slice — exactly the
//! `slab`'s cross-arena caveat, and exactly what the branded index's identity-binding `'id`
//! *prevented* at the cost of virality. The brand buys identity; the refinement gives it up to stay
//! brand-free. Naming that trade is half the point.
//!
//! ### CF1 — a forged `Refined` is rejected (fields private; only `refine` mints one):
//! ```compile_fail
//! # use seam_spsc::refined::Refined;
//! let _r = Refined { value: 9, bound: 3 }; // ERROR: fields are private — the predicate is not forgeable
//! ```

/// A refinement-typed index: a `usize` whose type contract is the **visible predicate
/// `value < bound`**. Brand-free (a plain `Copy` value — store it, return it, nest it) and legible
/// (the predicate is the contract). Minted only by [`refine`], which establishes the predicate;
/// fields are private so safe code cannot forge one that violates it.
///
/// Under a real refinement floor this is `i: usize{i < n}` with `n` the slice's length variable;
/// here `bound` carries `n` as data because the NLL floor cannot range over it in types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Refined {
    value: usize,
    bound: usize,
}

impl Refined {
    /// The proven offset. Safe to use unchecked against any slice whose length equals [`Refined::bound`].
    #[inline]
    pub fn value(self) -> usize {
        self.value
    }

    /// The bound the predicate was proven against (a real Flux floor keeps this in the type, not as data).
    #[inline]
    pub fn bound(self) -> usize {
        self.bound
    }
}

/// Establish the refinement `value < bound`, minting a [`Refined`] — or `None` if it does not hold.
/// On a real refinement floor the solver discharges this at compile time and it erases; here it is a
/// runtime check standing in for the solver (the disclosed floor gap). Note there is **no brand and
/// no closure**: the returned `Refined` is a free value the caller may keep indefinitely.
#[inline]
pub fn refine(value: usize, bound: usize) -> Option<Refined> {
    if value < bound {
        Some(Refined { value, bound })
    } else {
        None
    }
}

/// Index `slice` by a refined index with **no bounds check**. Sound because `Refined` guarantees
/// `value < bound` (its private invariant) and this binds the refinement to a slice **at least as
/// long as `bound`** by checking `bound <= slice.len()`; together they imply `value < slice.len()`.
///
/// The bound here is `<=`, not `==`: safety needs only `bound <= len` (a `Refined` proven `value <
/// bound` is in-bounds for *any* slice of length `>= bound`), and a real refinement floor admits this
/// by predicate subtyping `{v < bound} <: {v < len}` for `bound <= len`. Using `==` would reject the
/// provably-in-bounds `Refined{1,3}` on a length-5 slice — strictly less expressive than the floor
/// modelled (the cold panel's R2-equality finding). The `bound <= len` compare is the residual
/// runtime cost a real refinement floor erases (it relates the length variables statically); it is
/// **not** a bounds check — the per-access `value < len` check *is* eliminated. Panics on a too-short
/// slice (a logic error, never UB).
#[inline]
pub fn at<T>(slice: &[T], i: Refined) -> &T {
    assert!(
        i.bound <= slice.len(),
        "refined index proven against length {} used on a slice of length {} (too short)",
        i.bound,
        slice.len()
    );
    // SAFETY: `Refined`'s invariant is `value < bound` (established by `refine`, unforgeable: private
    // fields), and the assert proves `bound <= slice.len()`. Therefore `value < bound <= slice.len()`,
    // so `value < slice.len()`. The per-access bounds check is elided; only the length-binding remains.
    unsafe { slice.get_unchecked(i.value) }
}
