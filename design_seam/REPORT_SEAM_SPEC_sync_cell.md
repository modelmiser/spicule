# Seam phase report — SEAM_SPEC_sync_cell.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 42) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** inside-seam (stayed decidable: True)
- **headline:** Unconditional unsafe impl Sync is an undischarged axiom; safe thread::scope races non-atomic set — real UB.
- **rationale:** Every attacker and the judge confirm Horn 1: &SyncCell<i32> shared across thread::scope races two non-atomic writes with zero consumer unsafe, satisfying Sync by axiom, not by any checked rule. The SpinMutex counterexamples refute the §6 'no value-adding seam' classification but describe a different, sound seam — they do not make SyncCell sound. The unchecked unsafe impl deletes the auto-trait check rather than costing it; obstruction sits inside the seam, break is O(1) runtime UB.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let c = SyncCell::new(0); std::thread::scope(|s| { s.spawn(|| c.set(1)); s.spawn(|| c.set(2)); });
  ```
