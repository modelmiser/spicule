# Seam phase report — SEAM_SPEC_refined.md

*1/1 consecutive cold survivals. SEAM HELD with recorded cost.*

## Gauntlet 1 (round 10) — ⚖️ TRADEOFF

- **horn classifier:** SEAM-COSTS [horn2-decidability]
- **obstruction sits at:** inside-seam (stayed decidable: False)
- **headline:** Sound and brand-free, but horn 2 un-bundles: needs external SMT; fragment too weak for real index math.
- **rationale:** All attackers concede no UB; horn 1 holds because R2 is established (panic, not UB) and Config-1's 'UB' only erases R2, violating the rule's precondition. Surviving cost is horn 2 (attacker-implementation/ergonomics): QF-LIA/UTVPI can't discharge bitwise/modular/stride indexing, so it falls back to a runtime check plus an impossible Option branch, and even where decidable it requires an external solver in the trusted loop — decidability un-bundled from solver-freedom.
