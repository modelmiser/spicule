# Meta-Synthesis — are the incompatibilities one theorem?

*Over 6 recorded incompatibilities.*

## One theorem (universal property: Sound re-import — the requirement that crossing back from the seam restore the safe fragment's invariants with a witness that launders no UB into safe code; it is the casualty named in all six.)

> Because the seam must concentrate on foundational primitives whose true preconditions are semantic heap-, value-, and concurrency-level invariants expressible only in an undecidable, certificate-free concurrent separation logic (Iris/RustBelt grade), while the safe fragment can receive only a witness that is simultaneously decidable, proof-free, and zero-cost-erasable (at most NLL regions + affinity + bounded arithmetic), the re-import obligation is necessarily strictly weaker than the precondition it must certify; the missing conjunct forces exactly one of three fatal outcomes — drop it (unsound re-import, UB in safe code), keep it faithfully (undecidable/proof-bearing/external prover, breaking decidability and one-toolchain), or re-check it per use (ordinary code crosses the seam, breaking concentration) — so sound re-import, a decidable-and-proof-free zero-cost safe fragment, and seam concentration cannot all three hold.

## How each follows

- **#1 heap preconditions vs decidable-proof-free** — Instantiates the gap on aliasing/provenance/temporal heap facts and takes the drop horn — a decidable place-logic can only weaken pre_j, laundering UB.
- **#2 linearity witness vs NLL regions** — Instantiates the gap on value-level uniqueness (unique &mut from a Copy handle) and explicitly traverses all three horns: re-mint aliasing &mut / undecidable / ATS2.
- **#3 machine-checked kernel vs concurrent re-import** — Instantiates the gap on Send/Sync and the memory model and takes the keep horn — the only sound logic is undecidable and certificate-free, so any decidable kernel obligation omits the thread-safety conjunct.
- **#4 structural borrow checker vs semantic heap invariant** — States the gap as the structural-vs-semantic bridge: cheap obligations leave the frame open (drop), strong ones become RustBelt's fundamental theorem (keep = the forbidden external prover).
- **#5 re-import vs NLL+affinity vocabulary** — Instantiates the gap on happens-before/cardinality/liveness/variance, hits drop→UB or keep→ATS2, and notes concentration can't rescue because the foundation proofs are themselves the open Iris-grade logic.
- **#6 decidable-proof-free seam logic too weak** — Instantiates the gap on the arithmetic-refinement layer (UTVPI/QF-LIA, ℤ-vs-modular), which is actively unsound if kept weak and undecidable/concentration-collapsing if strengthened — the same trilemma.

---

## Full reasoning

## One theorem. Not six.

All six are the same impossibility, photographed from six angles. The proof that unifies them is a single **expressiveness-gap-plus-trilemma** argument; each listed incompatibility is that argument instantiated on a different *class of semantic invariant* and driven into a different *horn*.

### The common skeleton

Strip the labels and every one of the six has the identical three-line shape:

1. **What concentration forces onto the seam.** The hard constraint says the seam must concentrate on a *fixed* set of foundational primitives — allocators, atomics, lock-free structures, MMIO. This is not negotiable, so the seam's true preconditions are *whatever those primitives actually require*.

2. **What those preconditions actually are.** They are semantic, heap- and value-level invariants: aliasing/provenance/temporal heap facts (#1, #4), value-level linearity/uniqueness — minting a unique `&mut` from a `Copy` handle (#2), concurrent happens-before / `Send`+`Sync` / memory-model facts (#3, #5), whole-heap representation validity — refcount/bit-pattern/no-hidden-alias (#4), relational/cardinality/liveness facts (#5), and nonlinear / modular-vs-ℤ arithmetic (#6). Every one of these lives **only** in an undecidable, certificate-free concurrent separation logic — Iris / RustBelt grade.

3. **What the safe fragment can receive.** By concession the safe fragment is fixed: a hardcoded NLL borrow checker + affinity, and any obligation it consumes must be *simultaneously* decidable, proof-free, and zero-cost-erasable. The strongest witness language meeting all three is at most **NLL regions + affinity + UTVPI/QF-LIA-bounded refinements** — strictly, provably weaker than the logic in line 2.

Because the re-import obligation can only be expressed in the weaker language of line 3 but must certify the stronger precondition of line 2, it is *necessarily* strictly weaker than the truth. The residual — the missing conjunct — forces **exactly one of three outcomes, all fatal:**

- **Drop** the conjunct → re-import is unsound; UB laundered into safe code.
- **Keep** it faithfully → undecidable / proof-bearing / an external prover — violating decidable-proof-free, one-toolchain, and erasure at once.
- **Re-check** it per use → ordinary code crosses the seam everywhere — the ATS2 cliff, which destroys concentration.

That is the whole theorem. The reason it reads as "six findings" is only that an adversary, to *survive* challenge, had to pin each instance to a concrete invariant class and the concrete horn it could not escape — but the structural reason ("the required witness is strictly more expressive than any decidable+proof-free+erasable witness language can be, so re-import, decidability, and concentration cannot co-hold") is one reason, stated once.

### Why it is genuinely one and not a forced merge

The unification is not cosmetic, because the *same single move* refutes any attempt to escape any of the six: widen the witness language to close the gap and you cross into undecidability/proof-obligations (collapsing decidable-proof-free and pushing proofs into callers, i.e. collapsing concentration — #2,#4,#5,#6); narrow it to stay decidable and the conjunct you shed is always the soundness-critical one (#1,#3). There is no third axis being traded; "concentration cannot rescue it" (#5) is just the observation that the foundation proofs are themselves open Iris developments, i.e. they *are* line 2. A real second theorem would have to introduce an invariant that is NOT in the undecidable semantic logic yet still required by re-import — none of the six does; all six invariants are the same RustBelt-grade semantic predicate seen through different primitives.

### How each facet drops out

- **#1** — picks the *heap-aliasing/provenance/temporal* invariant class, driven into the **drop** horn (decidable place-logic can't state it → Seam-Elim weakens `pre_j` → UB).
- **#2** — picks the *value-level linearity/uniqueness* class; explicitly walks all three horns (re-mint two `&mut` / RustBelt-undecidable / ATS2).
- **#3** — picks the *concurrency: `Send`/`Sync`/memory-model* class, driven into the **keep→external-prover/one-toolchain** horn (any decidable kernel obligation omits the thread-safety conjunct → data race).
- **#4** — picks the *whole-heap representation* class and states the gap structurally: cheap obligation leaves the frame open (drop), strong obligation = RustBelt's fundamental theorem (keep = the forbidden external prover).
- **#5** — picks the *relational/cardinality/liveness/variance* class; drop → UB, keep → ATS2 tax, plus "concentration can't rescue" = the foundation proofs are line 2.
- **#6** — picks the *arithmetic-refinement* class; the decidable logic collapses to UTVPI/QF-LIA and is actively unsound (ℤ vs modular usize), and strengthening hits keep/re-check.

So: same target invariant (semantic separation-logic precondition), same fixed ceiling (decidable+proof-free+erasable witness), same forced trilemma, different cross-section each time.

```json
{"one_theorem": true,
 "universal_property": "Sound re-import — the requirement that crossing back from the seam restore the safe fragment's invariants with a witness that launders no UB into safe code; it is the casualty named in all six.",
 "statement": "Because the seam must concentrate on foundational primitives whose true preconditions are semantic heap-, value-, and concurrency-level invariants expressible only in an undecidable, certificate-free concurrent separation logic (Iris/RustBelt grade), while the safe fragment can receive only a witness that is simultaneously decidable, proof-free, and zero-cost-erasable (at most NLL regions + affinity + bounded arithmetic), the re-import obligation is necessarily strictly weaker than the precondition it must certify; the missing conjunct forces exactly one of three fatal outcomes — drop it (unsound re-import, UB in safe code), keep it faithfully (undecidable/proof-bearing/external prover, breaking decidability and one-toolchain), or re-check it per use (ordinary code crosses the seam, breaking concentration) — so sound re-import, a decidable-and-proof-free zero-cost safe fragment, and seam concentration cannot all three hold.",
 "facets": [
   {"incompat": "#1 heap preconditions vs decidable-proof-free", "how_it_follows": "Instantiates the gap on aliasing/provenance/temporal heap facts and takes the drop horn — a decidable place-logic can only weaken pre_j, laundering UB."},
   {"incompat": "#2 linearity witness vs NLL regions", "how_it_follows": "Instantiates the gap on value-level uniqueness (unique &mut from a Copy handle) and explicitly traverses all three horns: re-mint aliasing &mut / undecidable / ATS2."},
   {"incompat": "#3 machine-checked kernel vs concurrent re-import", "how_it_follows": "Instantiates the gap on Send/Sync and the memory model and takes the keep horn — the only sound logic is undecidable and certificate-free, so any decidable kernel obligation omits the thread-safety conjunct."},
   {"incompat": "#4 structural borrow checker vs semantic heap invariant", "how_it_follows": "States the gap as the structural-vs-semantic bridge: cheap obligations leave the frame open (drop), strong ones become RustBelt's fundamental theorem (keep = the forbidden external prover)."},
   {"incompat": "#5 re-import vs NLL+affinity vocabulary", "how_it_follows": "Instantiates the gap on happens-before/cardinality/liveness/variance, hits drop→UB or keep→ATS2, and notes concentration can't rescue because the foundation proofs are themselves the open Iris-grade logic."},
   {"incompat": "#6 decidable-proof-free seam logic too weak", "how_it_follows": "Instantiates the gap on the arithmetic-refinement layer (UTVPI/QF-LIA, ℤ-vs-modular), which is actively unsound if kept weak and undecidable/concentration-collapsing if strengthened — the same trilemma."}
 ]}
```