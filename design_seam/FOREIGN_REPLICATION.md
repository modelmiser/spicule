# Foreign-model replication — the monoculture control (run report)

**Run date:** 2026-08-12 · **Foreign model:** `gpt-5.6-sol-high` (GPT-5.6 Sol, high
reasoning effort, OpenAI) · **Transport:** Cursor CLI (`FOREIGN_PROVIDER=cursor-cli`,
subscription auth — no raw API key; see `orchestrator/foreign_replication.py` docstring
for the two disclosed transport deltas) · **Sample:** the 8-seam stratified sample,
7 foreign calls per seam (5 attackers ∥, judge, horn-classifier), 56 calls total,
one gauntlet per seam. Raw transcripts under `runs_foreign/` (untracked, like `runs/`);
machine-readable result in `runs_foreign/agreement.json` + `run_meta.json`.

## Headline

**Outcome-class agreement 6/8.** All four recorded BREAKS reproduced exactly —
**including identical horn labels** (`horn1-sound-reimport` on all four). Both COSTS
reproduced at outcome-class level (horn subcategory differs: `refined`
horn2→other, `slab` other→none). **Both divergences are the two recorded HOLDS,
demoted to COSTS.** The foreign panel never contradicted a recorded break and never
upgraded a verdict; every disagreement is about where the HOLDS/COSTS boundary sits.

| seam | recorded (Claude) | foreign (GPT-5.6 Sol) | agree |
|---|---|---|---|
| spsc | SEAM-HOLDS [none] | SEAM-COSTS [horn1-sound-reimport] | ✗ |
| spin_lock | SEAM-HOLDS [none] | SEAM-COSTS [horn3-concentration] | ✗ |
| slab | SEAM-COSTS [other] | SEAM-COSTS [none] | ✓ |
| refined | SEAM-COSTS [horn2-decidability] | SEAM-COSTS [other] | ✓ |
| intrusive | SEAM-BREAKS [horn1] | SEAM-BREAKS [horn1] | ✓ |
| secret | SEAM-BREAKS [horn1] | SEAM-BREAKS [horn1] | ✓ |
| sync_cell | SEAM-BREAKS [horn1] | SEAM-BREAKS [horn1] | ✓ |
| ordered_locks | SEAM-BREAKS [horn1] | SEAM-BREAKS [horn1] | ✓ |

## The two divergences, read closely

**spin_lock (HOLDS → COSTS[horn3]).** The foreign *judge* ruled SURVIVED — "the four
Send/Sync obligations block every safe data-race path in the stated API" — and the
foreign *horn-classifier* then priced the concentrated foundation proof burden as a
horn-3 cost. The recorded rubric read concentration-by-design as clean ([none]). This
is a rubric-boundary disagreement about whether deliberate concentration counts as a
cost, not a disagreement about soundness.

**spsc (HOLDS → COSTS[horn1]).** The stable foreign objection — raised by both foreign
runs (see provenance note below) — is a *specification-wording* defect: `Send` plus
`Arc<Mutex<Producer<T>>>` permits serialized use from multiple thread identities, so
thread-affinity does not discharge **literal** P4 ("one producer thread *ever*"). The
foreign classifier is explicit that no UB path was shown ("does not launder UB…
`&mut` access and the mutex prevent overlapping producers"); the demand is that P4 be
restated as non-overlapping same-role operations, or enforced via `!Send`/runtime
affinity. The quantifier ranges over a wider type than the certificate assumed —
the objection is to the claim's stated domain, not to the mechanism.

## Foreign-side verdict instability (provenance note)

A prior single-seam foreign run (same model, same day, ~40 min earlier) returned
**SEAM-BREAKS[horn1]** on spsc, headlined by a covariance/lifetime-laundering attack
(safe UAF via a retagged short-lived producer paired with `Consumer<&'static u8>`).
On the full-sample re-run the soundness attack was ruled **argued-past** and the
covariance claim did not recur; the P4 objection recurred in both. **The first run's
spsc transcripts were overwritten by the full run before being staged** — an operator
error, disclosed here; the run-1 judge JSON and verdict line survive verbatim in the
session record only. Foreign tier drift on spsc (BREAKS→COSTS across two gauntlets)
mirrors the recorded Claude-side drift on the same seam (gauntlet 1 TRADEOFF, then
SURVIVED×2) and the measured 1-of-3 tier-drift base rate: spsc is a genuinely
near-boundary seam under both families.

## Disclosed deltas vs the recorded runs

1. No web tool on the foreign side (prior-art attacker reasons parametrically).
2. No temperature pin (HTTP providers pin 0.0; Cursor CLI exposes none).
3. Role system prompt prepended to the user prompt (Cursor CLI has no system channel).
4. One gauntlet per seam (the recorded 1-of-3 drift base rate applies to both sides).

## What this licenses (and what it doesn't)

The monoculture threat named in §6 was sharpest against the **negative** claims — a
shared corpus prior emitting breaks regardless of ground truth. A foreign family
reproducing all four breaks with identical horn attributions sharply weakens that
threat for the breaks and for the trilemma's horn-1 structure. What it does *not*
establish: the two flagship HOLDS are family-robust. On this evidence the HOLDS
tier is **rubric-relative at the HOLDS/COSTS boundary** — the foreign panel agrees
nothing breaks, and disagrees about what counts as a cost. Residual limits: one
foreign family (a second — Gemini/open-weights — would strengthen this), one
gauntlet per seam, and the spsc P4 wording defect, which is actionable regardless
of family.
