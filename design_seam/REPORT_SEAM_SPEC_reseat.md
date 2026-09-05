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

---

## CORRECTION APPENDED 2026-09-04 (post-hoc; the r27 record above is unaltered)

The verdict **stands**: SEAM-BREAKS [horn 1], FAILED, fence FORCED not movable. One premise in the
rationale is false, and correcting it makes the verdict *stronger*.

**False premise:** "the into_inner-less subset is sound-but-degenerate". It is not sound. The subset
`new` / `Deref` / `DerefMut` / `Drop`, with no `into_inner` anywhere, double-frees:

    let mut slot = DropCount(&drops);
    { let mut g = Reseat::new(&mut slot);
      *g = DropCount(&drops);      // drops the original; the slot's duplicate bits now name it
      core::mem::forget(g); }      // skip the reseat; the slot keeps the stale bits
    // slot's owner drops them again -> drop count 2

`ptr::read` leaves a bitwise duplicate in the slot; `DerefMut` lets safe code destroy what those bits
name; `mem::forget` skips the `Drop` that would overwrite them. No `unsafe` in consumer code. The
`String` form (`g.push_str(...)` forcing a realloc) is a real heap double-free that glibc does not trap
and that exits 0.

**Why this strengthens the verdict.** r27 reached "fence FORCED" by taking the subset to be sound and
having the ergonomics and expressiveness seats rule it degenerate — i.e. the result rested on a
judgement about what counts as "the primitive". That judgement is no longer needed: there is no sound
repackaging for the fence to move to, because the horn-1 break is present in the *minimal* subset. The
fence is forced by **soundness**, not by usefulness.

**Why the panel missed it.** The attackers were pointed at `into_inner`, which is where the spec put the
crux, and the subset was assessed on ergonomics rather than attacked. The test that certified the
premise, `forget_is_a_leak_not_a_double_free`, could not fail: its payload owned no heap and its
"mutation" was `let _ = &mut *g;`, which invalidates nothing. It passed with the defect present.

**What changed in the tree:** the module prose in `seam/src/reseat.rs` is corrected at every site that
asserted the leak claim (the correction is marked inline, dated, and states what it replaces); the test
is replaced by `forget_after_a_replacing_mutation_double_drops` plus
`forget_without_a_replacing_mutation_is_only_a_leak`, which together draw the real boundary — leak vs
double-free is decided by whether `DerefMut` was used destructively, not by `mem::forget`.

Found by a parallel unsafe-audit sweep of all 46 unsafe sites in `seam/src`; see
`.crystal/audits/UNSAFE-sweep-2026-09-04.md` finding 2.
