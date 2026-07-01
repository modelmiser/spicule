//! # Thirty-third seam — TOTAL(-ISH) DIVISION: the panic-freedom / totality probe.
//!
//! *(Project ordinal: the thirty-third primitive, and the **first pivot off the session family** — that arc
//! (connectives #18–#28, completion #29/#30, recursion #31/#32) is fully mapped. This is a fresh **fourth-domain
//! candidate**: **panic-freedom / totality** — the SPARK / Frama-C / refinement-type "absence of run-time errors"
//! property (no div-by-zero, no overflow, no out-of-bounds). The two previous fourth-domain probes both DEFLATED —
//! `crdt` #21 folded into application-correctness, `wcet` #22 broke unsound — so the confirmed map (memory safety /
//! information flow / liveness) looks special. This probe asks: is "the program aborts at runtime" a fourth
//! **soundness** domain, or does it fold?)*
//!
//! ## The primitive — integer division, re-exported so div-by-zero cannot be written
//!
//! `a / b` in Rust **panics** on two independent inputs, and this is the whole reason division is a sharp
//! panic-freedom witness:
//! 1. **div-by-zero** — `b == 0` (`'attempt to divide by zero'`);
//! 2. **signed overflow** — `i32::MIN / -1` (`'attempt to divide with overflow'`), the one quotient that escapes
//!    the type, checked in *all* build modes.
//!
//! The seam re-exports division through a [`Divisor`] — a value certified **nonzero** by a single check
//! concentrated at construction ([`Divisor::new`] returns `None` on `0`), witnessed thereafter by the type. So
//! **div-by-zero is foreclosed**: [`divide`] cannot even be *called* with a raw integer (see CF1), and a
//! `Divisor` can never be zero. The danger `/` is re-imported as a checked interface that cannot hit its first
//! panic.
//!
//! ## What the type carries — and the residue it pointedly does NOT (the honest half)
//!
//! - **Carries: div-by-zero-freedom (concentrated, on the *real* floor).** The check lives once, at
//!   [`Divisor::new`]; downstream the `Divisor` *is* the proof. Note this needs **no SMT and no hypothetical
//!   floor** — unlike the refined index (#6, which stipulates a Flux/Liquid-Haskell solver), the witness here is
//!   [`core::num::NonZeroI32`], the one refinement type stock Rust actually ships. Div-by-zero is the fragment of
//!   panic-freedom the real floor already closes.
//! - **Does NOT carry: the `i32::MIN / -1` overflow.** A *valid* nonzero `Divisor` can still be `-1`, and the
//!   dividend can still be `i32::MIN`. So a `-> i32` "total division" signature is a **lie** — it would panic on
//!   that pair. [`divide`] is honest instead: it returns `Result<i32, DivOverflow>`, `Err` **iff** `MIN / -1`
//!   (zero is already excluded), so the second panic source is relocated to a **runtime `Result`**, not carried by
//!   the type. That `Err` arm *is* the conceded residue.
//!
//! ## HYPOTHESIS (this is the artifact commit; the cold panel adjudicates, not the builder)
//!
//! Panic-freedom does **not** open a fourth *soundness* domain. A panic is **defined behaviour** — a clean abort,
//! never UB — so totality is a **correctness** property, not a "can safe code reach UB?" property; it should fold
//! toward the **application-correctness basis** (#19 `restore` / #21 `crdt`), *not* stand beside memory safety /
//! IFC / liveness. Yet — unlike `crdt`, whose algebraic law was wholly uncheckable — a **decidable fragment
//! genuinely closes**: div-by-zero, via the shipped `NonZero` refinement, a horn-3 concentration that *holds for
//! that fragment*. The general case escapes: `MIN / -1` (and arbitrary `+`/`*` overflow) needs whole-program
//! **range analysis** (SPARK's prover, refinement over QF-LIA) — undecidable on the bare floor. Predicted verdict:
//! **SEAM-COSTS [horn3]** that HOLDS for the div-by-zero fragment, the residue conceded up front (the #32
//! discipline — I claim only the carriable half, and explicitly *not* "total division"). Pre-registered
//! deflations the panel may sharpen: (a) the mechanism overlaps `refined` #6 (a nonzero refinement); the *domain*
//! finding is the contribution, not the mechanism; (b) the closing fragment may be judged **degenerate** (one
//! rarely divides by a statically-unknown value one has separately proven nonzero), echoing `wcet`'s degenerate-
//! Peano dock; (c) if the panel rules panic-freedom simply *is* application-correctness, this is a DEFLATION
//! (a `..._factors`, like `crdt`), not a fourth domain.
//!
//! ### CF1 — you cannot divide by a raw integer; the divisor must pass the nonzero construction check:
//! ```compile_fail
//! # use seam_spsc::divisor::divide;
//! let _ = divide(10, 3);   // ERROR: expected `Divisor`, found integer — no un-checked divisor reaches `/`
//! ```

use core::num::NonZeroI32;

/// A divisor certified **nonzero** — the div-by-zero panic source, foreclosed. The check is concentrated once at
/// [`Divisor::new`]; thereafter the type *is* the witness (`Copy`, so one certified divisor serves many
/// divisions). Wraps [`core::num::NonZeroI32`], the refinement type stock Rust ships — so this needs no SMT and
/// no hypothetical floor. It does **not** foreclose the `i32::MIN / -1` overflow: a nonzero divisor may be `-1`.
#[derive(Clone, Copy)]
pub struct Divisor(NonZeroI32);

impl Divisor {
    /// Mint a divisor, rejecting zero at the single concentrated check. `None` **iff** `d == 0`.
    #[inline]
    pub fn new(d: i32) -> Option<Self> {
        NonZeroI32::new(d).map(Divisor)
    }

    /// The certified-nonzero value.
    #[inline]
    pub fn get(self) -> i32 {
        self.0.get()
    }
}

/// The conceded residue: the **one** dividend/divisor pair a nonzero [`Divisor`] still cannot make total —
/// `i32::MIN / -1`, whose quotient `+2^31` does not fit in `i32`. This being a distinct, *runtime-checked* error
/// (not a type-carried impossibility) is the finding: the nonzero witness carries div-by-zero-freedom but not
/// overflow-freedom.
#[derive(Debug, PartialEq, Eq)]
pub struct DivOverflow;

/// Div-by-zero-free division. The [`Divisor`] type makes the zero panic **unwritable**; the `i32::MIN / -1`
/// overflow is caught here and returned as `Err(DivOverflow)` rather than panicking — so this function is
/// **total** (never panics), but only by relocating the second panic source to a runtime `Result`. A `-> i32`
/// signature would be an over-claim (it would panic on `MIN / -1`); this signature is the honest one.
///
/// `Err(DivOverflow)` **iff** `a == i32::MIN && d == -1` (zero is already excluded by [`Divisor`]).
#[inline]
pub fn divide(a: i32, d: Divisor) -> Result<i32, DivOverflow> {
    a.checked_div(d.get()).ok_or(DivOverflow)
}

/// Div-by-zero-free remainder — the same two-panic structure as [`divide`] (`% 0` panics; `i32::MIN % -1`
/// overflows), foreclosing the zero source via the same [`Divisor`] witness and relocating the overflow source to
/// `Err`. Included to show the nonzero witness serves the whole zero-divisor family, not one operator.
///
/// `Err(DivOverflow)` **iff** `a == i32::MIN && d == -1`.
#[inline]
pub fn remainder(a: i32, d: Divisor) -> Result<i32, DivOverflow> {
    a.checked_rem(d.get()).ok_or(DivOverflow)
}
