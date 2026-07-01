# Seam spec — total(-ish) division (the panic-freedom / totality probe) (re-import rule under attack)

This is a **design proposal** for a thirty-third *seam*, the **first pivot off the session family** (connectives
#18–#28, completion #29/#30, recursion #31/#32 — fully mapped). It is a fresh **fourth-domain candidate**:
**panic-freedom / totality** — the SPARK / Frama-C / refinement-type "absence of run-time errors" property (no
div-by-zero, no overflow, no out-of-bounds). Both previous fourth-domain probes DEFLATED — `crdt` #21 folded into
application-correctness, `wcet` #22 broke unsound — so the confirmed map (memory safety / information flow /
liveness) looks special. This seam asks whether "the program **aborts** at run time" is a fourth **soundness**
domain, or folds.

The primitive is **integer division**, chosen because `a / b` panics on **two independent** inputs: div-by-zero
(`b == 0`) and signed overflow (`i32::MIN / -1`). That two-source structure lets one operator show, self-contained,
that **one source closes and one escapes** — the honest partial result.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/divisor.rs`, default
`no_std` build — `core` only, **no `unsafe`**, self-contained over `core::num::NonZeroI32`). Tests pass (default),
including a `compile_fail` doctest (CF1: division by a raw, un-checked integer does not compile).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing; `core::num::NonZeroI32` (a shipped refinement type — a `u32`/`i32`
the constructor has proven `!= 0`, `Option`-returning `new`); `i32::checked_div` / `checked_rem` (returning `None`
on div-by-zero **or** `i32::MIN / -1`). Division `/` and `%` panic on those inputs in **all** build modes. Do not
attack `NonZeroI32`, `checked_div`/`checked_rem`, or `i32` arithmetic semantics. Attack the **`Divisor` re-import
rule** and the §6 deliverable: *does the nonzero witness genuinely concentrate div-by-zero-freedom; is the
`MIN / -1` residue correctly conceded rather than hidden; and does panic-freedom fold into application-correctness
rather than opening a fourth soundness domain?*

## 1. The primitive

**Div-by-zero-free integer division.** A `Divisor(NonZeroI32)` is minted only by `Divisor::new(i32) -> Option<Self>`
(`None` iff `0`), `Copy`, witnessing a nonzero divisor thereafter. `divide(a: i32, d: Divisor) -> Result<i32,
DivOverflow>` and `remainder(a, d) -> Result<i32, DivOverflow>` are `a.checked_div(d.get())` / `checked_rem`,
returning `Err(DivOverflow)` **iff** `a == i32::MIN && d == -1` (zero already excluded). Div-by-zero cannot be
written; the `MIN / -1` overflow is relocated to the `Err` arm, not carried by the type.

## 2. The true precondition (cathedral side)

- **Div-by-zero-freedom (the thing this seam carries):** every `divide`/`remainder` has a **nonzero** divisor,
  because a `Divisor` cannot be constructed from `0`. The check is concentrated once, at `Divisor::new`; downstream
  the type is the witness. This holds on the **real** floor — `NonZeroI32` is shipped — with no SMT and no
  hypothetical solver (unlike the refined index #6, which stipulates a Flux/Liquid-Haskell floor).
- **Overflow-freedom (NOT carried, conceded residue):** a valid nonzero `Divisor` may be `-1`, and the dividend may
  be `i32::MIN`, so `MIN / -1` is not foreclosed by the nonzero type. The general overflow question (arbitrary
  `+`/`*`) needs whole-program **range analysis** (SPARK's prover, refinement over QF-LIA) — undecidable on the bare
  floor. A `-> i32` "total division" signature would be an over-claim; the `Result` signature is the honest one.

## 3. The witness — one shipped refinement type, `NonZeroI32`

`Divisor` wraps `NonZeroI32`. Its constructor performs the single zero-check (`Option`); `get` returns the certified
value. `divide`/`remainder` route through `checked_div`/`checked_rem`, so the ONLY residual `None`/`Err` is the
`MIN / -1` corner. There is no way to reach `/` with a zero (or raw, unchecked) divisor: CF1 — `divide(10, 3)` — does
not compile (`3` is an integer, not a `Divisor`). The div-by-zero fragment is therefore closed at compile time modulo
the one runtime construction check, exactly as a refinement type closes a bounds check.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Divisor`-gated division vends `Divisor::new`, `divide`, `remainder`. Safe code composes them
> with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Div-by-zero-freedom** — `divide`/`remainder` cannot be called with a zero divisor (no `Divisor` exists for
>    `0`; no raw integer coerces to `Divisor`). The first panic source of `/`/`%` is **unwritable**. Decidable,
>    concentrated at construction, on the real floor (no SMT).
>
> It does **NOT** guarantee (conceded residue, deliberately): **overflow-freedom** — `divide(i32::MIN, -1)` is the
> `Err(DivOverflow)` arm, a runtime `Result` check, not a type-carried impossibility; and no whole-program totality
> (arbitrary overflow needs range analysis, undecidable here).

## 5. Threat model

In scope: any **safe** path that (a) **reaches a panic** through the vended API — gets `/`/`%` to panic (div-by-zero
or overflow) with no `unsafe`, defeating the advertised div-by-zero-freedom or catching the seam hiding the overflow
residue; (b) argues the closing fragment is **degenerate** — that one essentially never divides by a
statically-unknown value separately proven nonzero, so the concentration buys nothing useful (the `wcet`-style dock);
(c) argues panic-freedom is **not a fourth soundness domain** — that a panic is defined behaviour and totality folds
into the application-correctness basis (#19/#21), so this is a DEFLATION not a new domain; (d) argues the seam
**over-claims** — that "div-by-zero closes, overflow is the residue" mis-draws the line, or that the mechanism is
just `refined` #6 re-stated (a nonzero refinement) with no domain contribution. Out of scope: `NonZeroI32`,
`checked_div`/`checked_rem`, `i32` semantics; general overflow/range analysis (conceded undecidable).

## 6. The scoring rubric — soundness, then the domain deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the `Divisor` gate only *removes* reachable panics (never admits UB —
  a panic is not UB). Confirm div-by-zero is genuinely unreachable, and that the `MIN / -1` overflow surfaces as
  `Err`, not a panic.
- **Horn 2 — decidable & proof-free.** The nonzero check is a single `Option`-returning constructor, `Θ(1)`, no SMT.
  Confirm.
- **Horn 3 — concentration.** Div-by-zero-freedom concentrates in the `Divisor` construction check — the whole
  program's zero-divisor obligation collapsed to one boundary. Confirm it genuinely concentrates the fragment.

Then **the domain deliverable** — the point of the phase. Rule on each:

1. **Does the nonzero witness genuinely foreclose div-by-zero while the `MIN / -1` overflow is honestly conceded?**
   The claim: yes — no `Divisor` for `0`, and `divide` returns `Err(DivOverflow)` on `MIN / -1` rather than
   panicking or hiding it. Confirm — or exhibit a **safe** path that panics (div-by-zero reached, or overflow
   surfacing as a panic the seam claimed to have handled).
2. **Is the closing fragment useful, or degenerate?** The claim: div-by-zero-freedom on the real floor (shipped
   `NonZero`) is a genuine, non-hypothetical concentration. Confirm — or argue it is degenerate (`wcet`-style: the
   statically-unknown-but-proven-nonzero divisor is a rare shape; real division either has a literal nonzero divisor,
   trivially safe, or a runtime one already needing an `Option`, so the witness adds little).
3. **Does panic-freedom open a fourth soundness domain, or fold?** The claim: it FOLDS — a panic is defined
   behaviour (a clean abort, never UB), so totality is a **correctness** property belonging to the
   application-correctness basis (#19/#21), NOT a peer of memory-safety/IFC/liveness; the checkable fragment is
   refinement-typed (like `refined` #6), the general case needs a prover (SPARK). Confirm the fold — or argue
   panic-freedom is genuinely a fourth soundness coordinate with its own basis (in which case rule what that basis
   is and what it costs).
4. **Prior-art deflation (pre-registered).** `NonZero*` in `core`; refinement types (Flux, Liquid Haskell, F\*,
   Dafny) for totality; SPARK / Frama-C "absence of run-time errors"; the classic `MIN / -1` overflow. Confirm the
   contribution is the **domain finding** (panic-freedom folds; its real-floor fragment is div-by-zero via shipped
   `NonZero`), **not** a new mechanism — and that it overlaps `refined` #6 in mechanism while differing in domain.
   Or rule it a **redundant re-statement** of `refined` (a nonzero refinement adds nothing new).

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the `Divisor` nonzero witness concentrate div-by-zero-freedom on the real
floor with no soundness regression; is the `MIN / -1` overflow correctly conceded as a runtime residue rather than
hidden; is the closing fragment useful or degenerate; and does panic-freedom FOLD into application-correctness
(a DEFLATION, `..._factors`, like `crdt`) rather than opening a fourth soundness domain — with no new domain,
coordinate, or mode (N = 9; the FAILED broken-seam tally stays 8; the predicted result is SEAM-COSTS [horn3] holding
for the div-by-zero fragment, or a deflation)?**
