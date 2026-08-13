# Foreign-model replication — the monoculture control (run report)

**Run date:** 2026-08-12 · **Foreign families:** `gpt-5.6-sol-high` (GPT-5.6 Sol, high
effort, OpenAI) and `gemini-3.1-pro` (Gemini 3.1 Pro, Google) · **Transport:** Cursor
CLI (`FOREIGN_PROVIDER=cursor-cli`, subscription auth — no raw API key; see
`orchestrator/foreign_replication.py` docstring for the disclosed transport deltas) ·
**Sample:** the 8-seam stratified sample, 7 foreign calls per seam (5 attackers ∥,
judge, horn-classifier), 56 calls per family, one gauntlet per seam. Raw transcripts
under `runs_foreign_gpt-5.6-sol-high/` and `runs_foreign_gemini-3.1-pro/` (untracked,
like `runs/`); machine-readable results in each dir's `agreement.json` + `run_meta.json`.

## Headline

Outcome-class agreement: **GPT-5.6 Sol 6/8, Gemini 3.1 Pro 5/8** — in both families,
both recorded HOLDS were demoted to COSTS, and Gemini escalated one recorded COSTS to
BREAKS. Within that: **both families reproduced all four recorded BREAKS exactly,
horn labels included (8/8 break-cells [horn1])** — the strongest cells in the table,
with the caveat that all four recorded breaks share the modal horn label. On the
full-sample runs no recorded HOLD was ruled BREAKS; an earlier single-seam run did
rule one BREAKS, and was later vindicated (see the provenance note and lineage).

| seam | recorded (Claude) | GPT-5.6 Sol | Gemini 3.1 Pro |
|---|---|---|---|
| spsc | SEAM-HOLDS [none] | COSTS [horn1] ✗ | COSTS [horn3] ✗ |
| spin_lock | SEAM-HOLDS [none] | COSTS [horn3] ✗ | COSTS [horn3] ✗ |
| slab | SEAM-COSTS [other] | COSTS [none] ✓ | COSTS [none] ✓ |
| refined | SEAM-COSTS [horn2] | COSTS [other] ✓ | **BREAKS [horn1] ✗** |
| intrusive | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| secret | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| sync_cell | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |
| ordered_locks | SEAM-BREAKS [horn1] | BREAKS [horn1] ✓ | BREAKS [horn1] ✓ |

## The divergences, read closely

**Both HOLDS demoted, in both families — mostly, but not entirely, boundary pricing.**
Three of the four demotion cells are horn-3: on spin_lock, GPT's own *judge* ruled
SURVIVED before its classifier emitted horn-3; Gemini emitted horn-3 on both HOLDS.
(The horn-3 rationales differ in substance — GPT spin_lock prices an implicit `Sized`
bound and misuse-resistance gaps, Gemini spsc prices allocation and global storage,
and only Gemini spin_lock actually cites proof concentration — so "rubric-boundary
pricing" is our gloss over heterogeneous cost objections, not the transcripts' shared
stated reason.) The fourth cell, GPT spsc, is horn-1 and spec-defect-driven (the P4
wording objection below). On the two full-sample runs, no verdict on either HOLDS
seam claimed a UB path (all four `horn1_ub_path` fields are empty) — **but the
earlier single-seam GPT run did claim one** (SEAM-BREAKS via covariance laundering),
and the lineage below vindicated that attack as real against the v1 text. So the
honest summary is: full-sample demotions were argued at the HOLDS/COSTS boundary,
while one foreign run — later vindicated in-family — claimed a genuine UB path
against the flagship HOLD. The transport deltas (prompt prepending, no temperature
pin) and single-draw sampling are unquantified contributors to all of this.

**GPT's stable spsc objection was a specification-wording defect** (both GPT runs):
`Send` + `Arc<Mutex<Producer<T>>>` permits serialized migration across thread
identities, so thread-affinity does not discharge **literal** P4 ("one producer thread
*ever*") — explicitly "does not launder UB." This was repaired and re-adjudicated; see
the lineage section.

**Gemini's escalation on refined (COSTS[horn2] → BREAKS[horn1])** is the one two-way
divergence: its judge ruled the seam's central premise — a relational bound that is
simultaneously *brand-free* and *statically erased* — contradictory across function
boundaries ("an unparameterized value existentially hides its bound... impossible
without a runtime check or a viral type parameter"). That is a realizability claim
about the seam's central mechanism, not a severity dial; the recorded verdict priced
the same defect region as a horn-2 cost. **Re-examined line-by-line and adjudicated
out-of-family (2026-08-12, `REFINED_ESCALATION_ADJUDICATION.md`): the recorded
COSTS [horn2] is SUSTAINED.** Gemini's UB path requires a configuration the spec
does not vend (no-runtime-check AND no type-level relation; the spec discloses one
of the two in each configuration), and a blind GPT-5.6 adjudicator ruled the
mismatched call panics in the model and is ill-typed on the floor. The escalation's
durable content is kept: the erasure dichotomy (runtime check XOR type-level
relation) and the axis-#5 clarification that a shared structural length variable is
not a generative brand. The earlier "foreign panels never upgrade" reading remains
corrected — the upgrade happened; it just did not survive adjudication.

**Horn-label accounting, symmetrically.** The break agreement is horn-exact (8/8
cells [horn1]) — though all four recorded breaks share the modal label, so four-way
identity is weaker evidence than it looks. The COSTS agreements are horn-INEXACT
(slab: recorded [other] vs foreign [none]/[none]; refined: recorded [horn2] vs GPT
[other]) — the 6/8 and 5/8 figures score outcome class only. If horn concordance
counts as agreement depth on the breaks, horn discordance must be reported on the
agreements; it is, here.

## Foreign-side verdict instability (provenance note)

A first single-seam GPT run (same day, before the full run) returned
**SEAM-BREAKS[horn1]** on spsc, headlined by a covariance/lifetime-laundering attack.
On the full-sample re-run that attack was ruled argued-past and did not recur; the P4
objection appeared in both runs. **The first run's spsc transcripts were overwritten
by the full run before being staged** — an operator error, disclosed; the run-1
verdict line and judge JSON survive verbatim in the session record only, so every
run-1-dependent claim in this report (including "the P4 objection recurred in both"
and the two-family independence of the covariance find) rests on that session record,
not on disk. At the time this looked like foreign tier drift. **The lineage work
below vindicated the run-1 attack as real** — a strong corrective to treating a
non-recurring finding as noise. Note it is also direct evidence on sampling: the one
foreign cell ever drawn twice flipped tiers (BREAKS → COSTS), so foreign single-draw
instability is unquantified and plausibly higher than the recorded Claude-side
1-of-3 base rate.

## The spsc lineage (v1 → v5): repairing what the panels surfaced

Post-replication, the flagship seam's spec was amended and re-entered the Claude-side
gauntlet as new spec files (frozen v1 untouched). Each step killed one named defect
and surfaced the next:

| step | change | gauntlet verdict | what it surfaced |
|---|---|---|---|
| v1 (frozen) | — | HOLDS (3-streak, recorded) | — |
| v2 | P4 restated as role exclusivity (the GPT wording objection) | COSTS [horn1] | **P4-v2 held under all five attacks**; new: `N` unbounded+erased (P3 vacuous at N=0), teardown clause missing |
| v3 | `N >= 2` const bound at split; P6 teardown | **BREAKS [horn1]** | **unspecified VARIANCE**: covariant handles launder a dangling reference — the run-1 GPT attack, independently rediscovered in-family |
| v4 | invariance pinned in `T` | **BREAKS [horn1]** | **dropck ownership**: v4's own `PhantomData<*mut T>` marker gives invariance but not drop-ownership; under `#[may_dangle]`, P6's glue drops a payload whose borrow died |
| v5 | plain `Drop` (no `#[may_dangle]`) + `PhantomData<T>` ownership marker | **SEAM-COSTS [horn2-decidability]** — SETTLED (fresh full panel; the stopped first attempt was rerun from scratch) | "All five attackers concede no UB from safe code; v5's five-tuple closes variance and dropck." Surviving price: `where [(); N-2]:` is off-floor, fires post-monomorphization, and syntactic abstract-const unification makes generic-over-N consumers restate the obligation per boundary. (Also: P6's stated invariant over-promises under `mem::forget`/cycles — a wording defect, ruled not-UB.) |

**Two consequences.**

1. **The recorded v1 HOLD is contradicted as stated, and the lineage has now
   settled — at SEAM-COSTS [horn2], not back at HOLDS.** The covariance attack
   applies to the v1 text (same token section, variance never fixed), was found by
   two model families (GPT run 1 on v1 — per the disclosed session record — and the
   Claude gauntlet on v3, on disk), and no stopper in §4 fires against it. Real
   implementations dodge it via `UnsafeCell`-forced invariance in the shared core.
   The v5 adjudication (fresh full panel) closed the enumeration — no UB path
   survives the completed five-tuple — but the completion is *priced*: the `N >= 2`
   const bound is an off-floor, per-generic-boundary obligation (horn-2). The
   settled reading: **the repaired flagship is sound but costed; the recorded
   "HOLDS [none]" tier is not recovered.** Single-gauntlet caveat applies to the
   tier (COSTS-vs-HOLDS is exactly the fine distinction the measured 1-of-3 drift
   base rate makes provisional); the no-UB coarse verdict is the load-bearing part.
2. **The lineage's meta-finding:** the seam token must pin the ENTIRE auto-derived
   declaration surface — auto-traits (v1 did), **variance** (v3's break), **dropck
   ownership** (v4's break) — not just the auto-traits. Each is floor-checkable and
   decidable once named; each was invisible until an adversarial panel needed it.

## Disclosed deltas vs the recorded runs

1. No web tool on the foreign side (prior-art attacker reasons parametrically).
2. No temperature pin (HTTP providers pin 0.0; Cursor CLI exposes none).
3. Role system prompt prepended to the user prompt (Cursor CLI has no system channel).
4. One gauntlet per seam-per-family. The recorded Claude-side 1-of-3 tier-drift base
   rate is a lower-bound proxy only: the one repeated foreign cell (spsc, GPT) flipped
   tiers, so foreign single-draw instability is unquantified and plausibly higher.

## What this licenses (and what it doesn't)

The monoculture threat named in §6 has two components, and this control addresses
exactly one of them. **(a) Family-specific judge bias** (self-preference, one
family's idiosyncratic priors): sharply weakened — two foreign families reproducing
all four breaks with horn-exact attributions is strong evidence the breaks are not
*family* folklore. **(b) Corpus-shared consensus prior** ("a seamless dependent
systems language is impossible" as folklore common to every model trained on
overlapping corpora): **in principle unaddressable by any LLM-family replication,
including this one** — cross-family agreement is what a shared-corpus prior predicts
too, and the shared role prompts are additionally a shared instrument. Component (b)
remains open; only non-LLM evidence (the Lean skeleton, human adversaries, or a
mechanized counterexample) can bear on it. What the control also does *not*
establish: the flagship HOLDS at their recorded tiers. On this evidence the
HOLDS/COSTS boundary is rubric-relative in every family tested, and the flagship
HOLD itself carried a spec defect found by two families (one attested only by the
disclosed session record); the v5 adjudication has now run and settled the lineage
at SEAM-COSTS [horn2] — sound (no UB path survives the completed surface) but
priced, so the recorded HOLDS tier is not recovered. Residuals: one gauntlet per
seam per family; three transport deltas. (The refined escalation, formerly listed
here, was re-examined and adjudicated out-of-family: recorded tier sustained —
`REFINED_ESCALATION_ADJUDICATION.md`.)
