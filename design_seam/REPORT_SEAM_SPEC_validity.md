# Seam phase report — SEAM_SPEC_validity.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 17) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** inside-seam (stayed decidable: True)
- **headline:** Stated precondition b<=2 transmutes byte 1 into an invalid Tag — validity UB.
- **rationale:** attacker-soundness lands: §2/§6 designate 'b<=2' the true precondition the deliverable costs, but for discriminants {0,5,200} it admits invalid bytes 1,2. Safe tag_at on byte 1 passes 1<=2, transmutes to a Tag with no discriminant 1, then an exhaustive match miscompiles. Every consumer step is safe; horn 1 drops as written.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let buf = [1u8];                  // byte 1
  let t = tag_at(&buf, 0).unwrap(); // 1 <= 2 passes → transmute(1) : Tag has no discriminant 1
  match t { Tag::A => {}, Tag::B => {}, Tag::C => {} } // exhaustive, no fallthrough
  ```
