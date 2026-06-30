# Seam phase report — SEAM_SPEC_delegation.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 31) — ❌ FAILED

- **horn classifier:** SEAM-COSTS [horn3-concentration]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Sound and decidable; completion/leak obligation pushed undischargeable onto three-party consumer code — horn-3 cost.
- **rationale:** Soundness attacker confirmed no wrong-Q retyping (invariance + dual-by-construction pin Q); worst case is a memory-safe leaked/starved session, not UB. HasDual stays structural/polynomial — horns 1 and 2 intact. But the spec concedes completion and leak-freedom are unchecked, and the ergonomics attacker sharpens it: Result<_,Self> bans `?`, no mid-protocol abort verb, so the caller carries an undischargeable per-session completion obligation now spanning third party C. Decidable, marked price.
