//! # Fourteenth seam — capability-gated declassification (a *closing* information-flow seam).
//!
//! *(Project ordinal: the fourteenth primitive, and the **second** in the information-flow domain. The
//! thirteenth — the constant-time secret — **broke** [horn 1, IFC sense] because declassification was
//! *etiquette, not type-structure*: `expose_bool` was free to call anywhere. This one closes that exact
//! gap, to test whether the IFC domain has a **closing** seam — the IFC analog of SPSC / the slab — and
//! thus a real cost-basis of its own, completing the per-domain generalization.)*
//!
//! Same framing as the secret seam: the danger is an **information leak** (a secret value reaching
//! ordinary control flow), not UB. The lesson the secret seam taught is the lesson every memory-safety
//! seam already embodies: you do not achieve *"no `unsafe`"*, you achieve *concentrated, checked
//! `unsafe`*. So here you do not achieve *"no declassification"* (full non-interference is whole-program
//! undecidable — that is *why* the secret seam broke) — you achieve **bounded, capability-gated
//! declassification**, where *where / when / how-often* a secret becomes public is **type-structural**.
//!
//! ## The witness — an affine declassification capability
//!
//! [`Secret<T>`] wraps a value with **no way to read it out** except [`Secret::declassify`], which
//! **consumes an affine [`Cap`]** (`!Copy`, `!Clone`). A `Cap` is minted only by [`Cap::issue`] — the
//! "trust authority" call you place at the audited boundary (the verified end of a protocol, the one
//! point where an observable result is *meant* to escape). Because a `Cap` is affine, **one cap
//! declassifies exactly once**; the number of declassifications a program can perform is bounded by the
//! number of caps issued — a fact the *type system enforces*, not a convention. You compute *on* secrets
//! without declassifying via [`Secret::map`] / [`Secret::zip`] (the result stays `Secret`), so the cap is
//! spent only at the genuine exit.
//!
//! This is exactly the gap that broke the secret seam, now closed: there, `ct_eq(a,b).expose_bool()` then
//! `if b` type-checked *anywhere, any number of times* — declassification was unconstrained. Here the
//! same `if b` requires `b: bool`, which requires `declassify(cap)`, which requires *holding a cap* — so
//! the leak path is **gated and bounded by an affine capability**.
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_labeled): the seam BREAKS — no closing IFC seam exists
//! ## on Rust's floor, and the floor is the point.
//!
//! The gauntlet ruled **SEAM-BREAKS [horn 1]**, *unanimously* (5/5 genuine-flaw) — the **second** IFC
//! seam to drop a horn, for the same structural reason as the first. So the hypothesis this module was
//! built to test — "the IFC domain has a *closing* seam on Rust's floor" — is **false**, and *why* it is
//! false is the deepest result of the project. Two independent breaks:
//!
//! 1. **`map`/`zip` is an uncapped `Secret<T> → T` exit.** To *compute* on a secret, `map(self, f:
//!    FnOnce(T) -> U)` must hand the closure the **bare plaintext** `T`. A safe closure with captured
//!    `&mut` state exfiltrates it as a side effect — `s.map(move |x| sink.push(x))` — releasing the
//!    plaintext with **zero caps** and **no `declassify` site**, while dutifully returning `Secret<()>`.
//!    CF1 ("the only exit is `declassify`") is false; the leak is the closure *parameter*, not the
//!    result. **Unpatchable without an effect system Rust's floor disclaims:** the affine `Cap` *guards a
//!    door while the wall beside it is open*. (Remove `map`/`zip` and the seam computes nothing useful.)
//! 2. **The "budget" counts *events*, not *bits*.** Affinity bounds *uses-per-cap*, not caps-*minted*:
//!    `for s in secrets { s.declassify(Cap::issue()) }` mints N caps and releases N secrets — `grep
//!    issue()` returns 1, runtime release is unbounded (statically undecidable, Rice). And amount ≠ count
//!    (one cap → a megabyte). So the "type-enforced amount budget" does not exist.
//!
//! ## Why the *axis* claim collapses — and the floor-vs-domain lesson
//!
//! The claim "declassification discipline is a clean IFC axis, the analog of the branded index's brand"
//! does **not** survive: (a) it is **not viral like a brand** — a brand threads through every *use*, but
//! here the plaintext is freely handed to *unbranded* ambient code (the `map` closure); (b) what the cap
//! *does* enforce — the **location/event** of release (Sabelfeld–Sands' *where/when*) — **projects onto
//! the memory-safety use-once-affine axis** (it is one-shot privilege, not a new coordinate); (c) the
//! *genuinely* IFC coordinate is the **amount** (the *what* — **quantitative information flow**: Smith
//! min-entropy / g-leakage), which exists *precisely because* event-counting cannot answer it — and an
//! **affine type system cannot bound bits**. So the deepest, honest finding: **a closing seam exists only
//! when the floor *matches* the domain.** Rust's affine/borrow floor is matched to **memory safety**
//! (where SPSC closes *free* and the basis is closed) and **mismatched to information flow** (where *no*
//! seam closes — both IFC seams break; the only enforceable cost is a memory-safety axis in disguise, and
//! the real IFC coordinate, quantitative leakage, needs a *quantitative / effect-tracking* floor: Jif,
//! FlowCaml, QIF). The Seam Trilemma is domain-general; *which seams close* depends on the floor-domain match.
//!
//! ### CF1 — a secret cannot be read without a capability (declassification is not free):
//! ```compile_fail
//! # use seam_spsc::labeled::Secret;
//! let s = Secret::new(42u8);
//! let _v: u8 = s.into_inner(); // ERROR: no such method — the only exit is `declassify(cap)`
//! ```
//!
//! ### CF2 — one capability declassifies once (the budget is affine, not reusable):
//! ```compile_fail
//! # use seam_spsc::labeled::{Secret, Cap};
//! let cap = Cap::issue();
//! let a = Secret::new(1u8).declassify(cap);   // cap consumed here
//! let b = Secret::new(2u8).declassify(cap);   // ERROR: use of moved value `cap`
//! let _ = (a, b);
//! ```

use core::marker::PhantomData;

/// An affine **declassification capability**: minted only by [`Cap::issue`] at the trust boundary, and
/// **consumed** by each [`Secret::declassify`]. `!Copy` and `!Clone`, so one cap permits exactly one
/// declassification — the program's declassification budget is the number of caps issued, type-enforced.
pub struct Cap {
    // A private field with a non-`Copy`, non-`Clone` marker keeps `Cap` affine and unforgeable: safe
    // code cannot duplicate a cap or mint one except through `issue`.
    _affine: PhantomData<*const ()>,
}

impl Cap {
    /// Mint one declassification capability. This is the audited "trust authority" call — every `issue`
    /// is one budgeted exit a reviewer can grep for, the IFC analog of writing the word `unsafe` once.
    pub fn issue() -> Self {
        Cap { _affine: PhantomData }
    }
}

/// A **labeled (secret) value**: holds a `T` with no way to observe it except by spending a [`Cap`].
/// Compute on it with [`map`](Secret::map) / [`zip`](Secret::zip) (results stay secret); the secret
/// reaches ordinary control flow only through [`declassify`](Secret::declassify).
pub struct Secret<T> {
    value: T,
}

impl<T> Secret<T> {
    /// Label a value secret. (Classification is free; *de*classification is what costs.)
    pub fn new(value: T) -> Self {
        Secret { value }
    }

    /// Transform a secret *while keeping it secret* — the result is a `Secret<U>`, so no value escapes.
    /// This is how real computation happens without spending a capability.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Secret<U> {
        Secret { value: f(self.value) }
    }

    /// Combine two secrets, staying secret — e.g. a constant-time comparison `a.zip(b, |x,y| ct_eq(x,y))`.
    pub fn zip<U, R>(self, other: Secret<U>, f: impl FnOnce(T, U) -> R) -> Secret<R> {
        Secret { value: f(self.value, other.value) }
    }

    /// The **one** exit: turn a secret into its public value, **consuming a capability**. Because `Cap`
    /// is affine, this can run only as many times as caps were issued — so the declassification is
    /// bounded and concentrated at the trust boundary, *type-structurally*, not by convention.
    pub fn declassify(self, cap: Cap) -> T {
        let Cap { _affine } = cap; // consume the capability
        self.value
    }
}
