# Adversarial faithfulness review of `SeamGap.lean`

A cold reviewer (fresh context, PL-theory + Lean 4, instructed to be hostile and find
the cheat) was given only `SeamGap.lean`, `README.md`, and the trilemma claim — not any
of the author's framing. Its job was *faithfulness*, not type-checking (the proof checks;
that was never in question). Verdict summary below; this file ships *with* the proof on
purpose — the project is adversarial, so its own mechanization gets the same treatment.

## Verdict: faithful but minimal — survives the attack on faithfulness, not on significance

- **Axis 1 — is `V` artificially weakened?** *Survives, with a caveat.* For the named
  primitive (a `Copy` interior-mutable handle), `V`'s blindness to `aliased` is faithful:
  a `Copy` value is exempt from affine move-tracking, so a second live copy is genuinely
  invisible to the borrow checker — exactly why `Cell::get/set` take `&self`, why
  `UnsafeCell` is the sole interior-mutability primitive, and why its safety obligation is
  comment-level. **Caveat:** the phrase "NLL's region-*only* vocabulary" is loaded — NLL is
  region+affine; the proof's force comes entirely from `Copy` *disabling affinity*, not
  from regions being weak. Faithful to the chosen primitive, not to "NLL in general."
- **Axis 2 — does the indistinguishability pair cheat?** *Survives.* Fair pair given the
  (Axis-1-justified) blindness.
- **Axis 3 — is the theorem content-free?** *The sharpest hit.* Stripped of names,
  `P_not_V_measurable` is "no function of one field equals a predicate over two fields" —
  an information-theoretic triviality that would prove *any* two-field/one-projection
  setup. **The Lean carries zero PL content; all PL substance is in the unverified
  modeling dictionary** (`V`↔NLL, `P`↔soundness, `Copy`↔forced-blindness) asserted in the
  prose. This is the normal division of labor in a formalization, *but* "machine-checked
  core" under-discloses it: what's checked is a tautology; what matters is the hand-asserted
  bridge.
- **Axis 4 — horn-mapping honesty.** *Mostly honest; the 2+3 collapse is explicitly
  disclosed.* The mechanized "trilemma" is really a di-lemma; `no_decision_has_all_three`
  is a 3-line corollary of the gap. `Usable` = totality is the *strongest* reading of
  usability (weakest theorem) — defensible, worth stating.
- **Axis 5 — overclaim.** *"What is NOT proved" is unusually candid about scope*, but
  (1) the "generalizes to any primitive…" parenthetical is circular (it generalizes to any
  primitive *defined* to have the gap), and (2) neither file states the strongest caveat:
  the checked lemma is trivial; confidence rests on the prose bridge. Bounds *scope* well,
  under-discloses *depth*.

## Author's response

All accepted. The README now (a) fixes the "region-only" phrasing to "region+affine, force
from `Copy` disabling affinity", (b) removes the circular generalization parenthetical,
(c) adds an explicit **depth disclosure** — the mechanized lemma is information-theoretically
trivial and all PL content lives in the unverified modeling dictionary — and (d) states that
`no_decision_has_all_three` is a corollary of the gap and `Usable` is the strongest reading.
No Lean strengthening was attempted, because any such move would merely relocate the same
faithfulness assertion (the reviewer's own point); genuine added significance would require
a RustBelt/Iris-scale operational model, out of scope here. The mechanization's honest value:
it checks the *logical skeleton* (P∉σ(V) ⇒ no sound+usable+concentrated decision) and forces
the faithfulness claim to be stated explicitly and reviewed — not that it proves the PL
substance, which remains "survived adversarial challenge."
