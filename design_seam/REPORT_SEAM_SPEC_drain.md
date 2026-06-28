# Seam phase report — SEAM_SPEC_drain.md

*1/1 consecutive cold survivals. SEAM HELD.*

## Gauntlet 1 (round 14) — ✅ SURVIVED

- **horn classifier:** SEAM-HOLDS [none]
- **obstruction sits at:** none (stayed decidable: True)
- **headline:** Pre-pessimized drain keeps all three horns; leaked guard loses tail, never double-frees.
- **rationale:** buf.len=0 set before first yield discharges D2 without relying on Drain::drop, so mem::forget mid-iteration leaks the tail (drop-count 0) while the head is moved-once and the len-0 buffer drops nothing — no double-free (attacker-soundness). D1 is stock NLL, D2 a runtime assignment, O(1) crossings (attacker-implementation). The tail-loss cost hits only the leak path, not the boundary.
