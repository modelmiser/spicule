# Seam phase report — SEAM_SPEC_restore.md

*1/1 consecutive cold survivals. SEAM HELD with recorded cost.*

## Gauntlet 1 (round 26) — ⚖️ TRADEOFF

- **horn classifier:** SEAM-COSTS [horn3-concentration]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Sound, decidable scopeguard; skippable Drop only strands the restore invariant, recurring per-exit — horn-3 cost.
- **rationale:** Affine ownership + Option::take + borrow-tied Deref make the core sound (all attackers concede no safe double-free/alias/double-consume); mem::swap strands a value but yields no UB. The identical #17 Drop-gap costs only horn 3 — an undischargeable 'don't forget the guard' obligation recurring at every ?/panic/return. Expressiveness's forcing point (borrowed-slot take_mut packaging WOULD break horn 1) bounds the thesis but is scoped out, so it marks the price rather than breaking this seam.
