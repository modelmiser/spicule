# Designing the impossible language — and proving why it's impossible

*An adversarial multi-agent process, run over cold Claude Code CLI instances, was
asked to design a systems language with seven properties at once. It settled none of
the five core design questions, and instead proved — by survived adversarial
challenge, with the instrument calibrated in both directions — that the seven
properties collide along a single fault line (§1–8). Asked the constructive
follow-up — "then is a **seamful** language worth pursuing?" — the same process
characterized the escape hatch's exact limit: the **Seam Trilemma** (§9). Net: the
unified language is impossible, the seamful one is worth building, and there is a
precise wall it hits. This is that result.*

---

**The result in plain terms.** Someone specced a "dream" systems language: as safe and
proof-carrying as Lean, as low-level and zero-overhead as Rust, with proofs and systems
code in *one seamless language* whose type-checker needs no hand-written proofs. I pointed
an adversarial debate harness at it — one AI proposes a concrete design, five *cold* AIs
(no shared context) each try to break it, an AI judge rules — and no piece of the spec
survived. The reason is sharp and always the same: **you can have a type-checker that's
decidable and needs no manual proofs, *or* one powerful enough to verify real
shared-memory/hardware code soundly — not both**, because the hard cases force a check
that is provably undecidable. The natural escape — "keep one language, but wrap the
dangerous parts in a clean, machine-checked *seam*, like a formal version of Rust's
`unsafe`" — doesn't escape either. It hits a precise wall, the **Seam Trilemma** (§9):
*sound*, *cheap-to-check*, and *"ordinary code never touches the seam"* — pick two. The
verdict isn't "give up"; it's "you are choosing which of the three to drop — and here is
which drop every real language already makes."

**How to read this.** §1–8 are the impossibility result (method → the sweep → the theorem
→ why the instrument is trustworthy). §9 is the constructive follow-up and the Seam
Trilemma. `proof/` holds a machine-checked (Lean) core, with an honest account — forced by
a cold review — of exactly how much it proves.

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
facets of a single theorem (`design/THEOREM.md`).

*In plain terms: a type-checker can be **automatic** (decidable, no hand-written proofs)
or it can be **strong enough** to prove that real shared, aliased, weak-memory code is
safe — but not both. The safety facts those hard cases need are, at the points where the
checker has to combine them (a branch join, a loop, an `unsafe`/atomic boundary), provably
beyond what any always-terminating check can decide.* Formally:

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

A few languages *together* already do everything — that framing is right, and the
precise way to say it is that they form a **covering**, not an **intersection**. For
any *one* property, some language has it (the table above); but no single language has
all seven, because each one *concedes* a property to stay coherent. The union covers
the seven; no point in the union holds all seven.

The substantive question is *why the covering can't be collapsed into one language* —
and the answer is not an engineering or packaging gap that enough cleverness (a
meta-language, a unifying IR, a clever FFI) could close. **"Combine them into one seamless language" is property #2**, and #2 is
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

So the result is not bad news. A few languages, connected by seams, genuinely cover
the whole job — and the seam that resists full fusion is *doing real work*, which is
why deleting it (the dream of one unified language) is exactly what's impossible.
**The job takes ~one deliberate concession, a seam is where you pay it, the seam is
load-bearing — so "you can't combine them completely, and that's fine" is not a
limitation you tolerate; it is the design win.** It is also where verified-Rust
toolchains already are.

## 7. Honest bounds

- This is "proven by survived adversarial challenge" in the seed's sense — a strong,
  calibrated empirical result — **not a mechanized proof.** The undecidability
  reductions it cites (Hilbert's 10th, the array-property fragment) are real and
  load-bearing, but the synthesis assembling them was not machine-checked.
  *Partial exception:* the Seam Trilemma's **logical skeleton** is now machine-checked
  in Lean 4 (`proof/SeamGap.lean`, axiom-clean: the core gap needs no axioms, the
  trilemma only `propext`). But honesty requires the depth caveat that a cold review
  forced (`proof/REVIEW.md`): the *checked* lemma is information-theoretically trivial
  ("a one-field projection can't recover a two-field predicate") — all the PL substance
  lives in the unverified, prose-level **modeling dictionary** (`V` = what NLL sees,
  `P` = the soundness precondition, `Copy` forces the blindness). The mechanization
  checks the skeleton and forces the faithfulness claim to be stated and attacked; it
  does not lift the PL substance out of "survived adversarial challenge."
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

---

## 9. The constructive flip — is a seamful language worth pursuing, and the Seam Trilemma

The negative result says: concede exactly one of #2 (no-seam), #3 (derived ownership),
#6 (zero-cost erasure). The most natural concession is #2 — keep one language, but make
the boundary between "proof-land" and "systems-land" a **first-class, in-language,
machine-checked seam** instead of an FFI to a separate prover. Rust + `unsafe` is this
shape informally; Verus, F\*/Low\*, Dafny, RefinedRust make it formal. So the question
is not "should a seamful language exist" but "*which* calibration, and what's its
limit?" We ran the same process on it (`--program seamful`).

The aimed-for calibration: the safe fragment stays **decidable and proof-free** (the
common case), user proofs appear **only at seam crossings**, and — the make-or-break
constraint — the seam **concentrates** in foundational primitives (allocators, atomics,
lock-free structures, MMIO) that ordinary code composes through a *checked safe
interface* **without itself crossing the seam**. If ordinary code constantly crosses
the seam, you've rebuilt the ATS2 ergonomics cliff.

### Two runs, because the first was contaminated

**`seamful` (concede #2, keep #3):** 0/6 settled, six incompatibilities, two TRADEOFF
*near-misses* (seam-soundness and zero-cost-erasure each survived one cold gauntlet, then
a fresh one found the hole). Meta-synthesis: one theorem, casualty = **sound re-import**.
But this run was *contaminated*: keeping #3 forced the proposer to *derive a borrow
checker from scratch*, and it kept shipping almost-right rules with a dropped premise.
Several failures read "Rust-parity, sound in spirit, defective as written" — proposer
formalization bugs, not proven structure. The honest move was a controlled re-run.

**`seamful2` (concede #2 *and* #3):** the safe fragment is *stipulated* as Rust's NLL
as shipped (a known-good checker), so the entire gauntlet bears on the seam. Still
**0/6** — but now clean. Question [1] failed with the judge certifying *"the safe
interface (NLL types) is fine"*; the recorded obstruction is *"sound re-import needs a
**linearity witness NLL's region-only vocabulary cannot carry**."* Removing #3 didn't
rescue the seam — it **isolated** the obstruction onto the seam's re-import boundary,
exactly where the `seamful` theorem (which leans on #4/#6, never #3) predicted it would
land.

### The result: the Seam Trilemma

*In plain terms: the safe part of the language can only carry **simple** facts (lifetimes,
uniqueness). The dangerous primitives behind the seam — atomics, lock-free queues, MMIO —
are only safe under **rich** facts (who-aliases-what, memory ordering). To use such a
primitive soundly, the rich fact has to be checked **at the place you use it** — but the
safe language has no way to hold it. So one of three things must give.* Formally:

The decontaminated meta-synthesis states it as a trilemma. The seam must concentrate on
foundational primitives whose true preconditions are semantic heap-, value-, and
concurrency-level invariants — expressible only in an **undecidable, certificate-free
concurrent separation logic** (Iris/RustBelt grade). But the safe fragment can receive
only a witness that is simultaneously **decidable, proof-free, and zero-cost-erasable**
(at most NLL regions + affinity + bounded arithmetic). The re-import obligation is
therefore *strictly weaker than the precondition it must certify*, and the missing
conjunct forces exactly one of three fatal horns:

> **Pick at most two of:**
> 1. **Sound re-import** — crossing back launders no UB into safe code.
> 2. **Decidable, proof-free, zero-cost safe fragment** — ordinary code needs no proofs.
> 3. **Seam concentration** — ordinary code never crosses the seam.
>
> Keep #1+#2 → **drop concentration**: ordinary code carries the obligation at every use
> site (the verified-Rust reality: annotations at use sites, not just inside `unsafe`).
> Keep #1+#3 → **drop decidability**: the witness goes proof-bearing / needs an external
> prover (Verus/F\*; breaks one-toolchain + proof-free).
> Keep #2+#3 → **drop soundness**: the interface silently weakens the precondition and
> UB launders into safe code (the bad-control failure mode).

### Is it worth pursuing? Yes — with eyes open

The trilemma is not a "no." It is a *design constraint that names your options*. Every
shipping point on the map is a deliberate choice of which horn to drop:

- **Rust** drops concentration informally — `unsafe`'s obligation lives at use sites in
  the programmer's head; the type system doesn't carry it.
- **Verus / F\* / Creusot** drop decidability — they accept proof obligations (SMT or
  tactic) at the seam.
- The **unsound** corner is the one to avoid, and the process's positive-rejection
  control shows it is detectable.

So a seamful language is worth building — the productive version is probably **"Rust,
but `unsafe` is `proof`": drop decidability at the seam (horn 1+3), keep the safe
fragment decidable and proof-free, and concentrate the proof burden in foundation
authors.** What this project adds is the knowledge that you are choosing a horn, not
escaping the trilemma — and *which* horn, and *why* the fourth option (all three) does
not exist. That foreknowledge is the whole value of running the negative and
constructive halves together.

### The three-program map

| Program | Concedes | Settled | Result |
|---|---|---|---|
| `unified` | nothing | 0/5 | Decidability (#4) ✕ {#2, #3, #6} — keeping all 7 is impossible |
| `seamful` | #2 | 0/6 | sound re-import is the casualty (contaminated by keeping #3) |
| `seamful2` | #2 + #3 | 0/6 | **the Seam Trilemma**, decontaminated and structural |

Artifacts for the constructive half: `design_seamful2/THEOREM.md` (the trilemma),
`design_seamful2/DESIGN.md`, and the `seamful`/`seamful2` programs in
`orchestrator/` (`python3 -m orchestrator.run --program seamful2 --all`).

### Methodological note

The decisive step was **decontamination**: the first seamful run mixed "proposer can't
write a bug-free checker" with "the seam is impossible." Conceding #3 — a concession the
unified calibration control had *already proven costless* — removed the first and left
the second alone. A 0/N adversarial result is only as good as your ability to rule out
"the proposer was just weak," and the cleanest way to do that is to *hand the proposer
the known-good sub-component and watch the failure move, not disappear.*

## 10. The constructive phase — build and break three real seams

The trilemma is a map, so the next step was to stop arguing in the abstract and **build**.
A different harness shape: the negative half used cold instances to *propose* (and they
contaminated the result with almost-right rules). Construction is continuity, demolition is
coldness — so here a *stateful builder* writes the smallest thing that compiles, and the
cold harness is the **red team**, pointed only at the re-import rule. Each seam ships as a
text spec *and* a real `rustc`-checked crate (`seam/`) whose consumer code is
`#![forbid(unsafe_code)]` and whose laundering paths are `compile_fail` tests. Seven
primitives were chosen because their preconditions have seven different *shapes* — and the
later ones because each became the witness un-bundling an axis the trilemma had folded.

The result is sharper than "seams are hard." **Whether a seam closes is a function of the
precondition's shape** — specifically, whether every caller-observable conjunct of the true
precondition can be carried by the floor's witness vocabulary (affinity, NLL lifetimes,
decidable trait bounds):

| Primitive | Cold verdict | Horn | Seam width (measured) | Why |
|---|---|---|---|---|
| **SPSC ring buffer** | SEAM-HOLDS (4 cold panels) | none drops | **O(1)/buffer**, 0/element | *Fully encapsulable* — cardinality→affinity, storage→NLL `'a`, payload→decidable `T: Send`; happens-before leaves no caller-visible residue. This is `heapless::spsc`/`rtrb`; RustBelt machine-checked the class. |
| **MMIO read-to-clear** | SEAM-COSTS [horn 3] | sound+decidable, marked cost | O(1) *with* a foundation `Send` opt-in | *Sound modulo an external axiom* — "this address really is this register" is a physical-world fact, a TCB `assume` boundary (Low\*/seL4/CompCert), **not a fourth horn**. The price is a horn-3 `Send`-opt-in for the ISR use. |
| **Generational slab** | SEAM-COSTS [#6 zero-cost] (one revision) | all 3 trilemma horns hold | **O(1)**, fully-safe interface | *`Copy`-handle validity — runtime-checkable* — a `Copy` key defeats affinity, but a per-slot generation+occupancy check carries it at run time. Closes the trilemma but spends **zero-cost erasure (#6)** — `RefCell`/`slotmap`'s move. |
| **Intrusive linked list** | SEAM-BREAKS [horn 1] | horn 1 drops | O(accesses); repair forbids early reclaim | *Aliasing + liveness escapes to the caller* — the `GhostCell` brand carries L1/L4 (aliasing) but **not L2** (membership-vs-storage lifetime), so the rule type-accepts a use-after-free. |
| **Branded index** | SEAM-COSTS [other] | all 3 horns hold | **O(1)/slice** code crossings, yet a *viral `'id`* in every type | *Cross-object identity via a generative brand* — closes all three horns (RustBelt-verified `runST`/`indexing` lineage) but un-bundles **concentration** itself: pays on **API-locality (#5)** (the brand infects consumer types, traps indices in a `scope` closure) and *defers* zero-cost (#6) to the range-branding it scopes out. |
| **Refined index** | SEAM-COSTS [horn 2] | all 3 horns hold | **O(1)/index**, brand-free (the value *escapes*) | *In-bounds via a legible predicate* (Flux/Liquid Haskell) — the **(brand-free, legible)** corner that **closes the #5⟂#7 cell** against the slab's brand-free-but-illegible handle. Un-bundles **horn 2** into *decidability* ✓ vs *solver-freedom* ✗ (SMT in the TCB — the `unified` "keep" horn, not a new axis). The R2 trade: brand-free ⟹ runtime check; zero-cost ⟹ length-brand. |
| **Shared (Rc)** | SEAM-COSTS [bare-metal] | all 3 horns hold | **O(1)**, brand-free; needs a heap | *Dynamic shared ownership via a runtime refcount manager* — recovers seed **#5 bare-metal**: the sole seam needing `alloc` (compile-enforced; the crate is `#![no_std]`). A genuine seventh axis (⟂ zero-cost via the slab/`heapless` off-diagonal), but the judge deflated novelty — it *is* Rust's `core`/`alloc` boundary (RFC 2480). |
| **Type-level index** | **FAILED** [axis deliverable]; sound | all 3 horns hold; pays a recorded horn-2 cost | **O(1)**, brand-free, erases | *In-bounds via a compile-time trait proof `I: Lt<Len>`.* Sound (sealed `Nat` + orphan rule, unanimous), zero-cost, solver-free. Pitched as recovering an eighth **checker-tractability** axis — **rejected**: the off-diagonal is confounded with a strict expressiveness loss (static-index-only), the `recursion_limit` cost is encoding-relative (binary numerals erase it), and the axis is mislabeled (`Lt` is *linear* — the real concern is **terminating-resolution**, Paterson / `UndecidableInstances`). No new axis; **N stays 7**. |
| **Take-and-replace** | SEAM-COSTS [none] (TRADEOFF) | all 3 horns hold; pays the 8th axis | **O(1)**, brand-free, zero-cost happy path | *Transient broken invariant across an unwind, via an abort-bomb (`take_mut`).* Moving a value out of a `&mut T` and back leaves a duplicate in the gap; an unwinding panic drags the caller into a double-drop. Aces all seven prior axes, pays on **unwind-safety (8th axis)** alone — the *clean* off-diagonal the type-level index lacked. Confirmed **N ≥ 8**, deflated to pre-existing (Abrahams; `noexcept`; `UnwindSafe`; RFC #1508 rejected this exact `take_mut` from std on this cost). The axis = *exception-safety-guarantee-level*; abort is one **discharge** (vs `replace_with` paying API-surface) — a discharge menu. |
| **Drain-and-yield** | SEAM-HOLDS (SURVIVED) | all 3 horns hold; pays the 9th axis | **O(1)**, brand-free, no closure | *The leakpocalypse: `mem::forget` is safe, so a guard whose `Drop` does load-bearing cleanup can be leaked → double-free.* Sound via **pre-pessimization** (`Vec::drain`'s trick — empty up front, so a leaked guard loses the tail, never double-frees). Aces all eight prior axes, pays on **leak-freedom (9th axis)** alone. Confirmed **N ≥ 9**, **distinct from unwind-safety** (off-diagonal both ways: take is unwind-sensitive/leak-safe, drain is leak-sensitive/unwind-safe). Deflated to pre-existing (leakpocalypse / RFC 1066). The *thinnest* axis but real; aces API-locality *because* the guard owns its data (no closure — the confound that would sink a scoped-thread witness). |
| **Self-ref cell** | SEAM-HOLDS (TRADEOFF) | all 3 horns hold; pays an **inseparable** danger | **O(1)**, but a viral `&mut`-revoking `Pin` | *A value whose **address** is load-bearing (a self-pointer); moving it dangles it.* Sound via `Pin` over a `!Unpin` cell (the post-`init` move is unconstructible). **Ruled REAL-BUT-INSEPARABLE (outcome 3, N stays 9)** — a distinct danger (no other axis concerns a value's address) with **no isolable cost**: the price is *immovability* (heavier than API-locality), **non-uniform** across witnesses (stack-pin → API-locality, `Box::pin` → bare-metal). A new verdict category. Caveats: contingent (RFC 2349 rejected `?Move` immovable types); crisp only for a non-composable toy. |
| **Validity** (bytes→enum) | SEAM-HOLDS (TRADEOFF) | all 3 horns hold; the **closed-basis** test | **O(1)**/read; a viral sealed ctor for the view | *A **validity invariant** — a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even without a deref.* A genuinely new danger class, built to ask: does it add an axis? Sound via `transmute`-after-**set-membership** (a *range* check, unsound for non-contiguous discriminants, was the bug two gauntlets forced out). **Unanimous: NO tenth axis — it projects onto the basis.** Its own cost is **(3) inseparable** (2nd example): per-read → #6, the zero-copy `&[u8]→&[Tag]` view → API-locality. **The cost-basis is closed.** Non-novel (`num_enum`/`bytemuck`/`zerocopy`). |
| **Constant-time secret** (different DOMAIN) | SEAM-BREAKS [horn 1, IFC sense] | a horn drops — *in the information-flow domain* | no `unsafe`; the danger is timing | *Branching on a secret leaks it via timing — **not UB**.* The first probe outside memory safety. `ct_eq(a,b).expose_bool()` then `if b` leaks a relationship using only vended ops — **declassification is etiquette, not type-structure**; closing it needs whole-program **undecidable** IFC. **The intrusive list's horn-drop re-run in a new domain — the trilemma recurs, sharper.** Panel ruling: information-flow is **out of frame**, a *different domain* with its **own basis** (declassification discipline, …). **Closure is per-domain; the framework is domain-parameterized.** (Correction: constant-time codegen is *dischargeable* — CompCert-CT/Jasmin/FaCT — not MMIO-shaped.) |

**Six primitives forced a deeper finding: the Seam Trilemma is a 3-of-N projection (N ≥ 9).** The
**slab** keeps all three horns yet pays on *zero runtime cost* (#6), the property the trilemma had
folded into "concentrated & proof-free." The **branded index** keeps all three horns *and* aces
seam-width, yet the cold panel found a real cost "outside the three horns": its generative brand is a
**viral type parameter** (API-locality, #5) — provably *not* seam-width, since the `runST` (PLDI'94)
/ lightweight-capabilities (PLPV'06) lineage pays the identical tax in languages with no seam or
`unsafe` at all. So "concentration" was itself a bundle of a *code* face (seam-width) and a *type*
face (API-locality), and it surfaced **#7 (AI-legibility)** — a brand is a nominal token. The
**refined index** (Flux/Liquid Haskell) then *witnessed* #7's independence: it is the (brand-free,
legible) corner — a refined value escapes its constructor, unlike `Idx<'id>` — so paired against the
slab (reclassified brand-free-but-illegible, two attackers concurring) it flips #7 at fixed #5, and
the judge ruled **#5⟂#7 closes — yes**. It also un-bundled **horn 2** into *decidability* (✓) vs
*solver-freedom* (✗ — an SMT solver in the TCB), the latter being the original `unified` "keep" horn.
The **shared-ownership seam** (a minimal `Rc`) then recovered seed property **#5 bare-metal**: dynamic
shared ownership needs a runtime refcount *manager* (a global heap), which a fixed-pool slab does not —
so it adds a genuine seventh axis (the judge confirmed it ⟂ zero-cost via the slab/`heapless`
off-diagonal). The **take-and-replace seam** then recovered an eighth: moving a value out of a `&mut T`
and back is sound across an unwind only via an abort-bomb, which pays **unwind-safety** — a clean
off-diagonal (it aces all seven prior axes, splitting from SPSC on the guarantee-level alone), confirmed
N ≥ 8 and deflated to pre-existing (Abrahams' exception-safety). The **drain-and-yield seam** then
recovered a ninth: a `Drain` guard whose `Drop` does load-bearing cleanup is sound under a leak
(`mem::forget` is safe) only via **pre-pessimization**, which pays **leak-freedom** — a clean
off-diagonal *distinct from* unwind-safety (take is unwind-sensitive/leak-safe, drain is
leak-sensitive/unwind-safe), confirmed N ≥ 9 and deflated to pre-existing (the leakpocalypse). The
taxonomy of *what the missing conjunct is* and *which axis closing it spends*: floor-expressible →
closes free (SPSC); runtime-checkable → spends #6 (slab; `RefCell`); cross-object identity via a brand →
spends API-locality, #5↔#6 *trade* (branded index; `indexing`); in-bounds via a legible predicate →
spends solver-freedom, R2 trade again (refined index; Flux); dynamic shared ownership → spends
bare-metal, bare-metal↔capacity trade (shared; `Rc`/`heapless`); transient broken invariant across an
unwind → spends unwind-safety via a *discharge menu* (abort / repair / poisoning)
(take; `take_mut`↔`replace_with`); a load-bearing guard `Drop` on a leak-permitting floor → spends
leak-freedom via pre-pessimization (drain; `Vec::drain`); a value whose *address* is load-bearing →
spends **immovability**, which has *no clean coordinate* (self-ref cell; `Pin`); a value whose *bit
pattern* must be valid → projects onto **#6** (per read) or **API-locality** (the zero-copy view),
adding *no new axis* — the closed-basis proof (validity; `num_enum`/`zerocopy`); external physical axiom
→ sound modulo a TCB `assume`, outside the axes (MMIO); whole-heap aliasing+liveness escaping to the
caller → a horn genuinely drops (intrusive). The recovered axes are exactly the casualties of the
original `unified` result, put back on the map one witness at a time — **and, the honest headline,
each turns out to be an already-named PL boundary** (zero-cost abstraction, generativity, refinement-
legibility, `core`/`alloc`, exception-safety, leak-freedom). The panel showed **five** adjudication
modes. It **confirmed-but-deflated** three new dimensions to "real but pre-existing" (bare-metal;
unwind-safety, on take-and-replace; and leak-freedom, on drain-and-yield). It **sharpened without
extending** once (solver-freedom = horn 2's "keep" face). It **rejected outright** once — the *first*
eighth-axis probe, a type-level-proof index pitched as *checker-tractability*: confounded with a strict
expressiveness loss (static-index-only), encoding-relative (binary type-level naturals erase the cost),
and mislabeled (terminating-resolution — Paterson — `Lt` being *linear*). It ruled **two** dangers
**real but structurally inseparable** — the self-ref cell's *move-stability* (a value's address) and the
validity seam's *bit-validity* (a recurring category, both with no isolable cost). **And — the capstone
— it certified a new danger class projects entirely onto the existing basis** (validity adds no tenth
axis), which is what *closes* the basis. The cost-space is a **small fixed set** — nine clean axes + an
inseparable category + two outside-the-axes — that dangers *project onto*; *"what does the witness cost?"*
has **finitely many answers**. So the result is the *unification* of known PL axes under one rubric, not
their discovery — by a method that confirms clean witnesses, refuses confounded ones, recognises the
inseparable, and certifies when a danger adds nothing.

**And one generalization.** That closed object is the **memory-safety** domain's. A thirteenth primitive
— a constant-time secret — stepped outside it, re-importing an *information-flow / timing* danger (not
UB). The panel ruled information-flow **out of frame**: a *different soundness domain* with its **own
basis**, so the closure is **per domain**. Two confirmations: the secret seam **breaks** in the IFC sense
(the intrusive list's horn-drop re-run in a new domain — `ct_eq(a,b).expose_bool()` then `if b` leaks a
secret relationship; declassification is etiquette, not type-structure; sound non-interference needs
whole-program *undecidable* IFC — **the trilemma recurs, sharper**), and the IFC domain has its own axis
with **no memory-safety analog** (*declassification discipline*). So the deepest form of the result is
**domain-parameterized**: the Seam Trilemma and the cost-basis are the same shape in every soundness
domain, with a different basis each — memory safety mapped (basis closed), information flow sketched. The
constructive arc concludes here. Full face lattice: `design_seam/REALIZABILITY.md`.

**Decontamination, across primitives.** The seamful2 theorem predicted that a failing seam's
obstruction *relocates* to the re-import boundary rather than vanishing. SPSC: it **vanished**
(precondition factors through the witness). MMIO: it **moved and stayed decidable** (the
`Send` opt-in). Slab: it **moved off the *static* witness onto a *runtime* one** (the
generation+occupancy check carries what affinity cannot, at a runtime cost). Intrusive: it
**moved and broke soundness** (L2 UAF), and every repair walks into another horn — collapse
lifetimes → drop usability (the `seam/` implementation is this "design A": sound, but it forbids
the early reclaim that is the primitive's purpose); full separation logic → drop decidability.
Branded index: it **moved onto the type surface** (a viral `'id`) and stayed sound — the
obstruction went somewhere genuinely new, neither code-crossing nor runtime. Refined index: it
**moved into the trusted toolchain** (an SMT solver) and stayed sound *and* brand-free — out of the
program entirely. Shared (Rc): it **moved into a runtime subsystem** (the global allocator) and
stayed sound + brand-free — into a heap the no-`std` target does not have. Type-level index: it
**moved onto the trait resolver's fuel budget** (`recursion_limit`) and stayed sound — but *that*
landing site turned out **not to be a clean axis** (it confounds with expressiveness and is
encoding-relative). Take-and-replace: it **moved onto the unwind path** (a mid-take panic becomes a
process abort) and stayed sound — and *that* site **was** a clean axis (unwind-safety). Drain-and-yield:
it **moved onto the leak path** (a forgotten guard loses the un-yielded tail) and stayed sound — and
*that* site was a clean axis too, distinct from the unwind one (leak-freedom). Self-ref cell: it
**moved onto immovability** (a `Pin`-revoked move capability) and stayed sound — but *that* site is a
real danger with **no clean coordinate** (its cost is non-uniform: API-locality for stack-pin, bare-metal
for heap-pin). Validity: it **moved onto an *already-occupied* axis** (bit-validity → #6 per read, or
API-locality for the zero-copy view) and stayed sound — adding *nothing new*, the closed-basis signature.
"Moves, doesn't vanish" held for all eleven non-trivially-encapsulable primitives, and *where* it moved
was the whole verdict — including the one mirage (the fuel knob), the two genuine new axes (unwind, leak),
the one *real-but-inseparable* destination (immovability), and the one that landed *on coordinates already
on the map* (validity) — the last of which is what closes the basis.

**The adversary earned its keep — twelve times.** On MMIO it named the axiom's category correctly
(a TCB boundary, not a horn) and surfaced a horn-3 cost the builder had under-weighted. On the
intrusive list it proved something *worse* than the builder predicted: branding does not restore
soundness at all (a brand is an *access* capability, not a *storage* lifetime), correcting a real
overclaim. On the slab it found a genuine UB in the builder's *spec* (a generation-only guard
authenticates a fresh slab's vacant gen-0 slot) — though the *implementation* checked occupancy
first and a regression test proved it rejects the exact exploit; the revised spec then survived.
On the branded index it confirmed the fifth axis with four decades of grounding *and* corrected a
zero-cost overclaim (single-index `try_index` relocates rather than eliminates the bounds check),
then argued the sixth-axis probe. On the refined index it ruled the #5⟂#7 cell **closed**, found an
undisclosed expressiveness bug (R2 used `==` where safety needs `≤` — fixed), exposed the **R2 trade**
(brand-free + zero-cost + sound is unachievable at once), and *deflated* an overclaim of novelty
(solver-in-TCB is DML/Liquid Types/Flux's documented founding tradeoff — the `unified` "keep" horn,
not a new axis). On the shared seam it confirmed bare-metal is a real seventh axis yet *deflated its
novelty* — it is Rust's `core`/`alloc` boundary (RFC 2480), with `heapless` already witnessing the
bare-metal↔capacity trade — forcing the honest "unification, not discovery" framing. And on the
**eighth probe** it did the hardest thing of all: conceded soundness but **rejected the axis** — three
independent attackers converged that the type-level index's "checker-tractability" off-diagonal is
confounded with expressiveness (static-index-only, intrinsically), encoding-relative (binary numerals
erase the cost), and mislabeled (terminating-resolution, not tractability; `Lt` is linear). The
builder took it on the chin — N stayed 7 on that probe. And on the **second eighth-axis probe**
(take-and-replace) it did the complementary thing: confirmed unwind-safety as a real axis (**N ≥ 8**)
on a clean off-diagonal, but corrected two builder framings — the axis is the
*exception-safety-guarantee-level* (abort is merely this seam's discharge of it; `replace_with` pays
API-surface instead, a discharge menu), and prior-art surfaced that **RFC #1508 rejected this exact
`take_mut` from std** on the very abort cost scored, so "ace seven, pay one" was the historically losing
argument. And on the **ninth axis** (drain-and-yield) it confirmed leak-freedom on a clean off-diagonal
*and* ruled it **distinct from** unwind-safety — refusing the tempting merge of the two
destructor-execution siblings (leak = `Drop` never runs; unwind = `Drop` runs at a bad time) — while
flagging it the thinnest, most-deflatable axis in the lattice (yet still real: a populated off-diagonal,
not the non-axis the checker-tractability probe was). And on the **self-referential cell** it did the
subtlest thing of all: conceded soundness, conceded that move-stability is a *genuinely distinct danger*,
yet ruled it **real but structurally inseparable** — no isolable cost (the price is immovability,
non-uniform across `Pin` witnesses) — and corrected three builder framings (the cost is immovability not
"API-locality alone"; the inseparability is *contingent*, RFC 2349 having rejected `?Move` immovable
types; the crisp reading is a non-composable toy). And on the **validity seam** it earned its keep to
the last: two of three gauntlets found real defects — a *range* check unsound for non-contiguous
discriminants (four attackers converging), then a residual stale `b<=2` the builder left when fixing the
first — before the corrected seam survived; and it refined the builder's prediction (validity is a
*second* inseparable danger, not the uniform #6 predicted) while **unanimously upholding the meta-claim:
no tenth axis, the cost-basis is closed.** And on the **constant-time secret** it did the most generative
thing of all: it **broke the seam** (the IFC-domain horn-drop — `ct_eq(a,b).expose_bool()` then `if b`
leaks a secret relationship; declassification is etiquette, not type-structure) *and* vindicated and
enriched the builder's domain hypothesis — information-flow is **out of frame**, a different domain with
its own axis (declassification discipline), closure is **per domain** — while correcting two over-claims
(the §4 universal; the false MMIO-axiom analogy, since constant-time codegen is dischargeable per
CompCert-CT/Jasmin/FaCT). Spec↔impl divergences, sharpened overclaims, three novelty-deflations, one
axis-rejection, one relabel, one distinctness-ruling, an inseparable-category finding (twice), the
closed-basis certification, and a domain-generalization (with a horn-drop in the new domain) — exactly
what build/break separation exists to surface. A discovery procedure that can say "no", "the right name",
"why it doesn't merge", "real but unmeasurable", "real but adds nothing — the basis is closed", and "this
is a *different domain* entirely" is measuring something real; this one did all six.

**Mechanization.** `proof/SeamGap.lean` was upgraded from the one-direction triviality its own
cold review had flagged to a genuine characterization — `seam_closes_iff_factors`: a seam
closes **iff** its precondition factors through the safe witness — with the regimes as
machine-checked instances (`spsc_seam_closes`, `intrusive_seam_does_not_close`, the slab's
`slab_runtime_closes_but_static_does_not`: the same precondition closes through a *runtime*
witness but not the *static* one, and `branded_seam_closes`/`refined_seam_closes`/`shared_seam_closes`/`typed_nat_seam_closes`/`take_seam_closes`/`drain_seam_closes`/`selfref_seam_closes`/`validity_seam_closes`/`secret_typed_discipline_factors`:
all close like SPSC, the recovered axes invisible to the kernel), `propext` only. Honest bound unchanged: the
*math* is now a real characterization, but the faithfulness of "precondition" and "witness" to
actual Rust primitives — *which axis a closing seam spends* (zero-cost, API-locality, legibility,
solver-freedom, bare-metal, unwind-safety, leak-freedom), and whether that "axis" is even *separable*
(the self-ref cell closes identically yet its move-stability cost is inseparable) — still lives in the
prose modeling dictionary, not the kernel, which sees only whether a Boolean witness suffices.

**The one-sentence upgrade.** The trilemma says a *seamless* language is impossible; the
constructive phase says **which** seams you can build losslessly anyway — exactly the ones
whose dangerous precondition is fully encapsulable — while the rest force you, predictably and
measurably, to choose a horn *or pay one of the recovered axes* (zero-cost, API-locality,
legibility, bare-metal), with the precondition's shape telling you in advance *which* horn or axis
and *how wide* the seam will be. The three horns are one shadow of an ≥7-axis object — each a
*pre-existing* named PL boundary that the trilemma had bundled together — and each constructive
primitive lit one up by being the witness that two trilemma-identical configurations differ; the
result is the unification, not the dimensions. Full write-up: `design_seam/SYNTHESIS.md`; face
lattice: `design_seam/REALIZABILITY.md`.
