# Seam phase report — SEAM_SPEC_priority.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 33) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** reimport-boundary (stayed decidable: True)
- **headline:** Public role() wires a fully priority-typed ring that compiles and deadlocks; §4.3 is false.
- **rationale:** Per attacker-soundness (judge: decisive): §3's collapse to p:Lt<p> needs one Nat per edge — a constructor invariant the spec admits is 'not a per-endpoint type fact.' The exposed role::<N1,N2> lets three N1<N2 relays form ring A→B→C→A; every Lt bound discharges, it compiles with no unsafe, and deadlocks. The seam's own horn-1 falsifier — 'a typed cyclic assembly that deadlocks' — is met; deadlock-freedom is a relay()-discipline guarantee, not a well-typed-assembly property.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let (ab_s, ab_r) = channel(); let (bc_s, bc_r) = channel(); let (ca_s, ca_r) = channel();
  let a = role::<N1, N2>(ca_r, ab_s);   // recv on CA @ N1, send on AB @ N2  — needs N1: Lt<N2> ✓
  let b = role::<N1, N2>(ab_r, bc_s);   // recv on AB @ N1, send on BC @ N2  — needs N1: Lt<N2> ✓
  let c = role::<N1, N2>(bc_r, ca_s);   // recv on BC @ N1, send on CA @ N2  — needs N1: Lt<N2> ✓
  ```
