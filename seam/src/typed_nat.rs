//! # Eighth seam — a type-level proof of in-bounds indexing (the checker-tractability axis).
//!
//! Same floor, same rules. This primitive re-imports the *same* danger as the branded and refined
//! seams — `slice.get_unchecked(i)`, out-of-bounds is instant UB — with a *third* kind of witness:
//! a **type-level proof** `I: Lt<Len>` discharged entirely by the compiler's trait resolver. No
//! brand, no runtime check, no external solver, no value: the proof is a constraint the type checker
//! either resolves or rejects, and it **erases completely** (it is `PhantomData` and a `const`).
//!
//! It exists to settle one open question from the refined-index phase. The refined index un-bundled
//! horn 2 ("decidable & proof-free") into **decidability** ⊗ **solver-freedom** — it is decidable but
//! needs an SMT solver in the trusted loop. H8 asks whether *decidability itself* is a further
//! bundle: **{a yes/no answer exists}** ⊗ **{the native checker actually halts in bounded resource}**.
//!
//! ## The dissociation this primitive witnesses
//!
//! | | refined index (`Refined`) | type-level index (`I: Lt<Len>`) |
//! |---|---|---|
//! | **decidable** (answer exists) | ✓ (QF-LIA terminates) | ✓ (Peano `Lt` is well-founded) |
//! | **solver-free** (no external prover in TCB) | ✗ — needs Z3 | **✓** — native trait resolution only |
//! | **checker-tractable** (halts without an arbitrary resource knob) | ✓ — QF-LIA is *efficiently* decidable | **✗** — resolution depth = the index value; halts only under `#![recursion_limit]` |
//!
//! So `typed_nat` agrees with the refined index on **decidable** but flips **solver-free** (it needs
//! no solver) *and* flips **checker-tractability** (it is the one whose checker does not cheaply
//! halt). Held against SPSC — which is decidable, solver-free, *and* trivially-terminating
//! (`T: Send` resolves in one step) — `typed_nat` agrees on {sound, solver-free, zero-cost, decidable}
//! and splits **only** on checker-tractability. That is the off-diagonal: it isolates
//! **checker-tractability** as a face of horn 2 distinct from *both* decidability and solver-freedom.
//! Horn 2 is then **decidability ⊗ solver-freedom ⊗ checker-tractability** — three faces, not two.
//!
//! ## Decidability-by-fuel, made concrete (the axis you can compile)
//!
//! `I: Lt<Len>` is resolved by recursive trait impls (see below): proving `i < n` costs `i` resolution
//! steps. Rust caps that depth with `#![recursion_limit]` (default 128). So the *same in-bounds index*
//! compiles or fails to compile depending on a knob that has **nothing to do with whether the access
//! is safe** — only with how much fuel the checker was given. Raise the limit and a deeper index
//! type-checks; the proof is no "harder", the checker just had more fuel. That is the operational
//! meaning of "decidable but not tractable without an arbitrary resource bound" — and it is the
//! limiting approach to genuine undecidability: Rust trait resolution is Turing-complete, so an
//! ill-founded `Lt`-style relation would *diverge* without the cap rather than merely run deep. This
//! seam witnesses the fuel-gated form (well-founded, so it would terminate given unbounded fuel); the
//! divergent form is the same mechanism with a non-well-founded impl, named but not built here.
//!
//! ## The honest floor gap (same modeling-dictionary caveat as the refined index)
//!
//! Binding the type-level length `Len` to a real slice's runtime length is the one fact the bare-NLL
//! floor cannot prove in types: [`Bound::new`] checks `data.len() == Len::VALUE` at run time, standing
//! in for what a dependently-typed floor (Idris, Agda, Lean, or Rust with full const-generic naturals)
//! proves statically. Once bound, **per-access indexing carries no runtime check** — the in-bounds
//! fact is the type-level proof `I: Lt<Len>`, which erases. So the witness *form* (a compile-time
//! proof that costs checker fuel) is faithful and real; only the one-time length binding is modelled.
//!
//! ## Soundness — why a consumer cannot forge a proof (the sealed-trait stopper)
//!
//! `Nat` is a **sealed** trait: `Nat: sealed::Sealed`, and `Sealed` is implemented *only in this
//! crate*, for `Z` and `S<N>`. A consumer can impl the (public) `Lt` trait for their own local type
//! — the orphan rule permits `impl ForeignTrait for LocalType` — but that type can **never be `Nat`**,
//! so [`get`]'s bound `I: Nat + Lt<Len>` rejects it, and there is no other way to supply `I::VALUE`.
//! Forging `impl Nat for Evil { const VALUE = 999; }` is a compile error (cannot impl the sealed
//! supertrait). The two `Lt` impls are the *only* ones reachable for `Nat` indices, and they encode
//! `<` exactly (`0 < n+1`; `i+1 < n+1 ⟺ i < n`; nothing is `< 0`), so `I: Lt<Len> ⟹ I::VALUE <
//! Len::VALUE = data.len()`. The unchecked read is in bounds.
//!
//! ### CF1 — a too-deep (but in-bounds) index fails to *check* under a low fuel cap (the axis itself):
//! ```compile_fail
//! #![recursion_limit = "4"]
//! # use seam_spsc::typed_nat::{Bound, get, N6, N7};
//! let v = [10u32; 7];
//! let b = Bound::<u32, N7>::new(&v).unwrap();   // length 7, bound to type-level 7
//! let _x = get::<N6, _, _>(&b);                 // 6 < 7 — IN BOUNDS, yet ERROR:
//!                                               // "overflow evaluating `S<..>: Lt<..>`" — out of fuel,
//!                                               // not out of bounds. Raise recursion_limit → it compiles.
//! ```
//!
//! ### CF2 — an out-of-bounds index has *no proof* and is rejected (soundness, not fuel):
//! ```compile_fail
//! # use seam_spsc::typed_nat::{Bound, get, N2, N3};
//! let v = [10u32; 2];
//! let b = Bound::<u32, N2>::new(&v).unwrap();   // length 2
//! let _x = get::<N3, u32, N2>(&b);              // ERROR: `S<Z>: Lt<Z>` is not satisfied — proving
//!                                               // `3 < 2` unwinds to the false base case `1 < 0`.
//! ```
//! (Annotated fully so the unsatisfied `Lt` surfaces directly; with `Len` left to inference the same
//! out-of-bounds index instead forces `Len` to be at least `S<S<S<S<_>>>>` and conflicts with the
//! length-2 buffer as a type mismatch — sound either way, the index is simply unconstructible.)
//!
//! ### CF3 — a consumer cannot forge a `Nat` to lie about `VALUE` (sealed supertrait):
//! ```compile_fail
//! # use seam_spsc::typed_nat::Nat;
//! struct Evil;
//! impl Nat for Evil { const VALUE: usize = 999; } // ERROR: `Evil: Sealed` is not satisfied
//! ```

use core::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Z {}
    impl<N: super::Nat> Sealed for super::S<N> {}
}

/// Type-level zero.
pub struct Z;

/// Type-level successor of `N`.
pub struct S<N>(PhantomData<N>);

/// A type-level natural number. **Sealed** — only `Z` and `S<N>` implement it, so safe code cannot
/// introduce a type with a forged [`Nat::VALUE`]. `VALUE` reflects the numeral to a `const usize`.
pub trait Nat: sealed::Sealed {
    /// The runtime value of this type-level numeral (used once, to bind a slice's length).
    const VALUE: usize;
}

impl Nat for Z {
    const VALUE: usize = 0;
}
impl<N: Nat> Nat for S<N> {
    const VALUE: usize = 1 + N::VALUE;
}

/// A compile-time proof that `Self < N` (both type-level naturals). Carries no data and no runtime
/// cost: a `Self: Lt<N>` bound is discharged by trait resolution and erases. The proof costs the
/// checker `VALUE` resolution steps — the fuel the **checker-tractability** axis is about.
///
/// Public so it can appear in `where` clauses, but it cannot be abused: a consumer impl is only legal
/// for a *local* type (orphan rule), which can never be the sealed [`Nat`] that [`get`] also requires.
pub trait Lt<N> {}

// 0 < n+1 for every n.
impl<M> Lt<S<M>> for Z {}
// i+1 < n+1  ⟺  i < n. (And nothing is `< 0`: there is no `Lt<Z>` impl.)
impl<I, M> Lt<S<M>> for S<I> where I: Lt<M> {}

/// A slice whose length is bound, once, to the type-level numeral `Len`. Construct with [`Bound::new`].
pub struct Bound<'a, T, Len> {
    data: &'a [T],
    _len: PhantomData<Len>,
}

impl<'a, T, Len: Nat> Bound<'a, T, Len> {
    /// Bind `data` to the type-level length `Len`. Returns `None` unless `data.len() == Len::VALUE`.
    ///
    /// This one runtime compare is the disclosed floor gap (a dependently-typed floor proves it
    /// statically). After it succeeds, the length is a *type-level fact* and per-access indexing needs
    /// no further check.
    #[inline]
    pub fn new(data: &'a [T]) -> Option<Self> {
        if data.len() == Len::VALUE {
            Some(Bound { data, _len: PhantomData })
        } else {
            None
        }
    }

    /// The bound length, as data (equals `Len::VALUE` and `self.data.len()`).
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Always `false` after a successful `new` with `Len::VALUE > 0`; present so `len`'s clippy peer
    /// is satisfied without editorializing.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Index a [`Bound`] buffer by the type-level index `I`, with **no runtime bounds check**. Sound
/// because the bound `I: Lt<Len>` is a compile-time proof that `I::VALUE < Len::VALUE`, and
/// [`Bound::new`] established `Len::VALUE == data.len()`; together, `I::VALUE < data.len()`.
///
/// The proof erases — there is no witness value at run time. Its only cost is the checker fuel spent
/// resolving `I: Lt<Len>` (depth `I::VALUE`), bounded by `#![recursion_limit]` — the axis. `I` is the
/// turbofish argument (`get::<N3, _, _>(&b)`); `T` and `Len` infer from the buffer.
#[inline]
pub fn get<'a, I, T, Len>(buf: &Bound<'a, T, Len>) -> &'a T
where
    I: Nat + Lt<Len>,
    Len: Nat,
{
    // SAFETY: `I: Lt<Len>` proves `I::VALUE < Len::VALUE` (the two `Lt` impls encode `<` exactly and
    // `Nat` is sealed, so `I::VALUE` cannot lie); `Bound::new` proved `Len::VALUE == buf.data.len()`.
    // Hence `I::VALUE < buf.data.len()`: the unchecked read is in bounds. No per-access check remains.
    unsafe { buf.data.get_unchecked(I::VALUE) }
}

/// Readable aliases for the small type-level numerals the tests and doctests use.
pub type N0 = Z;
pub type N1 = S<N0>;
pub type N2 = S<N1>;
pub type N3 = S<N2>;
pub type N4 = S<N3>;
pub type N5 = S<N4>;
pub type N6 = S<N5>;
pub type N7 = S<N6>;
pub type N8 = S<N7>;
