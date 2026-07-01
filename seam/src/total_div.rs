//! # Thirty-fourth seam — TOTAL DIVISION: the salvage of #33, concentrating the overflow I over-conceded.
//!
//! *(Project ordinal: the thirty-fourth primitive. The **direct salvage of #33** (`divisor`), mirroring the
//! #31→#32, #29→#30, and #26→#27 reject→salvage rhythm. #33 was SOUND but its §6 deliverable was REJECTED (r40,
//! the rejection mode's 8th firing): I **over-conceded** — I filed the `i32::MIN / -1` overflow as an *undecidable
//! whole-program range-analysis* residue, when it is a **decidable point-exclusion**. `divide` overflows on
//! **exactly** `a == i32::MIN && d == -1`, whose divisor-side conjunct lives on the very side the seam already
//! refines — so a witness `d ∉ {0, -1}` makes `a / d` **total for every `a`**, via a `Θ(1)` construction check
//! identical to the nonzero one. This seam **builds that witness** and asks the panel to confirm both of division's
//! panic sources genuinely concentrate, closing the over-concession. NOT a fourth-domain bid; the #33 domain finding
//! stands — panic-freedom still FOLDS into application-correctness. This corrects the *residue line*, not the
//! domain.)*
//!
//! ## The primitive — a divisor certified `∉ {0, -1}`, vending genuinely total `-> i32` division
//!
//! [`TotalDivisor`] is minted only by [`TotalDivisor::new`] (`None` iff `d == 0 || d == -1`), `Copy`. Because the
//! witness excludes **both** `0` (div-by-zero) and `-1` (the overflow's divisor-side conjunct), [`total_divide`]
//! and [`total_remainder`] return a plain `-> i32`: **they cannot panic for any `a: i32`.** No `Result`, no
//! `.unwrap()`, no panic path — the exact thing #33 could not express (its `divide` returned `Result`, forcing
//! callers of provably-total divisions to write a panic-inducing `.unwrap()`).
//!
//! ## Why a *finite* exclusion suffices (and why it costs nothing safe)
//!
//! `a / b` panics on precisely two inputs: `b == 0`, and `a == i32::MIN && b == -1`. The panic set is **finite on
//! the divisor side** — `{0, -1}` — so excluding those two values at construction closes *both* sources for every
//! dividend. And excluding `-1` removes no safe capability: `a / -1 == -a`, and the **only** `a` for which `a / -1`
//! overflows (`a == i32::MIN`) is *exactly* the one `a` for which `-a` overflows too. Division-by-`-1` was never
//! safely more expressive than negation; the witness drops the hazard, not a use case.
//!
//! ## What this HOLDS — and the three things it does NOT claim (dodging the #27 over-claim trap, as #32 did)
//!
//! - **HOLDS: both of division's panic sources concentrate at construction (`Θ(1)`, no SMT).** Total `-> i32`
//!   division is realized; the r40 over-concession is closed — division's overflow was a decidable corner, not an
//!   undecidable residue.
//! - **Does NOT reopen a fourth domain.** The #33 finding stands: panic-freedom FOLDS into the
//!   application-correctness basis (#19/#21); a panic is defined behaviour, never UB. This seam sharpens *division's
//!   residue*, it does not make totality a soundness domain.
//! - **Does NOT generalize the trick to arbitrary `+`/`*`.** Division is special: its panic set is **finite** and
//!   one conjunct sits on the refined side. Integer `+`/`*` overflow depends on **both** operands' full ranges — a
//!   genuine range-analysis problem (SPARK's prover), which stays the honest undecidable NONE.
//! - **Concentration stays value-threaded (the unchanged #33 prior-art cost).** The witness collapses divisors
//!   *threaded by value* through [`TotalDivisor`]; a divisor proven `∉ {0, -1}` only by a **path condition** (a
//!   computed value) still forces a runtime `Option` or a prover.
//!
//! ## HYPOTHESIS (this is the artifact commit; the cold panel adjudicates, not the builder)
//!
//! Predicted **SEAM-COSTS [horn3]** that HOLDS (or **SEAM-HOLDS**) — the salvage of #33, mirroring #30 and #32:
//! both panic sources concentrated at the construction check, total division vended, the over-concession closed,
//! with the three non-claims above conceded up front. Lean `total_div_both_sources_closes` (a `..._closes`).
//!
//! ### CF1 — you cannot divide by a raw integer; the divisor must pass the `∉ {0, -1}` construction check:
//! ```compile_fail
//! # use seam_spsc::total_div::total_divide;
//! let _ = total_divide(10, 3);   // ERROR: expected `TotalDivisor`, found integer — no un-checked divisor
//! ```

/// A divisor certified `∉ {0, -1}` — **both** of integer division's panic sources, foreclosed at one construction
/// check. `Copy`, so one certified divisor serves many total divisions. Wraps a plain `i32` behind the invariant;
/// the invariant (not the representation) is the point. Because `-1` is excluded, `i32::MIN / d` cannot overflow,
/// and because `0` is excluded it cannot divide by zero — so division by a `TotalDivisor` is total for every `i32`.
#[derive(Clone, Copy)]
pub struct TotalDivisor(i32);

impl TotalDivisor {
    /// Mint a total divisor, rejecting **both** `0` (div-by-zero) and `-1` (the `i32::MIN / -1` overflow's
    /// divisor-side conjunct) at the single concentrated check. `None` **iff** `d == 0 || d == -1`.
    #[inline]
    pub fn new(d: i32) -> Option<Self> {
        if d == 0 || d == -1 {
            None
        } else {
            Some(TotalDivisor(d))
        }
    }

    /// The certified value, guaranteed `∉ {0, -1}`.
    #[inline]
    pub fn get(self) -> i32 {
        self.0
    }
}

/// **Total** integer division: `a / d` for a divisor certified `∉ {0, -1}`. Returns a plain `-> i32` because it
/// **cannot panic for any `a: i32`** — the divisor witness forecloses both div-by-zero (`d != 0`) and the
/// `i32::MIN / -1` overflow (`d != -1`). This is what #33's `divide` could not express without a `Result` and a
/// panic-inducing `.unwrap()`; here the panic is gone, not relocated.
#[inline]
pub fn total_divide(a: i32, d: TotalDivisor) -> i32 {
    a / d.get()
}

/// **Total** integer remainder: `a % d` for a divisor certified `∉ {0, -1}`. Total for the same reason — `%` panics
/// only on `d == 0` or `i32::MIN % -1`, both excluded by the witness. (`i32::MIN % -1` has the defined answer `0`,
/// which #33's `remainder` was forced to return as `Err`; here it never arises, because `-1` is not a divisor.)
#[inline]
pub fn total_remainder(a: i32, d: TotalDivisor) -> i32 {
    a % d.get()
}
