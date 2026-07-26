# Spicule

The **seam face** of the Radiant type-theory work — the *Seam Trilemma*,
explored build-and-break.

Where [Corona](../corona) reduces an invariant to a compile-time error and
[Sol](../sol) proves the reduction sound, Spicule stress-tests the **seams**:
it constructs a seam, tries to break it, and records what survives. The
37-seam build-and-break pass is closed; the writeup lives in `PAPER.md`
(Tier 1+2). The open ceiling is the monoculture question.

## Layout
- `seam/`, `design*/`, `proof/` — the seam constructions and their analysis.
- `runs*/` — build-and-break run records.
- `PAPER.md`, `WRITEUP.md`, `TUTORIAL.md`, `THE-SAME-WALL.md` — the writeups.
- `orchestrator/` — run tooling.

Private research repo.
