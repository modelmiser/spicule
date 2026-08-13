# Seam phase report — SEAM_SPEC_spsc_v3.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 46) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Unspecified variance lets covariant handles launder a dangling reference into safe code
- **rationale:** Section 3 emits !Copy/!Clone/Send/!Sync but never fixes variance; an admissible covariant declaration (NonNull over Box<[MaybeUninit<T>]>) makes Producer<&'static u32> <: Producer<&'a u32> by subsumption. No stopper fires: no clone, no send, no unsafe, P1-P4 intact. Section 5 grants only correct atomics; variance is declaration surface. Repair (invariance or T: 'static) is decidable — but the spec as written breaks.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  an admissible covariant `Producer<T>` permits the pure-subsumption coercion `Producer<&'static u32> <: Producer<&'a u32>`, letting single-threaded safe code push a borrow of a local and pop it back through an uncoerced `Consumer<&'static u32>` as a dangling reference. [attacker's literal program listing was truncated in the supplied report; quoted from its summary line, confirmed by the judge]
  ```
