# Seam phase report — SEAM_SPEC_secret.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 19) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** consumer-code (stayed decidable: True)
- **headline:** Source-level timing leak via expose_bool survives; horn-1 breaks in IFC sense, not UB.
- **rationale:** Per judge, ct_eq(secretA,secretB).expose_bool() then `if b` type-checks with only §3 ops, leaking a secret relationship through control flow — §4's universal is false. Stoppers foreclose branching on Secret, not on the untainted bool expose_bool returns; closing it needs whole-program undecidable IFC, so Horn 2 only passes by never checking the real property. No UB (AM ignores time): information-flow is out-of-frame, a different domain.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let same: Choice = secretA.ct_eq(&secretB); let b: bool = same.expose_bool(); if b { path_A() } else { path_B() }  // NOT UB: a source-level timing leak of a secret-secret relationship; the AM models no time, so the UB slot is empty
  ```
