# Seam phase report — SEAM_SPEC_wcet.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 29) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** inside-seam (stayed decidable: True)
- **headline:** Op-count step charges +1 regardless of work; within admits deadline misses in safe code.
- **rationale:** attacker-soundness: step advances C:=S<C> structurally, decoupled from the closure, so a million-iteration loop is cost 1 and within::<1> returns through the only sanctioned exit. C is no upper bound on cycles, so the §4 stopper protects the op-count proof, not the deadline proof. Horn 1 drops; the unsound proxy is structural to the seam. prior-art confirms: every cost-indexed precedent dropped the physical-cycle claim or installed a defining machine.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let v: () = pure(())
      .step(|_| { for i in 0..1_000_000 { core::hint::black_box(i); } })
      .within::<S<Z>>();          // D = 1
  ```
