# Seam phase report — SEAM_SPEC_lock_registry.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 22) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Thread-keyed HELD vs task-keyed control: async work-stealing launders an undetected inversion.
- **rationale:** The registry keys HELD on the OS thread, but async tasks are the control unit and are in-scope std consumer code. A guard held across .await migrates to a thread with empty HELD, so a lower rank is accepted: an executed, untrapped non-increasing co-hold. This is the false-accept attacker-soundness wrongly called unreachable; the !Send fix only re-exposes the single-threaded false-reject.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  // multi-threaded work-stealing runtime, safe async consumer code
  let g = mtx_rank5.lock();   // resumes on thread A: HELD_A = [5]
  some_io().await;            // task migrates across .await; resumes on thread B, HELD_B = []
  let h = mtx_rank3.lock();   // check: 3 <= max(HELD_B)=empty -> passes, NO PANIC
  // task now co-holds rank 5 and rank 3 in non-increasing order, undetected inversion; HELD_A keeps stale rank 5 forever
  ```
