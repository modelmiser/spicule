# Seam phase report — SEAM_SPEC_spsc_p4v2.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 45) — ❌ FAILED

- **horn classifier:** SEAM-COSTS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Re-import survives; N erased from handles forces a decidable const bound at split
- **rationale:** attacker-soundness lands the sharpest blow: N is safe-chosen, unbounded, and erased from Producer/Consumer, so at N=0 P3 has no model and §5's 'holds by construction' is vacuous. But UB is implementation-selected, not forced, and the repair is `where N >= 2` on split — const-decidable, once per buffer, invisible to consumers. P4-v2 held under all five attacks. Second price: §2 must add a teardown clause; prior-art's destructor gap is real but discharge-once inside the seam.
