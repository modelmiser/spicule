# Seam phase report — SEAM_SPEC_spsc_v6.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 1) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Unbounded Consumer<T>: Send launders Rc across threads; horn 1 drops on enumeration
- **rationale:** §3 claims exhaustiveness and states Consumer<T>: Send unbounded; §4 supplies T: Send for Producer only. Safe code sends the consumer holding Rc payloads, racing a non-atomic refcount (RUSTSEC-2020-0150). Repair is one decidable bound. Prior-art's deeper hit — P2 names publication as 'the only' edge, omitting reclamation Release/Acquire on head — indicts §2's completeness inside the seam, but §5 stipulates correct atomics.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  channel::<Rc<u32>,4>(), keep the producer local, Send the consumer, pop clones on thread 2 → non-atomic refcount race
  ```
