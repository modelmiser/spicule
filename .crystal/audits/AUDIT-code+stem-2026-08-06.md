# Crystal Audit: spicule

**Date**: 2026-08-06
**Domain**: code + stem (mixed artifact — Rust/Python implementation + impossibility-argument paper + Lean skeleton)
**Method**: 8 parallel lenses. Code: data flow / behavioral contracts / duplication / **forcing mechanisms**.
STEM: principles+prior-art / regime boundaries / 10 assumption categories / **conservation + software-artifact addendum**.
**Prior state**: no existing crystal. Known-and-excluded: the monoculture ceiling, the
10→11 broken-seam recount (fixed 2026-07-01), the Tier-3(b) verdict-stability re-run.

---

## Summary

The counting discipline is genuinely good — 37 closes at every arithmetic interface, the
11-break census is corroborated, and the self-caught 10→11 correction is real and
propagated. **Every defect found is at an interface**, and three of them were confirmed
by execution rather than by reading.

The leading finding is a polarity inversion: `proof/SeamGap.lean` machine-checks that
**ten seams the paper's census records as BREAKS satisfy the *closing* proposition**,
with proof terms byte-identical to the HOLDs. The paper discloses that the per-seam
lemmas are *trivial*; it does not disclose that they are *inverted*.

**Candidates**: 12
**Origin**: 2 observed · 5 prospected · 5 both
**Enforcement**: 0 type-enforced · 1 runtime-checked · 3 convention · **8 unguarded**
**Confirmed by execution**: 3 (two compile-tests, one Lean typecheck + census)

---

## Candidates

### 1. LeanVerdictPolarity (CRITICAL — contradiction)

**Implicit contract**: a per-seam mechanized lemma should distinguish a seam that closes
from a seam that breaks.

**Origin**: both · **Enforcement**: unguarded · **Value**: critical

**Evidence** (all re-verified directly, not taken from the lens):
- `proof/SeamGap.lean` — `grep -c ':= V p$'` → **34**. Every per-seam precondition is
  definitionally `V`.
- `SeamGap.lean:161-166` `Pspsc := V p`; `spsc_seam_closes` (paper: **HOLDS**)
- `SeamGap.lean:1211-1222` `Psynccell := V p`; `sync_cell_datarace_factors` (paper:
  **BREAKS [horn1]**)
- Both have the identical statement `∃ d, SoundW P_ d ∧ UsableW P_ d ∧ ConcW V d` and the
  identical proof `rw [seam_closes_iff_factors]; exact ⟨id, fun _ => rfl⟩`.
- Ten of the eleven census BREAKS carry such a theorem: reseat `:659`, secret `:428`,
  labeled `:456`, ordered_locks `:485`, lock_registry `:513`, linear `:549`, priority
  `:892`, wcet `:729`, sync_cell `:1219`, spin_mutex `:1251`.
- `intrusive_seam_does_not_close` (`:172`) is the **only** negative instantiation, and it
  uses the generic two-field `P`, not a seam-specific `Pintrusive`. So
  `seam_closes_iff_factors` — advertised in §4 as "the dividing line" — is never
  instantiated on the negative side for any named seam.
- `lean SeamGap.lean` → **exit 0**. Zero `sorry` / `admit` / `axiom` / `native_decide`.
  The file is sound; the problem is what the theorems *say*.

**What currently happens instead**: the hold/break distinction is carried by identifier
spelling (`_closes` vs `_factors`) and by docstrings. `SeamGap.lean:1213-1218` states it
outright — *"the real guarantee … is FALSE in safe code … lives in the prose, invisible
to the kernel."* A checker recovers no break.

**Disclosure gap**: §6 and Appendix C disclose the *triviality* ("instantiate the
precondition as the witness … record the modeling claim"). Neither states that
BREAK-recorded seams carry theorems of the identical *closing* form. §6 names only
`_closes` and never mentions `_factors`.

**Formalization path**: give BREAK seams a genuinely different statement — instantiate
the negative side of `seam_closes_iff_factors` (as `intrusive` does) with a seam-specific
`P` that is *not* `:= V p`. Until then, delete the 34 trivial lemmas: they add 1150 lines
of apparent mechanization whose content is one instance of one lemma.

---

### 2. GuardSyncBound (CRITICAL — live bug in shipped code)

**Implicit contract**: a guard vending `&T` via `Deref` may be shared across threads only
if `T: Sync`. The lock excludes guard *creation*, not guard *sharing* — the crate
enumerates this obligation set itself at `seam/src/spin_lock.rs:20-27`.

**Origin**: both · **Enforcement**: unguarded · **Value**: critical

**Evidence**:
- `seam/src/lock_registry.rs:174-176` — `pub struct Guard<'a,T> { lock: &'a OrderedMutex<T> }`,
  no manual `Sync` impl anywhere in the file.
- `seam/src/lock_registry.rs:140` — `unsafe impl<T: Send> Sync for OrderedMutex<T> {}`
- `seam/src/lock_registry.rs:194-200` — `Deref` vends `&T` out of the `UnsafeCell`.
- **CONFIRMED BY COMPILATION**: an external crate asserting
  `assert_sync::<Guard<'static, Cell<i32>>>()` compiles clean against
  `seam-spsc --features lockdep`. `Cell<i32>` is `Send` but `!Sync`. Two threads sharing
  `&Guard` both `deref()` → two `&Cell<i32>` → racing non-atomic writes from a
  `#![forbid(unsafe_code)]` consumer.
- The fix exists verbatim one module over: `seam/src/spin_lock.rs:203`
  `unsafe impl<T: Sync> Sync for SpinLockGuard<'_,T> {}`, with a static-assert harness at
  `seam/tests/spin_lock_use.rs:59-68`. Neither was applied here.
- `seam/src/ordered_locks.rs:224` carries the same-shaped `unsafe impl` but is **not**
  vulnerable — its API is a closure form (`:244 with(...)`) with no guard struct. The
  RAII-guard shape is what introduces the hole, and `lock_registry` is the one
  guard-shaped lock with no guard impl.

**Why this is worse than an ordinary bug**: this *is* the paper's §5.5 signature finding
— "the builder discharged three and dropped the guard's `T: Sync`, the omission recurred
inside the very salvage meant to prove it fixed, and a third attempt enumerated all four
and held." That third attempt held **only for the spin_lock lineage**. The same omitted
conjunct is live in `lock_registry`.

**Census consequence**: `lock_registry` is filed in Appendix A under *Liveness*,
attributed by §5.3 to granularity (thread-vs-task). It is additionally a horn-1
**memory-safety** break. The 11-seam census counts it once, in the wrong domain, for the
wrong cause.

**Formalization path**: add the bound + a compile-fail/static-assert harness mirroring
`spin_lock_use.rs:59-68`. Then re-adjudicate its horn classification.

---

### 3. SpscEndpointSync (CRITICAL — contradiction)

**Implicit contract**: the prose states a property of the artifact; the artifact has it.

**Origin**: prospected · **Enforcement**: unguarded · **Value**: critical

**Evidence**:
- `seam/src/lib.rs:790` — *"The unique producer endpoint. Affine (`!Clone`, `!Copy`);
  `Send` iff `T: Send`; **`!Sync`**."* Same at `:797` for `Consumer`.
- `seam/src/lib.rs:804-806` — *"We do NOT implement `Sync`: a shared `&Producer` must
  never reach a second thread, or two producers could race `tail` (would break P4)."*
- `seam/src/lib.rs:738` — `unsafe impl<T: Send, const N: usize> Sync for RingBuffer<T,N> {}`
- **CONFIRMED BY COMPILATION**: `assert_sync::<Producer<'static,u32,4>>()` and
  `assert_sync::<Consumer<'static,u32,4>>()` both compile. `Sync` is an auto trait —
  declining to write an impl does not withhold it. `Producer` holds
  `&'a RingBuffer<T,N>`, which is `Sync` whenever `RingBuffer` is.

**Honest scoping**: not currently exploitable — `&Producer` exposes no methods, and real
safety comes from `push`/`pop` taking `&mut self` (`lib.rs:812`, `:835`), a *different*
mechanism than the one documented. It becomes exploitable the moment any `&self` method
is added.

**Why it matters anyway**: this is on SPSC — the §5.1 pole, the only all-holds row in
Appendix B, the 3-streak flagship, and the first entry of the foreign-replication sample.
The crate has 64 `compile_fail` doctests and a static `assert_sync` harness; neither pins
the auto-trait shape of its flagship type.

**Formalization path**: `PhantomData<*const ()>` (or a static assert that the type is
`!Sync`) on both endpoints, plus a `compile_fail` doctest.

---

### 4. AdjudicatorReconciliation (CRITICAL)

**Implicit contract**: when two cold adjudicators disagree, a stated rule decides which
verdict is recorded.

**Origin**: prospected · **Enforcement**: unguarded · **Value**: critical

**Evidence** (recomputed directly from `design_seam/state.json` + the 39 classifier lines
in `design_seam/REPORT_SEAM_SPEC_*.md`; rounds aligned to seams via the question text):

| judge outcome | classifier BREAKS | COSTS | HOLDS |
|---|---|---|---|
| FAILED | 11 | **10** | 0 |
| TRADEOFF | **1** | 10 | 2 |
| SURVIVED | 0 | 2 | 3 |

- **11 of 39 rounds (28%) disagree on break/no-break. All 11 are recorded on the
  non-break side.**
- 10 are judge=FAILED / classifier=SEAM-COSTS: `slab` r7, `typed_nat` r12, `validity` r16,
  `delegation` r31, `multiparty` r32, `projected` r34, `choice` r35, `must_complete` r36,
  `recursion` r38, `divisor` r40. Spot-checked against Appendix A: `divisor` is recorded
  *"folds (COSTS [horn 3]; the eighth rejection-mode firing)"* — a non-break.
- 1 is the reverse (`crdt` r28, classifier=BREAKS / judge=TRADEOFF) and is the **only one
  the paper discloses** (`PAPER.md:167`), resolved toward the judge — again non-break.

**The asymmetry is the finding.** When the classifier is more lenient than the judge, the
classifier is recorded; in the single case where the classifier is harsher, the judge is.
The operative rule is not "the classifier decides" — it is "the more lenient of the two
decides." That rule is nowhere stated.

**Compounding**: the horn-classifier role — which produces *every* Appendix A verdict
(`orchestrator/seam_phase.py:39`, `orchestrator/roles.py:366`) — appears **twice** in
PAPER.md (`:167`, `:251`) and is **never introduced in §3 Methods**, which names only
proposer / attackers / judge / synthesizer / reconciler / historian. A §3 reader
reasonably believes Appendix A records judge verdicts.

**What currently happens instead**: nothing. The reconciliation is a warm, unadjudicated
coding step applied to cold output — structurally the same failure mode as the 10→11
miscount the paper discloses, one level up.

**Formalization path**: state the rule; report the disagreement rate (it is free — the
data is already in the repo); mark in Appendix A which verdicts came from judge-FAILED
gauntlets. §6 currently offers an n=3 same-family stability re-run as the robustness
estimate while a 28% inter-adjudicator disagreement sits unreported.

---

### 5. ClassifierVocabularyClosure (CRITICAL)

**Implicit contract**: the instrument's answer space is wide enough to contain the
finding being claimed.

**Origin**: prospected · **Enforcement**: unguarded · **Value**: critical

**Evidence**:
- `orchestrator/roles.py:396-399` — *"'SEAM-BREAKS' = horn 1 dropped (a real UB path
  exists). 'SEAM-COSTS' = sound and decidable but a real, marked price…"*
- The classifier therefore **cannot emit** `SEAM-BREAKS [horn2]` or `[horn3]`.
- §5 and Appendix A present "all eleven breaks are horn-1 breaks" as a structural finding
  about precondition shape. It is forced by the prompt.
- Internal inconsistency from the same closure: two seams carry
  `SEAM-COSTS [horn2-decidability]` while the prompt defines `SEAM-COSTS` as "sound **and
  decidable**"; `refined` carries it *with* `stayed_decidable: False`.
- Related: `roles.py:387-395` closes the output over
  `{HOLDS, BREAKS, COSTS} × {horn1, horn2, horn3, none, other}` — so §5.2's "no tenth
  axis" and §5.4's "no fourth domain" are findings the instrument could not have
  contradicted.

**Formalization path**: allow `SEAM-BREAKS [hornN]` for all N, or state explicitly that
horn-1-only breaking is a definitional property of the vocabulary rather than a result.

---

### 6. SeamRecordProvenance (CRITICAL)

**Implicit contract**: the per-seam verdict record is one fact, consistent everywhere it
appears.

**Origin**: both · **Enforcement**: unguarded · **Value**: critical

**Evidence** — the same record exists in **six** unlinked representations with no single
source of truth and nothing detecting divergence:
1. module doc, `seam/src/<name>.rs:1` (ordinal + VERDICT line, two spellings)
2. `seam/src/lib.rs` doc above each `pub mod` (a third spelling)
3. `seam/tests/<name>_use.rs` header (a fourth)
4. `proof/SeamGap.lean` theorem name (`_closes` vs `_factors`)
5. `design_seam/REPORT_SEAM_SPEC_*.md`, regex-scraped at
   `orchestrator/foreign_replication.py:149`
6. `PAPER.md` Appendix A

**Live divergences already present**: Appendix A files `mmio` as "outside the horns" while
its report says `SEAM-COSTS [horn3-concentration]` (same for `selfref`, `validity`);
`SEAM-COSTS [none]` appears in 3 reports, violating §5's own definition that the bracket
names the cost; `SeamGap.lean:~1283` says spin_lock is "HYPOTHESIS SEAM-COSTS [horn3]"
while Appendix A says `SEAM-HOLDS [none]`; §6 says "twenty seams" where the Lean has 36;
`mmio` has no Lean lemma at all; 6 of 11 breaks in Appendix A carry a bare **BREAKS** with
no horn, in the appendix that claims the census is "checkable from this appendix alone."

**Why this is the finding and not the off-by-one**: the 10→11 recount fixed an
*instance*. This is the *mechanism* — and the DEVLOG names it exactly:
*"same-provenance verification cannot catch source errors."* Six copies, one editor, no
reconciliation.

**Formalization path**: a `SEAMS` manifest (ordinal, name, verdict, horn, round, Lean
theorem, CF count) generated-from or checked-against all six sites in CI. That also makes
the "17 modules have no `compile_fail` block" and "27 seams have no Lean back-pointer"
gaps mechanically detectable.

---

### 7. NonLlmOracle (HIGH)

**Implicit contract**: a claim of the form "safe code cannot reach UB" is checked by an
instrument that can observe UB.

**Origin**: prospected · **Enforcement**: unguarded · **Value**: high

**Evidence**:
- `grep -rn 'miri\|loom'` over `seam/` (`.rs`, `.toml`, `.yml`) → **zero hits**.
- 64 `compile_fail` doctests across 35 sources verify **rejection** — that the type system
  refuses bad code. Horn 1 is a claim about **accepted** code. Every mechanical check in
  the repo runs in the direction that is not load-bearing.
- `seam/src/spin_lock.rs` uses `compare_exchange_weak(Acquire, Relaxed)` / `store(Release)`.
  Its `SEAM-HOLDS [none]` — the pivotal DRF-reclassification result and one of four
  three-streak seams — rests on an LLM panel plus single-threaded `cargo test`, which
  exercises approximately none of the memory-ordering claim.

**Why it is independent of the monoculture ceiling**: a foreign-model replication is still
an LLM panel. Miri (`-Zmiri-tree-borrows`, `-Zmiri-preemption-rate`) and loom would be the
**only non-LLM oracle** in the system. The crate is `no_std` and small; this is cheap.

---

### 8. ForeignControlBias (HIGH)

**Implicit contract**: the monoculture control measures agreement on the verdicts that are
actually in doubt.

**Origin**: both · **Enforcement**: unguarded · **Value**: high

**Evidence** — two opposite, undisclosed biases in the same 8-seam experiment:
- **Upward**: `orchestrator/foreign_replication.py:54-63` — `STRATIFIED_SAMPLE` is
  `spsc, spin_lock, slab, refined, intrusive, secret, sync_cell, ordered_locks`. It
  contains the by-design strawman (`sync_cell`) and both poles — cases where any competent
  adjudicator agrees — and **none** of the 11 disagreement-set seams from finding #4.
- **Downward**: `foreign_replication.py:151-159` + `:200-201` — the `"?"` sentinel is
  handled asymmetrically. `agree` is `None` (unscored) only when the *foreign* side fails
  to parse; when the *recorded* side is missing, `rec_class == "?"` compares unequal to
  everything → `agree = False`, counted as divergence. A renamed or absent
  `REPORT_SEAM_SPEC_<stem>.md` silently deflates the published figure.

**Better alternative, actionable**: Qin et al. (PLDI 2020), Astrauskas et al. (OOPSLA
2020), Evans et al. (ICSE 2020) constitute an existing **human-generated, model-
independent** corpus of real unsafe-encapsulation soundness failures. Re-adjudicating a
stratified sample of those through the same gauntlet tests specificity against human
ground truth, needs no foreign API key, and is a materially stronger control than the one
§6 nominates.

---

### 9. MockArtifactHygiene (HIGH)

**Origin**: both · **Enforcement**: convention · **Value**: high

`runs_foreign/agreement.json` is committed at repo top level and reads as a result:
8 rows, `"agree"` true/false, **2/8 = 25%** against the real recorded verdicts. It is
marked fake only by an inconspicuous per-row `"foreign_headline": "mock"`. Every
`foreign_outcome` is `SEAM-HOLDS` because the stub
(`foreign_replication.py:137-139`) returns that unconditionally — so the two
"agreements" are exactly the two recorded HOLDs and the metric is degenerate rather than a
wiring check.

§6 is **honest** ("only the foreign credential is outstanding") — nothing is suppressed.
But §6 also calls the harness "implemented and turnkey", and the only executed evidence is
a stub that has never been demonstrated capable of emitting a BREAK or a non-`none` horn.

**Formalization path**: a top-level `"MOCK": true` key; make the stub emit varied verdicts
so a dry run exercises the comparison path.

---

### 10. ReviewScopeDrift (HIGH)

**Origin**: prospected · **Enforcement**: unguarded · **Value**: high

`PAPER.md:259` (App. C) and `:186` (§6) cite `proof/REVIEW.md` as the adversarial depth
caveat for `SeamGap.lean`. `proof/REVIEW.md:4` states it "was given only `SeamGap.lean`,
`README.md`, and the trilemma claim" — and `SeamGap.lean:109-110` marks the Characterization
section as **"added in the constructive build-and-break phase"**, i.e. *after* the review.
No REVIEW axis touches `seam_closes_iff_factors` or the per-seam instantiation — precisely
where finding #1 lives. `REVIEW.md:44` ("No Lean strengthening was attempted") is now false
of the artifact.

Separately, `proof/README.md:71` says "**One primitive only**" while the file instantiates
36; README documents ~145 of 1293 lines.

---

### 11. AxiomClaimUncheckable (MEDIUM)

**Origin**: observed · **Enforcement**: convention · **Value**: medium

`PAPER.md:186` asserts "`proof/SeamGap.lean`, axiom-clean modulo `propext`". The file
contains **zero** `#print axioms` commands. `lean SeamGap.lean` exits 0 and prints
nothing, so §8's reproduction recipe verifies **type-checking only, never the axiom
claim**. The claim also covers 2 of 44 theorems; the other 42 each route through
`rw [seam_closes_iff_factors]` (Iff-rewriting ⇒ `propext`). One `#print axioms` line per
headline theorem discharges this.

---

### 12. PriorArtGaps (MEDIUM — informational)

**Origin**: prospected · **Confidence**: fundamental / established · **Value**: medium

Uncited work with direct structural overlap (all verified absent by grep):
- **Rice's theorem** — 0 hits. §2 concedes its non-triviality rests on forcing
  *completeness* onto a sound decidable checker for a non-trivial semantic property. That
  conjunction *is* Rice (1953). Once "must accept" is granted, Rice suffices and the
  reduction skeleton at `PAPER.md:66` does no additional work; if it isn't granted, no
  reduction machinery helps. The paper is stronger and shorter conceding this.
- **Decidable separation-logic fragments** — Berdine–Calcagno–O'Hearn (FSTTCS 2004);
  PTIME (CONCUR 2011); Iosif–Rogalewicz–Šimáček (CADE 2013). §2 cites only the
  *undecidability* half of the same authors. The "restricted-subset escape hatch" §2 says
  the gauntlet closed six times is populated with published results and shipping tools
  (Infer, Viper/Prusti).
- **Abstract-interpretation completeness** — Giacobazzi–Ranzato–Scozzari (JACM 2000);
  Cousot–Cousot. `seam_closes_iff_factors` is the kernel-factorization / AI-completeness
  characterization, and the `V → Vrt` refinement at `SeamGap.lean:190` is exactly the
  known completeness-repair move. 0 hits for `Galois`/`abstract interpret`.
- **Bierhoff–Aldrich (OOPSLA 2007)** — §5.1's headline result, 19 years earlier, in the
  permission-theoretic typestate line the paper's citation stops just short of.
- **Kind-mismatch (highest-value item here)**: §2 argues an obstruction of
  *computability* (undecidability — the fact is determined but not computable; remedy:
  annotations or an external prover). `SeamGap.lean:54` proves an obstruction of
  *information* (`P` is not a function of `V`; remedy: a richer witness, which is what
  `Vrt` does). `Prog` is a 2-bit record — a 4-element state space — so decidability is
  **vacuous** throughout the mechanization. `THE-SAME-WALL.md:19` asserts the conservation
  thesis ("erase the seam and the undecidability reappears at the weld") while the
  artifact mechanizing the weld contains no undecidability at all. The paper's caveats
  cover *depth* and *grain*; not *kind*.

---

## Recommendations

| # | Candidate | Grade | Action | Scope |
|---|---|---|---|---|
| 1 | GuardSyncBound | unguarded/critical | add `unsafe impl<T: Sync> Sync for Guard`, mirror the `spin_lock_use.rs:59-68` harness, re-adjudicate the horn | 30 min |
| 2 | LeanVerdictPolarity | unguarded/critical | disclose in §6/App C; then either instantiate the negative side per BREAK seam or delete the 34 trivial lemmas | 3 hr |
| 3 | AdjudicatorReconciliation | unguarded/critical | state the rule; report the 28% rate; introduce the horn-classifier in §3; mark judge-FAILED gauntlets in App A | 3 hr |
| 4 | SpscEndpointSync | unguarded/critical | `PhantomData<*const ()>` on both endpoints + `compile_fail` doctest | 30 min |
| 5 | ClassifierVocabularyClosure | unguarded/critical | widen the vocabulary or state horn-1-only-breaking as definitional | 30 min |
| 6 | SeamRecordProvenance | unguarded/critical | a `SEAMS` manifest checked in CI against all six sites | 3 hr |
| 7 | NonLlmOracle | unguarded/high | `cargo +nightly miri test`; loom for the 4 concurrency seams | 3 hr |
| 8 | ForeignControlBias | unguarded/high | add the disagreement-set seams; fix the `"?"` asymmetry; consider the Qin corpus instead | 3 hr |
| 9 | ReviewScopeDrift | unguarded/high | re-review the Characterization section, or narrow App C's citation | 30 min |
| 10 | MockArtifactHygiene | convention/high | top-level `"MOCK": true`; vary the stub's output | 30 min |
| 11 | AxiomClaimUncheckable | convention/medium | one `#print axioms` per headline theorem | 30 min |
| 12 | PriorArtGaps | model-predicted/medium | Rice + decidable fragments + AI-completeness + Bierhoff–Aldrich into §7; disclose the kind-mismatch | 3 hr |

## Priority logic

1. **Live bug in shipped code** (#1) — the only finding that is UB reachable from safe code
2. **Contradictions** — prose asserts a property the artifact lacks (#2, #4)
3. **Instrument defines the result** (#3, #5) — findings the measurement could not have contradicted
4. **Unguarded + both** (#6) — the mechanism behind an already-fixed instance
5. Everything else

## Method notes / negative results

- **Not reproduced**: one lens claimed "a parse failure is scored as a clean survival"
  (`round.py:79-80`). Verified false — `verdict` defaults to `"?"`, not `"SURVIVES"`, so a
  parse failure makes the predicate false; the `"?"` appears only in the *severity* tuple
  and that block is the **coldness alarm**, a blind-spot warning, not survival scoring.
  Judge decisions fail *closed* (`or "FAILED"`). The coldness alarm at `round.py:79` is a
  genuine self-aware instrument control that §3 does not mention and should credit.
- **Not reproduced**: one lens claimed SPSC is "spec-only, not a compiled artifact." False
  — `seam/src/lib.rs:717-852` contains the implementation; SPSC is the crate root.
- **Resolved conflict**: two lenses reported the mock agreement rate as 2/8 and 3/8.
  Recomputed: **2/8**.
- **Counting verified sound**: 37-seam partition in Appendix A sums correctly; §5.4's
  "remaining twenty-one" checks out; the 11-break census is corroborated by
  `design_seam/SYNTHESIS.md:517`; 37 specs / 37 reports, zero orphans.

## Next steps

- [ ] `/mm-xtal:gate .crystal/audits/AUDIT-code+stem-2026-08-06.md` — challenge these
      before acting; expect several to be re-graded CIRCUMSTANTIAL
- [ ] `/cold-review:run` after the boundaries are formalized
