# The realizability table — the Seam Trilemma is a 3-of-N projection

*Phase D. The constructive question the slab opened and the branded index settled: is the Seam
Trilemma the whole story (3 horns), or a low-dimensional shadow of a larger tension? Answer: it is a
**3-of-N projection with N ≥ 6**. Each constructive primitive lit up one more axis by being the
**witness that two configurations the trilemma treated as identical actually differ.***

---

## How a "new axis" is established (the method, not an assertion)

An axis is genuinely independent of the others iff some primitive **dissociates** it: two primitives
that agree on every other named axis but split on this one. That split is a realizability fact — a
witness — not a definition. The slab gave the template (SPSC and slab agree on {sound, decidable,
concentrated} but split on zero-cost ⇒ zero-cost is a real 4th axis). Phase D applied the same test
to find the 5th, and the cold panel supplied the 6th's dissociation argument.

Every ✓/✗ below is computed from real `rustc`-checked code in `../seam/` and an **independent cold
verdict** (`REPORT_SEAM_SPEC_*.md`), never asserted by the builder.

## The axes (columns)

The original seed had seven properties. In the *seam* framing the floor concedes #1 (executable
spec) and #3 (derived ownership) by construction (the floor is a fixed checker), and #5 (bare-metal)
holds for every primitive here (none needs a GC/runtime allocator). The live, per-primitive axes:

| Axis | Seed # | What it asks | Folded into the trilemma as |
|---|---|---|---|
| **Sound** | (the wrapper) | Does re-import launder no UB into safe code? | horn 1 |
| **Decidable & proof-free** | #4 | Does the safe fragment need no solver, no hand proof? | horn 2 |
| **Concentration (seam-width)** | #2 (code face) | Does *ordinary code* cross the `unsafe` boundary only O(1)/primitive? | horn 3 |
| **API-locality (brand-freedom)** | #2 (type face) | Does the witness stay out of consumer **type signatures** (no viral `'id`, no forced scope-closure)? | **folded inside "concentration"** — un-bundled by the branded index |
| **Zero-cost erasure** | #6 | Does the witness erase at compile time (no runtime residue)? | **folded inside "concentrated & proof-free"** — un-bundled by the slab |
| **AI-legible** | #7 | Is the witness's *meaning* reconstructible (a readable predicate), not a nominal token? | not in the trilemma at all — surfaced by the branded index |

Two separate "cost" categories sit *outside* the axes and must not be miscounted as horns:
- **TCB axiom** (MMIO): a physical-world fact ("this address really is this register") no checker
  can discharge — a trusted `assume` boundary (seL4/CompCert/Low\*), not a property the seam fails.

## The table

Legend: ✓ holds / aced · ✗ paid or dropped · ◐ conditional · *(plain)* a non-viral ordinary lifetime.

| Primitive | Sound | Decidable | Concentration | **API-locality** | **Zero-cost** | **AI-legible** | What it costs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|---|
| **SPSC ring buffer** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `'a`)* | ✓ | ✓ | nothing — the fully-encapsulable corner |
| **MMIO read-to-clear** | ✓ | ✓ | ✓ O(1)¹ | ✓ | ✓ | ✓ | a **TCB axiom** (outside the axes) |
| **Generational slab** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Key`)* | **✗** | ✓² | **zero-cost (#6)** — runtime generation+occupancy check |
| **Branded index** | ✓ | ✓ | ✓ O(1) | **✗** *(viral `'id`)* | ◐³ | **✗**⁴ | **API-locality (#5)** + deferred zero-cost; brand is nominal |
| **Intrusive list** (branded) | **✗**⁵ | ✓ | ✗ O(accesses) | ✗ | ◐ | ✗ | a **horn genuinely drops** — no sound+usable+concentrated decision exists |

¹ with a foundation `Send` opt-in, else O(ISRs). ² the occupancy+generation compare is a readable
predicate. ³ **aced only under amortization** — index reuse, or the range-branding the spec scopes
out; naive per-element `try_index` relocates the bounds check (same cost as `slice[i]`), per the
cold prior-art attacker. ⁴ the brand `'id` is a *nominal* token; the in-bounds proof lives in the
foundation `unsafe`, not in any consumer-readable type. ⁵ sound *only* by collapsing all nodes to
one arena lifetime (design A), which drops usability (no early reclaim); design B keeps usability
and is unsound (L2 UAF). Either way a horn falls.

## What the table proves

**1. SPSC is the only all-✓ row.** Exactly one primitive aces every axis — and it is the one whose
precondition is *fully floor-expressible* (every escaping conjunct maps to affinity / NLL `'a` /
decidable `T: Send`). Everything else pays somewhere. "Seamless" is achievable, but only for
SPSC-shaped preconditions.

**2. The recovered axes are pairwise dissociable — so they are real, independent dimensions:**

| Claimed-independent pair | Witness that splits them | Status |
|---|---|---|
| Zero-cost ⟂ Concentration | slab (conc ✓, zero-cost ✗) vs SPSC (both ✓) | **gauntlet-confirmed** (slab phase) |
| API-locality ⟂ Concentration | branded index: O(1) seam-width yet viral `'id`; and the `runST`/lightweight-capabilities lineage pays the brand tax **in languages with no seam or `unsafe` at all** — a cost that exists where seam-width is *undefined* cannot *be* seam-width | **gauntlet-confirmed** (this phase) |
| API-locality ⟂ Zero-cost | slab (zero-cost ✗, API-local ✓) vs branded index (brand erases, API-local ✗) — opposite corners | **gauntlet-confirmed** |
| AI-legible ⟂ API-locality | a refinement type `{v:Int \| v < len}` (Liquid Haskell / F\*/Dafny) is **brand-free yet legible**; a brand is **viral yet illegible** | **argued** (cold prior-art attacker), **not yet witnessed** — in this 5-primitive corpus #5 and #7 still co-vary (the off-diagonal — brand-free-but-illegible, or viral-but-legible — has no built witness). The honest next experiment. |

**3. The trilemma is exactly ONE forbidden pattern in this space.** {Sound ∧ Decidable ∧
Concentrated} is unachievable *only* when the precondition factors through no available witness — and
the intrusive list is the witness that the forbidden pattern is real (whole-heap aliasing+liveness
escapes every witness vocabulary, so a horn drops). Every other primitive **closes** the three
horns and pays on a *recovered* axis instead. The trilemma never said otherwise — it is a true
shadow; the table is the object casting it.

## The recursion: each primitive un-bundled one folded axis

```
unified (negative result)   7 properties, none-of-all; #4 the universal antagonist, #2/#3/#6 forcing functions
        │  project onto the seam
Seam Trilemma               3 horns: {sound, #4 decidable, #2 concentration}
        │                   — bundling #6 INTO "concentrated & proof-free"
        │                   — bundling API-locality INTO "concentration"
   slab │  un-bundles #6          ⇒ 3-of-4  (zero-cost is its own axis)
 branded│  un-bundles API-locality ⇒ 3-of-N  (the type-face of "no-seam" ≠ the code-face)
        │                          + surfaces #7 (legibility), dissociable from #5
        ▼
Realizability table         N ≥ 6 axes; the trilemma is one 3-D shadow
```

Each constructive primitive is the **witness that two trilemma-identical configurations differ** —
which is the only way to prove a folded axis was ever really there. The slab proved a concentrated
seam can still cost runtime; the branded index proved a concentrated, erasing seam can still cost
the type surface, and that legibility is a further axis again.

## Structure among the axes (not just a count)

The axes are not independent toggles you can set freely — the branded index exhibits a **trade**:
the range-branding machinery that cashes in zero-cost (#6) *deepens* the API-locality tax (#5)
(ranges, partitions, and derived indices are all branded too — more viral types). So #5 and #6 are
independent dimensions that **exchange** within one primitive. A full systems language would face an
*exchange rate*, not a free lunch — the design lesson the count alone misses.

## Honest bounds

- N ≥ 6 is a **lower bound**: five gauntlet-confirmed live axes (sound, decidable, concentration,
  API-locality, zero-cost) plus one argued (legibility). More may exist; the method (build a
  dissociating witness, let the cold panel adjudicate) is open-ended, not exhausted.
- The #5⟂#7 dissociation is the one unwitnessed claim. Building a refinement-typed seam (Flux-style
  `i: usize{i < len}`) is the next experiment: it should sit at (API-local ✓, legible ✓,
  zero-cost ✓) — and if it does, it is *another* all-but-one-axis witness, separating legibility for
  real. Until built, #7 is a credible 6th axis, not a confirmed one.
- The Lean kernel (`../proof/SeamGap.lean`) sees only whether a precondition factors through a
  Boolean witness — so it certifies *closure* (branded index closes, like SPSC) but is blind to
  *which axis a closing seam spends*. Every column past "concentration" lives in this
  modeling-dictionary, by the same honest caveat the slab's `Vrt` carries.
