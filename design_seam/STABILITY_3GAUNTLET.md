# Verdict-stability re-run — the three single-gauntlet SEAM-HOLDS seams at 3 consecutive gauntlets

*Tier-3(b), key-free (Claude path). 2026-07-01. Answers the cold reviewer's "one-gauntlet
basis" critique: 36 of 37 seams ran a single cold gauntlet — how stable is a 1-panel verdict?
Method: re-run each single-gauntlet SEAM-HOLDS seam (`drain`, `spin_lock`, `total_div`) to
THREE consecutive fresh cold gauntlets (the sweep's bar), output redirected out of `design_seam/`
so the canonical reports are untouched. `spsc` already ran 3/3 and is the pre-existing control.*

## Result

| Seam | Recorded (1 gauntlet) | Gauntlet 1 | Gauntlet 2 | Gauntlet 3 | Streak | Stability |
|---|---|---|---|---|---|---|
| **drain** | SEAM-HOLDS [none] | HOLDS | HOLDS | HOLDS | 3/3 | **verdict-class stable** |
| **spin_lock** | SEAM-HOLDS [none] | HOLDS | HOLDS | HOLDS | 3/3 | **verdict-class stable** |
| **total_div** | SEAM-HOLDS [none] | COSTS [horn3] | COSTS [horn3] | COSTS [horn3] | 3/3 | **survived, drifted one tier** |

(Gauntlet cell = the cold horn-classifier's outcome that round. "Streak" counts SURVIVED+TRADEOFF;
a TRADEOFF is a survival-with-recorded-cost, not a break. No gauntlet FAILED for any seam.)

## Reading

- **Coarse soundness verdict (holds-vs-breaks) was stable for all three.** Zero HOLDS became a
  BREAK; no horn-1 UB path appeared under re-examination. The *soundness* map is not a
  single-panel artifact in this sample.
- **The pivotal `spin_lock` reproduced exactly** — three fresh cold panels each returned
  SEAM-HOLDS [none], re-confirming the four-obligation discharge and finding no fifth omitted
  conjunct. The concurrency payoff (#35→#36→#37) is robust, not a lucky single panel.
- **`total_div` drifted one verdict *tier*** — from the recorded clean SEAM-HOLDS [none] to a
  unanimous SEAM-COSTS [horn3-concentration] across all three re-runs. It did **not** break
  (still sound, still decidable): the three panels *priced* the once-per-construction crossing a
  dynamic divisor forces (`TotalDivisor::new(n).unwrap()`) plus the thin `checked_div` delta —
  i.e. they charged as a horn-3 concentration cost the very tradeoffs the original single gauntlet
  had **recorded but waived**. The original characterization "the cleanest verdict in the project,
  no horn even costed" does not survive re-adjudication; the settled verdict is SEAM-COSTS [horn3].

## Consequence for the paper

Single-gauntlet **coarse** verdicts (holds vs breaks — the soundness map) are robust; single-gauntlet
**fine** verdicts (clean-hold vs marked-cost — the horn *tier*) are provisional and can shift under
re-examination. `drain`/`spin_lock` relabel to three-consecutive-survived (joining `spsc`);
`total_div`'s clean-HOLD framing is downgraded to SEAM-COSTS [horn3]. A fuller sweep (all HOLDS,
and re-running the BREAKS) remains future work. Independently, the `total_div` drift re-vindicates
the paper's reflexive thesis: "the cleanest verdict, no horn costed" was exactly the clean-extreme
over-claim a cold panel corrects.

*Reproduce: `python3 -m orchestrator.seam_phase --spec design_seam/SEAM_SPEC_<stem>.md --gauntlets 3
--design-dir <scratch> --runs-dir <scratch>` for stem ∈ {drain, spin_lock, total_div}.*
