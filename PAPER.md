# The Seam Is Load-Bearing: An Impossibility Result for Seamless Safe Systems Languages, and a Constructive Map of What You Can Build Instead

**Status: SKELETON / scaffold.** Section headers, per-section thesis, the concrete claims each section must make, and `[SOURCE]` pointers into the three settled documents this paper stitches: `WRITEUP.md` (the impossibility narrative), `design_seam/SYNTHESIS.md` (the constructive cross-cutting findings), `design_seam/REALIZABILITY.md` (the axis table). Prose here is stub-level; the claims and structure are load-bearing. Fill top-to-bottom; do not re-derive — every claim already has a home in a source doc, cited inline.

Abbreviations spelled out on first use: information-flow control (IFC), data-race freedom (DRF), non-lexical lifetimes (NLL), undefined behavior (UB), satisfiability modulo theories (SMT), concurrent separation logic (CSL), trusted computing base (TCB), quantitative information flow (QIF), control-flow graph (CFG).

---

## Abstract  `[stub — write last]`

One paragraph, ~200 words. Must land, in order:
1. **The ask.** Can one language be simultaneously decidable/proof-free, seamless (no proof/systems FFI boundary), and give garbage-collector-free memory safety *derived* from its proof layer, at zero cost, on bare metal? (The seven-property seed.)
2. **The method.** Not a hand proof: an *adversarial process* of cold, context-stripped model instances (proposer / five attackers / judge / synthesizer), which settles a design only after three consecutive survived gauntlets, and which is *calibrated* (settles a known-good design, rejects known-garbage, still fails the subject).
3. **The negative result.** No such language exists: at every ownership-state merge point the entailment the checker must decide is undecidable, and the three "seamless / derived / zero-cost" requirements each delete the one boundary that could relocate that obligation. Existing languages ship by conceding *exactly one* of three properties.
4. **The constructive flip.** Concede seamlessness deliberately → a first-class in-language seam → **the Seam Trilemma** (pick two of: sound re-import / decidable-proof-free-zero-cost floor / seam concentration).
5. **The build-and-break.** 37 real Rust seam primitives built and adversarially attacked. Whether a seam closes losslessly is a *function of the precondition's shape*; the trilemma is a **3-of-N projection of a closed cost-basis (N ≥ 9 axes)**; the result is **domain-parameterized** (memory safety / IFC / liveness), each domain closes only to the degree the *floor matches the precondition* (a **graded** match), and a reflexive finding: the cold gauntlet **measures the warm builder's miscalibration**.
6. **Honest bound.** "Proven by survived adversarial challenge," calibrated and machine-checked *in skeleton* (Lean 4), but not a mechanized end-to-end proof.

---

## 1. Introduction  `[SOURCE: WRITEUP §1, §6]`

**Thesis:** the seductive goal — one seamless language that *derives* memory safety and erases it at zero cost — is not merely unbuilt, it is provably unbuildable, and knowing *why* tells you exactly what to build instead.

- State the seven properties verbatim `[WRITEUP §1]`: executable spec, unified proof/systems layer, derived GC-free ownership, decidable checking, bare-metal, zero-cost erasure, AI-legible semantics.
- State the seed's *actual* success criterion: a **precise map of which combinations are incompatible, proven by survived adversarial challenge, not asserted by authority** `[WRITEUP §1]`.
- The covering-vs-intersection framing `[WRITEUP §6 "Covering, not intersection"]`: existing languages *together* cover all seven; no single point holds all seven; the substantive question is *why the covering can't be collapsed*, and the answer is that the collapse **is** property #2 (seamlessness) — the forbidden move.
- **Contributions** (the list the rest of the paper delivers):
  - **C1.** A single impossibility theorem unifying six independently-discovered incompatibilities `[§4]`.
  - **C2.** A *calibrated* adversarial-model methodology for design-space impossibility claims `[§2, §5]`.
  - **C3.** The **Seam Trilemma** — the structural cost of conceding seamlessness `[§5-trilemma]`.
  - **C4.** A **build-and-break** empirical map of 37 seams → the trilemma is a 3-of-N projection of a **closed** cost-basis; domain-parameterized; floor-match graded `[§6]`.
  - **C5.** A **reflexive** methodological finding: a cold stateless adversary measures a warm stateful builder's calibration `[§6.5]`.

---

## 2. The Impossibility Result  `[SOURCE: WRITEUP §3, §4]`

**Thesis:** five questions, 24 gauntlet rounds, **nothing settled**; the six recorded incompatibilities are facets of *one* theorem, and #4 (decidability) is the universal antagonist.

- The result table `[WRITEUP §3]`: 0/5 settled; six HARD incompatibilities; **all six place decidability (#4) on one side of the collision**. Reproduce the six-row table.
- The varied-but-convergent failure reasons `[§3]`: admissible-contraction double-free; non-associative `meet` laundering into `Moved`; poison past a location-indexed seam; implicit-flow secret laundering; independent-reads-independent-writes (IRIW) certifying a consensus POWER allows to fail.
- **The theorem** `[§4]`, stated both plainly and formally. Plain: a checker can be *automatic* or *strong enough for real aliased weak-memory code*, not both. Formal: no type system is both (a) decidable/proof-free in the common case and (b) the sole auto-checked discharge site for the substructural, heap-quantified, value/view-dependent invariants that GC-free ownership over aliased weak-memory state requires — because at every merge point (CFG join, loop fixpoint, erasure-time equality, unsafe/atomic/DMA seam) the entailment is undecidable.
- The reduction skeleton `[§4]`: substructural (no contraction) + heap-address-quantified (∀-footprints) + value/view-dependent ⟹ entailment subsumes bi-abduction / Hilbert's 10th at the join, the Bradley–Manna–Sipma array-property fragment on footprints, open-term definitional equality at erasure. The decidable zero-annotation fragment is *necessarily the dual*: finite-height, quantifier-free, first-order, contraction-admitting — provably too weak to name the invariant.
- **#4 is the universal antagonist; #2/#3/#6 are three forcing functions** `[§4]` — three ways to delete the boundary that could relocate the obligation.

---

## 3. Method: The Cold Adversarial Gauntlet  `[SOURCE: WRITEUP §2, §5]`

**Thesis:** the load-bearing trick is *coldness* — each role is a fresh context-stripped model instance — and the instrument is *calibrated* three ways, so a 0/N result measures the subject, not the process.

- **The roles** `[§2]`: PROPOSER (one concrete falsifiable decision) → five cold ATTACKERs (ergonomics / soundness / implementation / expressiveness / prior-art) → JUDGE (genuine-flaw / argued-past / tradeoff → SURVIVED / FAILED / TRADEOFF) → SYNTHESIZER (distills convergent failure into a recorded incompatibility) → RECONCILER → HISTORIAN (deterministic state).
- **Why coldness is load-bearing** `[§2]`: fresh `claude -p` subprocess, replaced system prompt, no settings/hooks/MCP, clean temp cwd; verified cold (an attacker asked about unrelated operator context answers "no"). An attacker that secretly shares the proposer's framing is not an adversary.
- **Faithful termination** `[§2]`: three *consecutive* survived cold gauntlets, fresh attackers each; any FAILED gauntlet feeds landed attacks back and resets the streak.
- **Calibration — the trust argument** `[§5]`, the section that turns "0/5" from ambiguous to diagnostic:
  - **True negative:** Rust NLL with #2/#3/#6 waived → **SETTLED**, and honestly (rev0 FAILED on a real `dropck`/`#[may_dangle]` use-after-free the process *caught*, rev1 survived three).
  - **True-positive rejection:** an obviously-unsound non-design → **FAILED** all revisions (soundness attacker refutes `check=⊤ ⟹ ∀e.safe(e)` with a well-typed use-after-free).
  - **Stronger-proposer stress test:** a 3-shot self-hardening proposer on the most-precedented question **still FAILED** — a 1-shot proposer settled Rust, a 3-shot could not settle the constrained core ⟹ **the constraints, not proposer weakness, are decisive.**
  - **Net:** settles the good, rejects the bad, fails the subject → measuring the *subject*.

---

## 4. The Constructive Flip: The Seam Trilemma  `[SOURCE: WRITEUP §9]`

**Thesis:** concede seamlessness (#2) deliberately — make the proof/systems boundary a first-class in-language seam — and the residual obstruction sharpens into a clean trilemma; a *decontamination* step is what makes the result structural rather than a proposer artifact.

- The natural concession `[§9]`: keep one language, make the boundary a first-class machine-checked seam (Rust+`unsafe` informally; Verus/F\*/Low\*/Dafny/RefinedRust formally). The question is *which calibration and what's its limit*.
- **Two runs, because the first was contaminated** `[§9]` — the methodological centerpiece:
  - `seamful` (concede #2, keep #3): 0/6, but *contaminated* — keeping #3 forced the proposer to *derive a borrow checker from scratch* and it shipped almost-right rules with dropped premises.
  - `seamful2` (concede #2 **and** #3): safe fragment *stipulated* as shipped Rust NLL → the whole gauntlet bears on the seam → still 0/6, now **clean**: obstruction isolated onto the seam's re-import boundary ("sound re-import needs a linearity witness NLL's region-only vocabulary cannot carry"). **Removing #3 didn't rescue the seam — it isolated the obstruction.**
- **The Seam Trilemma** `[§9]`, stated formally — pick at most two of:
  1. **Sound re-import** (crossing back launders no UB into safe code).
  2. **Decidable, proof-free, zero-cost safe fragment.**
  3. **Seam concentration** (ordinary code never crosses the seam).
  - The three drops, each a shipping point: keep 1+2 → drop concentration (verified-Rust reality); keep 1+3 → drop decidability (Verus/F\*); keep 2+3 → drop soundness (the detectable bad corner).
- The three-program map table `[§9]`: `unified` (concedes nothing, 0/5) / `seamful` (#2, contaminated) / `seamful2` (#2+#3, the clean trilemma).
- **Decontamination as method** `[§9 methodological note]`: hand the proposer the known-good sub-component and watch the failure *move, not disappear* — the general recipe for ruling out "the proposer was just weak."

---

## 5. Build and Break: 37 Seams  `[SOURCE: SYNTHESIS + REALIZABILITY]`

**Thesis:** the trilemma is a *map*, so stop arguing and build — 37 real Rust primitives, each re-exporting a dangerous core behind a checked-safe interface, each run through the same cold gauntlet. The findings below are what survived.

### 5.1 Setup: the build-and-break harness  `[SOURCE: WRITEUP §10]`
- Harness shape: proposer *builds* an artifact (rustc-compiling Rust + Lean instance + frozen spec), the cold panel attacks the *artifact*; verdicts SEAM-HOLDS / SEAM-COSTS [horn] / SEAM-BREAKS.
- The floor is fixed = shipped Rust NLL + affinity + decidable trait bounds. Every seam asks: *can every caller-observable conjunct of the primitive's true precondition be carried by the floor's witness vocabulary — and if not, is the missing conjunct an external axiom, a runtime-checkable fact, or genuinely unrepairable?*

### 5.2 The headline: closure is a function of precondition shape  `[SOURCE: SYNTHESIS "headline result" + master table]`
**Thesis:** whether a seam closes is **not** a property of "seams" — it is a function of the precondition's shape; the trilemma bites *selectively* and predictably.
- The canonical contrast: **SPSC ring buffer** SEAM-HOLDS (every escaping conjunct floor-expressible; happens-before leaves no caller-visible residue) vs **intrusive list** SEAM-BREAKS [horn 1] (whole-heap aliasing + liveness escapes to the caller; the brand carries aliasing but not membership-vs-storage lifetime).
- Reproduce (or excerpt) the master per-seam verdict table `[SYNTHESIS lines 153–180]` — all 37, with horn outcome, measured seam width, and precondition-shape reason.

### 5.3 The closed cost-basis: the trilemma is a 3-of-N projection (N ≥ 9)  `[SOURCE: SYNTHESIS "the deeper result" + REALIZABILITY]`
**Thesis:** the three horns are a low-dimensional shadow of a **closed object** — nine clean axes, an inseparable category, two outside-the-axes — recovered one primitive at a time, and *shown closed* by a new danger class projecting cleanly onto it.
- The un-bundling chain, each primitive the *witness* that two previously-identical configurations differ: slab → zero-cost (#6); branded index → concentration splits into code-face (seam-width) vs type-face (API-locality), + AI-legibility (#7); refined index → horn 2 splits into decidability vs solver-freedom; shared/`Rc` → bare-metal (#5); type-level index → **rejected** (confounded — the discipline can refuse); take-and-replace → unwind-safety (N≥8); drain-and-yield → leak-freedom (N≥9); self-ref cell → move-stability = **real-but-inseparable** (a fourth verdict category); validity → **no tenth axis, basis CLOSED** (a new danger class projects onto the existing basis).
- The two-directional closure argument `[SYNTHESIS "the dual"]`: `get_unchecked` family = one danger, many witnesses spending different axes; validity = many dangers converging on one axis ⟹ the cost-space is a **small fixed basis** dangers *project onto*.
- Honest coda `[SYNTHESIS]`: each axis is a *pre-existing* named PL boundary (runST, RFC 2480, Abrahams exception-safety, the leakpocalypse, `Pin`, `num_enum`); the contribution is their **unification under one "what does the witness cost?" rubric** — a question with finitely many answers — not their discovery.

### 5.4 Domain-parameterization and the graded floor-match  `[SOURCE: SYNTHESIS "closed per domain" … "graded"]`
**Thesis:** the closed object is the *memory-safety* domain's; the same shape recurs with a different basis in every soundness domain, and *which* seams close depends on how well the floor matches the domain's precondition — a **graded** match.
- **Three soundness domains, closed per domain:** memory safety (mapped, matched floor) / information flow (every seam breaks on Rust's affine floor) / liveness (partial match).
- IFC probes: constant-time secret SEAM-BREAKS [horn 1, IFC sense] (declassification is etiquette, not type-structure); capability-gated declassify SEAM-BREAKS unanimously (affine budget counts *uses* not *bits*; the IFC coordinate is quantitative — Smith min-entropy — an affine floor can't bound). **Finding: a closing seam exists only when the FLOOR matches the DOMAIN.**
- Liveness probes: type-level lock ordering SEAM-BREAKS two ways (granularity: per-token vs per-thread-global; coverage: static-vs-dynamic acyclicity) ⟹ the match is **graded** (full / partial / none); runtime lock registry shows the grade is **non-monotonically liftable** (upgrading the floor fixes coverage but granularity recurs one rung finer — the containment-scope **ladder** token < thread < task, no ambient floor tops out).

### 5.5 The second movement: composite preconditions, the fourth-domain probes, and the dichotomy  `[SOURCE: SYNTHESIS "the second movement"]`
**Thesis:** the remaining twenty seams close the account **without adding a horn, an axis, or a domain** — the map is finite and now full.
- **Liveness has exactly three coordinates** — completion, leak-freedom, progress — each on a lock face *and* a session face; completion turns out *concentrable* (a scoped continuation-passing combinator, per-endpoint); only *termination* is left as an undecidable NONE.
- **Four fourth-domain probes, four deflations:** convergence (CRDT) *folds* onto application-correctness; worst-case execution time *breaks* on the cost/cycles gap; totality (division) *folds*; data-race freedom *reclassifies* into memory safety ⟹ **domain count = three, closed** — plus an application-correctness *basis* for semantic-law dangers.
- **Trichotomy → dichotomy** `[the sharpest structural claim]`: IFC is the lone *floor-closes-nothing* domain; memory-aliasing **and** data-race freedom are the *same* floor-propagates-but-concentrating shape, with a **static** end (SPSC, the slab — compiler-verified) and a **dynamic** end (`SpinLock`, `RefCell` — runtime witness); an `unsafe impl` is a proof obligation either **discharged** or **false** (RustBelt semantic type soundness, floor-relativized).
- **The adjudication vocabulary closed at nine verdict modes over a ten-seam broken tally.**
- The composite-precondition finding `[SYNTHESIS one-sentence upgrade]`: a floor matches a *composite* precondition **component-wise** (session-typed channel: protocol order closes via affine typestate, completion is a horn-3 concentration cost); *which horn a danger threatens is set by how the re-import rule packages it* — but that packaging dial is itself **bounded** by whether soundness can be decoupled from the danger (owned ⇒ danger costs; borrowed-across-unwind / address-pinned / borrow-scoped ⇒ forces a fixed horn).

### 5.6 The reflexive finding: coldness as a calibration instrument  `[SOURCE: SYNTHESIS "second movement" reflexive coda]`
**Thesis:** the discovery only *this* method could produce — a warm stateful builder red-teamed by a cold stateless gauntlet yields a *measurement of the builder's own blind spots*.
- The measured tic: **flee the decidable middle toward the cleaner extreme** (total concentration / irreducible-global / undecidable). Caught **eight times** as sound-but-rejected §6 deliverables, each refuted from the builder's *own* citations.
- The arc is **non-monotonic**: horn-deciding early (#8–#31); receding to a marked-tradeoff-below-threshold as the builder learned to pre-concede the uncarriable half (#32, #34); then re-firing at **full strength** on the fresh concurrency domain (#35, #36) before receding (#37) — on novel ground the tic fires *repeatedly until the gauntlet exhausts the obligation set, then recedes*.
- **Five settled salvage lineages** (four one-step: #26→#27, #29→#30, #31→#32, #33→#34; one **two-step**: #35→#36→#37, two steps because the salvage itself first broke) — the constructive dual of the impossibility result: you cannot build a *seamless* language, but against a cold adversary a single *seam* can be **converged** to soundness, and the round-count of that convergence **measures how far the danger sat from what the builder already knew.**
- Worked instance for the paper `[SEAM_SPEC_spin_lock, FACTS_37]`: the four auto-trait obligations of a mutex; #36 discharged three and dropped the guard's `T: Sync`; #37 enumerated and discharged all four (the fix is *verbatim* Ralf Jung's June-2017 std `MutexGuard`-`Sync` bug + patch; RustBelt POPL 2018) — novelty ~nil, but the *process* finding (the tic firing inside the very salvage meant to prove it fixed) is the contribution.

---

## 6. Threats to Validity  `[SOURCE: WRITEUP §7 + SYNTHESIS honest bounds + proof/REVIEW.md]`

**Thesis:** state the bounds louder than the claims; "survived adversarial challenge" is a calibrated empirical result, not a mechanized proof.

- **Not a mechanized proof.** The undecidability reductions (Hilbert's 10th, array-property fragment) are real and load-bearing, but the synthesis assembling them is not machine-checked.
- **Partial exception (and its own caveat):** the trilemma's *logical skeleton* is machine-checked in Lean 4 (`proof/SeamGap.lean`, axiom-clean modulo `propext`) — **but** the checked lemma is information-theoretically trivial ("a one-field projection can't recover a two-field predicate"); all PL substance lives in the unverified prose-level **modeling dictionary** (`V` = what NLL sees, `P` = the soundness precondition). The mechanization checks the skeleton and *forces the faithfulness claim to be stated and attacked*; it does not lift the substance out of "survived adversarial challenge" `[WRITEUP §7 + proof/REVIEW.md]`.
- **Bounded proposer depth** (3 revisions; up to 3 self-strengthening shots) — the stress test pushes against this but does not eliminate it; a human PL theorist might find a path, though fighting the same reductions.
- **Shared model weights ⟹ shared blind spots** — the prior-art attacker used live web search to partially counter; full mitigation needs human-injected attack angles (a seed-specified hook not fully exercised). *Note the tension with §5.6:* the reflexive finding is itself produced by the warm/cold asymmetry, so its strength is bounded by how truly cold the cold instances are.
- **Novelty is deflationary throughout** — nearly every axis and every held seam maps to a pre-existing named result; the contribution is *unification and the map*, not new mechanisms. Say so plainly.

---

## 7. Related Work  `[assemble from citations scattered across all three docs]`

**Thesis:** every point on the map is somewhere in the literature; the paper's job is to show they are *one map*.

- **Verified-systems-Rust / seam toolchains:** RustBelt (POPL 2018), RefinedRust, Creusot, Aeneas, Prusti, Kani, Verus, F\*/Low\*, Dafny — the "covering across a seam" that *is* the trilemma's keep-1+2 and keep-1+3 corners.
- **Separation logic / concurrency:** Iris, iRC11, GPS, RustBelt — the undecidable CSL the seam's true precondition lives in.
- **Undecidability reductions:** Hilbert's 10th / bi-abduction; Bradley–Manna–Sipma array-property fragment; open-term definitional equality.
- **Refinement / capability / brand types:** Flux, Liquid Haskell; `runST` (PLDI'94), lightweight monadic regions (PLPV'06); GhostCell.
- **Exception / leak safety:** Abrahams basic/strong/nothrow guarantees; C++ `noexcept`; Rust `UnwindSafe`; the leakpocalypse / RFC 1066 / RFC #1508.
- **IFC / QIF:** Jif, FlowCaml; Smith min-entropy quantitative information flow; constant-time codegen (CompCert-CT, Jasmin, FaCT).
- **Liveness / deadlock-freedom:** Boyapati–Lee–Rinard (OOPSLA'02); Kobayashi level-based deadlock analysis; Havender (1968); FreeBSD WITNESS / Linux lockdep / Abseil; session types (Honda; Kobayashi linear-π; Dardha–Gay FoSSaCS 2018; Scribble/mpstthree); guarded corecursion (Coq/Agda).
- **The `MutexGuard`-`Sync` bug:** Ralf Jung (June 2017) + shipped patch — the #37 worked example.
- **Positioning:** what is *new* is not any mechanism but (a) the calibrated adversarial-model *method* for impossibility claims, (b) the unification of the scattered axes into one closed cost-basis under a single rubric, and (c) the reflexive builder-calibration finding.

---

## 8. Reproducing  `[SOURCE: WRITEUP §8, §10]`

- The negative-half sweep + meta-synthesis + the two calibration controls + the stress test (commands from `[WRITEUP §8]`).
- The constructive half: `python3 -m orchestrator.seam_phase --spec design_seam/SEAM_SPEC_<name>.md --gauntlets 1 --prior-art-web`; the seam crate (`cargo test --manifest-path seam/Cargo.toml`, `--features lockdep`); the Lean kernel (`lean proof/SeamGap.lean` from repo root).
- Artifacts index: `design/{DESIGN,THEOREM,CALIBRATION}.md`; `design_seamful2/THEOREM.md` (the trilemma); `design_seam/{SYNTHESIS,REALIZABILITY,SEAM_WIDTH}.md` + 37 `SEAM_SPEC_*` and `REPORT_*`; `proof/SeamGap.lean`.

---

## 9. Conclusion  `[SOURCE: SYNTHESIS "one-sentence upgrade" + closing coda]`

**Thesis:** the seam that resists full fusion is *doing real work* — deleting it (the dream of one unified language) is exactly what's impossible; and the constructive dual shows precisely which seams you can build losslessly and what every other one costs.

- Restate the load-bearing-seam framing `[WRITEUP §6]`: the job takes ~one deliberate concession, the seam is where you pay it, "you can't combine them completely, and that's fine" is the **design win**, not a limitation.
- The final compressed shape `[SYNTHESIS one-sentence upgrade]`: domain-general tension; per-domain cost-basis; a floor closes a domain's seams only to the degree it carries that precondition (graded — full memory-safety / partial-to-none liveness / none IFC); the grade cannot be freely lifted; the floor can fail by *structural logic* (affine vs linear) not only by domain; a composite precondition matches component-wise; and "which horn" is a design choice *within* a danger-intrinsic fence, not free.
- Close on the reflexive note: a warm builder against a cold adversary is not just a QA loop but an *epistemics experiment* — the round-count of convergence measures the distance between a danger and what the builder already knew.

---

## Appendix A — The 37-seam table (full)  `[SOURCE: SYNTHESIS lines 153–180, verbatim]`
## Appendix B — The realizability axis table  `[SOURCE: REALIZABILITY "The table" + "What the table proves"]`
## Appendix C — The Lean skeleton and its honest depth caveat  `[SOURCE: proof/SeamGap.lean + proof/REVIEW.md]`

---

### Drafting notes (delete before submission)
- **Venue shaping:** as a PL/systems paper the natural home is the impossibility theorem (§2) + method (§3) + trilemma (§4); the build-and-break (§5) is the empirical validation. As a *methods* paper the center of gravity shifts to §3 + §5.6 (calibrated adversarial models; coldness as a measuring instrument). Pick one framing before writing prose — the abstract's emphasis follows.
- **Do not overclaim.** Every "proven" in §2/§4 is "proven by survived adversarial challenge"; keep §6's caveats visible in the body, not quarantined in Threats.
- **The table is the paper.** The 37-seam verdict table (App. A) and the axis table (App. B) carry the empirical weight; the prose narrates, it does not re-derive.
- **Length budget:** the interesting-problem word budget goes to §4 (the theorem is one), §5.3 (the closed basis), and §5.6 (the reflexive finding) — the three claims a reader remembers.
