# The Seam Is Load-Bearing: An Impossibility Result for Seamless Safe Systems Languages, and a Constructive Map of What You Can Build Instead

*Draft. Prose realization of the settled findings in `WRITEUP.md`, `design_seam/SYNTHESIS.md`, and `design_seam/REALIZABILITY.md`. Framing: result-first (the seam is load-bearing), method-grounded (the cold gauntlet earns the claims), with the reflexive builder-calibration finding as the methodological payoff. Abbreviations spelled out on first use: information-flow control (IFC), data-race freedom (DRF), non-lexical lifetimes (NLL), undefined behavior (UB), satisfiability modulo theories (SMT), concurrent separation logic (CSL), trusted computing base (TCB), quantitative information flow (QIF), control-flow graph (CFG).*

---

## Abstract

We ask whether one programming language can be, at once, *decidable and proof-free* in the common case, *seamless* (no foreign-function boundary between "proof-land" and "systems-land"), and able to give garbage-collector-free memory safety *derived* from its own proof layer, at zero runtime cost, on bare metal. We answer no — and we do so not by hand-waving but by an adversarial process of *cold*, context-stripped language-model instances (a proposer, five single-angle attackers, a judge, a synthesizer) that settles a design only after three consecutive survived challenges, and that we *calibrate* three ways: it settles a known-good design, rejects known-garbage, and still fails the target. Across five design questions and 24 rounds, nothing settled; the six recorded incompatibilities are facets of one theorem: at every ownership-state merge point the entailment a checker must decide is undecidable, and the seamless/derived/zero-cost requirements each delete the one boundary that could relocate that obligation elsewhere. Existing languages ship by conceding *exactly one* property.

We then take the productive concession — make the boundary a first-class in-language **seam** — and find it obeys a **Seam Trilemma**: pick two of {sound re-import, decidable-proof-free-zero-cost safe fragment, seam concentration}. To map it, we *build and break* 37 real Rust seam primitives. Whether a seam closes losslessly is a function of the precondition's *shape*, not of "seams"; the trilemma is a **3-of-N projection of a closed cost-basis** (nine independent axes, an inseparable category, two costs outside the axes); the whole result is **domain-parameterized** (memory safety / IFC / liveness), and each domain closes only to the degree its *floor matches the precondition* — a **graded** match. Finally, a reflexive finding available only to this method: the cold gauntlet **measures** the warm builder's miscalibration, caught eight times and traced along a non-monotonic arc. We are explicit that this is "proven by survived adversarial challenge" — calibrated, and machine-checked in *skeleton* (Lean 4) — not a mechanized end-to-end proof.

---

## 1. Introduction

A recurring dream in language design is a single language that needs no seam: the specification *is* the type system *is* the compiler; memory safety is *derived* from a proof layer rather than hardcoded; that proof layer erases at zero cost; and the whole thing checks decidably, with no hand-written proofs in the common case, on bare metal. Concretely, the seed for this work (`LANG_DESIGN_SEED.md`) asked for seven properties together: (1) executable specification, (2) a unified proof/systems layer with no FFI seam, (3) GC-free ownership *derived* from the proof layer, (4) decidable proof-free checking, (5) bare-metal targeting, (6) zero-cost erasure of the proof layer, and (7) AI-legible semantics.

The seed's own success criterion was *not* "build the language." It was a precise map of which combinations of the seven are genuinely incompatible — **proven by survived adversarial challenge rather than asserted by authority.** This paper is that map, in two halves: a negative result (the seven are not jointly satisfiable, and the obstruction is precise) and a constructive one (having conceded exactly what must be conceded, *here is what you can build, and what everything else costs*).

The framing that makes the negative result land is **covering versus intersection**. A handful of languages *together* already provide all seven properties; but no single language holds all seven, because each concedes one to stay coherent. The union is a covering; no point in the union is the intersection. The substantive question is not "can we list languages that each have some property" — trivially yes — but *why the covering cannot be collapsed into one language.* Our answer is that the collapse **is** property (2): "combine them into one seamless language" is itself the seamlessness requirement, and seamlessness is one of the three things the theorem says you must give up. The act of merging is the forbidden move.

**Contributions.**
- **C1 (§2).** A single impossibility theorem unifying six independently-discovered property incompatibilities, with the reduction skeleton that makes decidability the universal antagonist.
- **C2 (§3).** A *calibrated* adversarial-model methodology for design-space impossibility claims — the cold-instance mechanism, and the three controls that make a 0/N result diagnostic rather than ambiguous.
- **C3 (§4).** The **Seam Trilemma**: the structural cost of conceding seamlessness, obtained cleanly via a decontamination step.
- **C4 (§5).** A **build-and-break** empirical map of 37 seam primitives: the trilemma is a 3-of-N projection of a *closed* cost-basis; the result is domain-parameterized; the floor-domain match is graded and not freely liftable.
- **C5 (§5.6).** A **reflexive** methodological finding: a cold stateless adversary *measures* a warm stateful builder's calibration — a use of the method that neither party could produce alone.

We keep the honest bounds (§6) visible throughout: every "proven" below means "survived calibrated adversarial challenge," not "mechanically verified."

---

## 2. The Impossibility Result

We ran the adversarial process (§3) on the seven-property target as five falsifiable design questions — a sequential ownership core, the decidability boundary, proof erasure, the unsafe boundary, and interior mutability under weak memory. Across 24 rounds, **nothing settled.** Every question exhausted its revisions, and the synthesizer recorded a hard incompatibility for each. The six incompatibilities share one structural feature: **all six place decidability (#4) on one side of the collision.**

| Question | Incompatibility |
|---|---|
| Sequential ownership core | derived ownership ✕ decidable Rust-cheap checking — at the control-flow merge point |
| Decidability boundary | derived *sound* ownership ✕ decidable proof-free checking — on real data structures |
| Proof erasure | zero-cost erasure needs witnesses *irrelevant* to equality; decidable seam-free checking needs them *relevant* |
| Unsafe boundary | a decidable auto-discharged seam can't restore the open, entangled invariant set soundness demands |
| Interior mutability & weak memory | decidable proof-free ✕ sound zero-cost typing of shared-mutable / weak-memory state |
| (bundled memory question) | decidable ✕ no-seam ownership over weak-memory aliasing |

The failure *reasons* were specific and varied — an admissible-contraction double-free; a non-associative `meet` table laundering conflicting ownership states into `Moved`; poison slipping past a location-indexed seam; an implicit-flow secret laundering; the independent-reads-independent-writes (IRIW) litmus test certifying a consensus that the POWER memory model permits to fail. Yet they converge on one thesis.

**The theorem (the six are one).** A meta-synthesis pass found the six are not independent findings but facets of a single theorem. In plain terms: *a type-checker can be automatic (decidable, no hand-written proofs) or it can be strong enough to prove that real shared, aliased, weak-memory code is safe — but not both, because the safety facts those hard cases need are, at the points where the checker must combine them, provably beyond what any always-terminating check can decide.* Formally:

> No type system is both (a) decidable and proof-free in the common case, and (b) the sole auto-checked discharge site for the substructural, heap-quantified, value-/view-dependent invariants that soundness of GC-free ownership over aliased weak-memory state requires — because at every ownership-state merge point (CFG join, loop fixpoint, erasure-time equality, unsafe/atomic/DMA seam) deciding entailment over that invariant class is undecidable, while the no-seam, derived-ownership, and zero-cost requirements each remove the only boundary that could move the obligation out of the auto-checked fragment.

The structure is uniform. Every facet bottoms out at a merge/decision point where the checker must decide entailment between two ownership-or-protocol states. Soundness of GC-free ownership over aliased weak-memory state forces those states to be *substructural* (linear, no contraction), *heap-address-quantified* (∀-indexed footprints), and *value-/view-dependent* (transferable protocol invariants). Deciding entailment over that class subsumes a provably undecidable problem — bi-abduction / Hilbert's tenth problem at the join, the Bradley–Manna–Sipma array-property fragment on the footprints, open-term definitional equality at erasure. The decidable, zero-annotation fragment is *necessarily the dual* of that class: finite-height, quantifier-free, first-order, contraction-admitting. So it is provably too weak to name the invariant, and enriching it enough to name the invariant inherits the undecidable entailment.

**#4 is the universal antagonist; #2, #3, and #6 are three forcing functions.** They are not separate causes but three ways of removing the boundary — a trusted seam (#2), a hardcoded analysis (#3), a runtime witness (#6) — that could otherwise relocate the hard obligation out of the decidable fragment. This is why existing languages survive (§4): each keeps one of those boundaries.

---

## 3. Method: The Cold Adversarial Gauntlet

A process that settles *nothing* is, from its outputs alone, indistinguishable between "the target is impossible" and "the instrument is rigged to fail." The methodological contribution is the machinery that resolves that ambiguity.

**The roles.** The process (implemented as `orchestrator/`) is a pipeline of language-model instances with disjoint jobs. A **proposer** emits one concrete, formal, falsifiable design decision. **Five cold attackers** — ergonomics, soundness, implementation, expressiveness, prior-art — each receive the proposal *only* and try to break it from a single angle. A **judge** rules each attack genuine-flaw / argued-past / tradeoff, then SURVIVED / FAILED / TRADEOFF. A **synthesizer** distills a question that fails every revision into a recorded property-incompatibility (the seed's payload). A **reconciler** checks whether two settled decisions, held together, force a new incompatibility. A **historian** keeps deterministic state.

**Why coldness is the load-bearing trick.** Each role is a fresh `claude -p` subprocess with the ambient context stripped: a replaced system prompt, no settings loaded (so no hooks), no MCP tools, a clean temporary working directory. We verified the coldness — an attacker asked about the operator's unrelated context answers "no." The reason this matters is adversarial: an attacker that secretly shares the proposer's framing is not an adversary, and warm shared context is exactly such a secret share. Termination is faithful to the seed: a decision settles only after surviving **three consecutive** cold gauntlets, with fresh attackers each time; any FAILED gauntlet feeds the landed attacks back and resets the streak.

**Calibration — the trust argument.** We tested the process against itself three ways, and this is what turns "0/5" from ambiguous into diagnostic.
- *True negative — a known-good design must settle.* Rust's NLL borrow checker, with the hard constraints (#2/#3/#6) waived, run through the identical gauntlet: **SETTLED** — and honestly. Revision 0 FAILED on a real `dropck`/`#[may_dangle]` use-after-free (the process caught an actual bug); revision 1 then survived three consecutive gauntlets, the judge ruling the residual attacks "puncture the author's reconstruction, not the design." It settles a good design, still requires correctness, and judges the design rather than the prose.
- *True-positive rejection — garbage must fail.* An obviously-unsound non-design ("programmers just don't write bugs," no checker) FAILED all four revisions, the soundness attacker instantly refuting `check = ⊤ ⟹ ∀e. safe(e)` with a well-typed use-after-free.
- *Stronger-proposer stress test.* Was 0/5 merely a weak one-shot proposer? We re-ran the most-precedented question (the sequential core) with a **3-shot self-hardening proposer** that critiques and strengthens its own proposal twice before any attacker sees it. It **still FAILED**, on the same fault line. The clean contrast: a 1-shot proposer *settled* Rust; a 3-shot proposer *could not settle* the constrained core. More effort on the harder-constrained problem still failed while less effort on the unconstrained one succeeded — so the **constraints, not proposer weakness, are decisive.**

Net: the instrument settles the good, rejects the bad, and fails the subject. It is measuring a property of the *subject*, not of itself.

---

## 4. The Constructive Flip: The Seam Trilemma

The negative result says: concede exactly one of #2 (no-seam), #3 (derived ownership), #6 (zero-cost erasure). The most natural concession is #2 — keep one language, but make the boundary between proof-land and systems-land a *first-class, in-language, machine-checked seam* instead of an FFI to a separate prover. Rust + `unsafe` is this shape informally; Verus, F\*/Low\*, Dafny, and RefinedRust make it formal. So the live question is not "should a seamful language exist" but *which* calibration, and what its limit is. The aimed-for calibration: the safe fragment stays decidable and proof-free, user proofs appear only at seam crossings, and — the make-or-break constraint — the seam **concentrates** into foundational primitives (allocators, atomics, lock-free structures, MMIO) that ordinary code composes through a *checked safe interface* without itself crossing the seam.

**Two runs, because the first was contaminated.** Conceding #2 while keeping #3 (`seamful`) gave 0/6 with two near-misses — but the run was contaminated: keeping #3 forced the proposer to *derive a borrow checker from scratch*, and it kept shipping almost-right rules with a dropped premise. Several failures read "Rust-parity, sound in spirit, defective as written" — proposer formalization bugs, not proven structure. The honest fix was a controlled re-run conceding #2 *and* #3 (`seamful2`): the safe fragment is *stipulated* as shipped Rust NLL (a known-good checker), so the entire gauntlet bears on the seam. Still 0/6 — but now clean. The recorded obstruction became "sound re-import needs a linearity witness NLL's region-only vocabulary cannot carry." **Removing #3 did not rescue the seam; it isolated the obstruction onto the seam's re-import boundary** — exactly where the `seamful` theorem predicted it would land.

**The trilemma.** The decontaminated meta-synthesis states the result as a trilemma. The seam must concentrate on foundational primitives whose true preconditions are semantic heap-, value-, and concurrency-level invariants, expressible only in an undecidable, certificate-free concurrent separation logic (Iris/RustBelt grade). But the safe fragment can receive only a witness that is simultaneously decidable, proof-free, and zero-cost-erasable. The re-import obligation is therefore strictly weaker than the precondition it must certify, and the missing conjunct forces exactly one of three fatal horns:

> **Pick at most two of:**
> 1. **Sound re-import** — crossing back launders no UB into safe code.
> 2. **Decidable, proof-free, zero-cost safe fragment** — ordinary code needs no proofs.
> 3. **Seam concentration** — ordinary code never crosses the seam.
>
> Keep 1+2 → **drop concentration** (the verified-Rust reality: annotations at use sites, not just inside `unsafe`).
> Keep 1+3 → **drop decidability** (Verus/F\*: the witness goes proof-bearing / needs an external prover).
> Keep 2+3 → **drop soundness** (the interface silently weakens the precondition; UB launders into safe code).

Every shipping point on the map is a deliberate choice of which horn to drop: Rust drops concentration informally (`unsafe`'s obligation lives at use sites, in the programmer's head); Verus/F\*/Creusot drop decidability (proof obligations at the seam); the unsound corner is the one to avoid, and the positive-rejection control (§3) shows it is detectable. The three-program map:

| Program | Concedes | Settled | Result |
|---|---|---|---|
| `unified` | nothing | 0/5 | Decidability (#4) ✕ {#2, #3, #6} — keeping all seven is impossible |
| `seamful` | #2 | 0/6 | sound re-import is the casualty (contaminated by keeping #3) |
| `seamful2` | #2 + #3 | 0/6 | **the Seam Trilemma**, decontaminated and structural |

**Decontamination as a general method.** The decisive move was handing the proposer the known-good sub-component (Rust NLL) and watching the failure *move, not disappear.* A 0/N adversarial result is only as good as your ability to rule out "the proposer was just weak"; the cleanest way to do so is to give the proposer the strongest available floor and localize what remains.

---

## 5. Build and Break: 37 Seams

The trilemma is a map, so the next step was to stop arguing in the abstract and **build.** We changed the harness shape: instead of a cold instance *proposing* a rule, the (warm, stateful) builder authored a real artifact — a rustc-compiling Rust primitive re-exporting a dangerous core behind a checked-safe interface, plus a Lean instance and a frozen specification — and the cold panel attacked the *artifact.* Verdicts are SEAM-HOLDS (all three horns hold, no cost), SEAM-COSTS [horn] (a marked cost outside the three horns), or SEAM-BREAKS (a horn genuinely drops). The floor is fixed: shipped Rust NLL plus affinity plus decidable trait bounds. Over 37 primitives we accumulated the findings below.

### 5.1 The headline: closure is a function of precondition shape

Whether a seam closes is **not** a property of "seams." It is a function of one question: *can every conjunct of the primitive's true precondition that the caller can observe be carried by the floor's witness vocabulary (affinity, NLL lifetimes, decidable trait bounds) — and if not, is the missing conjunct an external axiom, a runtime-checkable fact, or genuinely unrepairable?*

The two poles make this concrete. A single-producer single-consumer (SPSC) ring buffer **SEAM-HOLDS** across four independent cold panels: every escaping conjunct is floor-expressible (cardinality → affinity, storage-lifetime → an NLL lifetime, payload safety → a decidable `T: Send`), and the genuinely hard fact (the happens-before relation between producer and consumer) leaves no caller-visible residue. An intrusive doubly-linked list **SEAM-BREAKS [horn 1]**: whole-heap aliasing and liveness escape to the caller, and the `GhostCell` brand carries the aliasing (L1/L4) but not the membership-versus-storage lifetime (L2) — presented a correct token, the rule type-accepts a use-after-free. Same "seam," opposite verdict, because the preconditions have different shapes. The trilemma bites *selectively*, and where it bites is predictable.

### 5.2 The closed cost-basis: the trilemma is a 3-of-N projection (N ≥ 9)

The three horns are a low-dimensional shadow of a closed object, recovered one primitive at a time — each primitive the *witness* that two previously-indistinguishable configurations actually differ on some axis.

- The **generational slab** keeps all three trilemma horns yet still pays — on **zero-cost erasure (#6)**: a `Copy` key can outlive its slot, so affinity and NLL cannot carry validity, but a per-slot generation-plus-occupancy tag can, *at run time.* It closes the trilemma and spends #6 — `RefCell`'s move.
- The **branded index** un-bundles *concentration itself* into two faces: a **code face** (seam-width — one `scope` per slice, O(1) crossings) and a **type face** (**API-locality** — the brand `'id` never appears as a code crossing but appears in every consumer *type* and traps indices inside a `scope` closure). That the `runST` (PLDI'94) / lightweight-monadic-regions (PLPV'06) lineage pays the identical tax in languages *with no seam or `unsafe` at all* proves the type face is not seam-width. The same primitive surfaces **AI-legibility (#7)**: a brand is a nominal token.
- The **refined index** witnesses the #5⟂#7 dissociation the branded index argued for — it is the (brand-free, legible) corner where a refinement predicate `{v : Int | v < len}` escapes its constructor — and un-bundles **horn 2** into *decidability* (the refined fragment terminates) versus *solver-freedom* (it needs an SMT solver in the trusted loop, the `unified` "keep" horn).
- The **shared-ownership seam** (a minimal `Rc`) recovers **bare-metal (#5)**: dynamic shared ownership needs a runtime refcount manager and thus a global heap — the crate's sole `extern crate alloc`. The judge confirmed the axis but deflated its novelty: it is Rust's own `core`/`alloc` boundary (RFC 2480).
- The **type-level-proof index** *probed for an eighth axis and failed* — recorded as a result, not buried. It is sound but recovers no clean axis, on three convergent grounds: the off-diagonal is confounded (a turbofish type is static-index-only, a strict expressive subset), the cost is encoding-relative (binary type-level numerals erase the `recursion_limit` hit), and it is mislabeled (`Lt` resolution is *linear* — the real concern is terminating resolution, à la GHC `UndecidableInstances`, pre-existing). The discipline can refuse.
- **Take-and-replace** confirmed the eighth axis, **unwind-safety**: moving a value out of a `&mut T` and back leaves a duplicate in the gap, and an unwinding panic drags the caller into a double-drop; the sound witness is an abort-bomb (`take_mut`'s pattern). It aces all seven prior axes and pays on unwind-safety *alone* — the clean off-diagonal the type-level index lacked. The mandatory relabel: the axis is the *exception-safety-guarantee-level*, and the abort is this seam's *discharge* of it (a `replace_with` sentinel discharges the same obligation by paying API-surface instead) — so the axis carries a **discharge menu**, not a fixed cost.
- **Drain-and-yield** confirmed the ninth axis, **leak-freedom**, distinct from unwind-safety: `mem::forget` is *safe* (the Rust "leakpocalypse"), so a guard whose `Drop` does load-bearing cleanup can be leaked; the sound witness is pre-pessimization (`Vec::drain`'s trick — empty the buffer up front). The off-diagonal is populated *both* ways (take is unwind-sensitive/leak-safe; drain is leak-sensitive/unwind-safe — "Drop runs at a bad time" versus "Drop never runs"). The thinnest axis, but genuinely distinct.
- The **self-ref cell** probed for a tenth axis, **move-stability**, and returned the project's most novel verdict: **real but structurally inseparable (N stays 9).** The danger is genuinely distinct (no other axis concerns a value's *address*) and soundly handled (`Pin` over a `!Unpin` cell makes the post-init move unconstructible), but it has no isolable cost: `Pin<&Self>` revokes `&mut` (and that revocation *is* the soundness mechanism), and the cost is non-uniform across witnesses (stack-pin spends API-locality; `Box::pin` spends bare-metal). A fourth verdict category, neither clean axis nor confounded non-axis.
- The **validity seam** (a `#[repr(u8)]` enum byte that must be a valid discriminant — UB even without a deref) asked whether the cost-basis is **closed**, and the panel unanimously ruled it adds *no tenth axis*: every witness spends a pre-existing coordinate. Its own cost is inseparable in the move-stability sense (a per-read tag check spends #6; the zero-copy `&[u8] → &[Tag]` view spends #6 or a sealed constructor's API-locality) — a *second* inseparable example, making that a recurring category.

Read in both directions, the cost-space is a **small fixed basis** that dangers project onto: the `get_unchecked` family shows *one* danger with *many* witnesses spending *different* axes; validity shows *many* dangers converging on the *same* axis. The nine clean axes are Sound, Decidable, Concentration, API-locality, Zero-cost, AI-legible, Bare-metal, Unwind-safety, Leak-freedom; plus an inseparable category (move-stability, validity) and two costs *outside* the axes (the MMIO TCB axiom, the intrusive horn-drop). Appendix B gives the full axis table and the pairwise-dissociation witnesses that make each axis independent. The honest coda: each axis is a *pre-existing* named PL boundary; the contribution is their unification under one rubric — *what does the witness cost?* — a question with finitely many answers.

### 5.3 Domain-parameterization and the graded floor-match

The closed object above is the **memory-safety** domain's. The same shape recurs, with a different basis, in every soundness domain — and *which* seams close depends on how well the floor matches the domain's precondition.

- A **constant-time secret** steps outside memory safety: branching on a secret leaks it via timing — *not* UB. It **SEAM-BREAKS [horn 1, in the IFC sense]**: `ct_eq(a, b).expose_bool()` then `if b` leaks a relationship; declassification is *etiquette, not type-structure*, and closing it needs whole-program undecidable IFC. The panel generalized the whole result: information flow is *out of frame* — a different soundness domain with its own basis (declassification discipline, termination/concurrency channels). So "the cost-basis is closed" refines to **closed per domain.**
- A **capability-gated declassification** probes whether the IFC domain has a *closing* seam (the IFC analog of SPSC). It also breaks, unanimously: to compute on a secret, `map(self, f)` must hand the closure the *bare plaintext*, which a captured-`&mut` closure exfiltrates with no declassify site; and the affine "budget" counts *uses*, not *bits* (a loop mints N capabilities; one capability leaks a megabyte). What it *can* enforce — release *location* — projects onto the memory-safety affine axis; the genuinely-IFC coordinate is the *amount* (quantitative information flow, Smith min-entropy), which an affine floor cannot bound. **Both IFC seams break for one reason: Rust's affine floor carries none of the IFC precondition.** It is *matched* to memory safety and *mismatched* to information flow (the coordinate needs Jif / FlowCaml / QIF).
- **Type-level lock ordering** steps into a third domain, **liveness** (deadlock is neither UB nor a leak), and **SEAM-BREAKS [horn 1]** two independent ways, which is the richer result because it *grades* the match. (1) *Granularity*: `Floor::root` is re-entrant, so two sibling lineages let safe code type a low-then-high inversion (verified to compile and run) — the discipline is per-token while the precondition is per-thread-global, and Rust's floor has no ambient per-thread one-shot resource. (2) *Coverage*: even patched, the dynamic-acyclicity fragment (runtime-ordered / same-rank locks — bank-transfer `min/max`, hand-over-hand coupling) escapes the static floor, exactly as IFC's quantitative coordinate escaped the affine one. So the floor-domain match is **graded** (full / partial / none); liveness on a static-order floor is a *partial* match.
- A **runtime lock registry** (a `thread_local!` held-set checked per acquire — lockdep in miniature) asks whether *upgrading* the floor lifts the grade. It fixes the coverage gap and pays the predicted price (bare-metal — needs an OS thread-local, so not `no_std`; zero-cost — a per-acquire check; and a prevention→detection modality), but **granularity recurs one rung finer**: `HELD` is keyed on the OS *thread* while the unit of control in async is the *task*, so work-stealing migration launders an undetected inversion. The grade is **not monotonically liftable** — the granularity axis is a containment-scope *ladder* (token < thread < task) that no ambient Rust floor tops out (there is no task-local; async is a library).

### 5.4 The second movement: composite preconditions and the fourth-domain probes

The remaining twenty seams close the account **without adding a horn, an axis, or a domain** — the map is finite and now full.

**Liveness has exactly three coordinates** — completion, leak-freedom, and progress — each appearing on both a *lock* face and a *session* face. The session family (binary and multiparty session types, choice, delegation, recursion) mapped the progress and completion coordinates; a scoped continuation-passing combinator showed **completion is concentrable after all** (per-endpoint, via an unforgeable return-type obligation rather than a forgettable destructor — so `mem::forget` is defanged), correcting an earlier over-claim that completion was irreducible. Only *termination* is left as a genuinely undecidable NONE, and even guardedness/productivity — first mis-filed as undecidable — turned out to concentrate behind a sealed `Guarded` marker.

**Four probes for a fourth soundness domain, four deflations.** Convergence (a CRDT) *folds* onto an application-correctness basis; worst-case execution time *breaks* on the cost/cycles gap; totality (integer division) *folds*; and data-race freedom *reclassifies* into memory safety. Four candidate domains, four distinct deflation mechanisms (fold / break / fold / reclassify), none opening a fourth soundness domain — so the domain count is **three, closed**, with an application-correctness *basis* absorbing semantic-law dangers.

**The DRF reclassification collapses a trichotomy to a dichotomy.** A first data-race probe (`sync_cell`) broke by design and, more importantly, *refuted* the tempting claim that data-race freedom is "floor-closed." A correctly-bounded core-only spinlock (`spin_lock`, after one broken intermediate) **SEAM-HOLDS**, carrying an `unsafe impl` whose obligation is *discharged* by the lock invariant. The consequence: the prior implicit trichotomy — IFC closes nothing / aliasing concentrates / concurrency is floor-closed — collapses to a **dichotomy.** IFC is the lone *floor-closes-nothing* domain; **memory-aliasing and data-race freedom are the same** floor-propagates-but-concentrating shape, each with a **static** end (SPSC, the slab — compiler-verified) and a **dynamic** end (`SpinLock`, `RefCell` — a runtime witness). An `unsafe impl` is revealed as a proof obligation that is either **discharged** or **false** (RustBelt semantic type soundness, floor-relativized). The adjudication vocabulary itself closed at **nine verdict modes** over a **ten-seam** broken tally.

The unifying structural claim: a floor matches a *composite* precondition **component-wise.** A session-typed channel is the first out-of-memory-safety probe to *hold with a cost* — protocol order closes via affine typestate, while completion is a horn-3 concentration cost (the same `mem::forget` gap that *broke* an earlier seam only *costs* here, because completion sits beside a sound safety core). *Which horn a danger threatens is set by how the re-import rule packages it* — but that packaging dial is itself **bounded** by whether soundness can be decoupled from the danger: owned data lets the danger cost; borrowed-across-unwind, address-pinned, or borrow-scoped data forces a fixed horn.

### 5.5 The reflexive finding: coldness as a calibration instrument

The second movement's own discovery is reflexive, and it could only come from *this* method — a warm, stateful builder red-teamed by a cold, stateless gauntlet. **The builder has a measurable, domain-local miscalibration:** a tic to *flee the decidable middle toward whichever extreme is cleaner* — total concentration, or irreducible/global, or undecidable. The gauntlet caught it **eight** times as sound-but-rejected deliverables (a "rejection mode": the seam is sound, but the write-up over-claims a horn), and each time refuted the over-claim *from the builder's own citations.*

The arc is **non-monotonic**, which is what makes it a measurement rather than a verdict. It ran horn-deciding early; then, as the builder learned to pre-concede the uncarriable half, it receded to a marked tradeoff *below* the horn-deciding threshold; then it re-fired at *full* strength on the fresh concurrency domain, where the concurrency `unsafe impl`'s full obligation set only got mapped *under the gauntlet* — a mutex has four auto-trait obligations, the builder discharged three and dropped the guard's `T: Sync`, and the omission recurred *inside the very salvage meant to prove it fixed* before a third attempt enumerated all four and held. The lesson: on novel ground the tic fires *repeatedly until the gauntlet exhausts the obligation set, then recedes.* (That third seam's fix is verbatim Ralf Jung's June-2017 std `MutexGuard`-`Sync` bug and its shipped patch, later proved sound by RustBelt, POPL 2018 — novelty ~nil, but the *process* observation is the contribution.)

The constructive dual of the impossibility result is the **five settled salvage lineages** — four one-step (a rejected or broken seam whose prescribed fix then held) and one two-step (the fix itself first broke). You cannot build a *seamless* language; but against a cold adversary a single *seam* can be **converged** to soundness, and *the round-count of that convergence measures how far the danger sat from what the builder already knew.* That reframes build-and-break from a quality-assurance loop into an epistemics experiment: coldness is not merely a demolition tool but a calibration instrument.

---

## 6. Threats to Validity

We state the bounds louder than the claims.

- **This is not a mechanized proof.** "Proven by survived adversarial challenge" is a strong, calibrated *empirical* result. The undecidability reductions it cites (Hilbert's tenth problem, the Bradley–Manna–Sipma array-property fragment, open-term definitional equality) are real and load-bearing, but the synthesis assembling them was not machine-checked.
- **The Lean skeleton is honest about its own triviality.** The Seam Trilemma's logical skeleton *is* machine-checked (`proof/SeamGap.lean`, axiom-clean: the core gap needs no axioms, the trilemma only `propext`). But a cold review forced the depth caveat (`proof/REVIEW.md`): the checked lemma is information-theoretically trivial ("a one-field projection cannot recover a two-field predicate"); all the PL substance lives in the *unverified, prose-level modeling dictionary* (the map from the checker's view `V`, the soundness precondition `P`, and why `Copy` forces the blindness). The mechanization checks the skeleton and *forces the faithfulness claim to be stated and attacked;* it does not lift the substance out of "survived adversarial challenge."
- **Bounded proposer depth.** Revisions were capped (three; up to three self-strengthening shots). The stress test (§3) pushes against this but does not eliminate it; a human PL theorist with years might find a path the process did not — though they would be fighting the same reductions.
- **Shared model weights ⟹ shared blind spots.** All role instances share weights, so they share blind spots. The prior-art attacker used live web search to partially counter this; full mitigation needs human-injected attack angles (a seed-specified hook not fully exercised). This tensions specifically with §5.5: the reflexive finding is itself *produced by* the warm/cold asymmetry, so its strength is bounded by how truly cold the cold instances are — a warm leak would blunt the very calibration it measures.
- **Deflationary novelty throughout.** Nearly every axis and every held seam maps to a pre-existing, named result. The contribution is the *unification and the map* — one rubric under which nine scattered PL boundaries become one closed basis, and one method that produces impossibility-strength design claims with calibration in place of a proof — not the discovery of new mechanisms.

---

## 7. Related Work

Every point on the map is somewhere in the literature; the paper's job is to show they are *one* map.

**Verified-systems-Rust and seam toolchains.** RustBelt (POPL 2018), RefinedRust, Creusot, Aeneas, Prusti, Kani, Verus, F\*/Low\*, and Dafny are the "covering across a seam" that instantiates the trilemma's keep-1+2 and keep-1+3 corners — verified Rust *is* Rust plus a separation-logic verifier cooperating across a trusted boundary.

**Separation logic and concurrency.** Iris, iRC11, GPS, and RustBelt are the undecidable concurrent separation logic in which the seam's true precondition lives, and against which §2's reductions are drawn.

**Refinement, capability, and brand types.** Flux and Liquid Haskell (the refined index's legible predicate); `runST` (PLDI'94) and lightweight monadic regions (PLPV'06) (the branded index's viral `'id`, and the proof that its cost exists without any seam); GhostCell (the intrusive list's brand).

**Exception and leak safety.** Abrahams' basic/strong/nothrow guarantees, C++ `noexcept`, and Rust `UnwindSafe`/`catch_unwind`/`Mutex` poisoning (the unwind-safety axis, and RFC #1508, which rejected exactly the `take_mut` primitive on exactly this cost); the Rust "leakpocalypse," leak amplification, `Vec::drain`, and RFC 1066 / the `thread::scoped` removal (the leak-freedom axis).

**Information flow.** Jif and FlowCaml (the IFC floor the affine floor is *not*); Smith's min-entropy quantitative information flow (the *amount* coordinate an affine budget cannot bound); CompCert-CT, Jasmin, and FaCT (constant-time codegen as *dischargeable*, correcting an over-strong "irreducible axiom" reading).

**Liveness and deadlock-freedom.** Boyapati–Lee–Rinard (OOPSLA'02) and Kobayashi's level-based analysis (the type-level rank order); Havender (1968) (ordered acquisition as the soundness argument); FreeBSD WITNESS, Linux lockdep, and Abseil (all of which abandoned static ranks for dynamically-learned acquires-before graphs — foreclosing the registry design); session types (Honda; Kobayashi's linear-π; Dardha–Gay, FoSSaCS 2018; Scribble / mpstthree) and guarded corecursion (Coq/Agda) for the session family.

**Positioning.** What is new is not any mechanism but (a) the calibrated adversarial-model *method* for impossibility claims, (b) the unification of the scattered axes into one closed cost-basis under a single "what does the witness cost?" rubric, and (c) the reflexive builder-calibration finding.

---

## 8. Reproducing

The negative half: `orchestrator.run --all --survivals 3 --prior-art-web` (the sweep), `--meta-synthesize` (the theorem), `control --case good` / `--case bad` (the two calibration controls), and `run --question 0 --proposer-shots 3 --scratch` (the stress test). The constructive half: `orchestrator.seam_phase --spec design_seam/SEAM_SPEC_<name>.md --gauntlets 1 --prior-art-web` per seam; the seam crate builds and tests under `cargo test --manifest-path seam/Cargo.toml` (with `--features lockdep` for the registry suite); the Lean kernel checks from the repository root with `lean proof/SeamGap.lean`. Artifacts: `design/{DESIGN,THEOREM,CALIBRATION}.md` (negative payload), `design_seamful2/THEOREM.md` (the trilemma), `design_seam/{SYNTHESIS,REALIZABILITY,SEAM_WIDTH}.md` and the 37 `SEAM_SPEC_*` / `REPORT_*` pairs (the constructive map), and `proof/SeamGap.lean` with `proof/REVIEW.md` (the skeleton and its caveat). Raw per-call transcripts are under `runs/` and `runs_seam/`.

---

## 9. Conclusion

A few languages, connected by seams, genuinely cover the whole job; the seam that resists full fusion is *doing real work.* The job takes about one deliberate concession, a seam is where you pay it, and the seam is **load-bearing** — so "you cannot combine them completely, and that's fine" is not a limitation you tolerate but the design win, and it is where verified-Rust toolchains already sit.

Stated in full: the Seam Trilemma is a domain-general tension; each soundness domain has its own cost-basis; a floor closes a domain's seams only to the degree it carries that *precondition* — a graded match, full for memory safety, partial-to-none for liveness by precondition, none for information flow — and that grade cannot be freely lifted, because upgrading the floor trades one gap for a finer one and spends the closed cost-basis. The floor can fail a precondition not only by domain but by *structural logic* (affine "at most once" versus linear "exactly once"), and a single composite precondition can be matched *component-wise* — one horn closing while another costs, with which horn a danger threatens set by how the re-import rule packages it, yet that packaging dial itself bounded by whether soundness can be decoupled from the danger. And the method that produced all of this produced one more thing it alone could: a warm builder against a cold adversary is not just a quality-assurance loop but an *epistemics experiment* — the round-count of convergence measures the distance between a danger and what the builder already knew.

---

## Appendix A — The 37-seam verdict table (condensed)

The fully annotated per-seam table — verdict, horn outcome, measured seam width, and precondition-shape reason for all 37 — is `design_seam/SYNTHESIS.md` (the master table). Condensed here to primitive · domain · verdict · one-line reason; `#` is build order.

*Memory safety (the mapped domain).* SPSC #1 HOLDS (fully encapsulable) · MMIO #2 COSTS/TCB-axiom (physical fact, outside the horns) · slab #4 COSTS [#6] (runtime validity check) · branded index COSTS [API-locality] (viral `'id`) · refined index COSTS [solver-freedom] (SMT in TCB) · shared/`Rc` COSTS [bare-metal] (global heap) · type-level index FAILED-probe (sound; eighth-axis claim confounded) · take-and-replace COSTS [unwind-safety] (abort-bomb) · drain-and-yield HOLDS/pays leak-freedom (pre-pessimization) · self-ref cell TRADEOFF (move-stability, inseparable) · validity TRADEOFF (inseparable; basis shown closed) · intrusive list **BREAKS [horn 1]** (whole-heap aliasing+liveness escapes).

*Information flow (floor-closes-nothing).* constant-time secret **BREAKS [horn 1, IFC]** (declassification is etiquette) · capability-gated declassify **BREAKS [horn 1]** (affine counts uses not bits).

*Liveness (partial match; three coordinates).* type-level lock ordering **BREAKS** (granularity + coverage) · runtime lock registry **BREAKS** (granularity recurs finer: thread vs task) · binary/multiparty session, choice, delegation, priority, projected, must-complete, must-scoped, recursion, guarded — the session family mapping progress/completion (mix of COSTS [horn 3] concentrations, TRADEOFF holds, and rejection-mode soundnesses; per-channel priority typing **BREAKS**, the session face of the lock-order inversion).

*Fourth-domain probes (all deflate; three-domain map closed).* CRDT convergence — *folds* onto app-correctness · WCET — *breaks* on cost/cycles · integer division (divisor / total_div) — *folds* (the second a clean HOLD) · data-race (`sync_cell` break → `spin_mutex` broken salvage → `spin_lock` HOLD) — *reclassifies* into memory safety, the two-step settled lineage.

## Appendix B — The realizability axis table

Reproduces `design_seam/REALIZABILITY.md`: the nine live axes (Sound, Decidable, Concentration, API-locality, Zero-cost, AI-legible, Bare-metal, Unwind-safe, Leak-safe), the twelve memory-safety rows with per-cell holds/pays, and the pairwise-dissociation table (zero-cost ⟂ concentration via slab-vs-SPSC; API-locality ⟂ concentration via the branded index; AI-legible ⟂ API-locality via refined-vs-slab; bare-metal ⟂ zero-cost via slab-vs-shared; unwind-safety ⟂ all-seven via take-vs-SPSC; leak-freedom ⟂ unwind-safety via take-vs-drain) that establishes each axis as an independent dimension. SPSC is the only all-holds row — "seamless" is achievable, but only for SPSC-shaped preconditions.

## Appendix C — The Lean skeleton and its honest depth caveat

Reproduces `proof/SeamGap.lean` (the machine-checked trilemma skeleton — the seam-closes-iff-factors equivalence, and per-seam `..._closes` / `..._factors` lemmas) and `proof/REVIEW.md` (why the checked lemma is information-theoretically trivial and the PL substance lives in the unverified modeling dictionary). The mechanization's role is to force the faithfulness claim to be stated and attacked, not to replace the adversarial calibration.
