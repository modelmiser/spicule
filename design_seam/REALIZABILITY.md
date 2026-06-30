# The realizability table — the Seam Trilemma is a 3-of-N projection

*Phases D–R. The constructive question the slab opened, the branded index extended, the refined index
closed, the shared-ownership seam pushed to seven, the take-and-replace seam to eight, and the
drain-and-yield seam to nine: is the Seam Trilemma the whole story (3 horns), or a low-dimensional
shadow of a larger tension? Answer: it is a **3-of-N projection with N ≥ 9** — where the recovered axes
are known PL boundaries, not novel discoveries (see the meta-pattern below). Each constructive primitive
lit up one more axis by being the **witness that two configurations the trilemma treated as identical
actually differ** — and the method's discrimination is now demonstrated in **nine** modes, not just
accumulation: it **confirmed** axes (cleanly deflated to pre-existing), **sharpened** a horn without
extending N (solver-freedom), **rejected** a confounded candidate (checker-tractability → not an axis),
found one **real but structurally inseparable** (move-stability → a distinct danger with no isolable
cost), and — the capstone — found a new danger class that **projects entirely onto the existing basis**
(validity → no tenth axis), which together with the recurrence of inseparability (validity is a second
inseparable danger) is the evidence that **the cost-basis is CLOSED** (see "The cost-basis is closed",
below). N stays 9; the closed basis is the conclusion of the constructive arc — **and three further primitives
generalized it: a constant-time secret showed closure is *per domain* (information-flow is a *different*
soundness domain with its own basis); a capability-gated declassification showed that *which seams close*
depends on the *floor-domain match* (Rust's affine floor closes memory-safety seams but *no*
information-flow seam, because affinity bounds resource *uses*, not information *bits*); and a type-level
lock ordering, probing a *third* domain (liveness/deadlock), showed the floor-domain match is *graded —
full / partial / none*: it lands *partway*, the static lock-order fragment closing while the dynamic and
per-thread fragments escape, so the method now discriminates in **eight** modes (adding *partial-match*).
And a sixteenth — a *runtime* lock registry (the floor-*upgrade* probe) — showed that **upgrading the
floor is NON-MONOTONE**: it fixed the static seam's *coverage* gap (dynamic orderings now close) and paid
the predicted price (bare-metal #5, runtime-check #6, a prevention→detection modality), yet **granularity
recurred one rung finer** — the held-set is keyed on the OS *thread* but async *tasks* are the unit of
control, so work-stealing migration launders an undetected inversion. The *granularity* axis is a
containment-scope **ladder** (token < thread < task < …) that no ambient Rust floor tops out. And a
seventeenth — a *must-run cleanup guard* — probed **breadth-wise** (a new *precondition*, "this finalizer
MUST run", not a new domain or ladder rung): it **breaks** on a clean **none** grade, sharpening the
match-unit from *domain* to *precondition* (the *same* liveness domain grades deadlock-freedom *partial*
but must-run-cleanup *none*) and adding the **ninth** method mode — *wrong-structural-logic*: the floor is
**affine** ("at most once") where the precondition needs **linear** ("exactly once"), so `mem::forget`
skips the finalizer and *no decidable price* buys it back (Boats' scoped-task trilemma, linear horn
rejected; the leakpocalypse) — distinct from a domain-mismatch. And an eighteenth — a *session-typed
channel* (a `Chan<P>` typestate walk) — was the **first** out-of-(memory-safety) probe to **hold with a
cost** rather than break: **SEAM-COSTS [horn 3]**, because protocol fidelity is a *composite* matched
**component-wise** (the components orthogonal — `mem::forget` cannot shorten `'a` to start a second
session, so completion cannot infect safety): the *order-safety* half **closes** (affine typestate, no
`root`-hole) while the *completion* half is an undischargeable horn-3 *concentration* cost (the #17
linearity gap, recurring at every `?`/early-exit). The illumination: the **same** `mem::forget` gap that
**broke** #17's horn 1 only **costs** here — completion was #17's *whole* soundness promise but is *one
component beside a sound core* here, so **which horn a linearity gap threatens depends on how the rule
*packages* it**, not on the gap. (Deflations, accepted: a single-handle *walk* not a binary session
[no duality in `core`/no-`unsafe`, E0499]; "usable" only for infallible straight-line protocols; and the
split is documented folklore — Jespersen–Munksgaard–Larsen 2015, ECOOP 2022 *Stay Safe Under Panic* — so
the contribution is *locating* it on the lattice.) N stays 9; the five broken seams are unchanged — this
one holds.** And a nineteenth — a *value-owning scope-restore guard* (the real `scopeguard::guard`) —
turned that "**which horn depends on packaging**" observation from an incidental cross-primitive coincidence
into a **deliberate, controlled probe**: it carries #17's *identical* un-compellable-`Drop` danger but wraps
it in #18's structure (a sound affine-ownership *core* beside which the restore is a side-obligation), and
the cold panel (r26) ruled **SEAM-COSTS [horn 3]** — the *same* `mem::forget` gap that **broke** #17's horn 1
only **costs** here. **The dial is real — but the panel BOUNDED the thesis, all accepted:** the move is
*confounded* (#19 also adds the value-ownership core #17 lacked, so the value-ownership model co-varies — a
better-controlled probe than the incidental #17-vs-#18 pair, not a single-variable manipulation); the horn
is *forceable* (the same skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces
**horn-1 UB** — the #9 `take` danger — and `Pin`'s drop guarantee plus the `thread::scope` restructure
exhibit horns that **cannot** be repackaged, so *which* horn depends on **where safety rests** — partly
danger-*intrinsic*); and it is *not novel* (`JoinGuard` broke horn 1, `scopeguard` costs horn 3, concluded
in RFC 1066 in 2015 — so the contribution is the *method* and *locating the dial*, **no new mode**). The
refined finding: **which horn a danger threatens is a design choice *within a structural envelope* set by
whether soundness can be DECOUPLED from the un-compellable closure** (owned ⇒ costs; borrowed-across-unwind /
`Pin` / `thread::scope` ⇒ forces). Phase Q said the horn moves; Phase R says it moves *within a fence, and
names the fence*. N stays 9; the five broken seams unchanged — this one holds too.** (Phases L–R.)*

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
per-primitive axes — note **bare-metal (#5), unwind-safety (8th), and leak-freedom (9th) are not
uniform**: the shared-ownership seam pays the first, take-and-replace the second, drain-and-yield the third:

| Axis | Seed # | What it asks | Folded into the trilemma as |
|---|---|---|---|
| **Sound** | (the wrapper) | Does re-import launder no UB into safe code? | horn 1 |
| **Decidable & proof-free** | #4 | Does the safe fragment need no solver, no hand proof? | horn 2 — *itself a bundle* of **decidability** ⊗ **solver-freedom**, un-bundled by the refined index |
| **Concentration (seam-width)** | #2 (code face) | Does *ordinary code* cross the `unsafe` boundary only O(1)/primitive? | horn 3 |
| **API-locality (brand-freedom)** | #2 (type face) | Does the witness stay out of consumer **type signatures** (no viral `'id`, no forced scope-closure)? | **folded inside "concentration"** — un-bundled by the branded index |
| **Zero-cost erasure** | #6 | Does the witness erase at compile time (no runtime residue)? | **folded inside "concentrated & proof-free"** — un-bundled by the slab |
| **AI-legible** | #7 | Is the witness's *meaning* reconstructible (a readable predicate), not a nominal token? | not in the trilemma at all — surfaced by the branded index, **confirmed independent by the refined index** |
| **Bare-metal (runtime-freedom)** | **#5** | Does sound re-import avoid a *runtime memory manager* (no global allocator / GC)? Operationally: compiles under `#![no_std]` with no `alloc`? | not in the trilemma — surfaced by the shared-ownership seam; **= Rust's `core`/`alloc` boundary (RFC 2480)**, real & independent but pre-existing, *not* a novel dimension |
| **Unwind-safety (exception-safety-guarantee-level)** | — | Does sound re-import survive a `panic`/unwind *through a transiently-broken invariant* without downgrading the basic guarantee? | not in the trilemma nor the seed — surfaced by the take-and-replace seam; **= Abrahams' basic/strong/nothrow guarantees** (C++ `noexcept`, Rust `UnwindSafe`/`catch_unwind`/`Mutex` poisoning, `take_mut`↔`replace_with`), real & independent but pre-existing. **Carries a *discharge menu*** (abort / placeholder-repair / poisoning), not a fixed cost |
| **Leak-freedom** | — | Does sound re-import survive the witness being *leaked* (its `Drop` **never** running, e.g. `mem::forget`)? | not in the trilemma nor the seed — surfaced by the drain-and-yield seam; **= Rust's leakpocalypse** (`mem::forget`-is-safe; leak amplification; `Vec::drain`; RFC 1066 / `thread::scoped` removal; linear-vs-affine types), real & independent but pre-existing. **Distinct from unwind-safety** (leak = `Drop` never runs; unwind = `Drop` runs at a bad time). The *thinnest* axis — cost invisible in non-leak code |

Two separate "cost" categories sit *outside* the axes and must not be miscounted as horns:
- **TCB axiom** (MMIO): a physical-world fact ("this address really is this register") no checker
  can discharge — a trusted `assume` boundary (seL4/CompCert/Low\*), not a property the seam fails.

## The table

Legend: ✓ holds / aced · ✗ paid or dropped · ◐ conditional · *(plain)* a non-viral ordinary lifetime.

| Primitive | Sound | Decidable | Concentration | **API-locality** | **Zero-cost** | **AI-legible** | **Bare-metal** | **Unwind-safe** | **Leak-safe** | What it costs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|:--:|---|
| **SPSC ring buffer** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `'a`)* | ✓ | ✓ | ✓ | ✓ | ✓ | nothing — the fully-encapsulable corner |
| **MMIO read-to-clear** | ✓ | ✓ | ✓ O(1)¹ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | a **TCB axiom** (outside the axes) |
| **Generational slab** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Key`)* | **✗** | **✗**² | ✓⁹ | ✓ | ✓ | **zero-cost (#6)** — runtime generation+occupancy check |
| **Branded index** | ✓ | ✓ | ✓ O(1) | **✗** *(viral `'id`)* | ◐³ | **✗**⁴ | ✓ | ✓ | ✓ | **API-locality (#5)** + deferred zero-cost; brand is nominal |
| **Refined index** | ✓ | ◐⁶ | ✓ O(1) | ✓ *(brand-free; `Refined` escapes)* | ◐⁷ | **✓**⁸ | ✓ | ✓ | ✓ | **horn-2 solver-freedom** (SMT in the TCB) + the R2 trade |
| **Shared (Rc)** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Shared`)* | **✗** | ✓ | **✗**¹⁰ | ✓ | ✓ | **bare-metal (seed #5)** — needs a global-allocator refcount manager |
| **Type-level index** | ✓ | ◐¹¹ | ✓ O(1) | ✓ *(plain turbofish)* | ✓ | ✓ | ✓ | ✓ | ✓ | **eighth-axis probe FAILED** — sound, but the "new axis" is confounded + encoding-relative (see below) |
| **Take-and-replace** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `take`)* | ◐¹² | ✓ | ✓ | **◐**¹³ | ✓ | **unwind-safety (8th axis)** — sound via an abort-bomb; basic guarantee downgraded |
| **Drain-and-yield** | ✓ | ✓ | ✓ O(1) | ✓ *(plain `Drain<'a>`)* | ✓ | ✓ | ✓ | ✓ | **◐**¹⁴ | **leak-freedom (9th axis)** — sound via pre-pessimization; a leaked guard loses the tail |
| **Self-ref cell** | ✓ | ✓ | ✓ O(1) | **✗**¹⁵ *(viral, `&mut`-revoking `Pin`)* | ✓ | ✓ | ✓ | ✓ | ✓ | **move-stability — REAL but INSEPARABLE**, *not* a new axis (N stays 9); cost = *immovability*, non-uniform (stack-pin→API-locality, heap-pin→bare-metal) |
| **Validity (bytes→enum)** | ✓ | ✓ | ✓ O(1) | ◐¹⁶ | ◐¹⁶ | ✓ | ✓ | ✓ | ✓ | **validity — INSEPARABLE (2nd), and the CLOSED-BASIS proof**: a new danger class adds *no tenth axis* (unanimous); its own cost is non-uniform (per-read→#6, zero-copy view→API-locality) |
| **Intrusive list** (branded) | **✗**⁵ | ✓ | ✗ O(accesses) | ✗ | ◐ | ✗ | ✓ | ✓ | ✓ | a **horn genuinely drops** — no sound+usable+concentrated decision exists |

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
¹² **zero-cost ◐ — runtime-clean, codegen-dented.** The abort-bomb is a ZST whose `Drop` never runs on
the happy path (it is `mem::forget`-ten), so there is *no runtime residue*. But the live bomb forces an
unwind landing pad to exist and acts as an **optimization barrier** (the cold panel's recorded
qualification), so erasure is perfect at the runtime level and mildly imperfect at codegen. ¹³ **the
eighth axis — unwind-safety ◐.** Sound re-import of a transient-invariant-breaking op (move-out/move-
back) survives a `panic` through the gap only by paying: this seam installs an **abort-bomb** (a mid-
take panic aborts before the broken slot is observed), which is sound but **downgrades the basic
exception-safety guarantee to "abort"** (non-`catch_unwind`-recoverable). The cold panel confirmed this
a **real, independent eighth axis (N ≥ 8)** and a *clean* off-diagonal (vs SPSC: splits on the
guarantee-level alone), deflated to **pre-existing** (Abrahams basic/strong/nothrow; C++ `noexcept`;
`UnwindSafe`/`catch_unwind`; `Mutex` poisoning; RFC #1508 *rejected* this exact `take_mut` from std on
this very cost). Mandatory relabel: the axis is the **guarantee-level**, and abort is *this seam's*
discharge of it — a `replace_with` sentinel discharges the same obligation soundly-and-recoverably by
paying **API-surface** instead, so the axis carries a **discharge menu**, not a fixed cost. ¹⁴ **the
ninth axis — leak-freedom ◐.** Sound re-import of a guard whose `Drop` does load-bearing cleanup
survives the guard being *leaked* (`mem::forget` is safe — the Rust leakpocalypse) only by paying: this
seam **pre-pessimizes** (sets the buffer length to 0 *before* yielding), so a forgotten guard leaves a
valid empty buffer (the un-yielded tail leaks, no double-free). The cold panel confirmed it a **real,
independent ninth axis (N ≥ 9)** and a *clean* off-diagonal vs SPSC, **distinct from unwind-safety**
(the off-diagonal is populated both ways — take is unwind-sensitive/leak-safe, drain is
leak-sensitive/unwind-safe). Deflated to **pre-existing** (leakpocalypse / PPYP / leak amplification;
RFC 1066; `thread::scoped`/`JoinGuard` removal; linear-vs-affine types). Recorded nuance: the
**thinnest, most-deflatable** axis — its cost (leaked-tail data loss) is invisible in all non-leak code
— but a genuinely populated off-diagonal, and it aces API-locality precisely *because* the guard owns
its data (a free-standing `Drain<'a>`, no closure), unlike the scoped-thread leak-freedom witness.
¹⁵ **API-locality ✗, and the *cost itself* is the find.** The self-ref cell's `Pin<&Self>`/`Pin<&mut Self>`
is the viral-*restrictive* wrapper (it infects every receiver **and revokes `&mut`** — that revocation
is the load-bearing soundness mechanism), so it flips API-locality. But the deeper finding (see the
inseparable-axis section below) is that move-stability is a **distinct danger with no isolable cost** —
its real price is *immovability* (heavier than API-locality), and it is *non-uniform* across witnesses
(stack-pin spends API-locality, `Box::pin` spends bare-metal). So this row does **not** add a 10th axis
column; it adds a new *category*. Soundness held (the move is unconstructible: `Pin::get_mut` needs
`Unpin`, the cell is `!Unpin`).
¹⁶ **API-locality ◐ / zero-cost ◐ — non-uniform, hence inseparable; and the closed-basis proof.** A
`transmute`-after-set-membership-check read of a `#[repr(u8)]` enum byte. Sound (set membership = the
validity predicate *exactly*; moving from a *range* check, unsound for non-contiguous discriminants, was
the fix two gauntlets forced). Its cost is **witness-non-uniform**, the move-stability signature: a
single `tag_at` is a per-read runtime check (**zero-cost #6**), but the zero-copy `&[u8] → &[Tag]` view
that restores concentration needs either eager O(n) validation (#6) or a **sealed constructor**
(**API-locality**). So validity is **inseparable** — a *second* example, making inseparability a
recurring category. The unanimous meta-ruling: it adds **no tenth axis**; every witness spends a
pre-existing coordinate. Caveats: sound only in the *fieldless single-byte corner* (data-carrying /
`repr(C)` enums need a recursive `is_bit_valid`), and wholly non-novel (`num_enum` / `bytemuck` /
`zerocopy` + refinement types).

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
| Unwind-safety ⟂ all seven | **take-and-replace (unwind-safe ◐, all seven ✓) vs SPSC (all eight ✓): agree on every named axis, flip unwind-safety alone.** The cleanest off-diagonal in the set — and the panel explicitly contrasted it with the *failed* typed_nat probe (which secretly co-flipped expressiveness). | **gauntlet-confirmed** (take phase): the judge ruled unwind-safety a real, independent **eighth** axis (N ≥ 8), deflated to **pre-existing** (Abrahams basic/strong/nothrow; `noexcept`; `UnwindSafe`; `Mutex` poisoning). Mandatory relabel: the axis is the **guarantee-level**; abort is one **discharge** of it (see the discharge-menu note below). |
| Leak-freedom ⟂ unwind-safety | **take-and-replace (unwind-sensitive ◐, leak-safe ✓) vs drain-and-yield (leak-sensitive ◐, unwind-safe ✓): the off-diagonal is populated *both* directions.** "`Drop` never runs" (leak) vs "`Drop` runs at a bad time" (unwind) are independently witnessed. And drain vs SPSC flips leak-freedom alone (all eight else ✓). | **gauntlet-confirmed** (drain phase): the judge ruled leak-freedom a real, independent **ninth** axis (N ≥ 9), **distinct from unwind-safety — do not merge**, deflated to **pre-existing** (leakpocalypse / PPYP / leak amplification; RFC 1066). No confound: aces API-locality *because* the guard owns its data (free-standing, no closure). The **thinnest** axis (cost invisible in non-leak code), but a genuinely populated off-diagonal. |

**3. The trilemma is exactly ONE forbidden pattern in this space.** {Sound ∧ Decidable ∧
Concentrated} is unachievable *only* when the precondition factors through no available witness — and
the intrusive list is the witness that the forbidden pattern is real (whole-heap aliasing+liveness
escapes every witness vocabulary, so a horn drops). Every other primitive **closes** the three
horns and pays on a *recovered* axis instead. The trilemma never said otherwise — it is a true
shadow; the table is the object casting it.

## The eighth axis took TWO probes — one rejected, one confirmed (the method's discrimination)

The eighth axis was reached on the *second* try, and the contrast between the two attempts is the
sharpest evidence the method measures something real: **it rejected the confounded candidate and
confirmed the clean one.**

- **Probe 1 — checker-tractability (type-level index): REJECTED.** Pitched as "does the native checker
  halt without an arbitrary fuel knob?" The seam is sound, but the candidate axis is confounded
  (details below): it does *not* isolate one coordinate, so N stayed 7.
- **Probe 2 — unwind-safety (take-and-replace): CONFIRMED, N ≥ 8.** A different danger (*time*, not
  bounds): moving a value out of a `&mut T` and back leaves a duplicate in the gap, and an unwinding
  panic drags the caller through it into a double-drop. The sound witness is an **abort-bomb**
  (`take_mut`'s pattern): a mid-take panic aborts before the broken slot is observed. This seam **aces
  all seven prior axes and pays on unwind-safety alone** — the clean off-diagonal vs SPSC the panel
  demanded. The judge ruled it a real, independent eighth axis, deflated to pre-existing (Abrahams;
  `noexcept`; `UnwindSafe`; `Mutex` poisoning; the `take_mut`↔`replace_with` lineage; RFC #1508 rejected
  this exact primitive from std on this very cost). **Mandatory relabel:** the axis is the
  *exception-safety-guarantee-level*, and abort is merely *this seam's* discharge of it — a
  `replace_with` sentinel discharges the same obligation soundly-and-recoverably by paying API-surface
  instead. So the axis carries a **discharge menu**, not a fixed cost (see "Structure among the axes").

### Probe 1 in detail — why checker-tractability FAILED (the method's STOP)

The first three recovered axes (slab→zero-cost, branded→API-locality, refined→legibility) each came
from a primitive that **cleanly dissociated** one coordinate. The first eighth-axis probe — the
type-level index, pitched as recovering **checker-tractability** (does the native checker halt without
an arbitrary fuel knob?) — **did not**, and the cold gauntlet rejected it. This is recorded as a
first-class result, not hidden: it is where the method's own discipline drew a line.

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

**The honest verdict on probe 1: a sound seam that pays a recorded `recursion_limit` cost *inside* horn
2, but recovers no new axis.** The value of the *negative* result: the method is not a
novelty-generating machine. Three primitives isolated axes because their witnesses dissociated cleanly;
this one could not, and the off-diagonal discipline said so out loud. A method that only ever confirms
is not measuring anything — this one has a STOP. Probe 2 (take-and-replace, above) then supplied the
*clean* off-diagonal probe 1 lacked, and the panel confirmed unwind-safety as the eighth axis — so the
two probes together show both halves of a working instrument: it **rejects** the confounded candidate
and **confirms** the clean one.

## The fourth verdict: a *real but structurally inseparable* axis (move-stability)

Three verdict modes were on the table — **confirm** (a clean axis, deflated to pre-existing),
**sharpen** (a sub-face, not a new axis), **reject** (a confounded non-axis). The self-referential-cell
phase found a **fourth**: an axis that is a **genuinely distinct danger** yet has **no isolable cost**.

The **self-ref cell** holds a pointer into itself; *moving* it dangles that pointer — a danger no other
axis touches (none concerns a value's *address*). The seam is sound: `Pin` over a `!Unpin` cell makes
the post-`init` move unconstructible in safe code (every move path needs `Unpin`; `PhantomPinned` denies
it). So the danger is real, distinct, and soundly handled. **But it is not a clean tenth axis**, and the
cold panel ruled — all five attackers converging — exactly *why*:

- **It does not ace API-locality**, so the cost cannot be isolated. `Pin<&Self>` is the
  viral-*restrictive* wrapper (it infects every receiver **and revokes `&mut`**), and that revocation
  *is* the soundness mechanism — you cannot have the safety without the type-surface cost.
- **Its cost is *non-uniform across witnesses*** — stack-pin (`pin!`) spends **API-locality**, `Box::pin`
  spends **bare-metal**, a hypothetical `?Move` immovable type would spend an ambient bound. One danger,
  several different costs ⇒ not one mapped axis "wearing a hat" (so it is *not* the reduction the
  `typed_nat` probe was either).
- **So the honest cost is *immovability* — the revocation of the move/return/store capability**, which
  is distinct from and *heavier* than API-locality (a merely leaked type surface). Move-stability funds
  itself by *being* immovable; there is no separate coordinate to read.

Two caveats the panel insisted on, both now recorded: (a) the inseparability is **contingent, not
necessary** — RFC 2349's *rejected* `?Move`/`Move` immovable types *were* a designed free-standing
witness; Rust **chose** hookless `memcpy` moves + a library `Pin`, so "no free-standing witness can
exist" overstates it. (b) The crisp `core`-only reading is the isolation of a **non-composable**
single-frame toy; any real intrusive node / `Future` / slab entry forces `unsafe` pin-projection or
`Box::pin`. **N stays 9.** The find is the *category*: a real danger with no clean coordinate, only a
menu of *other* axes to spend. **And it is not a one-off** — the validity seam (next section) is a
*second* inseparable danger (bit-level validity, cost non-uniform: per-read #6 vs zero-copy-view
API-locality), so "inseparable" is a recurring class. Two members so far, both about a property the
witness can discharge in more than one place: an *address* (move-stability) and a *bit pattern*
(validity).

## The cost-basis is CLOSED — the constructive arc's conclusion

The validity seam (`seam/src/validity.rs`) was built not to hunt a tenth axis but to ask the question
all the previous probes were circling: **is the cost-basis closed?** It re-imports a danger no prior
primitive touched — a **validity invariant** (a `#[repr(u8)]` enum byte that must be a valid
discriminant, UB even without a dereference). A genuinely new danger class. And the cold panel's
**unanimous** ruling (across three gauntlets — two of which found real spec bugs first, the adversary
earning its keep to the last) is the headline:

> **A new danger class added NO new axis. Every witness for it spends a *pre-existing* coordinate.**
> *"What does the safe witness cost?"* has **finitely many answers** — the nine clean axes, plus the
> inseparable category. The lattice's cost-basis is **closed**.

Concretely, validity's cost is **(3) inseparable** (the panels split (2)-projects-onto-#6 vs
(3)-inseparable, and the more careful reading won): a single checked read spends **zero-cost #6** (a
runtime tag check, the slab's axis via a different predicate), but the zero-copy `&[u8] → &[Tag]` view —
the real reason systems code transmutes byte buffers — spends either eager #6 or a sealed constructor's
**API-locality**. Either way the cost lands on coordinates *already on the map*. This is the **dual** of
the `get_unchecked` family: there *one* danger (out-of-bounds) had *many* witnesses spending *different*
axes (branded → API-locality, refined → solver-freedom, typed_nat → confounded, slab → #6); here *many*
dangers (stale-key, bit-validity) converge on the *same* axis (#6). Read together, the two directions
say the cost-space is a small fixed basis that dangers *project onto*, not an open-ended list of
discoveries.

So the constructive arc concludes: the Seam Trilemma is a **3-of-N projection of a closed object** —
**nine clean axes** (sound, decidable, concentration, API-locality, zero-cost, AI-legibility,
bare-metal, unwind-safety, leak-freedom), **plus an inseparable category** (move-stability, validity) of
real dangers with no clean coordinate, **plus the two outside-the-axes categories** (the MMIO TCB axiom;
the intrusive list's genuine horn-drop). Every memory-safety danger probed since lands in one of these.
The honest bound stays a *lower* bound (a future danger could still surprise), but the evidence — a
brand-new danger class projecting cleanly onto the basis, and inseparability recurring rather than
appearing once — is what closure looks like.

### …but closed PER DOMAIN — the information-flow probe (the arc's generalization)

The closure is for **memory safety** (can safe code reach UB?). One further primitive tested whether it
is *absolute*: a **constant-time secret** (`seam/src/secret.rs`), re-importing a danger that is **not UB**
— an information-flow / **timing** side-channel (branching on a secret leaks it). The cold panel's ruling
**generalizes the whole result**:

> **Information-flow is *out of frame* — a different soundness domain, with its own basis. The closed
> cost-basis is closed *per domain*.** The Seam Trilemma and the cost-basis are **domain-parameterized**:
> memory safety is one domain; information flow is another, and the build-and-break *method* transfers to
> it (its very first IFC seam reproduced the trilemma and surfaced a new axis), but the *coordinates* are
> the new domain's, not these nine.

Two confirmations make it concrete. (1) The secret seam **BREAKS [horn 1] in the IFC sense** — the exact
analog of the intrusive list's memory-safety horn-drop: `ct_eq(a,b).expose_bool()` then `if b` leaks a
secret relationship through control flow, *declassification is etiquette not type-structure*, and closing
it needs whole-program **undecidable** IFC — so a *concentrated, decidable* seam cannot soundly enforce
non-interference (Horn 2 passes only because the strong property is never checked, a syntactic proxy).
**The trilemma recurs, sharper.** (2) The IFC domain's basis has genuinely *new* axes with **no
memory-safety analog** — **declassification discipline** (which / where / how-often a secret-derived bit
may re-enter control flow), plus termination and concurrency channels. (Honest correction the panel
forced: an earlier draft called the trust-the-codegen assumption "MMIO-shaped / irreducible"; it is in
fact **dischargeable** — CompCert-CT / Jasmin / FaCT formally preserve constant-time — a *contingent*
artifact of stock `rustc`/LLVM, a different category than MMIO's permanent axiom.)

#### The deepest refinement: a closing seam exists only when the FLOOR matches the DOMAIN

A second IFC primitive (`seam/src/labeled.rs`, capability-gated declassification) probed the natural
follow-up: does the IFC domain have a *closing* seam (the IFC analog of SPSC/the slab), confirming a real
cost-basis? It **also BREAKS [horn 1], unanimously** — and the *reason* is the project's deepest result.
To *compute* on a secret, `map(self, f: FnOnce(T) → U)` must hand the closure the **bare plaintext**, and
a captured-`&mut` closure exfiltrates it (`s.map(move |x| sink.push(x))`) — cap-free, no `declassify`
site: an uncapped exit, **unpatchable without an effect system Rust's floor disclaims**. And the affine
"budget" counts *uses-per-cap*, not caps-*minted* (a loop mints N) nor **bits** (one cap leaks a megabyte)
— so what it enforces is release *location* (Sabelfeld–Sands' *where/when*), which **projects onto the
memory-safety affine axis**; the genuinely-IFC coordinate is the *amount* (**quantitative information
flow** — Smith min-entropy / g-leakage), and **an affine floor cannot bound bits, only uses**.

So both IFC seams break, for one structural reason: **Rust's affine/borrow floor carries *none* of the
IFC precondition.** It is *matched* to memory safety — where SPSC closes *free*, the floor's affinity/NLL
carrying the precondition, and the basis is closed — and *mismatched* to information flow, where **no seam
closes**: there is no IFC SPSC-analog, the only enforceable cost is a memory-safety axis in disguise, and
the real coordinate (quantitative leakage) needs a *quantitative / effect-tracking* floor (Jif, FlowCaml,
QIF). So the deepest form of the result is layered:

> **The Seam Trilemma is domain-general** (it recurs in every soundness domain). **The cost-basis is
> per-domain** (memory safety has its nine closed axes; information flow has its own — declassification
> dimensions, quantitative leakage). **And *which seams close* depends on the floor-domain match** — a
> floor closes a domain's seams only when it carries that domain's precondition. Rust's affine floor
> closes memory-safety seams and *no* information-flow seam. Map a domain by first asking: *is this floor
> the right floor for it?*

Memory safety is mapped here (basis closed, on a matched floor). Information flow is sketched — a real,
distinct domain whose seams *all break on Rust's floor*, pointing at the floor it would need instead.

#### The match is GRADED — a third domain (liveness) lands *partway*

The IFC probes gave the *negative* half of the floor-domain match (no seam closes). A fifteenth primitive
(`seam/src/ordered_locks.rs`, type-level lock ordering) tested the **positive** half in a **third
domain — liveness / progress**, whose danger is **deadlock** (neither UB nor a leak: a deadlocked program
is memory-safe and confidential). The witness is a type-level lock *rank* `L` plus an affine `Floor<F>`
token, with `with` bounded `L: Below<F>` — so a lock-order inversion *should* be unconstructible, the
floor (a type-level order) seemingly carrying the precondition (an acyclic global order). The cold panel
ruled **SEAM-BREAKS [horn 1]**, and the break is the richer result: it shows the floor-domain match is
**not binary but graded**, because a floor can *under-carry* a domain's precondition along **two
independent axes**:

> **Granularity.** `Floor::root` is a re-entrant safe call, so two sibling roots mint two independent
> `Floor<TopRank>` lineages and safe code types a low-then-high inversion (it compiles). The discipline is
> *per-token*; the precondition is *per-thread-global*. Rust's floor has **no ambient per-thread one-shot
> resource** — so even the *static* fragment does not close without bolting on a thread-local consumed
> capability the language does not provide.
>
> **Coverage (static vs dynamic).** Even granting that fix, the canonical deadlock-free idioms —
> bank-transfer `min(i,j)` ordering, hand-over-hand list/tree coupling, address-ordered bucket locks,
> IRQ/cross-stack ordering — are **inexpressible** (same static rank + irreflexive `Below` + compile-time
> order). The **dynamic-acyclicity** coordinate (what `lockdep` checks at runtime) **escapes the static
> floor**, *exactly as IFC's quantitative coordinate escaped the affine floor*. Prior art forces this:
> it is **Boyapati–Lee–Rinard OOPSLA'02 minus their dynamic tree-order extension**, and **Kobayashi's
> natural-number levels** — both abandoned pure static levels for precisely this reason.

So the floor-domain match has **three grades**: **full** (memory safety on the affine floor — basis
closed), **partial** (liveness on the static-type-order floor — the static distinct-rank single-stack
fragment carried, the granularity + dynamic fragments escaped), **none** (information flow on the affine
floor — both seams break). What closes here is **lock-order-*inversion* freedom**, never "deadlock-freedom"
(condvar / channel / join cycles remain). And the affine floor gets *no* special credit: the order-carrying
floor predates Rust's affinity by two decades in two non-affine settings (BLR/Java, Kobayashi/π-calculus);
affinity buys only token non-duplication — and, per the granularity break, not even that.

#### The floor-upgrade is NON-MONOTONE — a sixteenth primitive walks one rung up the granularity ladder

If the match is graded by *how much of the precondition the floor carries*, the falsifiable next step is to
**upgrade the floor** and watch the match lift. A sixteenth primitive (`seam/src/lock_registry.rs`, feature
`lockdep`) keeps the liveness domain and the same danger but swaps the static type-level order for a
**runtime** one: a `thread_local!` held-set of runtime ranks, checked on each acquire (panicking on a
detected inversion). The cold panel (r22) ruled **SEAM-BREAKS [horn 1]** again — and the *way* it breaks is
the result: **upgrading the floor is non-monotone.**

> **Coverage lifts, at the predicted price.** Runtime ranks express the dynamic idioms the static seam
> could not — bank-transfer `min(i,j)`, hand-over-hand coupling — so the *coverage* fragment closes. But it
> is **bought, not free**: a runtime check per acquire (**zero-cost #6** — the slab's place in a new
> domain), OS thread-local storage (**bare-metal #5** — it literally *cannot be `no_std`*, which is why it
> is feature-gated, making the cost visible in the build graph), and a **modality shift** from static
> *prevention* (the inversion was a compile error) to runtime *detection* (it now compiles and is trapped
> only on paths that execute — soundness is path-coverage-dependent). The graded match is thus **coupled to
> the closed cost-basis**: you pay #5 + #6 + a modality downgrade to lift it.
>
> **Granularity recurs one rung finer.** The held-set is keyed on the OS *thread*, but the unit of
> sequential control in async is the *task*. A guard held across `.await` migrates (work-stealing) to a
> thread whose held-set is empty, so a lower rank is accepted with **no panic** — an executed, undetected
> inversion (the registry is *thread-blind*; verified by a two-thread reproduction). A single-threaded
> executor *false-panics* on two independent tasks sharing one thread; `!Send` blocks the migration but
> leaves the false-panic. So the **granularity axis is a containment-scope *ladder*** — *token < thread <
> task < …* — and each ambient Rust floor reaches exactly one rung (affine token #15, thread-local #16);
> there is no ambient *task-local* (async is a library). Prior art forecloses the static-rank design
> outright: **FreeBSD WITNESS / lockdep / Abseil** all abandoned static ranks for dynamically-*learned*
> acquires-before graphs, because hand-assigned global ranks do not compose.

So a partial match is **not lifted to full by upgrading the floor**: the upgrade trades one gap for a finer
one (coverage fixed, granularity recurred), spends cost-basis axes doing it, and the granularity dimension
turns out to be an *unbounded ladder* no ambient floor tops out. The match stays **partial**.

#### The match-unit is the PRECONDITION, not the domain — a seventeenth primitive breaks on *wrong structural logic*

The two liveness probes above both attacked *deadlock-freedom*. A seventeenth primitive
(`seam/src/linear.rs`, a `MustRun<F: FnOnce()>` cleanup guard) stayed in the **liveness** domain but took a
**different precondition** — *must-run-cleanup* ("this finalizer runs exactly once before the scope is
abandoned") — built **breadth-wise** rather than climbing the ladder. It is the *inverse* of the tenth seam
`drain`: `drain` **closed** by making a skipped `Drop` *harmless* (leak-**tolerance**); this needs a skipped
`Drop` to be *impossible* (leak-**intolerance**). The cold panel ruled **SEAM-BREAKS [horn 1]** across **two
independent rounds** (r23 + r24, breaking attackers included) — the **fifth** broken seam — and the break
adds two things the deadlock probes could not:

> **The match-unit is the *precondition*, not the *domain*.** The same liveness domain grades
> deadlock-freedom *partial* (#15/#16) and must-run-cleanup a clean **none**. So the Phase-M
> "domain-parameterized" result sharpens: *which seams close* is set per **precondition**, not per domain —
> a single domain hosts preconditions of different grades. The "partial-via-`#[must_use]`-lint" texture was
> **rejected**: `scopeguard` ships exactly that warning as a *cosmetic patch over a non-guarantee*, so the
> lint is a non-type advisory (silenced by `let _ =`, blind to `mem::forget`), not a floor fragment.
> Stipulating "`must_use` ∈ floor" only *manufactures* a false partial. **N is unchanged** — this is the
> *enforcement* face of the ninth axis (leak-freedom), where `drain` was the *tolerance* face.
>
> **A NEW verdict mode: *wrong-structural-logic*, not domain-mismatch.** `mem::forget` is safe (its
> unbounded `fn<T>(T)` consumes the guard by one legal **affine** move), so `Drop` never fires — the
> leakpocalypse (RFC 1066; `thread::scoped` removal, RFC 1084). "Runs *exactly* once" is **linear**; the
> floor is **affine** ("*at most* once"). Closing the gap needs either an *undecidable* finalizer-
> reachability pass (`Rc`-cycle escape ≅ halting) or a *viral `Leak`/linear bound* — every sound
> **decidable** enforcement collapses to the latter — so *no decidable boundary price* exists. The
> obstruction is the floor being the wrong **structural logic** (affine vs linear), not the wrong *domain*
> (the IFC sense); sharpened to **Boats' scoped-task trilemma**, of which Rust *deliberately rejected* the
> linear horn. And the meta-lesson the two FAILED rounds carry: a seam **designed to break** cannot be
> given a coherent "usable" spec — *the incoherence IS the break* (both rounds FAILED on spec-coherence, not
> on the conceded `forget` skip), which is itself positive evidence for the verdict.

#### The match is COMPONENT-WISE — an eighteenth primitive HOLDS with a cost, and the same gap lands on a different horn

The four out-of-(memory-safety) probes above all *broke*. An eighteenth — a **session-typed channel**
(`Chan<P>`, an affine typestate handle whose type parameter is the *remaining protocol*) — was the first
to **hold with a cost** instead: the cold panel (r25) ruled **SEAM-COSTS [horn 3]** (TRADEOFF, survived).
It is the crucible for the two prior liveness findings — #15 (typestate carries static *order*) and #17
(the floor cannot compel *completion*) — and it asks whether the floor matches a *composite* precondition
**component by component**.

> **Protocol fidelity decomposes, and the floor matches it component-wise — the components orthogonal.**
> The soundness attacker exhausted the surface (forge a `Chan` at a wrong state → blocked by nominal
> non-subtyping + private fields; skip/reorder → `E0599`; transport-aliasing → `E0499`; and the sharp
> cross-component move — can `mem::forget` shorten `'a` to start a second session? **no**, leaking a value
> holding a `&'a mut` does not shorten `'a`), so **completion cannot infect safety**. The *order-safety*
> component **closes** — sound, decidable, concentrated, and (unlike #15) with **no `root`-hole**, a
> genuine closing sub-seam. The *completion* component is an undischargeable **horn-3 concentration** cost:
> the "don't abandon" obligation recurs at every `?`/panic/early-return, de-concentrating the completion
> half — but no horn fully drops, so the seam holds.
>
> **The same linearity gap, a different horn.** The *same* safe `mem::forget` that **broke** #17's horn 1
> only **costs** here. Completion was #17's *whole* soundness promise (so the gap **unsounds** it); here it
> is *one component beside an independently-sound safety core* (so the gap merely **de-concentrates**).
> **Which horn a linearity gap threatens is not a property of the gap but of how the re-import rule
> *packages* it.** Deflations the panel forced, all accepted: the witness has no duality (`core`/no-`unsafe`
> forecloses two live endpoints, `E0499`), so it is a single-handle *walk*, not a binary session, and the
> abandonment is *self*-abandonment; "usable" holds only for *infallible straight-line* protocols (a
> fallible transport forces `?`, which *is* the abandonment); and the order-closes/completion-breaks split
> is **documented folklore** (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 *Stay Safe Under
> Panic*, Cutner–Yoshida–Vassor), so the contribution is **locating** the known split on the lattice. N
> stays 9; the five broken seams are unchanged — this one is the arc's first SEAM-COSTS *hold* outside
> memory safety.

#### The packaging dial, deliberately turned — a nineteenth primitive confirms it BUT names its fence

Phase Q's "which horn depends on packaging" was an *observation*: #17 and #18 were different primitives, so
the horn-move could be incidental. A nineteenth — a **value-owning scope-restore guard** (`Restore<T, F>`,
the real `scopeguard::guard(value, f)` + `into_inner` — the shape #17's `MustRun` deliberately declined) —
turns it into a **controlled experiment**: hold #17's *identical* un-compellable-`Drop` danger fixed
(`mem::forget` skips the restore), change only how the rule packages it (wrap it in #18's structure — a
sound affine-ownership *core* beside which the restore is a side-obligation), and watch the verdict move.
The cold panel (r26) ruled **SEAM-COSTS [horn 3]** (TRADEOFF, survived): three attackers conceded the
ownership core is sound (no safe double-free/alias/double-consume; `mem::swap` strands a value but yields no
UB; trivially decidable), and the *same* `mem::forget` gap that **broke** #17's horn 1 here only **costs**
horn 3 (the restore strands a *logical invariant* — a lost lease, a stale global — recurring at every
`?`/panic/return). **The dial is real.**

> **But the panel BOUNDED the thesis three ways — all accepted, and they sharpen it past the prediction.**
> (1) **Confounded, not single-variable.** "Only the packaging changed" is false: to package a danger as a
> *side*-obligation you must introduce a sound core for it to sit *beside*, so #19 also adds the
> value-ownership core #17 lacked — the value-ownership model **co-varies**. A better-controlled probe than
> the incidental #17-vs-#18 pair, not a clean manipulation. (2) **Bounded, not universal — the horn is
> FORCEABLE.** The *same* skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces
> **horn-1 UB** (a mid-restore panic double-drops the borrowed slot — the ninth-axis `take` danger), and
> `Pin`'s drop guarantee and the post-leakpocalypse `thread::scope` restructure (the closure shape that
> *forces* the join) exhibit horns that **cannot** be repackaged. So *which* horn depends on **where safety
> rests** — *partly danger-intrinsic*. (3) **Not novel — confirm-by-restaging, no new mode.** `JoinGuard`
> broke horn 1 and `scopeguard` costs horn 3 already in 2015 (RFC 1066, the leakpocalypse); the contribution
> is the **method** (Phase Q's observation made a *deliberate* probe) and **locating the dial**, not a new
> verdict structure. **The refined finding: which horn a danger threatens is a design choice *within a
> structural envelope* set by whether soundness can be DECOUPLED from the un-compellable closure** — owned ⇒
> costs (this seam, `scopeguard`); borrowed-across-unwind / `Pin` / `thread::scope` ⇒ forces horn 1. Phase Q
> said the horn moves; Phase R says it moves *within a fence, and names the fence*. N stays 9; the five
> broken seams are unchanged — this one holds.

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
typednat│  PROBES checker-tractability ⇒ REJECTED (off-diagonal confounded by expressiveness;
        │                              cost encoding-relative unary/binary; mislabeled = terminating-
        │                              resolution, Paterson/UndecidableInstances). N stays 7. METHOD STOP.
   take │  recovers unwind-safety       ⇒ 3-of-≥8  (exception-safety-guarantee-level; clean off-diagonal
        │                              vs SPSC — splits on it ALONE; = Abrahams/noexcept/UnwindSafe,
        │                              pre-existing; carries a DISCHARGE MENU: abort / repair / poisoning)
  drain │  recovers leak-freedom        ⇒ 3-of-≥9  (leakpocalypse; pre-pessimization; clean off-diagonal
        │                              vs SPSC AND vs take — leak ⟂ unwind, both directions; = mem::forget-
        │                              is-safe/RFC 1066, pre-existing; the THINNEST axis but genuinely distinct)
 selfref│  PROBES move-stability        ⇒ INSEPARABLE (a distinct danger — a value's address — with NO
        │                              isolable cost: stack-pin→API-locality, heap-pin→bare-metal; cost is
        │                              immovability. N stays 9. NEW VERDICT CATEGORY)
validity│  PROBES bit-level validity    ⇒ PROJECTS ONTO THE BASIS (a NEW danger class, NO tenth axis —
        │                              unanimous) + 2nd INSEPARABLE (per-read→#6, zero-copy view→API-locality).
        │                              THE COST-BASIS IS CLOSED (for memory safety). Arc's conclusion.
 secret │  PROBES a DIFFERENT DOMAIN    ⇒ info-flow is OUT-OF-FRAME (not UB; timing). BREAKS [horn1] in
        │  (constant-time / non-interf.)  the IFC sense (declassification = etiquette; needs undecidable
        │                              whole-program IFC). The TRILEMMA RECURS, sharper. Closure is PER DOMAIN.
 labeled│  PROBES a CLOSING IFC seam   ⇒ ALSO BREAKS (5/5): map/zip hands the closure bare plaintext (no
        │  (capability-gated declassify)  effect system); the "budget" counts events not BITS (affinity bounds
        │                              uses, not amount). NO closing IFC seam on Rust's floor. THE FLOOR is
        │                              matched to memory-safety, MISMATCHED to info-flow (needs Jif/QIF).
ordlocks│  PROBES the POSITIVE half   ⇒ BREAKS [horn1] (FAILED, 3/5). A THIRD domain (liveness/deadlock).
        │  (type-level lock ordering)    Two under-carryings: GRANULARITY (re-entrant Floor::root → 2 lineages
        │                              → typable inversion; per-token ≠ per-thread) + COVERAGE (dynamic
        │                              acyclicity — runtime/same-rank locks — escapes the static floor, like
        │                              IFC's amount escaped the affine one). The floor-match is GRADED: full /
        │                              partial / none. = BLR'02 minus dynamic ext.; Kobayashi nat-levels.
        ▼
registry│  UPGRADES the floor (runtime)  ⇒ BREAKS [horn1] (FAILED, r22). Floor-upgrade is NON-MONOTONE: it
        │  (thread-local lock registry)  FIXED coverage (dynamic orderings close) + paid the price (#5 bare-metal
        │                              — can't be no_std; #6 runtime check; prevention→detection modality), but
        │                              GRANULARITY RECURRED one rung finer — HELD keyed on OS thread, async TASKS
        │                              are the control unit → work-stealing migration launders an undetected
        │                              inversion (thread-blind). Granularity is a LADDER: token < thread < task <
        │                              … ; no ambient Rust floor tops out. = FreeBSD WITNESS/lockdep (learned graphs).
        ▼
 linear │  PROBES a NEW PRECONDITION   ⇒ BREAKS [horn1] (FAILED, r23+r24). Must-run cleanup = LINEARITY,
        │  (must-run cleanup guard)      breadth-wise (a precondition, not a new domain/rung). Affine floor =
        │                              clean NONE (the #[must_use] "partial" REJECTED — scopeguard ships it
        │                              cosmetic). Match-unit is the PRECONDITION not the domain (liveness:
        │                              deadlock PARTIAL, must-run NONE). N unchanged (ENFORCEMENT face of axis
        │                              9 leak-freedom; drain = TOLERANCE face). mem::forget skips Drop (RFC
        │                              1066/1084). NEW MODE wrong-structural-logic: affine≠linear (Boats'
        │                              scoped-task trilemma, linear horn rejected), not domain-mismatch.
        ▼
session │  COMPOSITE precondition       ⇒ HOLDS WITH COST: SEAM-COSTS [horn3] (TRADEOFF, survived r25). FIRST
        │  (session-typed channel)       out-of-domain probe to HOLD, not break. Matched COMPONENT-WISE
        │                              (orthogonal — mem::forget can't shorten 'a to start a 2nd session):
        │                              order-SAFETY CLOSES (affine typestate, no root-hole) / COMPLETION COSTS
        │                              horn-3 (the #17 linearity gap; recurs at every ?/early-exit — de-
        │                              concentrates, doesn't unsound). SAME gap, DIFFERENT horn: it BROKE #17's
        │                              horn1 (completion = whole promise) but only COSTS here (one component
        │                              beside a sound core) — which horn depends on how the rule PACKAGES it.
        │                              Deflations: single-handle WALK not a binary session (E0499); "usable"
        │                              infallible-straight-line only; FOLKLORE (JML'15; ECOOP'22 Stay Safe
        │                              Under Panic) → contribution = LOCATING it. N stays 9; 5 broken unchanged.
        ▼
restore │  CONTROLLED PACKAGING probe  ⇒ HOLDS WITH COST: SEAM-COSTS [horn3] (TRADEOFF, survived r26). #17's
        │  (value-owning scope-restore   IDENTICAL un-compellable-Drop danger wrapped in #18's structure (sound
        │   guard = real scopeguard)     affine-ownership CORE + restore as side-obligation). SAME mem::forget gap
        │                              that BROKE #17's horn1 only COSTS horn3 here → the packaging dial is REAL.
        │                              BUT BOUNDED 3 ways: (1) CONFOUNDED — #19 also adds the value-ownership core
        │                              #17 lacked, so ownership co-varies (better-controlled, not single-variable);
        │                              (2) FORCEABLE — same danger in a borrowed-slot/take_mut packaging FORCES
        │                              horn1 UB (the #9 take danger); Pin's drop-guarantee + thread::scope show
        │                              un-repackageable horns → which horn depends on WHERE SAFETY RESTS, partly
        │                              danger-intrinsic; (3) NOT NOVEL — JoinGuard horn1 / scopeguard horn3 already
        │                              in 2015 (RFC 1066). FINDING: the horn is a design choice WITHIN A STRUCTURAL
        │                              ENVELOPE set by whether soundness DECOUPLES from the closure (owned ⇒ costs;
        │                              borrowed-across-unwind/Pin/thread::scope ⇒ forces). No new mode; N stays 9.
        ▼
Realizability table         MEMORY-SAFETY domain (on a MATCHED affine floor): N ≥ 9 CLOSED axes (sound,
                            decidable, concentration, API-locality, #6, #7, bare-metal, unwind-safety,
                            leak-freedom) + INSEPARABLE (move-stability, validity) + 2 outside-the-axes
                            (MMIO TCB; intrusive horn-drop). INFORMATION-FLOW domain (on a MISMATCHED floor):
                            its own basis (declassification, quantitative leakage) but NO closing seam — both
                            IFC seams break. LIVENESS domain: on a static-order floor a PARTIAL match (the
                            static fragment closes, granularity + dynamic escape); UPGRADING to a runtime
                            registry floor is NON-MONOTONE — coverage lifts (paying #5 + #6 + modality) but
                            granularity RECURS thread→task (a containment-scope LADDER no ambient floor tops);
                            and a DIFFERENT liveness precondition — must-run-cleanup — grades a clean NONE and
                            BREAKS on wrong-structural-logic (affine≠linear), so the match-unit is the
                            PRECONDITION, not the domain. And a COMPOSITE precondition (session-typed channel)
                            is matched COMPONENT-WISE and HOLDS WITH COST (SEAM-COSTS [horn3]): order-safety
                            CLOSES, completion COSTS horn-3 — the same linearity gap that BROKE #17 only de-
                            concentrates here, because which horn it threatens depends on how the rule PACKAGES
                            it. And a CONTROLLED PACKAGING probe (value-owning scope-restore guard) turned that
                            into a deliberate experiment — HOLDS WITH COST (SEAM-COSTS [horn3], r26), the dial
                            REAL but BOUNDED: the move is confounded (ownership co-varies), the horn is forceable
                            (borrowed-slot/take_mut → horn1 UB; Pin / thread::scope un-repackageable), so the horn
                            is a design choice WITHIN A STRUCTURAL ENVELOPE set by whether soundness DECOUPLES from
                            the un-compellable closure. The Seam Trilemma is DOMAIN-GENERAL, the cost-basis
                            PER-DOMAIN, and WHICH SEAMS CLOSE depends on the FLOOR-DOMAIN MATCH — GRADED (full /
                            partial / none), set per PRECONDITION not per domain, and not monotonically liftable.
                            Nine method modes:
                            confirm / sharpen / reject / inseparable / projects-onto-basis / out-of-frame /
                            floor-mismatch / partial-match / wrong-structural-logic.
```

Each constructive primitive is the **witness that two trilemma-identical configurations differ** —
the only way to prove a folded axis was ever really there. The slab proved a concentrated seam can
still cost runtime; the branded index proved a concentrated, erasing seam can still cost the type
surface; the refined index proved legibility (#7) is its own axis (the brand-free-yet-legible
off-diagonal the slab-vs-branded pair could not reach) **and** that horn 2 was a bundle of
*decidability* and *solver-freedom*; the shared seam recovered seed #5 (bare-metal), the
runtime-manager axis, distinct from runtime *cost* (#6).

**The honest meta-pattern.** *Every* axis the method recovers turns out to be an **already-named PL
concern**: zero-cost abstraction (#6), generativity/API-locality, refinement-legibility (#7), the
`core`/`alloc` bare-metal split (RFC 2480), exception-safety (Abrahams' basic/strong/nothrow;
`noexcept`; `UnwindSafe`), and now leak-freedom (the leakpocalypse; `mem::forget`-is-safe; linear-vs-
affine types). The constructive phase did not discover new physics. What it found is that the trilemma's
three horns silently *bundle together a set of independently known PL design axes*, and that one rubric
— **"what does the safe witness cost?"** — unifies them. The contribution is the unification and the
method that recovers each axis (build the dissociating witness, let a cold panel adjudicate), not the
novelty of any single axis. The adjudicator showed **five distinct modes**, which is exactly what makes
it an instrument and not a rubber stamp: it **confirmed-but-deflated** three new dimensions (bare-metal,
unwind-safety, leak-freedom → "real, but pre-existing", growing N to 9); it **sharpened without
extending** once (solver-freedom = horn 2's "keep" face, not a new axis); it **rejected outright** once
(checker-tractability — off-diagonal confounded with expressiveness); it found two **real but
structurally inseparable** dangers (move-stability, validity — distinct dangers with no isolable cost; a
recurring category); and — the capstone — it ruled a new danger class **projects entirely onto the
existing basis** (validity adds no tenth axis), the evidence that **the cost-basis is closed**. The seam
tension is where *known* axes meet; the panel recovers the clean ones, refuses the confounded, recognises
the inseparable, and certifies when a new danger adds nothing. A discovery procedure with only a "yes" is
a rubber stamp; this one has "yes", "yes but pre-existing", "no — confounded", "sharpens not extends",
"real but inseparable", and "real but it projects onto what's already here" — each on the record, the
last one closing the basis.

## Structure among the axes (not just a count)

The axes are not independent toggles you can set freely — **two trades and a discharge menu** show up:

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
- **The unwind-safety *discharge menu* (the take-and-replace seam).** Unlike the first seven axes,
  whose cost lands in one fixed place, the eighth axis can be *discharged in more than one place* — the
  panel's mandatory correction. The obligation (survive an unwind through a broken invariant) is one
  axis; *where you pay it* is a free design variable: **abort** (this seam — sound, but downgrades the
  basic guarantee, non-recoverable), **placeholder-repair** (a `replace_with` sentinel — sound *and*
  recoverable, pays **API-surface / `T: Default`**), or **poisoning** (a lock — sound, pays a
  one-way-latch on availability). So the axis is the *guarantee-level*; the abort is one coordinate on
  it. This is a third kind of inter-axis structure — not a two-axis exchange, but one axis with a menu
  of discharge points that each spend a *different other* resource.

## Honest bounds

- N ≥ 9 is a **lower bound**: nine gauntlet-confirmed live axes — sound, decidable, concentration,
  API-locality, zero-cost (#6), AI-legibility (#7), bare-metal (#5, shared phase), unwind-safety
  (exception-safety-guarantee-level, take phase), and leak-freedom (drain phase). More may exist; the
  method (build a dissociating witness, let the cold panel adjudicate) is open-ended, not exhausted.
  **But "open-ended" is not "everything is an axis":** the eighth axis took *two* probes. The first
  (type-level index → checker-tractability) was built, attacked, and **rejected** — confounded with
  expressiveness, encoding-relative, a mislabeled re-name of a pre-existing terminating-resolution
  concern (Paterson / `UndecidableInstances`). The second (take-and-replace → unwind-safety) supplied a
  *clean* off-diagonal and was **confirmed**; the ninth (drain-and-yield → leak-freedom) likewise. So N
  reached 9 only on witnesses that hold every other axis fixed — the rejection of the confounded
  candidate is what makes the confirmations trustworthy. A tenth probe (self-ref cell → move-stability)
  found the **inseparable** category (a distinct danger with no isolable cost; N stays 9), and an
  eleventh (validity → bit-level validity) found *both* a **second** inseparable danger *and* the
  **closed-basis** result: a brand-new danger class that adds **no tenth axis**, every witness spending a
  pre-existing coordinate. So the honest bound is now: **≥ 9 clean axes + an inseparable category (≥ 2
  members) + 2 outside-the-axes (TCB axiom, horn-drop), and the basis is *closed* under the dangers
  probed** — "more may exist" remains true (it is a lower bound), but the *evidence* of a new danger
  projecting cleanly onto the basis, and inseparability recurring, is what closure looks like. The four
  shapes a *within-domain* probe can take are all attested: a clean axis, a confound, an inseparable, or a
  plain projection onto the basis — **plus three cross-domain shapes: *out of frame* (the secret seam — a
  different soundness domain), *floor-mismatch* (the labeled seam — a domain whose seams none close on this
  floor), and *partial-match* (the lock-ordering seam — a domain that closes *partway*).** The closure is
  therefore **per memory-safety domain**; information-flow has its own basis (declassification discipline,
  …) and its own — sharper — trilemma, with *no* seam closing on Rust's floor; liveness has its own basis
  (lock-order-inversion freedom, dynamic acyclicity) with a *partial* closure. The deepest bound is not
  "N ≥ 9 closed" but "**each soundness domain has its own basis, the seam tension is the same shape in
  every one, and *which seams close* is set by the floor-domain match — which is graded full / partial /
  none**" (memory safety mapped on a matched floor, information flow sketched on a mismatched one, liveness
  sketched on a partially-matched one).
- The #5⟂#7 dissociation **is witnessed** (refined index = brand-free + legible; slab = brand-free +
  illegible). Bare-metal ⟂ #6 **is witnessed** (slab = bare-metal + runtime-cost; Shared = global-heap
  + runtime-cost). Unwind-safety ⟂ all seven **is witnessed** (take-and-replace vs SPSC: the cleanest
  single-coordinate flip). Leak-freedom ⟂ unwind-safety **is witnessed both directions** (take =
  unwind-sensitive/leak-safe; drain = leak-sensitive/unwind-safe) — the two destructor-execution
  siblings are genuinely distinct, not one merged axis. The refined phase surfaced that horn 2
  un-bundles into decidability + solver-freedom — that face is the original "keep" horn, so it sharpens
  the map rather than extending N.
- **Novelty is bounded, and that is the honest headline.** Every recovered axis the cold panel deflated
  to "real but *pre-existing*" — solver-freedom (Liquid-Types' solver-in-TCB), bare-metal (RFC 2480's
  `core`/`alloc`), unwind-safety (Abrahams' exception-safety; RFC #1508 rejected the very `take_mut`
  scored here), and leak-freedom (Rust's leakpocalypse; RFC 1066; the `Vec::drain` leak-amplification
  fix). The lattice's value is *unification under one cost rubric*, not novel dimensions. Treat "N ≥ 9"
  as "at least nine *known* PL axes are all faces of the seam tension," not "nine discoveries." A note
  on *thinness*: the panel flagged leak-freedom as the thinnest, most-deflatable axis (its cost is
  invisible in non-leak code), yet still real — a reminder that "deflatable" and "not an axis" are
  different verdicts (the rejected checker-tractability probe is the latter; leak-freedom the former).
- The Lean kernel (`../proof/SeamGap.lean`) sees only whether a precondition factors through a
  Boolean witness — so it certifies *closure* (`spsc`/`branded`/`refined`/`shared`/`typed_nat`/`take`/`drain`/`selfref`/`validity` — and `session_order_safety_closes`, the eighteenth seam, whose *order-safety* component closes like a holding seam (it HOLDS with a horn-3 completion cost the kernel cannot see), and `restore_ownership_core_closes`, the nineteenth seam, whose *ownership* core closes the same way (it HOLDS with a horn-3 restore cost — the controlled-packaging probe: the same `mem::forget` gap that broke #17 only costs here, but bounded — the horn is forceable in a borrowed-slot/`take_mut` packaging) — (+ `secret`/`labeled`/`ordered_locks`/`lock_registry`/`linear`, which certify only the trivial typed sub-fact — the two IFC seams break, and three liveness-domain seams break: the static lock-ordering one partially-matches, the runtime registry shows the floor-upgrade is non-monotone, and the must-run-cleanup one is a clean NONE breaking on wrong-structural-logic, affine≠linear)
  all close, like SPSC) but is blind to *which axis a closing seam spends* — or whether that "axis" is
  even separable (the self-ref cell closes in the kernel exactly like the rest; its inseparability is
  pure modeling-dictionary). Every column past
  "concentration" lives in this modeling-dictionary, by the same honest caveat the slab's `Vrt` carries.
- One artifact caveat the panel found and we fixed: the reference model's R2 used `bound == len`
  (strictly stronger than safety's `bound ≤ len`); relaxed to `≤` (predicate subtyping). The shipped
  `seam/src/refined.rs` is faithful to the floor it models on this point.
