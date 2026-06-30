# Seam phase report — SEAM_SPEC_multiparty.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 32) — ❌ FAILED

- **horn classifier:** SEAM-COSTS [horn3-concentration]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Sound, decidable; global progress is a real horn-3 cost uncocentratable per-call, stays decidable.
- **rationale:** Soundness uncontested (worst case: memory-safe recv-Err hang, no UB) so horn1 holds; structural HasDual keeps horn2 decidable. But global progress can't be concentrated per-call: Kobayashi/Dardha-Gay priority typing—per-role, non-generative, no global type—rejects the ring via cyclic-priority p<p, refuting the 'only generative whole-protocol' overclaim while confirming the weaker truth—progress needs strictly more than the HasDual floor. A marked horn-3 price, decidable, carried whole-program not per-call.
