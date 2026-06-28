# Build-and-break the seam — A→B→C synthesis

The negative half of this project proved keeping all seven properties is impossible and
distilled the **Seam Trilemma**: a seam cannot simultaneously have (1) sound re-import,
(2) a decidable, proof-free safe fragment, and (3) seam concentration. The constructive
phase asked the next question: *take the trilemma as a map and actually build a seam.* The
stateful builder authored each seam spec + a real `rustc`-checked implementation; the cold
adversarial harness (5 attackers + judge + an independent horn-classifier) red-teamed the
**re-import rule only**, never the author. Seven primitives, chosen so their preconditions have seven
different *shapes* — and so that each later one becomes the witness un-bundling an axis the trilemma
had folded (zero-cost, API-locality, AI-legibility, solver-freedom, bare-metal) — plus an **eighth,
deliberate falsification**: a type-level-proof index pitched as recovering a *checker-tractability*
axis, which the gauntlet **rejected** (sound, but the off-diagonal is confounded). That rejection is
the strongest evidence the method measures something — it has a STOP, and the eighth probe found it.

## The headline result

**Whether a seam closes is not a property of "seams" — it is a function of the
precondition's shape.** The trilemma bites *selectively*, and exactly where it bites is
predictable from one question: *can every conjunct of the primitive's true precondition
that the caller can observe be carried by the floor's witness vocabulary (affinity, NLL
lifetimes, decidable trait bounds) — and if not, is the missing conjunct an external axiom,
a runtime-checkable fact, or genuinely unrepairable?*

| Primitive | Cold verdict | Horn outcome | Seam width (measured) | Why — precondition shape |
|---|---|---|---|---|
| **SPSC ring buffer** | SEAM-HOLDS (4 cold panels: A + B×3) | none drops | **O(1)/buffer**, 0/element | *Fully encapsulable.* Every escaping conjunct is floor-expressible: cardinality→affinity, storage-lifetime→NLL `'a`, payload safety→decidable `T: Send`. The hard facts (happens-before) leave no caller-visible residue. |
| **MMIO read-to-clear** | SEAM-COSTS [horn 3] | sound + decidable, marked cost | O(1) **iff** foundation adds a `Send` opt-in, else O(ISRs) | *Sound modulo an external axiom.* M1/M4 ("this address really is this register") is a physical-world fact no checker can discharge — a TCB `assume` boundary (Low\*/seL4/CompCert), **not a fourth horn**. The real price is a horn-3 concentration cost around the ISR `Send` opt-in. |
| **Generational slab** | SEAM-COSTS [#6 zero-cost] (after one revision) | **all three trilemma horns hold** | **O(1)**, fully-safe interface | *`Copy`-handle validity — runtime-checkable.* A `Copy` key can outlive its slot, so affinity/NLL can't carry S1–S3; but a per-slot generation + occupancy tag *can*, at run time. Closes the trilemma but spends the **zero-cost property (#6)** the trilemma folded away — `RefCell`'s move. |
| **Intrusive doubly-linked list** | SEAM-BREAKS [horn 1] | horn 1 drops (repair drops usability *or* decidability) | O(accesses) for the branded form; design A forbids early reclaim | *Whole-heap aliasing + liveness that escapes to the caller.* The `GhostCell` brand carries L1/L4 (aliasing) but **not L2** (membership-vs-storage lifetime). Token correctly presented, the rule type-accepts a use-after-free. |
| **Branded index** (sound unchecked indexing) | SEAM-COSTS [other] (1 cold panel) | all 3 horns hold | O(1)/slice — yet a *viral* `'id` in every type | *Cross-object identity, carried by a generative brand.* Closes all three horns (RustBelt-verified lineage), but un-bundles **concentration** itself: pays on **API-locality (#5)** (the `'id` infects consumer types, traps indices in a `scope` closure) and *defers* zero-cost (#6) to range-branding. The brand is a nominal token (#7 illegible). |
| **Refined index** (refinement-typed bounds) | SEAM-COSTS [horn2] (1 cold panel) | all 3 horns hold | O(1)/index — brand-free, the value *escapes* | *In-bounds, carried by a legible predicate (Flux/Liquid Haskell).* The **(brand-free, legible)** corner — it **closes the #5⟂#7 cell** (vs the slab's brand-free-but-illegible handle). Un-bundles **horn 2** into *decidability* ✓ vs *solver-freedom* ✗ (needs SMT in the TCB — the `unified` "keep" horn). The R2 trade: brand-free ⟹ a runtime check, zero-cost ⟹ a length-brand. |
| **Shared (Rc)** (refcounted ownership) | SEAM-COSTS [bare-metal] (1 cold panel) | all 3 horns hold | O(1) — brand-free; needs a heap | *Dynamic shared ownership, carried by a runtime refcount manager.* Recovers seed property **#5 bare-metal**: the sole seam needing a global allocator (compile-enforced — the only `extern crate alloc`). Independent of #6 (slab/`heapless` off-diagonal: fixed pool vs global heap), but the judge deflated novelty — it **= Rust's `core`/`alloc` boundary (RFC 2480)**, real but pre-existing. |
| **Type-level index** (proof by trait resolution) | **FAILED** [axis deliverable] — sound (1 cold panel) | sound; all 3 horns hold; pays a recorded horn-2 cost | O(1) — brand-free, erases | *Compile-time proof `I: Lt<Len>`, carried by the trait resolver.* Sound (sealed `Nat` + orphan rule block forgery, unanimous), zero-cost, solver-free. **But the eighth-axis claim was rejected:** the off-diagonal is confounded with a strict expressiveness loss (static-index-only), the `recursion_limit` cost is encoding-relative (binary numerals erase it), and the axis is mislabeled — `Lt` is *linear*, the real concern is **terminating-resolution** (Paterson / `UndecidableInstances`), pre-existing. No new axis; N stays 7. |

### The deeper result, forced four times: the Seam Trilemma is a 3-of-N projection (N ≥ 7)

The slab forced the first un-bundling. It keeps *all three* trilemma horns yet still pays — on
**zero-cost erasure (#6)**, a property the trilemma had abstracted into "concentrated & proof-free."
So the three horns were never the whole story: they project a four-property tension whose fourth
axis (zero runtime cost) only becomes visible when the dangerous conjunct is a *runtime-checkable*
fact about a `Copy` handle — exactly the `RefCell`/`slotmap` move.

The **branded index** then forced a second, sharper un-bundling — of *concentration itself*. It
aces seam-width (one `scope` per slice, O(1) code crossings) and is RustBelt-verified sound, yet the
cold panel located a real cost "outside the three horns": the brand `'id` is a **viral generative
type parameter** — it never appears in consumer *code* as a crossing, but it appears in every
consumer *type*, and traps every index inside a `scope` closure. That **API-locality** cost is *not*
seam-width: the `runST` (PLDI'94) / lightweight-capabilities (PLPV'06) lineage pays the identical
tax in languages with **no seam or `unsafe` at all** — a cost that exists where seam-width is
undefined cannot *be* seam-width. So "concentration" was a bundle of two faces, a *code* face
(seam-width) and a *type* face (API-locality), and the branded index pulls them apart. The same
primitive surfaced **#7 (AI-legibility)** as a further axis: a brand is a nominal token (the
in-bounds proof lives in the foundation `unsafe`, not in any readable type), and the panel argued it
dissociates from #5 — a refinement type `{v:Int | v < len}` is brand-free yet legible.

The **refined index** then *witnessed* that argued dissociation and forced a third un-bundling — of
*horn 2 itself*. It is the **(brand-free, legible)** corner: a refined value carries a visible
predicate and *escapes its constructor* (a compile error for the branded `Idx<'id>`), so paired
against the slab — reclassified **brand-free yet illegible** (two attackers endorsed it; a `Copy`
`Key`'s validity is not reconstructible from its type) — it holds #5 fixed and flips #7. The cold
judge ruled **#5⟂#7 closes — yes**: AI-legibility is a confirmed independent axis. And horn 2 turns
out to be a bundle of **decidability** (the refined fragment terminates) and **solver-freedom** (it
needs an SMT solver in the trusted loop) — the latter being the `unified` "keep" horn, not a new
axis, so the map gains resolution there rather than a dimension.

The **shared-ownership seam** (a minimal `Rc`) then forced a fourth un-bundling that *does* add a
dimension: it recovers seed property **#5 bare-metal**. Dynamic shared ownership — a live-owner set
fixed at run time — is a precondition shape affinity, NLL, and the slab's single-owner pool all
structurally cannot express; its only sound witness is a runtime refcount **manager**, which needs a
global heap. The judge confirmed bare-metal is a genuine seventh axis (dissociates from zero-cost via
the slab/`heapless` off-diagonal: a fixed pool is bare-metal, a global-heap `Rc` is not) **but
deflated its novelty** — it is Rust's own `core`/`alloc` boundary (RFC 2480), not a discovery.

The **type-level-proof index** then probed for an *eighth* axis — and **failed**, which is recorded
as a result rather than buried. Pitched as recovering **checker-tractability** (does the native
checker halt without an arbitrary `recursion_limit` knob?), it is sound (unanimous — sealing plus the
orphan rule block every forgery) but recovers no clean axis, on three convergent grounds the cold
panel converged on: (1) the off-diagonal is **confounded** — `get::<I>` takes a turbofish *type*, so
the seam is static-index-only, a strict expressive subset of the branded/refined indices; the claimed
one-coordinate flip silently also flips expressiveness, and that confound is *intrinsic* (a runtime
index cannot be carried by a compile-time proof); (2) the cost is **encoding-relative** — unary Peano
hits the limit (E0275), binary type-level naturals (typenum) are O(log N) and do not; (3) it is
**mislabeled** — `Lt` resolution is *linear*, cheaper than refined's QF-LIA, so the real concern is a
**terminating-resolution** fragment (Paterson conditions / GHC `UndecidableInstances`), pre-existing.
The lattice stays **N ≥ 7**; the probe's worth is the falsification — the discipline can refuse.

So the trilemma is a low-dimensional shadow of an **≥7-axis** object — and `unified`'s original
casualties (#6, #2's two faces, #4's two faces, and #5) are recovered one primitive at a time. The
taxonomy is now a map of *what the missing conjunct is* **and which axis closing it spends**:
- **Fully floor-expressible** → seam closes free, every axis ✓ (SPSC).
- **Runtime-checkable** → closes by spending **zero-cost/#6** (slab; `RefCell`).
- **Cross-object identity via a generative brand** → closes by spending **API-locality/#5** (and
  #7 legibility), with a #5↔#6 *trade* (range-branding buys erasure by deepening the brand tax)
  (branded index; `indexing`).
- **In-bounds via a legible predicate** → closes brand-free + legible, spending **solver-freedom**
  (SMT in the TCB) and hitting the R2 trade again (brand-free ⟹ runtime check; zero-cost ⟹
  length-brand) (refined index; Flux/Liquid Haskell).
- **Dynamic shared ownership** → closes brand-free, spending **bare-metal** (a global-allocator
  refcount manager), with a bare-metal↔capacity trade (a pool-backed `Rc` is bare-metal but bounded)
  (shared; std `Rc` / `heapless`).
- **An external physical axiom** → sound modulo a TCB `assume`, outside the axes (MMIO).
- **Whole-heap aliasing+liveness that escapes to the caller** → no sound+usable+concentrated
  decision exists; a trilemma horn genuinely drops (intrusive list).

**The honest meta-pattern.** Every recovered axis turns out to be an *already-named PL concern* —
zero-cost abstraction, generativity, refinement-legibility, the `core`/`alloc` split. The phase did
not find new physics; it found that the trilemma's three horns silently *bundle a set of
independently-known PL design axes*, unified by one rubric: **what does the safe witness cost?**
Twice the cold panel had to deflate a "new dimension" to "real but pre-existing" — and that deflation
is the result. The full face lattice (axes, dissociation proofs, the meta-pattern) is `REALIZABILITY.md`.

## Decontamination (the prompt's C1): did the obstruction move or vanish?

The seamful2 theorem predicted that when a seam fails to close, the obstruction *relocates*
to the re-import boundary rather than disappearing. Across the primitives:

- **SPSC — it vanished.** No surviving obstruction anywhere; the precondition factors
  through the witness, so a sound+usable+concentrated decision exists. (This is the
  `spsc_seam_closes` Lean instance.)
- **MMIO — it moved, and stayed decidable.** The obstruction relocated onto the re-import
  boundary (the foundation-author `Send` opt-in) and stayed inside floor trait resolution —
  SEAM-COSTS, no horn fully drops. Plus a TCB axiom that sits *outside* the trilemma.
- **Slab — it moved off the *static* witness onto a *runtime* one.** The obstruction could not
  be carried statically (a `Copy` key defeats the affine/NLL witness), but it *did* factor
  through a runtime witness (the generation+occupancy check) — so the seam closes, with the
  relocation showing up as the non-erasable runtime cost. (This is the
  `slab_runtime_closes_but_static_does_not` Lean pairing.)
- **Branded index — it moved onto the *type surface*, and stayed sound.** The obstruction (B3,
  cross-object identity) factors through a *generative brand* witness, so the seam closes
  (`branded_seam_closes`, like SPSC) — but the relocation shows up not in code crossings (O(1)) and
  not at run time (the brand erases), but as a **viral `'id` in every consumer type**. The
  obstruction went somewhere genuinely new: the API surface. This is what made "concentration"
  un-bundle into a code face and a type face.
- **Refined index — it moved into the *trusted toolchain*, and stayed sound + brand-free.** The
  obstruction (in-bounds) factors through a *legible predicate* witness, so the seam closes
  (`refined_seam_closes`, like SPSC) — brand-free and legible — but the relocation lands on neither
  code, type, nor (in principle) runtime: it lands on the **SMT solver in the trusted loop** (horn
  2's solver-freedom face). The obstruction moved *out of the program entirely* and into the
  toolchain's TCB. This is what closed the #5⟂#7 cell and un-bundled horn 2.
- **Shared (Rc) — it moved into a *runtime subsystem*, and stayed sound + brand-free.** The
  obstruction (the data-dependent last-drop) factors through a runtime refcount **manager**, so the
  seam closes (`shared_seam_closes`, like SPSC) — but the relocation lands neither in code, type, nor
  the toolchain: it lands on the **global allocator in the deployed binary** (the bare-metal axis).
  The obstruction moved into a runtime subsystem the no-heap target does not have.
- **Intrusive — it moved, and broke soundness.** The obstruction relocated to the boundary
  and what sits there is genuinely unsound (L2 UAF): horn 1 drops. Any repair walks into
  another horn (collapse lifetimes → drop usability; full separation logic → drop
  decidability). (This is the `intrusive_seam_does_not_close` Lean instance.)

"Moves, doesn't vanish" is confirmed for the seven non-trivially-encapsulable primitives — and
*where* it moves (decidable boundary bound / runtime witness / type surface / trusted toolchain /
runtime subsystem / `recursion_limit` fuel knob / genuinely unsound) is the whole verdict; "vanishes"
is the signature of the fully-encapsulable one. The decontamination method generalized cleanly across
all eight — including the eighth, where the cost moved to a place (the fuel knob) that turned out
*not* to be a clean axis at all.

## What the harness added that the builder missed

The adversarial process earned its keep seven times:

1. **MMIO:** the builder framed M1/M4 as possibly "a different shape than the trilemma." The
   harness sharpened it to the exact right category — a TCB `assume` boundary, standard in
   verified-systems practice — and found the *actual* recordable price was a horn-3 `Send`
   opt-in the builder had under-weighted.
2. **Intrusive:** the builder predicted a horn-2/3 *cost* (token threads through code). The
   harness proved something worse — branding does not restore soundness at all, because a
   brand is an access capability, not a storage lifetime (L2 stays open). This corrected a
   real overclaim in the build-side framing ("the sound fix threads a token"): the token
   fixes L1/L4, not L2.
3. **Slab:** the first gauntlet FAILED [horn 1] on a genuine UB the builder's *spec* had —
   the §4 guard was written as generation-equality only, so a fresh `Slab::new()`'s vacant
   slot 0 (generation 0) authenticates a first-ever key `{0,0}` and `assume_init` runs on
   uninitialized memory. Crucially the bug was in the **spec, not the implementation**:
   `seam/src/slab.rs` checks `Entry::Occupied` before the generation compare, and a regression
   test confirms it rejects the exact exploit. This is the phase's build/break separation
   surfacing a real spec↔impl divergence; the revised spec (occupancy AND generation) then
   SURVIVED. The harness also named the canonical fix (slotmap's `version | 1` parity).
4. **Branded index:** the builder framed it as the clean primitive that *aces* all four axes
   (sound, decidable, concentrated, zero-cost) and pays only a fifth. The prior-art panel both
   confirmed the fifth axis with four decades of grounding (`runST` / Kiselyov–Shan / GhostCell)
   *and* corrected a zero-cost overclaim: single-index `try_index` relocates the bounds check per
   element (same cost as `slice[i]`), so the erasure win is *deferred* to the range-branding the
   spec scoped out. It also settled the sixth-axis probe — #5 (virality) and #7 (legibility)
   dissociate, because a refinement type is brand-free yet legible.
5. **Refined index:** the builder claimed the (brand-free, legible) corner with "zero-cost erasure
   with *no* brand." The panel found that claim to be the union of two incompatible configs (the
   **R2 trilemma**: brand-free + sound ⟹ a runtime check; zero-cost + sound ⟹ a length-brand),
   found an undisclosed expressiveness bug (R2 used `==` where safety needs `≤` — now fixed), and
   deflated the novelty of the horn-2 un-bundling (solver-in-TCB is DML/Liquid Types/Flux's founding
   tradeoff, the `unified` "keep" horn, not a new 7th axis). It nonetheless ruled the deliverable
   cleanly: **#5⟂#7 closes — yes**, the slab→illegible reclassification endorsed by two attackers.
6. **Shared (Rc):** the builder pitched bare-metal as a possibly-new seventh dimension. The panel
   confirmed it is a real, independent axis (the slab/`heapless` off-diagonal holds) **but deflated
   the novelty**: it is precisely Rust's `core`/`alloc` boundary (RFC 2480, stable 1.36), and
   `heapless::pool` already witnesses the bare-metal↔capacity trade — so "drop the undiscovered-
   dimension framing." That deflation generalized the honest meta-pattern: every recovered axis is a
   pre-existing named PL concern; the result is the unification, not the dimensions.
7. **Type-level index (the eighth probe):** the builder pitched **checker-tractability** as a possible
   eighth axis and built the off-diagonal to isolate it. The panel conceded soundness unanimously but
   **rejected the axis** — the sharpest intervention of the project. Three independent attackers
   converged: the off-diagonal silently flips **expressiveness** (the seam is static-index-only, a
   strict subset of its siblings — and the confound is intrinsic to a compile-time proof); the
   `recursion_limit` cost is an **encoding artifact** (binary type-level naturals are O(log N) and
   never hit it); and the axis is **mislabeled** (`Lt` is linear, *cheaper* than refined's QF-LIA — the
   real, pre-existing concern is terminating-resolution, Paterson / `UndecidableInstances`). The
   builder took it on the chin: N stays 7. This is the keep that proves the rest — a discovery
   procedure with no rejection mode discovers nothing.

## Artifacts

- Specs (attacked as text): `SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat}.md`
- Cold verdicts: `REPORT_SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat}.md` (+ raw transcripts in `runs_seam/`)
- Real Rust (rustc-checked, `#![no_std]`): `../seam/` — `src/lib.rs` (SPSC), `src/mmio.rs`,
  `src/intrusive.rs`, `src/slab.rs`, `src/branded.rs`, `src/refined.rs`, `src/shared.rs` (the lone
  `alloc` user), `src/typed_nat.rs` (the eighth probe — sound, no new axis); consumer code is
  `#![forbid(unsafe_code)]`; laundering paths are `compile_fail` doctests (29 `tests/` integration
  tests + 16 `compile_fail` doctests = 45 total).
- Seam-width instrument: `SEAM_WIDTH.md`; full face lattice: `REALIZABILITY.md`
- Mechanization: `../proof/SeamGap.lean` — `seam_closes_iff_factors` + the
  `spsc`/`intrusive`/`slab`/`branded`/`refined`/`shared`/`typed_nat` instances (propext-only).

## The one-sentence upgrade to the trilemma

The Seam Trilemma says *a* seamless language is impossible; the constructive phase says
**which** seams you can build losslessly anyway — exactly the ones whose dangerous precondition
is fully encapsulable (SPSC-shaped) — while the rest force a predictable, measurable price set by
*what the missing conjunct is*: a runtime witness that spends zero-cost (slab), a viral generative
brand that spends API-locality (branded index), a legible predicate that spends solver-freedom
(refined index), a refcount manager that spends bare-metal (shared), an external TCB axiom (MMIO), or
a genuine horn drop (intrusive) — and the trilemma itself turns out to be a **3-of-N projection
(N ≥ 7)**, its three horns a shadow of an object whose further axes — zero-cost (#6, slab),
API-locality (branded index), AI-legibility (#7, refined index), and bare-metal (#5, shared) — each
surface only when a primitive becomes the witness that two previously-identical configurations
differ. Honest coda: those axes are each a *pre-existing* named PL boundary; the result is their
unification under one "what does the witness cost?" rubric, not their discovery.
