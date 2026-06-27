//! # Fifth seam — branded indices (sound unchecked indexing): the zero-cost axis is *aced*,
//! and a NEW axis appears.
//!
//! Same floor, same rules. This primitive exists to answer the question the slab opened: is the
//! Seam Trilemma a 3-of-**4** projection (the slab found zero-cost / #6 as the folded fourth
//! axis), or 3-of-**N**? The slab-grade test for "is this a genuinely new axis": find a primitive
//! that aces *all four* known axes — sound, decidable + proof-free, concentrated (O(1) seam
//! width), **and zero-cost** — yet still imposes a consumer-visible cost. Only a primitive clean
//! on the known four can isolate a fifth.
//!
//! **Sound unchecked indexing** (Gankra, "Sound Unchecked Indexing"; Bluss's `indexing` crate;
//! the `generativity` pattern) is exactly that primitive. A branded index *elides the bounds
//! check* — so it is the first seam here to fully ACE zero-cost (#6): the safe interface is
//! **faster** than ordinary `slice[i]`, and the brand is a zero-size `PhantomData` that erases
//! completely. Yet it pays a tax none of the first four pay:
//!
//! - **The brand `'id` is a *viral generative type parameter*.** Every index value carries it
//!   (`Idx<'id>`); every helper that touches a branded index must be generic over `'id`; and the
//!   consumer is forced to work *inside* a `scope(|view| { … })` closure from which indices
//!   **cannot escape**. That is a real cost to the consumer's API surface — and, crucially, it is
//!   **provably not seam width**: by `SEAM_WIDTH.md`'s definition (crossings = `unsafe` or
//!   foundation-boundary calls in consumer code), this primitive's seam width is **O(1)** (one
//!   `scope` per slice). A cost that is neither runtime (it erases) nor a seam crossing (O(1)) is,
//!   by elimination, a *fifth* axis. Call it **brand-freedom / API-locality**.
//!
//! The deep symmetry (cf. the intrusive list's design-A `compile_fail`): the property that makes
//! it **sound** and the property that makes it **costly** are the *same mechanism*. Generativity
//! — the index cannot leave the scope, cannot be applied to another slice — is simultaneously the
//! soundness proof (CF2/CF3 below) and the API-locality tax. You cannot keep one without the
//! other. That is the fifth axis showing its teeth.
//!
//! ## What the brand buys, mechanically
//!
//! A `BrandedSlice<'id, …>` freezes one slice's length and tags it with an invariant brand `'id`
//! minted fresh per `scope` (via a `for<'new>` HRTB, exactly like `GhostToken`). An `Idx<'id>` is
//! a raw `usize` *proven* `< len` by having passed `try_index` against a slice of the **same**
//! brand. Because `'id` is unique to one `scope` over one immutable slice whose length cannot
//! change for the scope's duration, `idx < len` is a type-level invariant — so indexing needs
//! **no runtime bounds check**. The single `unsafe` (`get_unchecked`) is concentrated in the
//! foundation; consumer code is `#![forbid(unsafe_code)]`.
//!
//! ## Honest scope
//!
//! This is the minimal faithful core of sound unchecked indexing — read-only, one slice per
//! scope, enough to exhibit the fifth axis. The real `indexing` crate also brands *ranges*,
//! partitions, and `&mut` (where length-stability needs more care); none of that changes the axis
//! this primitive isolates. The fifth-axis claim itself is adjudicated by the cold harness, not
//! asserted here — see `../design_seam/REPORT_SEAM_SPEC_branded.md`.
//!
//! ### CF1 — a forged index is rejected (fields private, no public constructor):
//! ```compile_fail
//! # use seam_spsc::branded::Idx;
//! let _i: Idx = Idx { idx: 0 }; // ERROR: fields are private; only `try_index` mints one
//! ```
//!
//! ### CF2 — generativity: an index from one scope cannot index another scope's slice.
//! This is the soundness crux — without it, an index proven in-bounds for a length-3 slice could
//! read out of bounds on a length-1 slice:
//! ```compile_fail
//! # use seam_spsc::branded::scope;
//! let a = [10, 20, 30];
//! let b = [99];
//! scope(&a, |va| {
//!     let ia = va.try_index(2).unwrap();      // proven < 3 for a's brand
//!     scope(&b, |vb| {
//!         let _ = &vb[ia];                    // ERROR: brand mismatch ('id of a ≠ 'id of b)
//!     });
//! });
//! ```
//!
//! ### CF3 — the API-locality tax IS the soundness mechanism: an index cannot escape its scope
//! (if it could, it would outlive the borrow that froze the length):
//! ```compile_fail
//! # use seam_spsc::branded::scope;
//! let a = [10, 20, 30];
//! let escaped = scope(&a, |va| va.try_index(1).unwrap()); // ERROR: 'id escapes the for<'id> binder
//! let _ = escaped;
//! ```

use core::marker::PhantomData;
use core::ops::Index;

/// An invariant brand: a lifetime that appears both co- and contra-variantly, so two distinct
/// `scope` calls' brands never unify. Identical mechanism to `intrusive::GhostToken`'s `'brand`.
type Invariant<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

/// A slice whose length is frozen and tagged with brand `'id`. Built **only** by [`scope`], so a
/// given brand corresponds to exactly one slice; the consumer cannot mint a second `BrandedSlice`
/// of the same brand over different data.
pub struct BrandedSlice<'id, 'a, T> {
    data: &'a [T],
    _brand: Invariant<'id>,
}

/// A proof-carrying index: a raw offset KNOWN to be `< len` of the brand-`'id` slice. `Copy`
/// (indices are values you reuse), but its `'id` traps it inside one [`scope`]. Fields are
/// private: the only way to obtain one is [`BrandedSlice::try_index`], which bounds-checks once.
#[derive(Clone, Copy)]
pub struct Idx<'id> {
    idx: usize,
    _brand: Invariant<'id>,
}

impl<'id, 'a, T> BrandedSlice<'id, 'a, T> {
    /// Bounds-check **once**, then mint a branded index that needs no further checks. `None` if
    /// out of range. This is the only seam crossing the consumer performs per index it creates —
    /// and it is an ordinary safe call returning `Option`, not an `unsafe` or foundation crossing.
    #[inline]
    pub fn try_index(&self, i: usize) -> Option<Idx<'id>> {
        if i < self.data.len() {
            Some(Idx { idx: i, _brand: PhantomData })
        } else {
            None
        }
    }

    /// The frozen length. Knowing it does not let safe code forge an index (no public `Idx`
    /// constructor); it is here so the consumer can iterate `0..len()` through `try_index`.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Indexing a branded slice by **its own** branded index: no bounds check. This is the zero-cost
/// win — the type system already proved `i.idx < len` and the length cannot have changed.
impl<'id, 'a, T> Index<Idx<'id>> for BrandedSlice<'id, 'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, i: Idx<'id>) -> &T {
        // SAFETY: an `Idx<'id>` exists only because it passed `try_index` against a slice of this
        // same brand `'id`. The brand `'id` is unique to one `scope` call over one immutable slice
        // whose length is frozen for the scope's duration (`scope` borrows `&'a [T]` for `f`). So
        // `i.idx < self.data.len()` is a type-level invariant — the bounds check is redundant and
        // is elided. This is the single foundation `unsafe`; consumer code carries none.
        unsafe { self.data.get_unchecked(i.idx) }
    }
}

/// Open an indexing scope. Mints a **fresh** brand `'id` (the `for<'id>` HRTB makes each call's
/// brand incomparable to every other's) and hands the consumer a branded view of `data`. Indices
/// derived inside `f` carry `'id` and therefore cannot escape `f` — which is exactly why they stay
/// sound, and exactly the API-locality cost (the fifth axis).
#[inline]
pub fn scope<'a, T, R>(data: &'a [T], f: impl for<'id> FnOnce(BrandedSlice<'id, 'a, T>) -> R) -> R {
    f(BrandedSlice { data, _brand: PhantomData })
}
