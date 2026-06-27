# Build-and-break the seam — A→B→C synthesis

The negative half of this project proved keeping all seven properties is impossible and
distilled the **Seam Trilemma**: a seam cannot simultaneously have (1) sound re-import,
(2) a decidable, proof-free safe fragment, and (3) seam concentration. The constructive
phase asked the next question: *take the trilemma as a map and actually build a seam.* The
stateful builder authored each seam spec + a real `rustc`-checked implementation; the cold
adversarial harness (5 attackers + judge + an independent horn-classifier) red-teamed the
**re-import rule only**, never the author. Three primitives, chosen so their preconditions
have three different *shapes*.

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

### The deeper result the slab forced: the Seam Trilemma is a 3-of-4 projection

The slab keeps *all three* trilemma horns yet still pays — on **zero-cost erasure (#6)**, a
property the trilemma had abstracted into "concentrated & proof-free." So the three horns were
never the whole story: they are a projection of a four-property tension whose fourth axis
(zero runtime cost) only becomes visible when the dangerous conjunct is a *runtime-checkable*
fact about a `Copy` handle. This is the precise constructive echo of the original `unified`
result, where **#6 (zero-cost) was one of the casualties** alongside decidability. The
constructive phase recovers it: when the precondition is runtime-checkable, you *can* have
sound + decidable + concentrated — by spending #6, exactly as `RefCell`/`slotmap` do.

The four shapes, then, are a small taxonomy of *what the missing conjunct is*:
- **Fully floor-expressible** → seam closes free (SPSC).
- **Runtime-checkable** → seam closes at the cost of zero-cost/#6 (slab; `RefCell`).
- **An external physical axiom** → seam is sound modulo a TCB `assume`, outside the trilemma (MMIO).
- **Whole-heap aliasing+liveness that escapes to the caller** → no sound+usable+concentrated
  decision exists; a trilemma horn genuinely drops (intrusive list).

## Decontamination (the prompt's C1): did the obstruction move or vanish?

The seamful2 theorem predicted that when a seam fails to close, the obstruction *relocates*
to the re-import boundary rather than disappearing. Across the three primitives:

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
- **Intrusive — it moved, and broke soundness.** The obstruction relocated to the boundary
  and what sits there is genuinely unsound (L2 UAF): horn 1 drops. Any repair walks into
  another horn (collapse lifetimes → drop usability; full separation logic → drop
  decidability). (This is the `intrusive_seam_does_not_close` Lean instance.)

"Moves, doesn't vanish" is confirmed for the three non-trivially-encapsulable primitives —
and *where* it moves (decidable boundary bound / runtime witness / genuinely unsound) is the
whole verdict; "vanishes" is the signature of the fully-encapsulable one. The decontamination
method generalized cleanly across all four.

## What the harness added that the builder missed

The adversarial process earned its keep three times:

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

## Artifacts

- Specs (attacked as text): `SEAM_SPEC_{spsc,mmio,intrusive,slab}.md`
- Cold verdicts: `REPORT_SEAM_SPEC_{spsc,mmio,intrusive,slab}.md` (+ raw transcripts in `runs_seam/`)
- Real Rust (rustc-checked): `../seam/` — `src/lib.rs` (SPSC), `src/mmio.rs`, `src/intrusive.rs`,
  `src/slab.rs`; consumer code is `#![forbid(unsafe_code)]`; laundering paths are `compile_fail`
  doctests (18 crate tests total).
- Seam-width instrument: `SEAM_WIDTH.md`
- Mechanization: `../proof/SeamGap.lean` — `seam_closes_iff_factors` + the `spsc`/`intrusive`/`slab`
  instances (propext-only).

## The one-sentence upgrade to the trilemma

The Seam Trilemma says *a* seamless language is impossible; the constructive phase says
**which** seams you can build losslessly anyway — exactly the ones whose dangerous precondition
is fully encapsulable (SPSC-shaped) — while the rest force a predictable, measurable price set by
*what the missing conjunct is*: a runtime witness that spends zero-cost (slab), an external TCB
axiom (MMIO), or a genuine horn drop (intrusive) — and the trilemma itself turns out to be a
3-of-4 projection, with zero-cost (#6) the fourth axis the slab makes visible.
