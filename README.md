# Spicule

The **seam face** of the Radiant type-theory work — the *Seam Trilemma*,
explored build-and-break.

Where [Corona](../corona) reduces an invariant to a compile-time error and
[Sol](../sol) proves the reduction sound, Spicule stress-tests the **seams**:
it constructs a seam, tries to break it, and records what survives. The
37-seam build-and-break pass is closed; the writeup lives in `PAPER.md`
(Tier 1+2). The monoculture control has run (2026-08-12, GPT-5.6 Sol as every
cold role): outcome-class agreement 6/8, all four BREAKS exact with identical
horn labels; both divergences demote the flagship HOLDS to COSTS at the rubric
boundary — see `design_seam/FOREIGN_REPLICATION.md`. Residual ceiling: a second
foreign family, and the spsc P4 wording defect the foreign panel surfaced.

## Layout
- `seam/`, `design*/`, `proof/` — the seam constructions and their analysis.
- `runs*/` — build-and-break run records.
- `PAPER.md`, `WRITEUP.md`, `TUTORIAL.md`, `THE-SAME-WALL.md` — the writeups.
- `orchestrator/` — run tooling.

Private research repo.
