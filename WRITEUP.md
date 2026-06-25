# Designing the impossible language — and proving why it's impossible

*An adversarial multi-agent process, run over cold Claude Code CLI instances, was
asked to design a systems language with seven properties at once. It settled none of
the five core design questions, and instead proved — by survived adversarial
challenge, with the instrument calibrated in both directions — that the seven
properties collide along a single fault line. This is that result.*

---

## 1. The ask

The seed (`LANG_DESIGN_SEED.md`) wanted one language with all of:

1. **Executable specification** — the spec *is* the type system *is* the compiler.
2. **Unified proof/systems layer** — no FFI seam between "proof-land" and "systems-land."
3. **Ownership without GC, *derived* from the proof layer** — not a hardcoded borrow checker.
4. **Decidable type checking** — the common case needs no user-written proofs.
5. **Bare-metal targeting** — no runtime, no RC spikes, no GC pauses.
6. **Zero-cost abstractions** — the proof layer erases at compile time.
7. **AI-legible semantics** — the corpus reconstructs the spec.

The seed's own success criterion (§8) was *not* "build the language." It was:

> A precise map of which combinations of the 7 properties are genuinely
> incompatible, **proven by survived adversarial challenge rather than asserted by
> authority.**

That map is the result below.

## 2. Method

An adversarial process (Section 5 of the seed), implemented as `orchestrator/`:

- **PROPOSER** emits one concrete, formal, falsifiable design decision.
- **Five cold ATTACKERs** — ergonomics, soundness, implementation, expressiveness,
  prior-art — each receive the proposal *only* and try to break it from one angle.
- **JUDGE** rules each attack genuine-flaw / argued-past / tradeoff, then SURVIVED /
  FAILED / TRADEOFF.
- **SYNTHESIZER** — when a question fails every revision, distills the convergent
  failure into a recorded property-incompatibility (the §8 payload).
- **RECONCILER** — when two decisions settle, checks whether holding both forces an
  incompatibility.
- **HISTORIAN** — deterministic state (`design/state.json` → `DESIGN.md`).

**The cold-instance mechanism is the load-bearing trick.** Each role is a fresh
`claude -p` subprocess with the ambient context stripped — replaced system prompt,
no settings loaded (`--setting-sources ""`, so no hooks), no MCP, a clean temp cwd.
Verified cold: an attacker asked about the operator's unrelated context answers "no."
An attacker that secretly shares the proposer's framing is not an adversary.

**Termination is faithful to the seed:** a decision settles only after surviving
**three consecutive** cold gauntlets (fresh attackers each time) without revision;
any FAILED gauntlet feeds the landed attacks back and resets the streak.

## 3. The result — 0 of 5 settled, 6 hard incompatibilities

Five questions, 24 rounds. **Nothing settled.** Every question exhausted its
revisions, and the synthesizer recorded a HARD incompatibility for each. **All six
place decidability (#4) on one side of the collision:**

| Question | Incompatibility |
|---|---|
| Sequential ownership core | derived ownership ✕ decidable Rust-cheap checking — at the control-flow merge point |
| Decidability boundary | derived *sound* ownership ✕ decidable proof-free checking — on real data structures |
| Proof erasure | zero-cost erasure needs witnesses *irrelevant* to equality; decidable seam-free checking needs them *relevant* |
| Unsafe boundary | decidable auto-discharged seam can't restore the open, entangled invariant set soundness demands |
| Interior mutability & weak memory | decidable proof-free ✕ sound zero-cost typing of shared-mutable / weak-memory state |
| (the original bundled memory question) | decidable ✕ no-seam ownership over weak-memory aliasing |

The failure reasons were specific and varied — an admissible-contraction double-free,
a non-associative `meet` table laundering conflicts into `Moved`, poison slipping past
a location-indexed seam, implicit-flow secret laundering, IRIW certifying a consensus
POWER allows to fail — yet they converged on one thesis.

## 4. The theorem (the six are one)

A meta-synthesis pass over the six found they are **not** independent findings but
facets of a single theorem (`design/THEOREM.md`):

> **No type system can be both (a) decidable and proof-free in the common case, and
> (b) the sole auto-checked discharge site for the substructural, heap-quantified,
> value-/view-dependent invariants that soundness of GC-free ownership over aliased
> weak-memory state requires** — because at every ownership-state merge point (CFG
> join, loop fixpoint, erasure-time equality, unsafe/atomic/DMA seam), deciding
> entailment over that invariant class is undecidable, while the no-seam,
> derived-ownership, and zero-cost requirements each remove the only boundary that
> could move the obligation out of the auto-checked fragment.

The structure: every facet bottoms out at a **merge/decision point** where the
checker must decide entailment between two ownership-or-protocol states. Soundness of
GC-free ownership over aliased weak-memory state forces those states to be
*substructural* (linear, no contraction), *heap-address-quantified* (∀-indexed
footprints), and *value-/view-dependent* (transferable protocol invariants). Deciding
entailment over that class subsumes a provably undecidable problem —
**bi-abduction / Hilbert's 10th** at the join, the **Bradley–Manna–Sipma
array-property fragment** on the footprints, **open-term definitional equality** at
erasure. The decidable, zero-annotation fragment is *necessarily the dual* of that:
finite-height, quantifier-free, first-order, contraction-admitting. So it is provably
**too weak to name the invariant**, and enriching it enough to name the invariant
inherits the undecidable entailment.

**#4 is the universal antagonist; #2, #3, #6 are three forcing functions** that push
the same obligation into the same fragment. They are not separate causes — they are
three different ways of removing the boundary (a trusted seam, a hardcoded analysis, a
runtime witness) that could otherwise relocate the hard obligation out of the
decidable fragment.

## 5. Is the instrument trustworthy?

A process that settles **nothing** is, from its outputs alone, indistinguishable
between "the target is impossible" and "the instrument is rigged to fail." So the
process was adversarially tested against itself, in three ways (`design/CALIBRATION.md`).

**True negative — a known-good design must settle.** Rust's NLL borrow checker, with
the hard constraints (#2/#3/#6) *waived*, run through the identical gauntlet:
**SETTLED.** And it settled honestly — rev0 FAILED on a real `dropck`/`#[may_dangle]`
UAF (the process caught an actual bug), then rev1 survived three consecutive gauntlets,
the judge ruling the residual attacks "puncture the author's reconstruction, not the
design." It settles a good design, *and* still requires correctness, *and* judges the
design rather than the prose.

**True-positive rejection — garbage must fail.** An obviously-unsound non-design (no
checker, "programmers just don't write bugs"): **FAILED** all four revisions, the
soundness attacker instantly refuting `check=⊤ ⟹ ∀e.safe(e)` with a well-typed UAF.
The process does not rubber-stamp garbage.

**Stronger-proposer stress test.** Was 0/5 just a one-shot-proposer weakness? The
most-precedented question (the sequential core) was re-run with a **3-shot
self-hardening proposer** that critiques and strengthens its own proposal twice before
attackers ever see it. It **still FAILED** all four revisions, on the same fault line.
The clean contrast: a **1-shot** proposer **settled Rust**; a **3-shot** proposer
**could not settle the constrained core**. More effort on the harder-constrained
problem still failed while less effort on the unconstrained one succeeded — so the
**constraints, not proposer weakness, are decisive.** The stress test even sharpened
the theorem: *locally-derived erasable ownership cannot encode memory safety's
non-local aliasing/liveness relation* — zero-cost erasure forces the ownership
evidence local; memory safety is non-local.

**Net:** the instrument settles the good, rejects the bad, and fails the subject. It
is measuring a property of the **subject**, not of itself.

## 6. What it means

The seven properties are not jointly satisfiable, and the obstruction is precise.
Existing languages survive by **conceding exactly one of #2/#3/#6**:

- **Rust** concedes #3 (derived) — its borrow checker is a *hardcoded* analysis, not
  derived from a proof layer — and quarantines the hard cases behind `unsafe` (a seam,
  conceding #2). That concession is *why* it ships.
- **Lean/F\*/Idris** concede #4 (decidable, proof-free) — they accept user-written
  proofs — and/or #5/#6 (they use RC/GC).
- **Separation-logic verifiers (RustBelt, iRC11, GPS)** concede #4 and #2 — the hard
  relaxed/aliased reasoning lives behind a trusted, manually-driven boundary.

The seed's genuinely novel ask — *derive* memory safety from one **decidable,
seamless, zero-cost** proof layer — is precisely the combination nothing concedes,
and the process shows mechanically why it can't be conceded-free. That is a real, if
negative, design result: **the productive move is to choose which one of #2/#3/#6 to
give up, deliberately, rather than to seek a design that keeps all three.**

### Covering, not intersection — and the seam is load-bearing

A tempting misreading: *"so there are a few languages that each do everything, and you
just can't merge them into one."* Two corrections, and the second is the real result.

First, none of those languages "does everything" — each **concedes** a property (the
table above). What holds is weaker and exact: for any *one* property, some language has
it. The seven are a **covering** by a handful of languages, never an **intersection**
in a single one.

Second — and this is the point — *why* you can't merge them is not an engineering or
packaging gap that enough cleverness (a meta-language, a unifying IR, a clever FFI)
could close. **"Combine them into one seamless language" is property #2**, and #2 is
one of the three things the theorem says you must give up. The act of merging is itself
the forbidden move. The **seam** between proof-land and systems-land — the trusted
boundary between, say, Rust and a separation-logic verifier — is exactly where the
undecidable ownership-merge obligation gets quarantined. That seam is not a wart in the
toolchain; it is the structural thing keeping the auto-checked fragment decidable.
Erase it (fuse into one no-seam language) and the undecidability **reappears at the
weld**. It is even-vs-odd at the merge point, not five tools that won't fit in one case.

The usable consequence:

- You **can compose them across a seam**, and the industry already does — "verified
  Rust" *is* Rust plus a separation-logic verifier (Creusot, Aeneas, Prusti,
  RefinedRust, Kani) cooperating across a trusted boundary. That is the covering at work.
- You **cannot fuse them into one** decidable, proof-free, seamless language — because
  the fusion is precisely the operation the theorem forbids.

So the corrected slogan is not *"five languages that each do everything but won't
merge."* It is: **the job takes ~one deliberate concession, a seam is where you pay it,
and the seam does real work — so deleting it (the dream of one unified language) is
exactly what is impossible.**

## 7. Honest bounds

- This is "proven by survived adversarial challenge" in the seed's sense — a strong,
  calibrated empirical result — **not a mechanized proof.** The undecidability
  reductions it cites (Hilbert's 10th, the array-property fragment) are real and
  load-bearing, but the synthesis assembling them was not machine-checked.
- Proposer depth was bounded (3 revisions; up to 3 self-strengthening shots). The
  stress test pushes against this but does not eliminate it. A human PL theorist with
  years might find a path the process did not — though they would be fighting the same
  reductions.
- All role instances share model weights, so they share blind spots. The prior-art
  attacker used live web search to partially counter this; full mitigation needs
  human-injected attack angles (a seed-specified hook not yet exercised).

## 8. Reproducing

```
python3 -m orchestrator.run --all --survivals 3 --prior-art-web   # the sweep
python3 -m orchestrator.run --meta-synthesize                      # the theorem
python3 -m orchestrator.control --case good                        # true negative
python3 -m orchestrator.control --case bad                         # true-positive rejection
python3 -m orchestrator.run --question 0 --proposer-shots 3 --scratch   # stress test
```

Artifacts: `design/DESIGN.md` (payload), `design/THEOREM.md` (unification),
`design/CALIBRATION.md` (instrument validity). Raw per-call transcripts in `runs/`.
