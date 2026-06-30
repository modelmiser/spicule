# Seam phase report — SEAM_SPEC_reseat.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 27) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** inside-seam (stayed decidable: True)
- **headline:** into_inner over a borrowed slot is a forced safe-code double-free; horn 1 breaks.
- **rationale:** attacker-soundness's 4-line String witness lands real UB in safe code: into_inner returns owned while the slot retains the ptr::read duplicate (two owners), and E0509 forces mem::forget(self), suppressing the reseat. Decidable and concentrated, but a UB path outranks all. Clean single-variable flip of #19; the into_inner-less subset is sound-but-degenerate, so the fence is FORCED, not movable.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  let mut s = String::from("hello"); let mut g = Reseat::new(&mut s); g.push_str(" world"); g.into_inner() // returns owned (p1), skips Drop; slot s keeps stale {p0,5,5}; s drops dangling p0 -> double free
  ```
