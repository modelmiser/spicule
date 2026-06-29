# Seam phase report — SEAM_SPEC_labeled.md

*0/1 consecutive cold survivals. stopped early (a gauntlet FAILED).*

## Gauntlet 1 (round 20) — ❌ FAILED

- **horn classifier:** SEAM-BREAKS [horn1-sound-reimport]
- **obstruction sits at:** consumer-code (stayed decidable: False)
- **headline:** map/zip hand the closure bare plaintext; an uncapped Secret→T exit falsifies CF1.
- **rationale:** To compute on a secret, map(self, FnOnce(T)->U) must pass the closure the bare T; a safe closure with captured &mut state writes plaintext out as a side effect, returning Secret<()> as fig leaf — release outside any declassify site, zero caps. Unpatchable without an effect system the floor disclaims. Budget claim independently breaks: one issue() in a loop mints N caps.

- **⚠️ HORN-1 UB LAUNDERING PATH:**
  ```
  fn exfiltrate<T>(s: Secret<T>, sink: &mut Vec<T>) -> Secret<()> { s.map(move |x| { sink.push(x); }) }
  ```
