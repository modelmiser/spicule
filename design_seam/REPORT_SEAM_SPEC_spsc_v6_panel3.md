# Seam phase report — SEAM_SPEC_spsc_v6.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 1) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Unbounded `Consumer<T>: Send` in the normative enumeration launders a data race
- **rationale:** §3 is exhaustive-normative and derives Send unbounded on BOTH handles; §4 bounds only the Producer impl, leaving the endpoint that hands T out across the thread boundary — exactly P5's endpoint — unguarded. attacker-soundness's `Consumer<Rc<u8>>` derivation satisfies P1–P4 and P6 and fails only P5: laundering, not misuse. Repair is a decidable one-line T: Send bound, but horn 1 dropped. Secondary: §3's `restate N>=2 in signatures` disjunct doesn't exist on stable, pushing horn 3 to once-per-leaf-instantiation.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  safe code can `thread::spawn` a `Consumer<Rc<u8>>`, pop the Rc on the second thread, and race its non-atomic strong count into a use-after-free — with P1–P4 and P6 all fully satisfied
  ```
