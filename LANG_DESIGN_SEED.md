# Adversarial Language Design Process — Seed Document
## For transfer to Claude Code

---

## 1. PROBLEM STATEMENT

The C++ specification is ambiguous by design — not by accident. The ambiguity is
load-bearing: it justifies the committee's existence, perpetuates the revision cycle,
and protects the tooling/consulting ecosystem built around interpretation. The same
dynamic exists to a lesser degree in every language designed by committee with
incumbent stakeholders.

**The diagnostic proposal:** Train a model on the corpus of all working C++ (and
Rust). Let model uncertainty surface exactly where the spec fails to pin down
behavior. Where a model trained on all working code is confused, the spec has failed.
Where compilers disagree, the spec has failed. Map those failures explicitly.

**The insight this surfaces:** A language whose semantics are fully formally specified
is a language where a model trained on its corpus returns an empty set of
spec-confusion failures. That's the target.

---

## 2. THE DESIGN TARGET

A language with these properties, simultaneously:

1. **Executable specification** — the formal spec IS the type system IS the compiler.
   Spec/implementation divergence is structurally impossible.

2. **Unified proof/systems layer** — proof obligations and systems code are the same
   language at different abstraction levels. No seams. No FFI between "proof land"
   and "systems land."

3. **Ownership semantics** — memory safety without GC, derived from the proof layer
   rather than hardcoded into a compiler primitive (contrast: Rust's borrow checker
   is a fixed, hardcoded proof strategy).

4. **Decidable type checking** — the common case requires no user-written proofs.
   Proof obligations surface only at explicitly unsafe boundaries, FFI, and
   user-defined invariants.

5. **Bare metal targeting** — no runtime, no RC spikes, no GC pauses. Hard real-time
   compatible.

6. **Zero-cost abstractions** — the proof layer disappears at compile time.

7. **AI-legible semantics** — a model trained on the corpus should be able to fully
   reconstruct the spec. If it can't, the spec has failed.

---

## 3. PRIOR ART AND WHY EACH FALLS SHORT

| Language/Project | What it gets right | Critical gap |
|---|---|---|
| **Rust** | Ownership semantics, zero-cost, systems-capable | Borrow checker is hardcoded proof strategy, not programmable. Spec still prose. async is messy. |
| **Lean4** | Unified proof/program layer, small auditable kernel, fast native codegen | Uses RC memory model. Not designed for bare metal. Systems programming not primary target. |
| **Flux (for Rust)** | Right ergonomic model — local proof obligations, refinement types | Parasitic on Rust's type system. Can't express properties Rust can't represent. |
| **ATS2** | Closest to the target theoretically | Ergonomics are brutal. Effectively unusable outside academia. |
| **Idris 2** | Full dependent types, quantity types (like linear types) | Performance and bare metal story weak. Ecosystem thin. |
| **Cedille** | Dependent type theory designed to *derive* languages from | Pure theory. No systems programming path. |
| **Vale** | Asking the right question about alternative memory models | Unfinished. Experimental. |
| **Ferrocene/Rust** | Formally qualified Rust for safety-critical (IEC 61508, ISO 26262) | Qualified the existing language, didn't fix its spec model. |
| **Hazel** | Unified editor/language, incomplete programs always meaningful | Different problem. But relevant to what "the spec" means. |

**Synthesis:** The candidate is a language that takes Lean4's foundational unification
philosophy, Flux's ergonomic model for proof obligations, Rust's ownership semantics
reimplemented as a *derived* system rather than a compiler primitive, and a memory
model that can target bare metal without RC.

That language does not currently exist.

---

## 4. KEY HARD CONSTRAINTS

Any proposal must survive challenge on all of these:

- **Decidability:** Type checking must terminate. Full dependent types are undecidable
  in general. The restricted fragment that covers 95% of systems programming while
  remaining decidable has not been convincingly identified. This is the central
  technical challenge.

- **Zero runtime cost:** Proofs must erase at compile time. No proof witnesses at
  runtime. No allocations the programmer didn't explicitly request.

- **Weak memory models:** The language must express and verify behavior under ARM,
  POWER, and RISC-V weak memory ordering. This is an unsolved problem in formal
  verification generally.

- **Correlated blind spots:** Adversarial instances share model weights. They will
  have correlated blind spots. Human domain experts must periodically inject attack
  angles the model cannot generate itself.

- **Ergonomics cliff:** ATS2 proves theoretical correctness is insufficient. The
  common case must require no more annotation burden than Rust. This is a hard
  constraint, not a nice-to-have.

---

## 5. ADVERSARIAL PROCESS ARCHITECTURE

### Roles

**PROPOSER**
- Generates concrete design decisions — not directions, specific choices
- Example output: "Ownership is encoded as a linear type with these specific
  formation, introduction, and elimination rules: [formal rules]"
- Must be falsifiable and precise enough to implement

**ATTACKER-ERGONOMICS**
- Cold instance. Receives proposal only.
- Attack angle: will a systems programmer actually use this? Show a real pattern
  (async I/O, arena allocation, lock-free queue) that breaks or becomes unacceptable
  under this proposal.

**ATTACKER-SOUNDNESS**
- Cold instance. Receives proposal only.
- Attack angle: formal correctness. Find a program the type system accepts that
  violates the stated invariants. Or prove the type checking is undecidable.

**ATTACKER-IMPLEMENTATION**
- Cold instance. Receives proposal only.
- Attack angle: can this actually be compiled? Find the compiler pass that becomes
  undecidable, exponential, or requires solving an open problem.

**ATTACKER-EXPRESSIVENESS**
- Cold instance. Receives proposal only.
- Attack angle: find a program that should be valid under the stated philosophy but
  cannot be expressed. Especially: unsafe operations, FFI, performance-critical
  patterns that require escaping the type system.

**ATTACKER-PRIOR-ART**
- Cold instance. Receives proposal only.
- Attack angle: has this been tried? Find the paper, the language, the failed project
  that attempted this exact design decision and document why it failed or what it
  cost.

**JUDGE**
- Does not generate proposals. Only evaluates.
- For each attack: genuine flaw (proposal must be revised) / argued past the point
  (attack fails) / reveals a tradeoff (neither side wins, tradeoff must be made
  explicit and recorded)
- Judge verdicts are themselves challengeable.

**HISTORIAN**
- Tracks: proposals made, attacks attempted, verdicts rendered, tradeoffs identified
- Prevents the process from looping on already-resolved questions
- Output is the running design document — the thing that survives is what matters

### Round Structure

```
Round N:
  1. PROPOSER generates design decision D
  2. All ATTACKER instances run cold against D (parallel)
  3. JUDGE evaluates each attack
  4. HISTORIAN records outcome
  5. If D survived: it enters the design document
  6. If D failed: PROPOSER revises and round repeats
  7. If tradeoff identified: explicit tradeoff recorded, both branches explored
```

### Termination

A design decision is "settled" when it survives 3 consecutive rounds of cold
adversarial challenge across all attack angles without revision. Settled decisions
can be reopened only if a new attack angle is introduced that wasn't previously tried.

---

## 6. WHAT THE TOOLING MUST DO

- Orchestrate role instances via Anthropic API
- Maintain HISTORIAN state across rounds (persistent, structured)
- Enforce cold context for ATTACKER instances (no conversational bleed)
- Inject human-supplied attack angles at any point
- Produce a running design document of survived decisions
- Flag correlated blind spots (when all attackers miss the same thing, that's
  suspicious)
- Track which prior art each decision has been checked against

### Implementation notes

- Language: Python (orchestration), with the design document in a structured format
  (likely TOML or JSON for machine readability, Markdown for human readability)
- Each instance call is stateless — full context reconstructed from HISTORIAN each round
- Parallelism: attacker instances run concurrently per round
- Human checkpoint: after every N rounds, process pauses for human review and
  optional attack injection

---

## 7. IMMEDIATE FIRST PROPOSALS TO ATTACK

These are the first design decisions the PROPOSER should generate concrete proposals
for, in order:

1. **The memory model** — what replaces Rust's borrow checker? Linear types?
   Region inference? Something derived from the proof layer?

2. **The decidability boundary** — precisely which fragment of dependent types is
   admitted, and what is the formal argument that it's decidable?

3. **The proof erasure mechanism** — how are proof witnesses eliminated at compile
   time with a formal guarantee they leave no runtime trace?

4. **The unsafe boundary** — what does "unsafe" mean in this language, and what
   invariants does crossing it break and restore?

5. **The weak memory model interface** — how does the type system express
   happens-before relationships and memory ordering constraints?

---

## 8. SUCCESS CRITERION

The process has produced something valuable when it outputs:

> A precise map of which combinations of the 7 target properties (section 2) are
> genuinely incompatible, proven by survived adversarial challenge rather than
> asserted by authority — with the surviving compatible combinations forming the
> basis of a concrete language design.

That document does not currently exist in the literature.

---

*Generated from adversarial design conversation. Transfer to Claude Code for
implementation of orchestration tooling and execution of process.*
