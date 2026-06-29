# Seam phase report — SEAM_SPEC_ordered_locks.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 21) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Re-entrant Floor::root mints a second top token; safe code types a lock-order inversion.
- **rationale:** The &mut-downgrade enforces descent only within one Floor lineage; §2's invariant is per-thread global. Two sibling root() calls each yield Floor<Top>, so A.with(top_a) then B.with(top_b) acquires high-while-holding-low — every L:Below<F> discharges, it compiles, two symmetric threads deadlock. Horn 1 drops; the 'unconstructible inversion' claim is false.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  Floor::root(|top_a| {
    Floor::root(|top_b| {                    // second, independent root
      A.with(top_a, |_a, _fa| {              // R1: Below<Top>  ✓   acquire LOW first
          B.with(top_b, |_b, _fb| {          // R2: Below<Top>  ✓   acquire HIGH while holding LOW
              /* holds A(1) then B(2): NON-decreasing — a lock-order inversion */
          })
      })
    })
  })
  ```
