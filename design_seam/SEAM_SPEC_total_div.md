# Seam spec — total division (the salvage of #33, concentrating the overflow I over-conceded) (re-import rule under attack)

This is a **design proposal** for a thirty-fourth *seam*, the **direct salvage of the thirty-third** (`divisor`),
mirroring the #31→#32, #29→#30, and #26→#27 reject→salvage rhythm. #33 was SOUND but its §6 deliverable was
**REJECTED** (r40, the rejection mode's 8th firing): I **over-conceded** — I filed the `i32::MIN / -1` overflow as an
*undecidable whole-program range-analysis* residue. The panel's correction: `divide` overflows on **exactly**
`a == i32::MIN && d == -1`, whose divisor-side conjunct lives on the very side the seam already refines, so a witness
`d ∉ {0, -1}` makes division **total for every `a`** via a `Θ(1)` construction check — no range analysis, no SMT.
This seam **builds that witness** and asks the panel to confirm both of division's panic sources genuinely
concentrate, closing the over-concession. It is **NOT a fourth-domain bid**: the #33 domain finding stands
(panic-freedom FOLDS into application-correctness); this corrects the *residue line*, not the domain.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/total_div.rs`, default
`no_std` build — `core` only, **no `unsafe`**, self-contained). Tests pass (default), including a `compile_fail`
doctest (CF1: division by a raw, un-checked integer does not compile).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing; `i32` arithmetic where `/` and `%` panic on `d == 0` and on
`i32::MIN {/, %} -1` in **all** build modes; `checked_div`/`checked_neg` semantics. Do not attack `i32` arithmetic
semantics or the panic set of `/`/`%`. Attack the **`TotalDivisor` re-import rule** and the §6 deliverable: *does the
`∉ {0, -1}` witness genuinely concentrate BOTH of division's panic sources, vending total `-> i32` division; and are
the three non-claims (no fourth domain; no generalization to `+`/`*`; value-threaded only) the right honest bounds —
or does the salvage now over-claim, the way #27 did after salvaging #26?*

## 1. The primitive

**Total integer division.** A `TotalDivisor(i32)` is minted only by `TotalDivisor::new(i32) -> Option<Self>` (`None`
iff `d == 0 || d == -1`), `Copy`, witnessing `d ∉ {0, -1}` thereafter. `total_divide(a: i32, d: TotalDivisor) -> i32`
and `total_remainder(a, d) -> i32` compute `a / d.get()` / `a % d.get()` and **cannot panic for any `a`** — the
witness forecloses both div-by-zero (`d != 0`) and the `i32::MIN / -1` overflow (`d != -1`). No `Result`, no
`.unwrap()`, no panic path.

## 2. The true precondition (cathedral side)

- **Totality of division (the thing this seam carries):** every `total_divide`/`total_remainder` has a divisor
  `∉ {0, -1}`, so the result exists in `i32` for every dividend. Both panic sources concentrate at the one
  construction check, `Θ(1)`, no SMT. This is the fix #33 owed: division's overflow is a **decidable point-exclusion**,
  not the undecidable residue #33's §2/§4 claimed.
- **NOT carried (conceded up front, dodging the #27 over-claim trap):**
  1. **No fourth domain.** The #33 finding stands — panic-freedom FOLDS into the application-correctness basis
     (#19/#21); a panic is defined behaviour, never UB. This seam sharpens *division's residue*, it does not make
     totality a soundness domain.
  2. **No generalization to `+`/`*`.** Division is special: its panic set is **finite** and one conjunct is on the
     refined side. Integer `+`/`*` overflow depends on **both** operands' full ranges — a genuine range-analysis
     problem (SPARK's prover) — which stays the honest undecidable NONE.
  3. **Value-threaded only (the unchanged #33 prior-art cost).** The witness collapses divisors *threaded by value*;
     a divisor `∉ {0, -1}` only by a **path condition** (a computed value) still forces a runtime `Option` or a
     prover.

## 3. The witness — a finite exclusion that costs nothing safe

`TotalDivisor::new` rejects `{0, -1}`. This suffices because `a / b` panics only on `b == 0` or `a == i32::MIN &&
b == -1`, both foreclosed. And excluding `-1` removes **no safe capability**: `a / -1 == -a`, and the only `a` where
`a / -1` overflows (`a == i32::MIN`) is *exactly* the one where `-a` overflows too (`i32::MIN.checked_neg() == None`),
so negation already covers division-by-`-1` for every non-overflowing dividend. The witness drops the hazard, not a
use case. CF1: `total_divide(10, 3)` does not compile — `3` is an integer, not a `TotalDivisor`.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `TotalDivisor`-gated division vends `TotalDivisor::new`, `total_divide`, `total_remainder`. Safe
> code composes them with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Totality** — `total_divide`/`total_remainder` cannot panic for any `a: i32` (both `d == 0` and `d == -1`
>    unconstructible), returning a plain `-> i32`. Decidable, concentrated at construction, no SMT.
>
> It does **NOT** claim (conceded): a fourth soundness domain (panic-freedom folds); totality of `+`/`*` (arbitrary
> overflow needs range analysis, undecidable); nor whole-program concentration (a path-condition-`∉{0,-1}` computed
> divisor still needs a runtime `Option`).

## 5. Threat model

In scope: any **safe** path that (a) **reaches a panic** through the vended API — constructs a `TotalDivisor` whose
`total_divide`/`total_remainder` panics or overflows for some `a`, defeating the totality claim; (b) argues the
salvage **now over-claims** (the #27 failure mode) — e.g. that excluding `-1` *does* cost a safe capability, that the
finite-exclusion trick secretly needs more, or that "total division" hides an obligation; (c) argues the exclusion is
**degenerate / useless** (the divisor shape is rare, `wcet`-style); (d) argues the three non-claims are mis-drawn.
Out of scope: `i32` arithmetic semantics; the panic set of `/`/`%`; arbitrary `+`/`*` overflow (conceded
undecidable); the #33 domain finding (panic-freedom folds — already adjudicated).

## 6. The scoring rubric — soundness, then the salvage deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the `TotalDivisor` gate only *removes* reachable panics. Confirm
  `total_divide`/`total_remainder` are total for every `a: i32` (neither panic source constructible).
- **Horn 2 — decidable & proof-free.** The `∉ {0, -1}` check is a single `Option`-returning constructor, `Θ(1)`,
  no SMT. Confirm.
- **Horn 3 — concentration.** BOTH panic sources concentrate in the one construction check. Confirm they genuinely
  concentrate (this is the salvage's whole point — it is what #33 declined to carry).

Then **the salvage deliverable** — the point of the phase. Rule on each:

1. **Does the `∉ {0, -1}` witness genuinely vend TOTAL `-> i32` division (both sources closed), with no panic path?**
   The claim: yes — `d == 0` and `d == -1` unconstructible ⇒ `total_divide`/`total_remainder` total for all `a`.
   Confirm — or exhibit a **safe** path that panics/overflows.
2. **Does excluding `-1` cost anything SAFE?** The claim: no — `a / -1 == -a`, and the sole overflow (`a == i32::MIN`)
   coincides with negation's own overflow. Confirm — or show a safe program that needs division-by-`-1` and cannot
   use negation.
3. **Are the three non-claims the right honest bounds — or do I now OVER-claim (the #27 trap)?** The claim: this does
   NOT reopen a fourth domain, does NOT generalize to `+`/`*`, and stays value-threaded. Confirm the bounds are
   honest — or argue the salvage over-reaches (e.g. it implicitly claims more than division's finite corner, or the
   "no fourth domain" concession is wrong).
4. **Prior-art deflation (pre-registered).** `core::NonZero`; refinement types (Flux, Liquid Haskell, F\*, Dafny)
   for totality; SPARK's `division_check`/`overflow_check`; the classic `i32::MIN / -1`. Confirm the contribution is
   **building** the carriable half the r40 verdict identified — a two-value construction exclusion realizing total
   division — **not** a new mechanism. Or rule it a **redundant re-statement** of #33/`refined` #6 (a refinement with
   one more excluded value adds nothing).

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the `∉ {0, -1}` witness concentrate BOTH of division's panic sources and
vend genuinely total `-> i32` division with no soundness regression and no panic path; does excluding `-1` cost
nothing safe; are "no fourth domain / no `+`/`*` generalization / value-threaded" the honest bounds or do I now
over-claim (the #27 trap #32 dodged); and does it deflate onto `core::NonZero` / refinement-type folklore as the
*built* carriable half of #33 (closing the over-concession) — with no new domain, coordinate, or mode (N = 9; the
FAILED broken-seam tally stays 8; this is the salvage that should HOLD)?**
