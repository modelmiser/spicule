# The realizability table — the Seam Trilemma is a 3-of-N projection

*Phases D–E. The constructive question the slab opened, the branded index extended, and the refined
index closed: is the Seam Trilemma the whole story (3 horns), or a low-dimensional shadow of a larger
tension? Answer: it is a **3-of-N projection with N ≥ 6**. Each constructive primitive lit up one more
axis by being the **witness that two configurations the trilemma treated as identical actually
differ.***

> **A note on the "#5/#6/#7" shorthand.** `#6` (zero-cost) and `#7` (AI-legible) are the literal
> seed-property numbers. `#5` is used throughout as shorthand for the *fifth lattice axis*,
> **API-locality / brand-freedom** — which is the *type-face of seed property #2 (no-seam)*, NOT seed
> property #5 (bare-metal, which is uniform and conceded here). So "the #5⟂#7 cell" = "API-locality
> ⟂ AI-legibility." The shorthand is kept because it is now shared vocabulary; this note disambiguates
> it from the seed numbering.

---

## How a "new axis" is established (the method, not an assertion)

An axis is genuinely independent of the others iff some primitive **dissociates** it: two primitives
that agree on every other named axis but split on this one. That split is a realizability fact — a
witness — not a definition. The slab gave the template (SPSC and slab agree on {sound, decidable,
concentrated} but split on zero-cost ⇒ zero-cost is a real 4th axis). Phase D applied the same test
to find the 5th (API-locality), and Phase E built the off-diagonal that witnesses the 6th
(AI-legibility) — promoting it from the branded phase's argued claim to a confirmed one.

Every ✓/✗ below is computed from real `rustc`-checked code in `../seam/` and an **independent cold
verdict** (`REPORT_SEAM_SPEC_*.md`), never asserted by the builder.

## The axes (columns)

The original seed had seven properties. In the *seam* framing the floor concedes #1 (executable
spec) and #3 (derived ownership) by construction (the floor is a fixed checker), and #5 (bare-metal)
holds for every primitive here (none needs a GC/runtime allocator). The live, per-primitive axes:

| Axis | Seed # | What it asks | Folded into the trilemma as |
|---|---|---|---|
| **Sound** | (the wrapper) | Does re-import launder no UB into safe code? | horn 1 |
| **Decidable & proof-free** | #4 | Does the safe fragment need no solver, no hand proof? | horn 2 — *itself a bundle* of **decidability** ⊗ **solver-freedom**, un-bundled by the refined index |
| **Concentration (seam-width)** | #2 (code face) | Does *ordinary code* cross the `unsafe` boundary only O(1)/primitive? | horn 3 |
| **API-locality (brand-freedom)** | #2 (type face) | Does the witness stay out of consumer **type signatures** (no viral `'id`, no forced scope-closure)? | **folded inside "concentration"** — un-bundled by the branded index |
| **Zero-cost erasure** | #6 | Does the witness erase at compile time (no runtime residue)? | **folded inside "concentrated & proof-free"** — un-bundled by the slab |
| **AI-legible** | #7 | Is the witness's *meaning* reconstructible (a readable predicate), not a nominal token? | not in the trilemma at all — surfaced by the branded index, **confirmed independent by the refined index** |

Two separate "cost" categories sit *outside* the axes and must not be miscounted as horns:
- **TCB axiom** (MMIO): a physical-world fact ("this address really is this register") no checker
  can discharge — a trusted `assume` boundary (seL4/CompCert/Low\*), not a property the seam fails.

## The table

Legend: ✓ holds / aced · ✗ paid or dropped · ◐ conditional · *(plain)* a non-viral ordinary lifetime.

| Primitive | Sound | Decidable | Concentration | **API-locality** | **Zero-cost** | **AI-legible** | What it costs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|---|
| **SPSC ring buffer** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `'a`)* | ✓ | ✓ | nothing — the fully-encapsulable corner |
| **MMIO read-to-clear** | ✓ | ✓ | ✓ O(1)¹ | ✓ | ✓ | ✓ | a **TCB axiom** (outside the axes) |
| **Generational slab** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Key`)* | **✗** | **✗**² | **zero-cost (#6)** — runtime generation+occupancy check |
| **Branded index** | ✓ | ✓ | ✓ O(1) | **✗** *(viral `'id`)* | ◐³ | **✗**⁴ | **API-locality (#5)** + deferred zero-cost; brand is nominal |
| **Refined index** | ✓ | ◐⁶ | ✓ O(1) | ✓ *(brand-free; `Refined` escapes)* | ◐⁷ | **✓**⁸ | **horn-2 solver-freedom** (SMT in the TCB) + the R2 trade |
| **Intrusive list** (branded) | **✗**⁵ | ✓ | ✗ O(accesses) | ✗ | ◐ | ✗ | a **horn genuinely drops** — no sound+usable+concentrated decision exists |

¹ with a foundation `Send` opt-in, else O(ISRs). ² **reclassified legible→illegible** (cold panel,
endorsed by two independent attackers vs the slotmap/generational-arena precedent): a `Copy` `Key`'s
validity (live slot at matching generation) is *not* reconstructible from the `Key` type or the
`get -> Option` signature — the occupancy+generation mechanism is foundation-internal. ³ **aced only
under amortization** — index reuse, or the range-branding the spec scopes out; naive per-element
`try_index` relocates the bounds check (same cost as `slice[i]`). ⁴ the brand `'id` is a *nominal*
token; the in-bounds proof lives in the foundation `unsafe`, not in any consumer-readable type.
⁵ sound *only* by collapsing all nodes to one arena lifetime (design A), which drops usability (no
early reclaim); design B keeps usability and is unsound (L2 UAF). ⁶ **horn 2 un-bundles**:
*decidable* ✓ (QF-LIA/UTVPI terminates) but *solver-free* ✗ — it needs an external SMT solver in the
trusted loop, and the fragment is too weak for the bitwise/modular/stride index math (`pos&(cap-1)`,
`i*stride`) that motivates `get_unchecked`. ⁷ the **R2 trilemma**: brand-free + sound forces a
runtime `bound ≤ len` compare (the shipped artifact — *not* zero-cost); recovering erasure forces a
viral length-index brand (`const N`) — the #5↔#6 trade, from the other side. ⁸ **legible** — the
predicate `value < bound` *is* the type's contract; the safety invariant is reconstructible from the
signature alone. This is the cell-closing witness.

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
| AI-legible ⟂ API-locality | **the off-diagonal, now built.** Refined index = (brand-free, **legible**) vs slab = (brand-free, **illegible**): holds #5 fixed, flips #7. Branded index = (viral, illegible) vs slab = (brand-free, illegible): holds #7 fixed, flips #5. Both directions witnessed. | **gauntlet-confirmed** (refined phase): the judge ruled "#5⟂#7 close? — **YES**", and the slab→illegible reclassification was endorsed by two independent attackers. Promoted argued→**confirmed**. |

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
        │                   — bundling solver-freedom INTO "decidable"
   slab │  un-bundles #6              ⇒ 3-of-4  (zero-cost is its own axis)
 branded│  un-bundles API-locality    ⇒ 3-of-N  (the type-face of "no-seam" ≠ the code-face)
        │                              + surfaces #7 (legibility), dissociable from #5
 refined│  un-bundles horn 2          ⇒ decidable ✓ vs solver-free ✗; CLOSES #5⟂#7 (off-diagonal)
        │                              (solver-freedom = the `unified` "keep" horn, NOT a new axis)
        ▼
Realizability table         N ≥ 6 axes (≥5 confirmed + #7 confirmed); the trilemma is one 3-D shadow
```

Each constructive primitive is the **witness that two trilemma-identical configurations differ** —
the only way to prove a folded axis was ever really there. The slab proved a concentrated seam can
still cost runtime; the branded index proved a concentrated, erasing seam can still cost the type
surface; the refined index proved legibility (#7) is its own axis (the brand-free-yet-legible
off-diagonal the slab-vs-branded pair could not reach) **and** that horn 2 itself was a bundle of
*decidability* and *solver-freedom*.

## Structure among the axes (not just a count)

The axes are not independent toggles you can set freely — **two trades** show up:

- **#5 ↔ #6 (the branded index, and again the refined index).** The range-branding that cashes in
  zero-cost (#6) *deepens* the API-locality tax (#5). The refined index hits the *same* trade from
  the other side — the cold panel's **"R2 trilemma"**: a refined index can be *brand-free + sound*
  (pay a runtime `bound ≤ len` check — not zero-cost) or *zero-cost + sound* (tie the length in the
  type = a viral length-index brand), but not both. So #5 and #6 do not just dissociate, they
  *exchange* — a design exchange-rate, not a free lunch.
- **horn 2 = decidability ⊗ solver-freedom.** The refined index is decidable yet not solver-free;
  pushing the predicate fragment up to discharge real index math (bitwise/modular/stride) only
  deepens the solver dependence. Note the honesty guard: solver-freedom is **not a new 7th axis** —
  it is the `unified` result's "keep the invariant faithfully → external prover" horn, the
  documented founding tradeoff of DML (1998) / Liquid Types (2008) / Flux (2023), now seen at the
  seam. The map gains *resolution*, not a dimension.

## Honest bounds

- N ≥ 6 is a **lower bound**: six gauntlet-confirmed live axes — sound, decidable, concentration,
  API-locality (#5), zero-cost (#6), and AI-legibility (#7, promoted argued→confirmed by the refined
  phase). More may exist; the method (build a dissociating witness, let the cold panel adjudicate) is
  open-ended, not exhausted.
- The #5⟂#7 dissociation **is now witnessed** (refined index = brand-free + legible; slab = brand-free
  + illegible). The refined phase also surfaced that horn 2 un-bundles into decidability + solver-
  freedom — but that face is the original "keep" horn, so it sharpens the map rather than extending N.
- The Lean kernel (`../proof/SeamGap.lean`) sees only whether a precondition factors through a
  Boolean witness — so it certifies *closure* (`spsc`/`branded`/`refined` all close, like SPSC) but is
  blind to *which axis a closing seam spends*. Every column past "concentration" lives in this
  modeling-dictionary, by the same honest caveat the slab's `Vrt` carries.
- One artifact caveat the panel found and we fixed: the reference model's R2 used `bound == len`
  (strictly stronger than safety's `bound ≤ len`); relaxed to `≤` (predicate subtyping). The shipped
  `seam/src/refined.rs` is faithful to the floor it models on this point.
