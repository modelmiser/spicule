# Seam phase report — SEAM_SPEC_intrusive.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 6) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: False)
- **headline:** Brand gates access, not liveness; §4 type-accepts an L2 use-after-free.
- **rationale:** 'brand is a universe tag, not a storage lifetime, so §3(a) discharges L4/L1 but leaves L2 wholly open. With the token correctly presented, a still-linked stack/pool node drops at scope exit and the cursor traverses freed storage — UB in pure safe code. Drop has no token parameter, so even §3(c) auto-unlink can't repair it; the certificate is unsound, not merely costly.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  { let n = Node::<'brand>::new(p); list.insert(&mut tok, &n); } // &n ends at insert's return
  list.cursor(&mut tok).next();                                  // traverse freed slot → L2 UAF
  ```
