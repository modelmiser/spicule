# Spicule

The **seam face** of the Radiant type-theory work — the *Seam Trilemma*,
explored build-and-break.

Where [Corona](../corona) reduces an invariant to a compile-time error and
[Sol](../sol) proves the reduction sound, Spicule stress-tests the **seams**:
it constructs a seam, tries to break it, and records what survives. The
37-seam build-and-break pass is closed; the writeup lives in `PAPER.md`
(Tier 1+2). The monoculture control has run under TWO foreign families
(2026-08-12, GPT-5.6 Sol 6/8 and Gemini 3.1 Pro 5/8, each as every cold role):
all four BREAKS reproduced exactly by both, horn labels included; every HOLDS
divergence is a rubric-boundary demotion. The control also caught a real miss:
the flagship SPSC HOLD's spec never pinned variance/dropck — a cross-family-
confirmed covariance laundering path, decidable repair, v2–v5 amendment lineage
in `design_seam/FOREIGN_REPLICATION.md`. Residual: v5 adjudication (run was
stopped mid-judge) and the Gemini `refined` escalation.

## Layout
- `seam/`, `design*/`, `proof/` — the seam constructions and their analysis.
- `runs*/` — build-and-break run records.
- `PAPER.md`, `WRITEUP.md`, `TUTORIAL.md`, `THE-SAME-WALL.md` — the writeups.
- `orchestrator/` — run tooling.

Private research repo.
