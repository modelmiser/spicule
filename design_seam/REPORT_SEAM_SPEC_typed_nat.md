# Seam phase report — SEAM_SPEC_typed_nat.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 12) — ❌ FAILED

- **horn classifier:** SEAM-COSTS [horn2-decidability]
- **obstruction sits at:** inside-seam (stayed decidable: True)
- **headline:** Sound; no UB; horn 2 pays a marked recursion_limit fuel-knob price, stays decidable.
- **rationale:** Soundness unanimously conceded: depth(I)<depth(Len)=len is airtight; sealing, orphan rule, conservative new block every forgery. No UB. But the disclosed price stands: deep in-bounds indices need recursion_limit raised (E0275). Per prior-art this marks an absent terminating-resolution fragment (UndecidableInstances/Paterson), not undecidability — Lt is linear. Stays decidable, proof-free, solver-free: a recorded horn-2 cost inside the seam, not a drop.
