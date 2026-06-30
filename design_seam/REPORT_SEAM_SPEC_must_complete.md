# Seam phase report — SEAM_SPEC_must_complete.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 36) — ❌ FAILED

- **horn classifier:** SEAM-COSTS [horn3-concentration]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** No UB; free-handle Drop-bomb can't force completion, decidable scoped-closure restores it at the re-import boundary
- **rationale:** mem::forget skips the bomb (soundness attacker), so the Drop bomb does NOT carry completion — a memory-safe liveness gap, horn 1 intact, not UB. The free handle leaves completion on the caller once-per-use (`?`=abort, ergonomics attack). The decisive expressiveness attack's scoped-closure combinator (with_session → brand-guarded Session<End>) forces completion by moving the obligation to the construction boundary, staying decidable. Marked price: once-per-session driver crossing; no horn fully drops.
