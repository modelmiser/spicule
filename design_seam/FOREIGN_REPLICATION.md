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

**Two independent foreign families reproduced all four recorded BREAKS exactly —
identical horn labels included (8/8 break-verdicts across both families).** Outcome-
class agreement: GPT-5.6 Sol 6/8, Gemini 3.1 Pro 5/8. In both families, both recorded
HOLDS were demoted to COSTS; no full-sample foreign run ruled a recorded HOLD a BREAKS.
Gemini additionally escalated one recorded COSTS to BREAKS (see below).

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

**Both HOLDS demoted, in both families — a rubric-boundary pattern.** On spin_lock,
GPT's own *judge* ruled SURVIVED before its classifier priced deliberate proof
concentration as a horn-3 cost; Gemini's classifier made the same horn-3 pricing on
both HOLDS. The recorded rubric read concentration-by-design as clean ([none]). Across
three foreign panels (two full-sample + one earlier single-seam run) no foreign
verdict on either HOLDS seam claimed a UB path in the seam-as-mechanism; the
disagreement is about whether deliberate concentration counts as a cost.

**GPT's stable spsc objection was a specification-wording defect** (both GPT runs):
`Send` + `Arc<Mutex<Producer<T>>>` permits serialized migration across thread
identities, so thread-affinity does not discharge **literal** P4 ("one producer thread
*ever*") — explicitly "does not launder UB." This was repaired and re-adjudicated; see
the lineage section.

**Gemini's escalation on refined (COSTS[horn2] → BREAKS[horn1])** is the one two-way
divergence: its judge ruled the seam's central premise — a relational bound that is
simultaneously *brand-free* and *statically erased* — contradictory across function
boundaries ("an unparameterized value existentially hides its bound... impossible
without a runtime check or a viral type parameter"). The recorded verdict priced the
same defect region as a horn-2 cost. A severity disagreement inside an already-
negative-tier seam — but it is an upgrade, and the earlier "foreign panels never
upgrade" reading is hereby corrected.

## Foreign-side verdict instability (provenance note)

A first single-seam GPT run (same day, ~40 min before the full run) returned
**SEAM-BREAKS[horn1]** on spsc, headlined by a covariance/lifetime-laundering attack.
On the full-sample re-run that attack was ruled argued-past and did not recur; the P4
objection recurred in both. **The first run's spsc transcripts were overwritten by the
full run before being staged** — an operator error, disclosed; the run-1 verdict line
and judge JSON survive verbatim in the session record only. At the time this looked
like foreign tier drift. **The lineage work below vindicated the run-1 attack as
real** — the strongest possible corrective to treating a non-recurring finding as
noise.

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
| v5 | plain `Drop` (no `#[may_dangle]`) + `PhantomData<T>` ownership marker | **authored, attacked, UNADJUDICATED** (run stopped at the judge stage) | — |

**Two consequences.**

1. **The recorded v1 HOLD is contradicted as stated:** the covariance attack applies
   to the v1 text (same token section, variance never fixed), was found independently
   by two model families (GPT run 1 on v1; Claude gauntlet on v3), and no stopper in
   §4 fires against it. Real implementations dodge it only by accident (`UnsafeCell`
   in the shared core forces invariance). The repair is decidable and once-per-seam —
   trilemma-shaped, not trilemma-breaking — but "SPSC HOLDS [none]" must be read
   with this disclosure attached until the v5 lineage settles.
2. **The lineage's meta-finding:** the seam token must pin the ENTIRE auto-derived
   declaration surface — auto-traits (v1 did), **variance** (v3's break), **dropck
   ownership** (v4's break) — not just the auto-traits. Each is floor-checkable and
   decidable once named; each was invisible until an adversarial panel needed it.

## Disclosed deltas vs the recorded runs

1. No web tool on the foreign side (prior-art attacker reasons parametrically).
2. No temperature pin (HTTP providers pin 0.0; Cursor CLI exposes none).
3. Role system prompt prepended to the user prompt (Cursor CLI has no system channel).
4. One gauntlet per seam-per-family (the recorded 1-of-3 tier-drift base rate applies).

## What this licenses (and what it doesn't)

The monoculture threat named in §6 was sharpest against the **negative** claims — a
shared corpus prior emitting breaks regardless of ground truth. **Two foreign families
reproducing all four breaks with identical horn attributions is strong evidence the
breaks are not family folklore.** What the control does *not* establish: the flagship
HOLDS at their recorded tiers. On this evidence the HOLDS/COSTS boundary is
rubric-relative in every family tested, and the flagship HOLD itself carries a
cross-family-confirmed spec defect under repair (lineage above, settlement pending
v5 adjudication). Residuals: one gauntlet per seam per family; three transport deltas;
the refined escalation adjudicated only by severity, not re-examined line-by-line.
