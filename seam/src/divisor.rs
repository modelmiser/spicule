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
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_divisor.md, r40): SEAM-COSTS [horn3], SOUND — but the DELIVERABLE is
//! ## REJECTED (the rejection mode's 8th firing): I OVER-CONCEDED — the `MIN/-1` overflow is DECIDABLE, not
//! ## undecidable range-analysis, so "one source closes, one escapes" is WRONG for division — BOTH close.
//!
//! Horn-classifier: **SEAM-COSTS [horn3-concentration]** — sound and decidable, but concentration overstated and
//! a "total division" forces a consumer-written `.unwrap()` panic. Judge: **FAILED** on the §6 deliverable.
//! Soundness and implementation attacks both **conceded the rule holds** (no zero-`Divisor` mintable; `MIN/-1` is
//! `Err`, not a panic; a panic is not UB; the check is `Θ(1)`, no SMT) — so Horns 1/2/3 hold and this is **SOUND**
//! (tally STAYS 8). But the JUDGE rejected the deliverable, the method's **REJECTION mode, 8th firing** (after
//! #8/#24/#25/#27/#28/#29/#31).
//!
//! **What SURVIVED — the domain finding:** panic-freedom **FOLDS** into the application-correctness basis
//! (#19/#21), **not** a fourth soundness domain. A panic is *defined behaviour* (a clean abort, never UB), so
//! totality is a correctness property; prior-art confirms the fold as the historical default (SPARK's
//! `division_check`/`overflow_check` split; Liquid-Haskell's nonzero-`div`; `core::NonZero`), and expressiveness
//! explicitly granted it. So the **third** fourth-domain probe deflates (after `crdt` #21 fold, `wcet` #22 break);
//! the confirmed map (memory safety / information flow / liveness) still looks special.
//!
//! **THE REJECTION (I OVER-CONCEDED, refuted from my own §2):** `divide` overflows on **exactly** `a == i32::MIN
//! && d == -1` — a conjunction whose *divisor-side* conjunct lives on the very side this seam already refines. So
//! a witness `d ∉ {0, -1}` makes `a / d` **total for every `a`**, via a `Θ(1)` constructor check identical in cost
//! and character to the nonzero check — **no range analysis, no SMT.** My §2/§4 claim that the overflow residue
//! "needs whole-program range analysis, undecidable" is therefore **false for the primitive I actually chose**: it
//! is a **decidable corner I declined to carry**, not an irreducible one. The pedagogical conceit — "two
//! independent sources; one closes, one escapes; the honest partial result" — is **wrong for division: BOTH
//! sources close.** By returning `Result` instead of vending total `-> i32` division, the panic-freedom seam
//! forces callers of provably-total divisions (`x / 60`, `sum / count`) to write `.unwrap()` — **reintroducing the
//! exact panic it exists to remove**. The `remainder` corner sharpens it: `i32::MIN % -1` has the defined answer
//! `0`, yet the API returns `Err`. Refuted from my own framing — §2 explicitly lumped division overflow with
//! "arbitrary `+`/`*`".
//!
//! **META — the SAME tic as #31, exactly inverted onto a new domain:** #31 over-conceded *guardedness* as an
//! undecidable NONE when it was a decidable marker; #33 over-conceded the *`MIN/-1` overflow* as undecidable
//! range-analysis when it is a decidable point-exclusion. Both are the project's unifying failure — **I flee the
//! MIDDLE (a decidable partial cost) toward whichever EXTREME is cleaner** — here in its *over-concession*
//! direction (toward "undecidable"), the 8th rejection. The natural **salvage (#34, mirroring #31→#32, #29→#30,
//! #26→#27):** a `TotalDivisor` witnessing `d ∉ {0, -1}` that vends total `-> i32` division for the common case,
//! proving the overflow residue concentrates too — leaving only *arbitrary* `+`/`*` range analysis as the genuine
//! undecidable NONE.
//!
//! **What the panel MARKED (tradeoffs, NOT horn-drops — the seam is sound):** (1) **concentration overstated** —
//! the newtype collapses only divisors *threaded by value* through `Divisor` to one boundary; a
//! provably-nonzero-but-*computed* divisor (nonzero by path condition) still forces a redundant runtime `Option`
//! or a prover, exactly the LH/SPARK use case (prior-art). (2) **standing ergonomic tax** — the mandatory
//! `Result<_, DivOverflow>` is worse than `/`/`checked_div` for one-shot shapes, and the `.unwrap()` discharge
//! reintroduces a panic (ergonomics; shared with expressiveness). No new domain, coordinate, or mode (N stays 9);
//! tally STAYS 8 (sound — a deliverable rejection). Lean `divisor_nonzero_division_closes` (a `..._closes`: the
//! div-by-zero fragment genuinely closes; the owed `-1` exclusion and the rejection live in the prose).
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
