# Seam phase report — SEAM_SPEC_linear.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 24) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Affine floor cannot compel the finalizer; safe mem::forget skips cleanup — re-import unsound by design
- **rationale:** Soundness attacker: forget's unbounded fn<T>(T) consumes MustRun<F> by one legal affine move, Drop never fires, finalizer runs zero times — horn 1 (must-run soundness, a liveness not-UB break) drops as pre-conceded. Closing it needs undecidable reachability or a linear (not affine) floor: wrong-structural-logic, no decidable boundary price. Horn 2 intact; horn 3's not-forget obligation undischargeable.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let g = MustRun::new(|| RELEASE_LOCK()); core::mem::forget(g); // scope ends — RELEASE_LOCK() never runs (also ManuallyDrop::new(g), Box::leak(Box::new(g)), Rc/Arc cycle)
  ```
