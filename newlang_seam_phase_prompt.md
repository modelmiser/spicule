# newlang — next phase: build-and-break the seam

You are the Claude Code instance running the `newlang` project — the same project
that produced `WRITEUP.md` (the impossibility result for the unified language and the
Seam Trilemma in §9). This prompt opens the **constructive phase**. It carries the
conclusions of a design conversation held *about* that writeup, so you don't have to
re-derive them. Read all of it before acting.

**Operating rules for this phase (do not skip):**
- **Inspect before assuming.** Start by reading your own repo state — `orchestrator/`,
  `design/`, `design_seamful2/`, `proof/SeamGap.lean`, `proof/REVIEW.md`, the
  `--program` plumbing, the role definitions (PROPOSER / ATTACKERs / JUDGE /
  SYNTHESIZER / RECONCILER / HISTORIAN), and the control cases. If anything this prompt
  references doesn't exist or differs, **report the discrepancy and ask** — do not
  invent or paper over missing pieces.
- **State assumptions up front (log them), then run straight through A → B → C without
  stopping for approval.** Record the result of each phase before moving to the next so
  there's closure and no cognitive debt — but do not wait on the operator between phases.
- **Worst case first** in any projection. No numbers without shown methodology.
- Language discipline: idiomatic, safe Rust for the checker/primitive code; Python for
  orchestrator glue (match existing); Lean 4 for any mechanization. Lean on the type
  system to enforce constraints at compile time. Don't add or remove features in
  existing code without asking.

---

## 1. What the design conversation concluded (carry this in)

These are the load-bearing conclusions. They are the *frame* for the work, not
hypotheses to re-litigate.

1. **The seam is the product.** The impossibility lives at exactly one place — the
   boundary where a rich, undecidable, cathedral-grade proof has to be handed down to a
   cheap, decidable, proof-free "floor." Everything else in a language like this is
   ordinary compiler engineering. So the seam is the *only* part worth the adversarial
   gauntlet, and it is the deliverable.

2. **Build and break are different machines — keep them separate.** The harness's
   load-bearing trick is *coldness* (statelessness, no shared context). That is the
   correct property for **demolition** and the wrong property for **construction**,
   because construction is mostly continuity. Therefore:
   - A **stateful builder** (operator, or a normal context-holding Claude) writes the
     smallest thing that compiles.
   - The **cold harness is the red team**, pointed at what the builder produced. It is
     the adversary, never the author. Using cold instances to *build* reproduces the
     `seamful`-run contamination already documented in the writeup ("almost-right rules
     with a dropped premise," "sound in spirit, defective as written"). Don't.

3. **Use the decontamination template — it's already your own best move (§9).** Hand the
   builder a **known-good** sub-component and watch whether the failure *moves* or
   *disappears*. Concretely: **stipulate the floor as Rust's NLL-as-shipped** (concede
   #3 — already proven costless by your `unified` calibration control). Don't derive a
   borrow checker; that's the contamination source. Build the seam *on top* of the
   known-good floor, aim cold attackers at the **re-import boundary only**, and observe
   whether the obstruction relocates onto re-import (structural — the trilemma biting,
   the predicted result) or actually closes (you found a sound seam). Moving ≠
   disappearing is the entire signal.

4. **The Seam Trilemma is your scoring rubric.** Every attack outcome is classified by
   which horn it threatens:
   - **Horn 1 — sound re-import:** crossing back launders no UB into safe code. A
     successful UB-laundering attack = horn 1 dropped = the failure mode to *detect and
     reject* (this is the bad-control corner).
   - **Horn 2 — decidable / proof-free / zero-cost safe fragment:** if making the seam
     sound forces an external prover or user proofs into ordinary code, horn 2 dropped.
   - **Horn 3 — seam concentration:** if ordinary code has to cross the seam constantly,
     horn 3 dropped (the ATS2 ergonomics cliff). **Measure this** — instrument how often
     safe code touches the seam; it is a first-class metric, not an afterthought.
     *(Greenfield — verified: the orchestrator has **no** seam-width metric today. This
     measurement is something you BUILD in Phase B, not a flag you pass.)*

5. **Conservation of hardness — make relocation visible.** You cannot delete coordination
   hardness, only move it (shared-memory invariant ↔ consensus obligation). If a design
   move pushes a shared-state obligation into message-passing, the new obligation must be
   made *as visible* as the deleted one. A red-team objective is to catch the monster
   moving **silently**. *(Greenfield — verified: there is no conservation-of-hardness
   auditor role today. Catching silent relocation is either a new attacker angle you add
   or a manual check you perform; it is not wired into the harness.)*

6. **Robustness over total soundness, honestly bounded.** State the threat model out
   loud, prove soundness *within* it, and degrade gracefully outside it rather than
   pretending the edge isn't there. A seam proven sound against its model is maximally
   fragile *outside* the model; name the boundary instead of hiding it.

7. **Honest epistemic bounds (carry forward, don't relax):**
   - "Survived adversarial challenge" is a strong empirical result, **not** a mechanized
     proof. Keep that distinction in every claim.
   - All role instances share model weights → shared blind spots. Use `--prior-art-web`,
     and surface where human-injected attack angles are needed. *(Note: the seed (§6)
     specified a human-injection hook, but — verified — it was never implemented in
     `orchestrator/`. Treat it as a gap to flag for the operator, not an existing feature.)*
   - `proof/SeamGap.lean` is, per `proof/REVIEW.md`, information-theoretically trivial
     today (a one-field projection can't recover a two-field predicate). A *real* win in
     this phase is lifting one genuine PL fact out of "survived challenge" into the
     checked Lean fragment — see Phase C.

---

## 2. The concrete artifact to attack

The smallest meaningful seam is one **dangerous primitive** plus the **re-import rule**
that lets safe (NLL) code consume it without re-proving its precondition. The artifact
has three parts:

- **Cathedral side:** the primitive's true precondition, expressed in the rich
  (undecidable, certificate-bearing) logic — the thing a foundation author proves once.
- **The certificate / token:** the cheap, sound, floor-checkable object the seam emits.
  By the trilemma it is **provably strictly weaker** than the precondition; the design
  question is whether what it *can* carry is *enough* to stay sound.
- **The re-import rule:** how NLL-floor code checks the token and uses the primitive
  through a safe interface. **This rule is what the cold attackers try to break.**

**What the harness actually attacks (verified):** `build_attacker_prompt(proposal_text)` —
the cold attackers red-team a **text proposal/spec**, NOT Rust source. So the seam spec
(precondition / token / re-import rule, written as a PROPOSER seed) is what goes through the
gauntlet. The Rust from Phase B is a **separate** validation, checked by *compilation and
examples*, not by the attackers: the floor + token compile, the intended sound use
type-checks, and the UB-laundering example is rejected. Do not feed Rust source to a
design-attacker and expect a meaningful verdict; keep "attack the rule" and "the code
implements the rule" as two distinct activities.

**Candidate primitives** (writeup names allocator / atomics / lock-free structure /
MMIO). Pick the smallest with a genuinely *rich* precondition and minimal surface.
Recommended default: a **single-producer / single-consumer lock-free ring buffer**, or a
**typed atomic cell with a release/acquire ordering invariant**. **Default to the SPSC ring
buffer; log your primitive and floor choice as assumptions (Phase A, step 0) and proceed —
do NOT wait for operator confirmation. The only thing that stops you is a genuine repo
discrepancy (see §Operating rules and §5).**

---

## 3. Staged plan — ship the smallest complete thought first

Run **A → B → C continuously.** Treat each phase as a complete, independently-useful
thought; at the end of each, **record** the outcome (ship / park / abandon, and what was
learned even if abandoned) and **continue automatically** to the next. No waiting on the
operator. If you abandon a line, say so and record why; don't ghost it.

### Phase A — smallest shippable increment (design + one cold gauntlet)

*Goal: a single SURVIVED/FAILED verdict on **one** re-import rule, classified by horn.*

0. Log assumptions: primitive = **single-producer / single-consumer lock-free ring
   buffer** (default; switch to the typed atomic cell only if inspection shows the ring
   buffer is a poor fit for existing scaffolding), floor = NLL-as-shipped, threat-model
   boundary stated explicitly. Proceed without waiting.
1. Write the seam spec in-repo: the primitive's rich precondition, the proposed token,
   and the re-import rule — as a formal spec doc plus a PROPOSER seed for a new seam
   program (reuse the `seamful2` scaffolding; the floor is *stipulated*, so the whole
   gauntlet bears on the seam, exactly as `seamful2` was designed to).
2. Run **one** cold gauntlet against the re-import rule (`--prior-art-web` on).
3. Classify the outcome by horn. If a UB-laundering attack lands, that's horn-1 detection
   working — record the exact laundering path.

**Independently useful even if abandoned here:** a hardened-or-killed seam rule and the
knowledge of which horn it hits. **Record the outcome, then continue to Phase B.**

### Phase B — real enough to show someone (build + 3-gauntlet survival)

1. Stateful builder implements the floor as stipulated NLL-as-shipped (do **not** derive
   a checker) and the one primitive's token mechanism as actual checkable Rust.
2. Cold harness red-teams the token-consumption rule through the faithful **three
   consecutive** gauntlets (fresh attackers each round; any FAILED feeds landed attacks
   back and resets the streak).
3. Emit the **seam-width count**: how many times safe code touched the seam to use the
   primitive. Report it as a horn-3 signal.

**Deliverable:** a toy that actually checks one token, with an adversarial survival
record and a measured seam width. **Record the outcome, then continue to Phase C.**

### Phase C — actually useful (decontamination experiment + a non-trivial lemma)

1. Run the decontamination experiment **for real**: does the obstruction *move* to the
   re-import boundary as the `seamful2` theorem predicts, or close? Record which.
2. Add a second (and maybe third) primitive; report whether seam width stays bounded
   (concentration holds) or blows up (horn 3 dropping).
3. Attempt to lift **one** genuine PL fact out of "survived challenge" into
   `proof/SeamGap.lean`, directly addressing the `proof/REVIEW.md` triviality caveat —
   i.e., make the checked lemma carry real substance, not a one-field projection. Keep it
   axiom-clean if you can; document any axiom used.
   **Scope warning — this is the rabbit-hole.** Doing this *non-trivially* needs a faithful
   operational-semantics model (RustBelt/Iris-shaped); per `proof/REVIEW.md` that is a
   multi-day effort, NOT a quick step. **Timebox it:** if a genuinely non-trivial lemma
   doesn't yield within a few focused attempts, **stop, disclose the gap honestly (as
   `proof/REVIEW.md` already does), and do not let it block the A→B→C writeup.** A recorded
   "attempted, here's how far it got, here's the wall" is a complete and acceptable outcome.

**Deliverable:** a defensible small seam fragment + a measured concentration result + a
non-trivial mechanized lemma. **Record the final outcome and write up the full A → B → C
run for the operator.**

---

## 4. Expected outcomes (worst case first)

- **Worst case:** Phase A's re-import rule is laundering UB (horn 1) and you can't repair
  it without an external prover (horn 2) — i.e., the trilemma bites immediately at the
  smallest scale. *This is still a clean result*: it's the `seamful2` prediction
  reproduced at minimum size. Record the horn, ship the negative finding, stop.
- **Median (most likely):** the obstruction **moves** to the re-import boundary rather
  than closing — the seam is sound only when you let it carry a proof obligation across
  (horn 2), confirming the "Rust, but `unsafe` is `proof`" verdict at toy scale. You get
  a working one-primitive seam whose honest price is a marked, measured proof crossing.
- **Best case:** for this *one* primitive the token turns out rich enough to stay sound,
  decidable, and concentration-preserving — a genuine sound seam fragment, plus a
  non-trivial Lean lemma. This would be a real positive result for one primitive; do
  **not** generalize it to "the trilemma is beaten" — test the next primitive.

Do not report any projection beyond these qualitative shapes without showing the
reasoning that produced it.

---

## 5. First action

Inspect the repo (§Operating rules). Log a short Phase-A assumptions statement — primitive
(default: SPSC ring buffer), floor = NLL-as-shipped, threat-model boundary, which existing
scaffolding you'll reuse — then **proceed directly into Phase A and run straight through to
C.** The only thing that stops you is a genuine repo discrepancy: if a referenced path,
program, or component is missing or diverges, flag it as a question and ask for the actual
code rather than inventing it. Otherwise, do not wait.
