# The realizability table — the Seam Trilemma is a 3-of-N projection

*Phases D–F. The constructive question the slab opened, the branded index extended, the refined index
closed, and the shared-ownership seam pushed to seven: is the Seam Trilemma the whole story (3 horns),
or a low-dimensional shadow of a larger tension? Answer: it is a **3-of-N projection with N ≥ 7** —
where the recovered axes are known PL boundaries, not novel discoveries (see the meta-pattern below).
Each constructive primitive lit up one more axis by being the **witness that two configurations the
trilemma treated as identical actually differ.***

> **A note on the "#5/#6/#7" shorthand (and a numbering collision).** `#6` (zero-cost) and `#7`
> (AI-legible) are the literal seed-property numbers. The "**#5⟂#7 cell**" shorthand, coined for the
> refined phase, uses `#5` for the *fifth lattice axis* — **API-locality / brand-freedom**, the
> *type-face of seed property #2 (no-seam)*. The shared phase then recovered the *actual* seed
> property #5 — **bare-metal** — as the seventh lattice axis. To avoid the clash: **API-locality is
> referred to by name or as the "#5 cell"; bare-metal is referred to by name (it is seed #5).** Lattice
> numbering ≠ seed numbering; this note is the key.

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
spec) and #3 (derived ownership) by construction (the floor is a fixed checker). The live,
per-primitive axes — note **seed #5 (bare-metal) is no longer uniform**: the shared-ownership seam
pays it:

| Axis | Seed # | What it asks | Folded into the trilemma as |
|---|---|---|---|
| **Sound** | (the wrapper) | Does re-import launder no UB into safe code? | horn 1 |
| **Decidable & proof-free** | #4 | Does the safe fragment need no solver, no hand proof? | horn 2 — *itself a bundle* of **decidability** ⊗ **solver-freedom**, un-bundled by the refined index |
| **Concentration (seam-width)** | #2 (code face) | Does *ordinary code* cross the `unsafe` boundary only O(1)/primitive? | horn 3 |
| **API-locality (brand-freedom)** | #2 (type face) | Does the witness stay out of consumer **type signatures** (no viral `'id`, no forced scope-closure)? | **folded inside "concentration"** — un-bundled by the branded index |
| **Zero-cost erasure** | #6 | Does the witness erase at compile time (no runtime residue)? | **folded inside "concentrated & proof-free"** — un-bundled by the slab |
| **AI-legible** | #7 | Is the witness's *meaning* reconstructible (a readable predicate), not a nominal token? | not in the trilemma at all — surfaced by the branded index, **confirmed independent by the refined index** |
| **Bare-metal (runtime-freedom)** | **#5** | Does sound re-import avoid a *runtime memory manager* (no global allocator / GC)? Operationally: compiles under `#![no_std]` with no `alloc`? | not in the trilemma — surfaced by the shared-ownership seam; **= Rust's `core`/`alloc` boundary (RFC 2480)**, real & independent but pre-existing, *not* a novel dimension |

Two separate "cost" categories sit *outside* the axes and must not be miscounted as horns:
- **TCB axiom** (MMIO): a physical-world fact ("this address really is this register") no checker
  can discharge — a trusted `assume` boundary (seL4/CompCert/Low\*), not a property the seam fails.

## The table

Legend: ✓ holds / aced · ✗ paid or dropped · ◐ conditional · *(plain)* a non-viral ordinary lifetime.

| Primitive | Sound | Decidable | Concentration | **API-locality** | **Zero-cost** | **AI-legible** | **Bare-metal** | What it costs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|---|
| **SPSC ring buffer** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `'a`)* | ✓ | ✓ | ✓ | nothing — the fully-encapsulable corner |
| **MMIO read-to-clear** | ✓ | ✓ | ✓ O(1)¹ | ✓ | ✓ | ✓ | ✓ | a **TCB axiom** (outside the axes) |
| **Generational slab** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Key`)* | **✗** | **✗**² | ✓⁹ | **zero-cost (#6)** — runtime generation+occupancy check |
| **Branded index** | ✓ | ✓ | ✓ O(1) | **✗** *(viral `'id`)* | ◐³ | **✗**⁴ | ✓ | **API-locality (#5)** + deferred zero-cost; brand is nominal |
| **Refined index** | ✓ | ◐⁶ | ✓ O(1) | ✓ *(brand-free; `Refined` escapes)* | ◐⁷ | **✓**⁸ | ✓ | **horn-2 solver-freedom** (SMT in the TCB) + the R2 trade |
| **Shared (Rc)** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Shared`)* | **✗** | ✓ | **✗**¹⁰ | **bare-metal (seed #5)** — needs a global-allocator refcount manager |
| **Type-level index** | ✓ | ◐¹¹ | ✓ O(1) | ✓ *(plain turbofish)* | ✓ | ✓ | ✓ | **eighth-axis probe FAILED** — sound, but the "new axis" is confounded + encoding-relative (see below) |
| **Intrusive list** (branded) | **✗**⁵ | ✓ | ✗ O(accesses) | ✗ | ◐ | ✗ | ✓ | a **horn genuinely drops** — no sound+usable+concentrated decision exists |

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
signature alone. This is the cell-closing witness. ⁹ **bare-metal ✓** — the slab is a fixed
`[Slot; N]` pool, `core`-only, no heap; it is the off-diagonal partner that proves bare-metal ⟂
zero-cost (both pay a runtime cost, only `Shared` needs a manager). ¹⁰ **bare-metal ✗** —
`Shared::new` allocates on the global heap; it is the sole reason the crate pulls in `extern crate
alloc` (compile-enforced: the other six are `core`-only). The axis is real and independent but *not
novel* — it is Rust's `core`/`alloc` boundary (RFC 2480), precisely *unbounded-dynamic-allocation vs
caller-provided-bounded-storage* (a `heapless`-pool `Rc` would be bare-metal but capacity-bounded).
¹¹ **decidable ✓, but pays a recorded `recursion_limit` cost inside horn 2 — *not* a new axis.** The
type-level index proves `I < Len` by trait resolution (`I: Lt<Len>`), which erases (zero-cost ✓),
needs no solver (solver-free, unlike refined), and is sound (sealed `Nat` + the orphan rule block
every forgery — conceded unanimously). Its cost: a deep-but-in-bounds index fails to *check* under the
default `recursion_limit` (E0275 overflow). The eighth-axis claim built on this **FAILED the gauntlet**
(see the dedicated section below): the cost is encoding-relative (binary numerals erase it) and
confounded with a strict expressiveness loss (static-index-only), so it isolates no clean axis.

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
| Bare-metal ⟂ Zero-cost | slab (zero-cost ✗ runtime check, **bare-metal ✓** fixed pool) vs Shared (zero-cost ✗ refcount, **bare-metal ✗** global heap): agree on #6, flip bare-metal. A runtime *check* ≠ a runtime *manager*. | **gauntlet-confirmed** (shared phase): the judge ruled bare-metal a real, independent seventh axis — *but* "reduces-to-X for novelty": X = Rust's `core`/`alloc` boundary (RFC 2480); `heapless::pool` already witnesses the bare-metal⟂capacity trade. **Real and independent, but pre-existing — not a new dimension.** |

**3. The trilemma is exactly ONE forbidden pattern in this space.** {Sound ∧ Decidable ∧
Concentrated} is unachievable *only* when the precondition factors through no available witness — and
the intrusive list is the witness that the forbidden pattern is real (whole-heap aliasing+liveness
escapes every witness vocabulary, so a horn drops). Every other primitive **closes** the three
horns and pays on a *recovered* axis instead. The trilemma never said otherwise — it is a true
shadow; the table is the object casting it.

## The eighth-axis probe — and why it FAILED (the method's STOP)

The first three recovered axes (slab→zero-cost, branded→API-locality, refined→legibility) each came
from a primitive that **cleanly dissociated** one coordinate. The eighth probe — the type-level index,
pitched as recovering **checker-tractability** (does the native checker halt without an arbitrary fuel
knob?) — **did not**, and the cold gauntlet rejected it. This is recorded as a first-class result, not
hidden: it is where the method's own discipline drew a line.

**The seam is sound** (unanimous): `I: Lt<Len>` proves `I::VALUE < Len::VALUE = len`; sealing `Nat`
(`Nat: Sealed`, in-crate only) plus the orphan rule plus `new`'s conservative length check block every
forgery. No UB path. That sub-result stands, and the primitive pays a real `recursion_limit` cost.

**But the eighth *axis* claim failed three convergent ways** — three independent attackers, one judge:

1. **The off-diagonal is confounded by expressiveness.** `get::<I>` takes a turbofish *type*, never a
   runtime `usize`, so the type-level index can only address **compile-time-constant** positions — a
   *strict expressive subset* of the branded and refined indices, whose whole reason for existing
   (`get_unchecked` on ring/arena/slab) is **data-dependent** indexing the seam cannot express. So
   "typed_nat vs refined flips *only* checker-tractability" is false: it silently *also* flips
   expressiveness. You cannot read a one-coordinate independence off two primitives that do not accept
   the same programs. This is the **OFF-DIAGONAL requirement** (the same gotcha Phase E had to honor to
   close #5⟂#7) failing — here unfixably, because carrying a *runtime* index with a *compile-time*
   proof is a contradiction in terms: the expressiveness loss is intrinsic to the witness kind.
2. **The decisive cost is encoding-relative.** The ✗ checker cell is an artifact of **unary Peano**
   (`S<S<…>>`, depth = the index value). **Binary** type-level naturals (typenum's `UInt<U,B>`) make
   resolution **O(log N)** — never approaching `recursion_limit` for realistic indices. The cell
   therefore measures *which numeral library you linked*, not a property of the primitive. A table
   coordinate that flips with an implementation detail is not an axis.
3. **It is mislabeled, and the real concern is pre-existing.** Even the friendly (prior-art) attacker
   granted separability but corrected the name: `Lt` resolution is **linear** in the index — strictly
   *cheaper* than refined's NP-hard QF-LIA — so the cost is **not** "tractability." What
   `recursion_limit`/E0275 actually marks is the absence of a guaranteed **terminating-resolution
   fragment**: exactly GHC's `UndecidableInstances` / the **Paterson conditions** / `-freduction-depth`,
   and the Turing-completeness of trait resolution (Mairson 1990 for the DEXPTIME lineage). A real,
   named, pre-existing PL boundary — but one this primitive cannot witness as a clean lattice axis.

**The honest verdict: a sound seam that pays a recorded `recursion_limit` cost *inside* horn 2, but
recovers no new axis. The lattice stays N ≥ 7.** The value of the probe is the **negative** result:
the method is not a novelty-generating machine. Three primitives isolated axes because their witnesses
dissociated cleanly; the fourth could not, and the off-diagonal discipline said so out loud. A method
that only ever confirms is not measuring anything — this one has a STOP, and the eighth probe found it.

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
  shared│  recovers seed #5 bare-metal ⇒ 3-of-≥7  (runtime-manager axis; ⟂ #6 via slab/heapless)
        │                              (= Rust's core/alloc boundary, RFC 2480 — real but pre-existing)
typednat│  PROBES checker-tractability ⇒ FAILED   (off-diagonal confounded by expressiveness;
        │                              cost encoding-relative unary/binary; mislabeled = terminating-
        │                              resolution, Paterson/UndecidableInstances). N stays ≥ 7. METHOD STOP.
        ▼
Realizability table         N ≥ 7 axes (sound, decidable, concentration, API-locality, #6, #7, bare-metal)
                            — the 8th probe found the method's falsification mode, not an 8th axis
```

Each constructive primitive is the **witness that two trilemma-identical configurations differ** —
the only way to prove a folded axis was ever really there. The slab proved a concentrated seam can
still cost runtime; the branded index proved a concentrated, erasing seam can still cost the type
surface; the refined index proved legibility (#7) is its own axis (the brand-free-yet-legible
off-diagonal the slab-vs-branded pair could not reach) **and** that horn 2 was a bundle of
*decidability* and *solver-freedom*; the shared seam recovered seed #5 (bare-metal), the
runtime-manager axis, distinct from runtime *cost* (#6).

**The honest meta-pattern.** *Every* axis the method recovers turns out to be an **already-named PL
concern**: zero-cost abstraction (#6), generativity/API-locality, refinement-legibility (#7), and now
the `core`/`alloc` bare-metal split (RFC 2480). The constructive phase did not discover new physics.
What it found is that the trilemma's three horns silently *bundle together a set of independently
known PL design axes*, and that one rubric — **"what does the safe witness cost?"** — unifies them.
The contribution is the unification and the method that recovers each axis (build the dissociating
witness, let a cold panel adjudicate), not the novelty of any single axis. The panel **deflated** a
"new dimension" claim twice (solver-freedom, bare-metal → "real, but pre-existing"), and the third time
(checker-tractability) it went further and **rejected the axis outright** — the witness's off-diagonal
was confounded with expressiveness and its cost was encoding-relative. That escalation is the sharpest
form of the result: the seam tension is where *known* axes meet, and the adjudicator that recovers them
is also willing to throw a candidate out. A discovery procedure with no rejection mode discovers
nothing; this one rejected its eighth candidate on the record.

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

- N ≥ 7 is a **lower bound**: seven gauntlet-confirmed live axes — sound, decidable, concentration,
  API-locality, zero-cost (#6), AI-legibility (#7), and bare-metal (#5, recovered by the shared
  phase). More may exist; the method (build a dissociating witness, let the cold panel adjudicate) is
  open-ended, not exhausted. **But "open-ended" is not "everything is an axis":** the eighth probe
  (type-level index → checker-tractability) was built, attacked, and **rejected** — its candidate axis
  is confounded with expressiveness, encoding-relative, and a mislabeled re-name of a pre-existing
  terminating-resolution concern (Paterson / `UndecidableInstances`). N stays **7**, not 8. The probe's
  worth is the falsification: the off-diagonal discipline can and did refuse a witness.
- The #5⟂#7 dissociation **is witnessed** (refined index = brand-free + legible; slab = brand-free +
  illegible). Bare-metal ⟂ #6 **is witnessed** (slab = bare-metal + runtime-cost; Shared = global-heap
  + runtime-cost). The refined phase surfaced that horn 2 un-bundles into decidability + solver-freedom
  — that face is the original "keep" horn, so it sharpens the map rather than extending N.
- **Novelty is bounded, and that is the honest headline.** Two of the recovered axes (solver-freedom,
  bare-metal) the cold panel deflated to "real but *pre-existing*" — they are named PL boundaries
  (Liquid-Types' solver-in-TCB; RFC 2480's `core`/`alloc`). The lattice's value is *unification under
  one cost rubric*, not novel dimensions. Treat "N ≥ 7" as "at least seven *known* PL axes are all
  faces of the seam tension," not "seven new discoveries."
- The Lean kernel (`../proof/SeamGap.lean`) sees only whether a precondition factors through a
  Boolean witness — so it certifies *closure* (`spsc`/`branded`/`refined` all close, like SPSC) but is
  blind to *which axis a closing seam spends*. Every column past "concentration" lives in this
  modeling-dictionary, by the same honest caveat the slab's `Vrt` carries.
- One artifact caveat the panel found and we fixed: the reference model's R2 used `bound == len`
  (strictly stronger than safety's `bound ≤ len`); relaxed to `≤` (predicate subtyping). The shipped
  `seam/src/refined.rs` is faithful to the floor it models on this point.
