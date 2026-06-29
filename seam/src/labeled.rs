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
//! ## The cost it pays (the §6 question) — a clean IFC-domain axis
//!
//! The price is **declassification discipline made type-structural**: a `Cap` must thread through the
//! code to every exit, and the budget of caps bounds the program's total declassification. That is a
//! *viral capability* — the **IFC-domain analog of the branded index's brand** (`'id`), but carrying a
//! *security* obligation (declassification budget) instead of a *memory-safety* one (in-bounds proof).
//! It is the IFC domain's own axis, surfaced by the secret seam and *paid* by this one — with **no
//! memory-safety analog**. The §6 deliverable: does this **close** the IFC seam (foreclosing the
//! unconstrained-declassification leak) by paying a *clean* IFC-domain axis, thereby confirming the IFC
//! domain has its own cost-basis (a closing seam + a clean axis), parallel to memory safety? Honest
//! limits to weigh: it enforces *bounded/concentrated* declassification (a real, standard IFC policy —
//! "where & how-much", per Sabelfeld–Sands), **not** full non-interference (still undecidable); and, as
//! with the secret seam, the AM models no time, so a constant-time obligation on the secret *computations*
//! remains a separate, dischargeable concern.
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
