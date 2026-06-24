# Meta-Synthesis — are the incompatibilities one theorem?

*Over 6 recorded incompatibilities.*

## One theorem (universal property: Decidable, zero-annotation (proof-free) type checking — the only property present on a side of all six pairs)

> No type system can be both (a) decidable and proof-free in the common case and (b) the sole auto-checked discharge site for the substructural, heap-quantified, value-/view-dependent invariants that soundness of GC-free ownership over aliased weak-memory state requires, because at every ownership-state merge point (CFG join, loop fixpoint, erasure-time equality, unsafe/atomic/DMA seam) deciding entailment over that invariant class is undecidable, while the no-seam, derived-ownership, and zero-cost requirements each remove the only boundary that could move the obligation out of the auto-checked fragment — so the decidable fragment is provably too weak to name the invariant and any fragment that can name it loses decidability.

## How each follows

- **decidable ✕ no-seam (weak-memory aliasing)** — no-seam forbids quarantining transferable protocol invariants, forcing relaxed/aliased reasoning into the auto-checked fragment where join-point bi-abduction is undecidable.
- **derived-ownership ✕ decidable (CFG merge)** — deriving (not hardcoding) ownership makes capabilities first-class, so the only decidable merge is the fixed syntactic join that IS the hardcoded checker; the general one is resource-logic entailment, undecidable.
- **derived-ownership ✕ decidable (data structures)** — sound GC-free ownership needs ∀-indexed substructural footprints that lie in the undecidable array-property fragment, which the contraction-admitting quantifier-free decidable layer cannot express.
- **zero-cost ✕ decidable (erasure)** — the witness that must be erasure-irrelevant is the same witness fed to the kernel's equality decision, and one axis cannot make it both irrelevant for cost and relevant for sound decidable equality.
- **decidable ✕ no-seam (invariant restoration)** — auto-discharge forces a closed finite lattice of grades, but the cross-seam invariants are flow/value-dependent and non-orthogonal, so they cannot be partitioned into independent erasable grades.
- **decidable ✕ no-seam (shared-mutable/weak-memory)** — shared-mutable idioms need invariants carrying runtime data (disequality, later-modality protocols, value-indexed resources, per-agent views) that are outside any finite first-order decidable fragment, so soundness forces a runtime witness or an interactive proof — i.e. decidability gives.

---

## Full reasoning

## Verdict: ONE theorem

All six are facets of a single theorem. The tell is mechanical: **decidable, proof-free type checking sits on one side of every one of the six pairs** — it is the only property that appears in all six. Incompatibility #6 even states the conclusion outright ("decidability is the property that must give"). The other three target properties (no-seam unification, proof-derived ownership, zero-cost erasure) are not independent antagonists; they are three different *forcing functions* that push the **same** obligation into the **same** fragment, where the **same** decision problem turns undecidable. That is the signature of facets of one theorem, not N independent findings.

## The single structural reason

Every facet bottoms out at a **merge/decision point** where the checker must decide *entailment between two ownership-or-protocol states*:

- CFG join — `if b {&x} else {&y}` (facet 2)
- loop-header fixpoint — free-list / ready-queue loops (facets 2, 3)
- erasure-time definitional equality — `0 ≡ S n`, `acquire ≡ relaxed`, `Vec n ≡ Vec m` (facet 4)
- the unsafe/atomic/DMA seam crossing — concurrently-written head, MMIO register (facets 1, 5, 6)

Soundness of **GC-free ownership over aliased, weak-memory state** forces those states to be *substructural* (linear, no contraction), *heap-address-quantified* (∀-indexed footprints), and *value-/view-dependent* (transferable protocol invariants, per-agent view closures). Deciding entailment over that class subsumes a provably undecidable problem — bi-abduction / Hilbert's 10th at the join, the array-property fragment (Bradley–Manna–Sipma) on the footprints, open-term definitional equality at erasure.

The decidable, zero-annotation fragment is — necessarily — the dual of that: finite-height, quantifier-free, first-order, contraction-admitting. So it is *provably too weak to name the invariant*, and the moment you enrich it enough to name the invariant, it inherits the undecidable entailment problem. That is the whole theorem.

## Why the other three properties are forcing functions, not separate causes

Each one removes the **only** escape hatch — the trusted boundary that would let the hard obligation live *outside* the auto-checked fragment:

- **No-seam unification** forbids quarantining the relaxed/aliased reasoning behind an FFI/`unsafe` floor → it must be auto-checked (facets 1, 5, 6).
- **Derived (not hardcoded) ownership** forbids the fixed syntactic join that *would* be decidable, because deriving capabilities makes them first-class/parametric → the join becomes general resource-logic entailment (facets 2, 3).
- **Zero-cost erasure** forbids keeping the witness at runtime, and in a unified language the same witness feeds the kernel's equality decision → it cannot be both erasable-irrelevant and decision-relevant on one axis (facet 4).

Strip any one of these and the obligation can hide behind a seam, a syntactic borrow checker, or a runtime flag — and the collision dissolves. They are alternative routes to the same cliff, not different cliffs.

## Minimal claim

You can have **at most one** of: {decidable, proof-free, Rust-cheap checking} and {sound, seamless, GC-free, zero-cost ownership of shared-mutable weak-memory state}. The decidable proof-free fragment is provably too weak to name the required substructural/heap-quantified/value-dependent invariant; any fragment expressive enough to name it inherits an undecidable entailment problem at the unavoidable merge point — and the no-seam / derived / zero-cost requirements remove every boundary that could host that reasoning elsewhere.

```json
{"one_theorem": true,
 "universal_property": "Decidable, zero-annotation (proof-free) type checking — the only property present on a side of all six pairs",
 "statement": "No type system can be both (a) decidable and proof-free in the common case and (b) the sole auto-checked discharge site for the substructural, heap-quantified, value-/view-dependent invariants that soundness of GC-free ownership over aliased weak-memory state requires, because at every ownership-state merge point (CFG join, loop fixpoint, erasure-time equality, unsafe/atomic/DMA seam) deciding entailment over that invariant class is undecidable, while the no-seam, derived-ownership, and zero-cost requirements each remove the only boundary that could move the obligation out of the auto-checked fragment — so the decidable fragment is provably too weak to name the invariant and any fragment that can name it loses decidability.",
 "facets": [
   {"incompat": "decidable ✕ no-seam (weak-memory aliasing)", "how_it_follows": "no-seam forbids quarantining transferable protocol invariants, forcing relaxed/aliased reasoning into the auto-checked fragment where join-point bi-abduction is undecidable."},
   {"incompat": "derived-ownership ✕ decidable (CFG merge)", "how_it_follows": "deriving (not hardcoding) ownership makes capabilities first-class, so the only decidable merge is the fixed syntactic join that IS the hardcoded checker; the general one is resource-logic entailment, undecidable."},
   {"incompat": "derived-ownership ✕ decidable (data structures)", "how_it_follows": "sound GC-free ownership needs ∀-indexed substructural footprints that lie in the undecidable array-property fragment, which the contraction-admitting quantifier-free decidable layer cannot express."},
   {"incompat": "zero-cost ✕ decidable (erasure)", "how_it_follows": "the witness that must be erasure-irrelevant is the same witness fed to the kernel's equality decision, and one axis cannot make it both irrelevant for cost and relevant for sound decidable equality."},
   {"incompat": "decidable ✕ no-seam (invariant restoration)", "how_it_follows": "auto-discharge forces a closed finite lattice of grades, but the cross-seam invariants are flow/value-dependent and non-orthogonal, so they cannot be partitioned into independent erasable grades."},
   {"incompat": "decidable ✕ no-seam (shared-mutable/weak-memory)", "how_it_follows": "shared-mutable idioms need invariants carrying runtime data (disequality, later-modality protocols, value-indexed resources, per-agent views) that are outside any finite first-order decidable fragment, so soundness forces a runtime witness or an interactive proof — i.e. decidability gives."}
 ]}
```