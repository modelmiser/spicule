# Seam spec — cost-typed computation (re-import rule under attack)

This is a **design proposal** for a twenty-second *seam*, the **second probe aimed at a fourth soundness
domain** — after the convergence probe (`SEAM_SPEC_crdt.md`) *deflated* (its gap was the generic
unchecked-typeclass-law phenomenon, projecting onto `restore`'s application-correctness basis). This one
targets **real-time / worst-case execution time (WCET)** — a cleaner candidate, because a cycle budget is a
**quantitative resource bound**, not an algebraic law.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/wcet.rs`, the
default `no_std` build — `core`-only, **no `unsafe`**, like the `secret`/`crdt` semantic-domain seams).

---

## 0. The floor (stipulated, not under attack)

Rust's type system as shipped, **plus** the eighth seam's sealed type-level `Nat`/`Lt` machinery (`typed_nat`)
as a *given* building block: `C: Lt<S<D>>` is a compile-time proof that `C ≤ D`, discharged by trait
resolution. The abstract machine models **no notion of time or cycles** — so the project's usual "can safe
code reach UB?" is *trivially* no (no `unsafe`). Do not attack trait resolution or the sealed `Nat`. Attack
the **re-import rule** (the WCET-bound claim) and the §6 deliverable.

## 1. The primitive

A **cost-typed computation**: `Timed<C, T>` is a value of type `T` produced by a computation whose worst-case
cost is the type-level numeral `C`, in a **"one unit per `step`"** cost model. `pure` is cost `0`; `step`
charges one unit and advances `C`; `within::<D>` returns the value **only if** `C ≤ D` (a compile-time proof).
The danger is a **deadline miss** — a computation that runs past its budget — a real-time hazard, not UB.

## 2. The true precondition (cathedral side) — elapsed cycles within the deadline

The computation's **real worst-case elapsed time** (cycles on the target hardware) is within the deadline
`D`. This is a quantitative bound on a *physical* resource. The type-level `C` is a *static op-count* offered
as a stand-in for it.

## 3. The witness — type-level cost accounting (the `typed_nat` machinery)

`Timed<C, T>` carries `C` as `PhantomData` over the sealed `Nat`. `step` advances `C` to `S<C>`; `within::<D>`
is gated on `C: Lt<S<D>>`. So the floor **does** carry the static op-count: over-budget straight-line code
fails to type-check, and the cost is unforgeable (sealed `Nat`). The **open question** is whether the
op-count is a *witness* for the real-time bound or merely an *unsound proxy* for it.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Timed<C, T>` vends a **deadline-bounded computation**: safe code composes `step`s with no
> `unsafe`, and `within::<D>` returns the value **only when the computation meets its deadline**, because the
> type tracks the cost and proves `C ≤ D`.

Structural stoppers (claimed): `C` accumulates by trait resolution; `within` enforces `C ≤ D` at compile
time; the value escapes only through `within` (no public field), so no result bypasses the deadline proof.

## 5. Threat model

In scope: any **safe** path where a computation that *type-checks* as within-budget **misses its real
deadline** — most directly, a single `step` whose closure does unbounded work (a million-iteration loop, a
cache-missing load, a division), charged as one unit. Also: whether the cost-model axiom ("one `step` = one
cycle") is *sound* for real hardware, and whether data-dependent loops can be expressed at all. Out of scope:
the AM's lack of a time model (stipulated §0); the `typed_nat` machinery's correctness; concrete hardware
WCET-analysis tools (aiT, cache models) — the object is the **re-import's real-time claim**, not a WCET tool.

## 6. The scoring rubric — soundness, then the SECOND FOURTH-DOMAIN deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Exhibit safe code that type-checks as within-budget but **misses the real
  deadline** — the spec's witness: `pure(()).step(|_| for _ in 0..1_000_000 {…})` is cost `1` and "meets" a
  one-unit deadline, yet does a million units of real work. Confirm this is a real-time soundness break in
  safe code — or argue the op-count *is* a faithful bound.
- **Horn 2 — decidable & proof-free.** `pure`/`step`/`within` are ordinary safe calls; `C ≤ D` resolves by
  trait selection. Confirm — *but note whether Horn 2 passes only because the strong property (real-time) is
  never actually checked* (the `secret`/`crdt` pattern: a decidable seam that type-checks precisely because
  it does not discharge the semantic property).
- **Horn 3 — concentration.** The accounting is concentrated in `Timed`; the consumer writes no cost
  bookkeeping. Confirm — or note the caller still carries the undischargeable obligation that each `step` is
  *actually* one unit.

Then **the second fourth-domain deliverable** — the point of the phase. Rule on each:

1. **Floor-mismatch — a proxy that is not the property.** Does the floor carry only a **static op-count**,
   which is an **unsound proxy** for real elapsed time (a `step` may hide unbounded work; one primitive is
   not one cycle; cache/pipeline are invisible)? If so, real-time/WCET has **no closing seam** — **SEAM-BREAKS
   [floor-mismatch]** — and *more sharply* than `secret`: the secret seam's proxy foreclosed *some* leaks
   (sound but incomplete); here the op-count can be **arbitrarily far** from the truth (unsound). Confirm.
2. **Does it DEFLATE, like convergence?** Rule on three projections: (a) the **static op-count** onto
   `typed_nat` (#8) — the witness is *literally* `typed_nat`'s sealed `Nat`/`Lt` (type-level resource
   accounting), reused; (b) the **trusted cost-model** ("one step = one cycle") onto an **`mmio`-shaped axiom
   (#2)** — but is it *faithful* like MMIO's hardware-register axiom, or an **unsound fiction** (real cycles
   ≠ op-count)? (c) the **data-dependent-loop gap** onto the liveness domain's **partial-match** (a `while`
   has no static `C`). If all three project onto existing findings, WCET **does not open a fourth domain** —
   a second deflation, by a *different mechanism* than convergence's.
3. **Or is real-time a genuine fourth domain?** Argue the contrary: the *quantitative resource-bound* basis
   (cycles, a cost-effect) is genuinely new — distinct from memory safety, from IFC's leakage coordinate, and
   from liveness's progress — even if no Rust floor carries it. Is WCET's quantitative coordinate **the same
   as IFC's** (both quantitative bounds affinity cannot carry), or distinct? Prior art to weigh: WCET analysis
   (Wilhelm et al. 2008), schedulability (rate-monotonic, EDF), resource/cost type systems (amortized
   resource analysis — Hofmann–Jost, RaML; sized types; gas/fuel metering — EVM, Erlang reductions).
4. **Verdict mode + the meta-question.** Is this **floor-mismatch** (no new mode) and/or **projects-onto-
   basis** (deflation)? Does N stay 9? And the meta-observation for the record: **two** fourth-domain
   candidates (convergence, WCET) have now been probed and **both deflated**, by different mechanisms — does
   that suggest the three confirmed domains (memory safety / information flow / liveness) are *special*, or
   merely that these two candidates were ill-chosen?

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
deadline-miss sequence). And state, explicitly: **does the floor carry only a static op-count that is an
unsound proxy for real elapsed time (SEAM-BREAKS [floor-mismatch], sharper than `secret`); does WCET deflate —
its static cost onto `typed_nat` (#8), its cost-model onto an `mmio`-shaped-but-unsound axiom (#2), its
data-dependent gap onto liveness's partial-match — rather than opening a genuine fourth domain; is real-time's
quantitative basis distinct from IFC's quantitative coordinate; does N stay 9; and what does it mean that two
fourth-domain candidates in a row deflated?**
