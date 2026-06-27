# Seam phase report — SEAM_SPEC_mmio.md

*1/1 consecutive cold survivals. SEAM HELD with recorded cost.*

## Gauntlet 1 (round 5) — ⚖️ TRADEOFF

- **horn classifier:** SEAM-COSTS [horn3-concentration]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Sound, proof-free; but the canonical ISR use needs a Send opt-in at the seam.
- **rationale:** No safe AM-UB path (soundness/implementation confirm; !Sync/!Send close the ISR race). But the primitive's own headline use—read-to-clear inside an ISR—forces per-handler unsafe static mut under the literal !Send default (Horn 3, per expressiveness/ergonomics). Neutralized only by the foundation author's Send auto-trait opt-in at the re-import boundary, which stays decidable (floor resolution) but is a marked price. M1/M4 is a trusted external axiom, not a horn.
