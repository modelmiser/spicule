# Build-and-break the seam — A→B→C synthesis

The negative half of this project proved keeping all seven properties is impossible and
distilled the **Seam Trilemma**: a seam cannot simultaneously have (1) sound re-import,
(2) a decidable, proof-free safe fragment, and (3) seam concentration. The constructive
phase asked the next question: *take the trilemma as a map and actually build a seam.* The
stateful builder authored each seam spec + a real `rustc`-checked implementation; the cold
adversarial harness (5 attackers + judge + an independent horn-classifier) red-teamed the
**re-import rule only**, never the author. Eighteen primitives, chosen so their preconditions have
eighteen different *shapes* — and so that each later one becomes the witness un-bundling an axis the
trilemma had folded (zero-cost, API-locality, AI-legibility, solver-freedom, bare-metal, unwind-safety,
leak-freedom). The discriminating moments exercise **nine** adjudication modes: a type-level-proof
index pitched as *checker-tractability* was **rejected** (off-diagonal confounded with expressiveness);
take-and-replace pitched as *unwind-safety* was **confirmed** (N ≥ 8, clean); drain-and-yield pitched as
*leak-freedom* was **confirmed** (N ≥ 9, distinct from unwind-safety); a self-referential cell pitched as
*move-stability* was ruled **real but structurally inseparable** (a distinct danger with no isolable
cost, N stays 9); a validity-checked bytes→enum read — the capstone — asked whether the **cost-basis is
closed** and got back, unanimously, **yes** (a genuinely new danger class projects entirely onto the
basis, no tenth axis); two information-flow probes stepped **outside memory safety** to find that the
closure is **per domain** *and floor-relative* — a constant-time secret showed info-flow is a *different
soundness domain*, and a capability-gated declassification showed **no IFC seam closes on Rust's affine
floor** (it bounds resource *uses*, not information *bits*); and a fifteenth — type-level lock ordering, in
a *third* domain (**liveness/progress**) — tested the *positive* half of the floor-domain match and
**broke**, grading the match from binary to **full / partial / none**; and a sixteenth — a *runtime* lock
registry, the same liveness danger on an **upgraded** floor — asked whether upgrading the floor *lifts* the
match, and **broke** too, showing the upgrade is **non-monotone**: it fixed the dynamic-coverage gap but
**granularity recurred one rung finer** (the held-set is keyed on the OS *thread*, but async *tasks* are
the control unit), so the "granularity" axis is a **containment-scope ladder** — token < thread < task —
no ambient Rust floor tops out; and a seventeenth — a *must-run cleanup guard* — probed **breadth-wise** (a
new *precondition*, "this finalizer MUST run", not a new domain or ladder rung) and **broke** on a clean
**none** grade, sharpening the match-unit from *domain* to *precondition* (the *same* liveness domain grades
deadlock-freedom *partial* but must-run-cleanup *none*) and adding the **ninth** mode —
*wrong-structural-logic*: a *safe* `mem::forget` skips the finalizer, and no decidable fix buys it back
(affine "at most once" vs linear "exactly once"), so the floor is the wrong structural *logic*, not the
wrong *domain*. **The cost-basis is closed (per domain), *which seams close* is GRADED by the
floor-domain–precondition match, and that grade is *not monotonically liftable*** — memory safety a **full**
match (basis closed), information flow **no** match (both seams break), liveness a **partial-to-none** match
that an upgraded floor relocates rather than fills (deadlock-freedom partial, must-run-cleanup none) — the
constructive arc's conclusion and its generalizations. And an eighteenth — a *session-typed channel* — is
the **first** out-of-(memory-safety) probe to **hold with a cost** rather than break (SEAM-COSTS [horn 3]):
it shows the floor matches a *composite* precondition **component-wise** — protocol **order** *closes* via
affine typestate (sound, no `root`-hole), while **completion** is a horn-3 *concentration* cost — and the
*same* `mem::forget` linearity gap that **broke** #17's horn 1 only **costs** here, because completion is one
component beside a sound safety core, not the whole promise: *which horn a danger threatens is set by how the
rule packages it.*

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
| **Take-and-replace** (move out and back) | SEAM-COSTS [none] — TRADEOFF (1 cold panel) | sound; all 3 horns hold; pays the 8th axis | O(1) — brand-free, zero-cost on the happy path | *Transient-invariant-breaking across an unwind, carried by an abort-bomb (`take_mut`'s pattern).* Moving a value out of a `&mut T` leaves a duplicate in the gap; an unwinding panic drags the caller into a double-drop. Aces all seven prior axes and pays on **unwind-safety (the 8th axis)** alone — the *clean* off-diagonal the type-level index lacked. Confirmed N ≥ 8, deflated to pre-existing (Abrahams; `noexcept`; `UnwindSafe`; RFC #1508). The axis = *exception-safety-guarantee-level*; abort is one **discharge** (vs `replace_with` paying API-surface). |
| **Drain-and-yield** (leaked guard) | SEAM-HOLDS / SURVIVED (1 cold panel) | sound; all 3 horns hold; pays the 9th axis | O(1) — brand-free, no closure | *The leakpocalypse: `mem::forget` is safe, so a guard whose `Drop` does load-bearing cleanup can be leaked → double-free.* Sound via **pre-pessimization** (`Vec::drain`'s trick — empty the buffer up front, so a leaked guard loses the tail, never double-frees). Aces all eight prior axes, pays on **leak-freedom (the 9th axis)** alone. Confirmed N ≥ 9, **distinct from unwind-safety** (off-diagonal both ways: take is unwind-sensitive/leak-safe, drain is leak-sensitive/unwind-safe). Deflated to pre-existing (leakpocalypse / RFC 1066). The *thinnest* axis but genuinely distinct; aces API-locality *because* the guard owns its data (no closure). |
| **Self-ref cell** (`Pin`-guarded) | SEAM-HOLDS / TRADEOFF (1 cold panel) | sound; all 3 horns hold; pays an **inseparable** danger | O(1) — but a viral, `&mut`-revoking `Pin` | *A value whose **address** is load-bearing (a pointer into itself); moving it dangles the self-pointer.* Sound via `Pin` over a `!Unpin` cell (the post-`init` move is unconstructible). A danger no other axis touches — **but ruled REAL-BUT-INSEPARABLE (outcome 3, N stays 9):** no isolable cost. The price is *immovability* (heavier than API-locality), **non-uniform** across witnesses (stack-pin → API-locality, `Box::pin` → bare-metal). The lattice's first *inseparable* entry — a new verdict category. Caveats: inseparability is contingent (RFC 2349 rejected `?Move` immovable types); crisp only for a non-composable toy. |
| **Validity** (bytes→enum) | SEAM-HOLDS / TRADEOFF (3 cold panels: 2 found spec bugs) | sound; all 3 horns hold; **CLOSED-BASIS test** | O(1) per read; a viral sealed ctor for the view | *A **validity invariant** — a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even without a deref.* A genuinely new danger class, built to ask: does it add an axis? Sound via `transmute`-after-**set-membership** (the move from a *range* check — unsound for non-contiguous discriminants — was forced by gauntlet 1). **Unanimous ruling: NO tenth axis — it projects onto the basis.** Its own cost is **(3) inseparable** (2nd example): per-read → #6, the zero-copy `&[u8]→&[Tag]` view → API-locality. **The cost-basis is closed.** Non-novel (`num_enum`/`bytemuck`/`zerocopy`). |
| **Constant-time secret** (different DOMAIN) | SEAM-BREAKS [horn 1, IFC sense] (1 cold panel) | breaks — a horn drops, *in the information-flow domain* | no `unsafe`; the danger is timing | *Branching on a secret leaks it via timing — **not UB**.* The first probe outside memory safety. `Secret<N>` forecloses branching *on* the secret, but `ct_eq(a,b).expose_bool()` then `if b` leaks a relationship — **declassification is etiquette, not type-structure**; closing it needs whole-program **undecidable** IFC. **The intrusive list's horn-drop, re-run in a new domain — the Seam Trilemma recurs, sharper.** Panel ruling: information-flow is **out of frame**, a *different domain* with its **own basis** (declassification discipline, …). **Closure is per domain; the framework is domain-parameterized.** |
| **Capability-gated declassify** (IFC domain) | SEAM-BREAKS [horn 1] — **5/5 unanimous** (1 cold panel) | breaks — *no closing IFC seam on Rust's floor* | an affine `Cap`, but the wall is open | *Probes whether the IFC domain has a **closing** seam.* It does not: `map(self, f)` hands the closure the **bare plaintext** (an uncapped exit a captured-`&mut` closure exfiltrates — **no effect system**), and the affine "budget" counts **uses, not bits** (a loop mints N caps; one cap → a megabyte). What it enforces — release *location* — **projects onto the affine memory-safety axis**; the real IFC coordinate (*amount* — **quantitative info-flow**, Smith min-entropy) an affine floor can't bound. **THE FINDING: a closing seam exists only when the FLOOR matches the DOMAIN.** Rust's affine floor: matched to memory safety, *mismatched* to IFC (needs Jif/FlowCaml/QIF). |
| **Type-level lock ordering** (third DOMAIN: liveness) | SEAM-BREAKS [horn 1] — FAILED, 3/5 broke it (1 cold panel, r21) | breaks — *partial* floor-match in a third domain | a viral `Floor<F>`, but the floor is open | *Acquiring a lock out of a global rank order deadlocks — **neither UB nor a leak**.* Type-level rank `L` + affine `Floor<F>`, `with` bounded `L: Below<F>`. Built to test the *positive* half of the floor-domain match; **broke two ways**, both "the floor under-carries the precondition." **(1) Granularity (fatal):** `Floor::root` is re-entrant, so two sibling lineages let safe code type a low-then-high *inversion* (verified compiles+runs) — per-token discipline vs a per-thread-global precondition; Rust's floor has *no ambient per-thread one-shot resource*. **(2) Coverage:** even patched, the dynamic-acyclicity fragment (runtime-ordered/same-rank locks — bank-transfer `min/max`, hand-over-hand coupling, IRQ ordering) escapes the *static* floor, exactly as IFC's quantitative coordinate escaped the affine one. **THE FINDING: the floor-domain match is GRADED (full/partial/none)** — liveness on a static-order floor is a **partial** match. Name = *lock-order-inversion freedom*, not "deadlock-freedom"; precedent forces it (Boyapati–Lee–Rinard OOPSLA'02 *minus* their dynamic extension; Kobayashi natural-number levels; soundness = Havender 1968). |
| **Runtime lock registry** (liveness; floor-UPGRADE) | SEAM-BREAKS [horn 1] — FAILED (1 cold panel, r22) | breaks — the floor-upgrade is **non-monotone** | a per-acquire runtime check; needs OS thread-local (not `no_std`) | *Same liveness danger as #15, on an **upgraded floor**: a `thread_local!` held-set of **runtime** ranks, checked per acquire (lockdep-in-miniature).* Built to ask whether *upgrading the floor lifts the match*. It fixed #15's **coverage** gap (runtime-chosen-fixed ranks close — bank-transfer `min/max`, hand-over-hand) and paid the predicted price — **#5 bare-metal** (needs OS thread-local → *cannot be `no_std`*, gated behind feature `lockdep` so the cost shows in the build graph), **#6 zero-cost** (a per-acquire check — the *slab's* place in a new domain), and a **prevention→detection modality** (the inversion compiles, is trapped only on *executed* paths; soundness path-coverage-dependent). **But granularity RECURRED one rung finer:** `HELD` is keyed on the OS *thread*, while the unit of control in async is the *task* — work-stealing migration resumes a rank-holding task on a fresh thread with an empty `HELD`, laundering an **undetected inversion** (verified: the registry is thread-blind); a single-threaded executor *false-panics* on independent tasks, and `!Send` closes neither direction fully. **THE FINDING: the floor-upgrade is non-monotone — the "granularity" axis is a containment-scope LADDER (token < thread < task), and no ambient Rust floor tops out** (there is no task-local; async is a library). Prior art forecloses the design: FreeBSD **WITNESS** / lockdep / Abseil all abandoned static ranks for dynamically-*learned* acquires-before graphs. |

### The deeper result: the Seam Trilemma is a 3-of-N projection (N ≥ 9)

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
That probe's worth is the falsification — the discipline can refuse.

The **take-and-replace seam** then supplied the *clean* off-diagonal the type-level index lacked, and
**confirmed an eighth axis (N ≥ 8): unwind-safety.** Its danger is *time*, not bounds — moving a value
out of a `&mut T` and back leaves a duplicate in the gap, and an unwinding panic drags the caller into
a double-drop; the sound witness is an **abort-bomb** (`take_mut`'s pattern). It **aces all seven prior
axes and pays on unwind-safety alone** — the panel explicitly contrasted this clean isolation with the
type-level index's confound. Deflated to pre-existing (Abrahams basic/strong/nothrow; C++ `noexcept`;
Rust `UnwindSafe`; RFC #1508 rejected this exact `take_mut` from std on this very cost). The panel's
mandatory relabel: the axis is the **exception-safety-guarantee-level**, and the abort is *this seam's*
discharge of it — a `replace_with` sentinel discharges the same obligation soundly-and-recoverably by
paying **API-surface** instead. So the axis carries a **discharge menu** (abort / repair / poisoning),
a third kind of inter-axis structure beyond the two exchange-trades.

The **drain-and-yield seam** then confirmed a **ninth axis (N ≥ 9): leak-freedom**, distinct from its
sibling unwind-safety. Its danger is the Rust **leakpocalypse**: `mem::forget` is a *safe* function, so
a `Drain` guard whose `Drop` does load-bearing cleanup can be leaked, and a naive impl double-frees the
moved-from slots. The sound witness is **pre-pessimization** (`Vec::drain`'s real fix — empty the buffer
up front, so a leaked guard loses the un-yielded tail but never double-frees). It **aces all eight prior
axes and pays on leak-freedom alone**, and crucially **aces API-locality** — a free-standing `Drain<'a>`
with *no closure* — which is why it avoided the confound that would have sunk the obvious leak-freedom
witness (a scoped-thread guard, whose closure *is* an API-locality cost). The panel ruled it **distinct
from unwind-safety**: the off-diagonal is populated both ways (take is unwind-sensitive/leak-safe, drain
is leak-sensitive/unwind-safe — "`Drop` never runs" vs "`Drop` runs at a bad time", codified separately
in Rust as *Leaking* vs *Exception Safety*). Deflated to pre-existing (leakpocalypse / PPYP / leak
amplification / RFC 1066), and flagged the **thinnest, most-deflatable** axis — its cost (leaked-tail
data loss) is invisible in non-leak code — yet a genuinely populated off-diagonal, not a non-axis.

The **self-referential-cell seam** then probed for a *tenth* axis — **move-stability** (a value whose
address is load-bearing) — and the panel returned the project's most novel verdict: **real but
structurally inseparable (N stays 9).** The danger is genuinely distinct (no other axis concerns a
value's address) and soundly handled (`Pin` over a `!Unpin` cell makes the post-`init` move
unconstructible). But it is **not a clean tenth axis**, because it has *no isolable cost*: `Pin<&Self>`
is a viral wrapper that **revokes `&mut`** (and that revocation *is* the soundness mechanism, so the
seam cannot ace API-locality), and the cost is **non-uniform across witnesses** (stack-pin spends
API-locality, `Box::pin` spends bare-metal). The honest price is **immovability** — heavier than
API-locality, and inseparable from the danger. So move-stability is neither a clean axis (like
unwind-safety/leak-freedom) nor a confounded non-axis (like checker-tractability): a **fourth category**.
Two caveats the panel extracted: the inseparability is *contingent* (RFC 2349's rejected `?Move`
immovable types were a designed free-standing witness Rust chose not to adopt), and the crisp `core`-only
reading holds only for a *non-composable* single-frame toy (real intrusive nodes / futures force
`unsafe` pin-projection or `Box::pin`).

The **validity seam** then asked the question all the probes were circling — **is the cost-basis
closed?** — and supplied the constructive arc's conclusion. It re-imports a danger no prior primitive
touched (a **validity invariant**: a `#[repr(u8)]` enum byte that must be a valid discriminant, UB even
without a deref) — a genuinely new danger *class*. The cold panel's **unanimous** ruling: it adds **no
tenth axis**; every witness for it spends a *pre-existing* coordinate. Its own cost is **(3) inseparable**
(a second example after move-stability): a single checked read spends **zero-cost #6** (a runtime tag
check, the slab's axis via a different predicate), but the zero-copy `&[u8]→&[Tag]` view that restores
concentration spends either eager #6 or a sealed constructor's **API-locality**. This is the **dual** of
the `get_unchecked` family: there *one* danger had *many* witnesses spending *different* axes; here *many*
dangers converge on the *same* axis. Read together, the two directions say the cost-space is a **small
fixed basis** that dangers *project onto*. (The adversary earned its keep to the last: two of three
gauntlets found real spec bugs — a *range* check unsound for non-contiguous discriminants, then a
residual stale `b<=2` — before the corrected seam survived; and the result is non-novel, the shipped
design of `num_enum`/`bytemuck`/`zerocopy`.)

So the trilemma is a low-dimensional shadow of a **closed object** — **nine clean axes** (#6, #2's two
faces, #4's two faces, #5, exception-safety, leak-freedom), **an inseparable category** (move-stability,
validity — recurring, not a one-off), and **two outside-the-axes** (the MMIO TCB axiom, the intrusive
horn-drop) — recovered one primitive at a time, with one candidate (checker-tractability) **rejected** as
confounded and the basis shown **closed** by a new danger class projecting cleanly onto it.

**…and closed *per domain*.** A thirteenth primitive — a **constant-time secret** — stepped outside
memory safety entirely, re-importing an *information-flow / timing* danger (branching on a secret leaks
it; **not UB**). The cold panel **generalized the whole result**: information-flow is **out of frame** — a
*different soundness domain* with its **own basis** — so "the cost-basis is closed" refines to **closed
*per domain***. Two confirmations: the secret seam **BREAKS [horn 1] in the IFC sense** (the intrusive
list's horn-drop re-run in a new domain — `ct_eq(a,b).expose_bool()` then `if b` leaks a secret
relationship; declassification is *etiquette, not type-structure*; closing it needs whole-program
undecidable IFC, so a concentrated decidable seam *cannot* enforce non-interference — **the trilemma
recurs, sharper**); and the IFC domain has its own axes with **no memory-safety analog** —
**declassification discipline**, plus termination/concurrency channels. (Honest correction the panel
forced: constant-time codegen is *dischargeable* — CompCert-CT/Jasmin/FaCT — not an MMIO-class irreducible
axiom.)

**The deepest refinement: a closing seam exists only when the *floor* matches the *domain*.** A
*fourteenth* primitive — a **capability-gated declassification** (`labeled.rs`) — probed whether the IFC
domain has a *closing* seam (the IFC analog of SPSC). It **also breaks, unanimously**: to *compute* on a
secret, `map(self, f)` must hand the closure the **bare plaintext**, and a captured-`&mut` closure
exfiltrates it (`s.map(move |x| sink.push(x))`) — cap-free, no `declassify` site, **unpatchable without an
effect system the floor disclaims**; and the affine "budget" counts *uses*, not *bits* (a loop mints N
caps; one cap leaks a megabyte). What it *can* enforce — release *location* — **projects onto the
memory-safety affine axis**; the genuinely-IFC coordinate is the *amount* (**quantitative information
flow**, Smith min-entropy), which an affine floor cannot bound. So **both IFC seams break, for one
reason: Rust's affine floor carries none of the IFC precondition.** It is *matched* to memory safety (SPSC
closes free, basis closed) and *mismatched* to information flow (no seam closes). So the deepest form of
the result is layered: **the Seam Trilemma is domain-general; the cost-basis is per-domain; and *which
seams close* depends on the floor-domain match.** Memory safety mapped (on a matched floor), information
flow sketched (every seam breaks on Rust's floor — pointing at the quantitative/effect floor it needs:
Jif, FlowCaml, QIF). The build-and-break method transfers — and *diagnoses the floor mismatch* itself.

**The match is GRADED, not binary — a third domain shows the middle value.** A *fifteenth* primitive —
**type-level lock ordering** (`ordered_locks.rs`) — stepped into a *third* domain, **liveness/progress**
(deadlock is neither UB nor a leak), to test the *positive* half of the floor-domain match: does a closing
seam reappear once the floor (a type-level rank order) carries the new domain's precondition (an acyclic
global order)? The cold panel ruled **SEAM-BREAKS [horn 1], FAILED** (3/5 attackers, one fatal) — and the
break is the richer result, because it grades the match. It fails along *two* independent dimensions, both
"the floor under-carries the precondition": **(1) granularity** — `Floor::root` is re-entrant, so two
sibling lineages let safe code type a low-then-high *inversion* (verified: it compiles and runs); the
discipline is *per-token* while the precondition is *per-thread-global*, and Rust's floor has no ambient
per-thread one-shot resource — so even the static fragment does not close; **(2) coverage** — even patched,
the *dynamic*-acyclicity fragment (runtime-ordered / same-rank locks — bank-transfer `min/max`,
hand-over-hand coupling, IRQ ordering) escapes the *static* type-level-order floor, exactly as IFC's
quantitative coordinate escaped the affine one. Prior art forces the ruling: this is **Boyapati–Lee–Rinard
OOPSLA'02 minus their dynamic tree-order extension**, and **Kobayashi's natural-number levels** — both
abandoned pure static levels for the same wall (soundness itself = Havender 1968). So the floor-domain
match is **GRADED**: **full** (memory safety on the affine floor — basis closed), **partial** (liveness on
the static-order floor — static fragment carried, per-thread + dynamic fragments escape), **none**
(information flow on the affine floor — every seam breaks). A floor can fail to carry a domain along (at
least) two axes — *granularity* and *static-vs-dynamic* — and the third domain fails on both. (Corrections
the panel forced: the right name is **lock-order-inversion freedom**, not "deadlock-freedom" — condvar /
channel / join cycles remain; and the claim that Rust's *affine* floor enables closure is overstated — the
order-carrying floor predates affinity by two decades in two non-affine settings, and affinity buys only
token non-duplication, which the re-entrant `root` shows it does not even buy.)

**…and the grade is not monotonically liftable — a sixteenth primitive proves it.** The obvious repair to
#15 is to *upgrade the floor*: a **runtime lock registry** (`lock_registry.rs`, lockdep-in-miniature) — a
`thread_local!` held-set of **runtime** ranks checked on each acquire. It directly targets #15's two gaps,
and the cold panel ruled **SEAM-BREAKS [horn 1], FAILED** (r22), the *fourth* broken seam — because the
upgrade is **non-monotone**. It *did* fix **coverage** (runtime-chosen-fixed ranks close — bank-transfer,
hand-over-hand) and *did* pay the predicted price (**#5 bare-metal** — needs OS thread-local, *cannot be
`no_std`*, hence feature-gated; **#6** a per-acquire runtime check; a **prevention→detection modality** —
the inversion compiles and is trapped only on *executed* paths, soundness now path-coverage-dependent). But
**granularity recurred one rung finer**: `HELD` is keyed on the OS *thread*, while the unit of control in
async is the *task*; a work-stealing migration resumes a rank-holding task on a fresh thread with an empty
`HELD`, laundering an **undetected inversion** (verified: the registry is thread-blind), and a
single-threaded executor *false-panics* on independent tasks (`!Send` closes neither direction). So the
**granularity axis is a containment-scope LADDER — token < thread < task** — and each ambient Rust floor
reaches exactly one rung (affine token #15, thread-local #16); there is no ambient *task-local*. **A
partial match is not lifted to full by upgrading the floor — the upgrade trades one gap for a finer one and
spends cost-basis axes (#5, #6) plus a modality downgrade doing it**, coupling the graded match to the
closed cost-basis. Prior art forecloses the static-rank design entirely: FreeBSD **WITNESS** / lockdep /
Abseil all abandoned static ranks for dynamically-*learned* acquires-before graphs (an even higher floor,
living wholly in the detection modality).

**…and the match-unit is the *precondition*, not the *domain* — a seventeenth primitive, breaking
breadth-wise, proves it.** Instead of climbing the ladder again, probe a *new precondition* in the *same*
liveness domain: **"this finalizer MUST run"** (a `MustRun<F: FnOnce()>` cleanup guard, `core`-only). The
cold panel ruled **SEAM-BREAKS [horn 1], FAILED** across **two** rounds (r23 + r24, breaking attackers
included), the *fifth* broken seam — and the break is compiler-settled: `mem::forget` is *safe*, so `Drop`
never fires and the finalizer runs zero times (the leakpocalypse — RFC 1066 / 1084). Three keeps. (a) The
grade is a clean **none** at the type level; the "partial-via-`#[must_use]`-lint" texture was **rejected**
(`scopeguard` ships that exact warning as a cosmetic advisory over a non-guarantee, not a floor fragment).
(b) So the *same* liveness domain grades deadlock-freedom **partial** (#15/#16) but must-run-cleanup
**none** — the floor-match unit is the **precondition, not the domain**, sharpening the Phase-M
domain-parameterized result. (c) The verdict mode is the **ninth, new one — *wrong-structural-logic /
out-of-frame***: every sound *decidable* fix collapses to a viral `Leak`/linear bound (the alternative,
heap-reachability, is undecidable ≅ halting), so the floor is the wrong *structural logic* (affine vs
linear), not the wrong *domain* — sharpened by prior art to Boats' *scoped-task trilemma*, of which Rust
deliberately rejected the linear horn. It is the **enforcement** face of the ninth axis (leak-freedom),
inverse to `drain` (#10, the *tolerance* face), so **N is unchanged**. A coda the two rounds taught: a seam
*designed to break* cannot be coherently dressed as usable — both FAILED on spec-coherence, not the break
(the disarm/commit-rollback mismatch, the fallible-finalizer scope), which is itself positive evidence for
the verdict.

The taxonomy is now a map of *what the missing conjunct is* **and which axis (or category) closing it spends**:
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
- **A transient broken invariant across a possible unwind** → closes by spending **unwind-safety** (the
  8th axis), with a *discharge menu*: abort (lose recoverability), placeholder-repair (pay API-surface),
  or poisoning (take-and-replace; `take_mut`↔`replace_with`).
- **A guard whose `Drop` is load-bearing, on a leak-permitting floor** → closes by spending
  **leak-freedom** (the 9th axis): pre-pessimize so a leaked guard loses data but never double-frees
  (drain-and-yield; `Vec::drain`). Distinct from unwind-safety (leak = `Drop` never runs).
- **A value whose *address* is load-bearing** → closes via `Pin`, but the cost is **inseparable**
  (immovability — non-uniform: stack-pin → API-locality, heap-pin → bare-metal). Move-stability is a
  real, distinct danger with **no clean coordinate** (self-ref cell; `Pin`). N stays 9 — a new category.
- **A value whose *bit pattern* must be valid** → closes via a set-membership check, projecting onto
  **zero-cost #6** for a single read, but **inseparable** for the zero-copy view (per-read #6 vs sealed
  ctor API-locality). Validity adds **no new axis** — the closed-basis proof (validity seam; `num_enum`).
- **An external physical axiom** → sound modulo a TCB `assume`, outside the axes (MMIO).
- **Whole-heap aliasing+liveness that escapes to the caller** → no sound+usable+concentrated
  decision exists; a trilemma horn genuinely drops (intrusive list).
- **A finalizer that *must run* on a leak-permitting floor** → no sound decision exists either, but for a
  *structural-logic* reason, not a domain one: a *safe* `mem::forget` skips the `Drop`, and no decidable fix
  buys it back (affine "at most once" cannot compel linear "exactly once"). The *inverse* of `drain` (which
  closed by tolerating a skipped `Drop`); the **enforcement** face of leak-freedom, N unchanged
  (must-run-cleanup guard; `scopeguard` / leakpocalypse).

**The honest meta-pattern, and the closed basis.** Every recovered axis turns out to be an
*already-named PL concern* — zero-cost abstraction, generativity, refinement-legibility, the
`core`/`alloc` split, exception-safety, leak-freedom. The phase did not find new physics; it found that
the trilemma's three horns silently *bundle a set of independently-known PL design axes*, unified by one
rubric: **what does the safe witness cost?** The cold panel showed **five** adjudication modes — it
**confirmed-but-deflated** three new dimensions to "real but pre-existing" (bare-metal, unwind-safety,
leak-freedom), **sharpened without extending** once (solver-freedom = horn 2's "keep" face), **rejected
outright** once (checker-tractability, confounded), ruled two **real but structurally inseparable**
(move-stability, validity — a recurring category), and — the capstone — certified a new danger class
**projects entirely onto the existing basis** (validity adds no tenth axis). That last mode is what
*closes* the basis: the cost-space is a small fixed set — nine clean axes + an inseparable category + two
outside-the-axes — that dangers project onto, with finitely many answers to "what does the witness cost?"
The full face lattice (axes, dissociation proofs, the five-mode meta-pattern, the closed-basis section)
is `REALIZABILITY.md`.

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
- **Take-and-replace — it moved onto the *control-flow / unwind path*, and stayed sound.** The
  obstruction (the transient broken invariant) factors through an *abort-bomb* witness, so the seam
  closes (`take_seam_closes`, like SPSC) — but the relocation lands neither in code, type, runtime
  (happy path), toolchain, nor heap: it lands on the **unwinding behavior** — a mid-take panic becomes
  a process abort. The obstruction moved onto the exception-safety guarantee level (the 8th axis).
- **Drain-and-yield — it moved onto the *leak path*, and stayed sound.** The obstruction (a guard whose
  `Drop` is load-bearing) factors through *pre-pessimization*, so the seam closes (`drain_seam_closes`,
  like SPSC) — but the relocation lands neither in code, type, runtime (happy path), toolchain, heap,
  nor the unwind path: it lands on the **leak path** — `mem::forget` the guard and the un-yielded tail
  is silently lost. The obstruction moved onto leak-freedom (the 9th axis), distinct from unwind-safety.
- **Self-ref cell — it moved onto *immovability*, and stayed sound.** The obstruction (a value's address
  being load-bearing) factors through `Pin` over a `!Unpin` cell, so the seam closes (`selfref_seam_closes`,
  like SPSC) — but the relocation lands on the **immovability** of the value (the revoked move/return/store
  capability), and *which named axis that spends is non-uniform* (stack-pin → API-locality, `Box::pin` →
  bare-metal). So unlike the previous nine, the obstruction moved to a place that is **not a single clean
  axis** — move-stability is real but inseparable (N stays 9).
- **Validity — it moved onto an *already-occupied* place, and stayed sound.** The obstruction (a byte's
  bit-level validity) factors through a set-membership check, so the seam closes (`validity_seam_closes`,
  like SPSC) — and the relocation lands on a coordinate *already on the map*: **zero-cost #6** for a
  single read (the slab's place, a different predicate), or **API-locality** for the zero-copy view. The
  obstruction moved, stayed sound, and added *nothing new* — the **closed-basis** signature.

"Moves, doesn't vanish" is confirmed for the eleven non-trivially-encapsulable primitives — and
*where* it moves (decidable boundary bound / runtime witness / type surface / trusted toolchain /
runtime subsystem / `recursion_limit` fuel knob / unwind path / leak path / immovability / *an
already-occupied axis* / genuinely unsound) is the whole verdict; "vanishes" is the signature of the
fully-encapsulable one. The decontamination method generalized cleanly across all twelve — including the
five axis probes past seven: one moved to a place (the fuel knob) that was *not* a clean axis, two (the
unwind path, the leak path) that **were** clean distinct axes, one (immovability) to a **real danger with
no clean coordinate** (inseparable), and one (validity) to a place that was **already on the map** — the
last one closing the basis.

## What the harness added that the builder missed

The adversarial process earned its keep eighteen times:

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
8. **Take-and-replace (the second eighth-axis probe):** the builder pitched **unwind-safety** as the
   eighth axis, having designed the seam to avoid the type-level index's confound (it aces all seven
   prior axes, pays only U2). The panel **confirmed it (N ≥ 8)** and explicitly ruled the off-diagonal
   *clean* (contrasting it with the rejected typed_nat). Two sharp corrections, both recorded: (a) the
   builder **mislabeled the axis as "the abort cost"** — it is the *exception-safety-guarantee-level*,
   and abort is one **discharge** of it (`replace_with` repairs the gap recoverably, paying API-surface;
   the axis carries a discharge menu); (b) prior-art surfaced that **RFC #1508 rejected this exact
   `take_mut` from std** on the very abort cost scored — the "ace seven, pay one" framing was the
   *historically losing* argument. Net: a clean eighth axis, deflated to pre-existing — the discovery
   half of the instrument, complementing the rejection half from probe 7.
9. **Drain-and-yield (the ninth axis):** the builder pitched **leak-freedom**, having reverse-engineered
   the primitive from the off-diagonal requirement — a free-standing guard (no closure) so it would ace
   API-locality, the confound the obvious scoped-thread witness would have hit. The panel **confirmed it
   (N ≥ 9)**, ruled it **distinct from unwind-safety** (off-diagonal populated both ways: take is
   unwind-sensitive/leak-safe, drain is leak-sensitive/unwind-safe — refusing to merge the two
   destructor-execution siblings), and confirmed no confound (it aces API-locality *because* the guard
   owns its data). One sharp recorded nuance: leak-freedom is the **thinnest, most-deflatable** axis (its
   cost is invisible in non-leak code), yet still a genuinely populated off-diagonal — "deflatable" and
   "not an axis" are different verdicts (probe 7 was the latter, this the former).
10. **Self-ref cell (the inseparable verdict):** the builder pitched **move-stability** as a tenth axis,
    honestly flagging the confound risk. The panel returned the project's most novel ruling — **real but
    structurally inseparable** — and corrected three framings: (a) the cost is **immovability** (revoked
    move/return/store), heavier than the builder's claimed "API-locality alone"; (b) "no free-standing
    witness *can* exist" overstates it — RFC 2349's *rejected* `?Move` immovable types were a designed
    free-standing witness, so Rust's denial is *contingent*; (c) the crisp `core`-only reading is a
    **non-composable toy** (real intrusive nodes / futures force `unsafe` pin-projection or `Box::pin`).
    This is the keep that *extended the method itself*: a fourth verdict mode (beyond confirm / sharpen /
    reject) — a real danger that cannot be made a clean coordinate. N stays 9.
11. **Validity (the closed-basis capstone):** the builder pitched a new danger class (bit-level validity)
    *not* to hunt a tenth axis but to test closure, predicting it projects onto zero-cost #6. The panel
    earned its keep to the last: two of three gauntlets found real defects — a **range** check unsound for
    non-contiguous discriminants (the headline bug, four attackers converging), then a residual stale
    `b<=2` the builder left when fixing the first (a self-inflicted miss). The corrected seam survived,
    and the verdict refined the builder's prediction: validity is **(3) inseparable** (the zero-copy view
    splits #6 vs API-locality), a *second* inseparable danger — but the **meta-claim is unanimous: no
    tenth axis, the cost-basis is closed.** Non-novel (`num_enum`/`bytemuck`/`zerocopy`). The keep that
    *concluded the arc*: a brand-new danger projecting cleanly onto the basis is what closure looks like.
12. **Constant-time secret (the generalization):** the builder stepped outside memory safety to test
    whether closure is absolute, predicting information-flow is *out of frame*. The panel **vindicated the
    domain hypothesis and enriched it** — but **broke the seam** (SEAM-BREAKS, IFC sense): two attackers
    independently found that `ct_eq(a,b).expose_bool()` then `if b` leaks a secret relationship using only
    vended ops — *declassification is etiquette, not type-structure*, the intrusive-list horn-drop re-run
    in a new domain. And two corrections: the §4 universal "no source-level leak" is false; constant-time
    codegen is **dischargeable** (CompCert-CT/Jasmin/FaCT), *not* MMIO-shaped. The keep that *generalized
    the result*: the IFC domain has its own axis (**declassification discipline**, no memory-safety
    analog), the trilemma recurs sharper, and closure is **per domain**.
13. **Capability-gated declassification (the floor-domain lesson):** the builder probed whether the IFC
    domain has a *closing* seam, paying declassification discipline as a clean axis. The panel **broke it
    5/5** — and the breaks *deepened* the result: `map`/`zip` hands the closure the bare plaintext (an
    uncapped exit, **unpatchable without an effect system**), and the affine cap bounds *uses*, not *bits*
    (prior-art: it enforces Sabelfeld–Sands' *where/when*, not the *amount* — **quantitative info-flow**).
    What it enforces *projects onto the affine memory-safety axis*; the real IFC coordinate an affine floor
    **cannot** bound. So both IFC seams break, for one reason — **Rust's floor carries none of the IFC
    precondition.** The keep that *finished the generalization*: a closing seam exists only when the
    **floor matches the domain**; the affine floor closes memory-safety seams and *no* IFC seam.
14. **Type-level lock ordering (the floor-domain match is GRADED):** the builder probed the *positive*
    half — does a closing seam reappear in a *third* domain (**liveness**) once the floor (a type-level
    rank order) carries its precondition? The panel **broke it [horn 1], FAILED** (3/5), and the break
    grades the match into **full / partial / none**. Two failure dimensions, both "the floor under-carries
    the precondition": **granularity** — re-entrant `Floor::root` mints a second lineage, so safe code
    types a low-then-high inversion (verified compiles+runs); the discipline is per-token, the precondition
    per-thread-global, and Rust has no ambient per-thread one-shot resource (even the *static* fragment
    fails) — and **coverage** — the *dynamic*-acyclicity fragment (runtime/same-rank locks) escapes the
    *static* floor, as IFC's quantitative coordinate escaped the affine one. Prior art forces the
    partial-match ruling (Boyapati–Lee–Rinard OOPSLA'02 *minus* their dynamic extension; Kobayashi's
    natural-number levels; Havender 1968). The keep: name it **lock-order-inversion freedom**, not
    "deadlock-freedom"; the affine-floor framing is overstated (the order predates affinity 20y).
15. **Runtime lock registry (the floor-upgrade is non-monotone; granularity is a ladder):** the obvious
    repair to #14 is to *upgrade the floor* — a `thread_local!` registry of **runtime** ranks (lockdep-in-
    miniature) — to lift the partial match. The panel **broke it [horn 1], FAILED** (r22), the *fourth*
    broken seam, because the upgrade is **non-monotone**. It fixed **coverage** (runtime-chosen-fixed ranks
    close — bank-transfer, hand-over-hand) and paid the predicted price (**#5 bare-metal** — needs OS
    thread-local, *cannot be `no_std`*; **#6** a per-acquire check; a **prevention→detection modality** —
    soundness path-coverage-dependent). But **granularity recurred one rung finer**: `HELD` is keyed on the
    OS *thread*, while async *tasks* are the control unit — work-stealing migration launders an undetected
    inversion (verified: thread-blind), and a single-threaded executor false-panics (`!Send` closes
    neither). So the **granularity axis is a containment-scope LADDER — token < thread < task** — no ambient
    Rust floor tops out (there is no task-local). The keep: **a partial match is not lifted to full by
    upgrading the floor** — the upgrade trades one gap for a finer one and spends cost-basis axes (#5, #6)
    plus a modality downgrade, coupling the graded match to the closed cost-basis. Prior art forecloses
    static ranks entirely (FreeBSD WITNESS / lockdep / Abseil → learned acquires-before graphs).
16. **Must-run cleanup guard (breadth-wise — the match-unit is the *precondition*; a NEW mode,
    wrong-structural-logic):** rather than climb the ladder again, probe a new *precondition* — "this
    finalizer MUST run" — against the affine floor. The panel **broke it [horn 1], FAILED** across **two**
    rounds (r23 + r24, breaking attackers included), the *fifth* broken seam: `mem::forget` is *safe*, so
    `Drop` never fires and the finalizer runs zero times (leakpocalypse, RFC 1066/1084). The grade is a
    clean **none** at the type level — the "partial-via-`#[must_use]`-lint" texture was **rejected**
    (`scopeguard` ships that exact warning as a cosmetic advisory, not a floor fragment). The keeps: (a) the
    floor-match unit is the **precondition**, not the *domain* — the *same* liveness domain grades
    deadlock-freedom *partial* (#14/#15) but must-run-cleanup *none*; (b) it is the **enforcement** face of
    the ninth axis (leak-freedom), inverse to `drain` (#10, the *tolerance* face), so **N is unchanged**;
    (c) a **new method mode — *wrong-structural-logic* / out-of-frame**: every sound *decidable* fix
    collapses to a viral `Leak`/linear bound (the alternative, heap-reachability, is undecidable ≅ halting),
    so the floor is the wrong *structural logic* (affine "at most once" vs linear "exactly once"), not the
    wrong *domain* — sharpened to Boats' *scoped-task trilemma* (Rust rejected the linear horn). Both rounds
    FAILED on *spec-coherence*, not the break — a seam designed to break cannot be coherently dressed as
    usable, which is itself evidence for the verdict.
17. **Session-typed channel (the composite — same gap, different horn; the first out-of-domain HOLD):** a
    `Chan<P>` affine typestate handle, built as the crucible for #15 (typestate carries static **order**) and
    #17 (the floor cannot compel **completion**). The panel ruled **SEAM-COSTS [horn 3]** (TRADEOFF, **survived
    1/1**, r25) — the **first** out-of-(memory-safety) probe to *hold with a cost* rather than break. The floor
    matches this *composite* precondition **component-wise**, and the components are *orthogonal* (the
    soundness attacker confirmed `mem::forget` cannot shorten `'a` to start a second session, so completion
    cannot infect safety): protocol **order** *closes* via affine typestate (sound, decidable, concentrated;
    **no `root`-hole**, unlike #15), while **completion** is an undischargeable horn-3 *concentration* cost
    (the #17 linearity gap, recurring at every `?`/panic/early-return). The illumination: the *same*
    `mem::forget` gap that **broke** #17's horn 1 only **costs** here, because completion was #17's *whole*
    promise but is *one component beside a sound safety core* here — **which horn a danger threatens is set by
    how the rule packages it.** Three deflations recorded: it is a single-handle typestate **walk**, not a
    binary session (`core`/no-`unsafe` forecloses dual endpoints, `E0499`; the abandonment is *self*-
    abandonment); "usable" holds only for *infallible straight-line* protocols; and the split is documented
    **folklore** (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 "Stay Safe Under Panic") — the
    contribution is *locating* it on the lattice, not discovering it.

## Artifacts

- Specs (attacked as text): `SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat,take,drain,selfref,validity,secret,labeled,ordered_locks,lock_registry,linear,session}.md`
- Cold verdicts: `REPORT_SEAM_SPEC_{spsc,mmio,intrusive,slab,branded,refined,shared,typed_nat,take,drain,selfref,validity,secret,labeled,ordered_locks,lock_registry,linear,session}.md` (+ raw transcripts in `runs_seam/`)
- Real Rust (rustc-checked, `#![no_std]`): `../seam/` — `src/lib.rs` (SPSC), `src/mmio.rs`,
  `src/intrusive.rs`, `src/slab.rs`, `src/branded.rs`, `src/refined.rs`, `src/shared.rs` (the lone
  `alloc` user), `src/typed_nat.rs` (eighth-axis probe 1 — sound, no new axis), `src/take.rs`
  (eighth-axis probe 2 — the confirmed unwind-safety axis), `src/drain.rs` (the confirmed leak-freedom
  axis), `src/selfref.rs` (move-stability — real but inseparable), `src/validity.rs` (the closed-basis
  capstone — a new danger projects onto the basis), `src/secret.rs` + `src/labeled.rs` (the two IFC-domain
  seams — *both break*; closure is per-domain, and no IFC seam closes on Rust's affine floor),
  `src/ordered_locks.rs` (the third domain — liveness; *breaks*, grading the floor-domain match
  full/partial/none), `src/lock_registry.rs` (the floor-UPGRADE probe — feature `lockdep`, needs `std`;
  *breaks*, the upgrade is non-monotone — granularity recurs token→thread→task), `src/linear.rs` (the
  breadth-wise *linearity* probe — `core`-only; *breaks*, a clean **none** grade and a new
  *wrong-structural-logic* mode — affine vs linear), `src/session.rs` (the *composite-precondition* probe —
  `core`-only, no `unsafe`; *holds with a cost* — SEAM-COSTS [horn 3]: order-safety closes, completion costs;
  the same `mem::forget` gap that broke #17's horn 1 only costs here); consumer code is
  `#![forbid(unsafe_code)]`; laundering paths are `compile_fail`
  doctests (65 `tests/` integration tests + 33 doctests = 98 total in the default `no_std` build; with
  `--features lockdep` the runtime registry adds its tests for **105 total**).
- Seam-width instrument: `SEAM_WIDTH.md`; full face lattice: `REALIZABILITY.md`
- Mechanization: `../proof/SeamGap.lean` — `seam_closes_iff_factors` + the
  `spsc`/`intrusive`/`slab`/`branded`/`refined`/`shared`/`typed_nat`/`take`/`drain`/`selfref`/`validity` instances,
  plus `secret_typed_discipline_factors`, `labeled_typed_discipline_factors`,
  `ordered_locks_typed_discipline_factors`, `lock_registry_typed_discipline_factors`, and
  `linear_typed_discipline_factors` (the two IFC seams' + the three liveness seams' trivial sub-facts —
  all five break), plus `session_order_safety_closes` (the eighteenth seam *holds* with a horn-3 cost — its
  order-safety component closes like a closing seam; the completion cost is modeling-dictionary) (propext-only).

## The one-sentence upgrade to the trilemma

The Seam Trilemma says *a* seamless language is impossible; the constructive phase says
**which** seams you can build losslessly anyway — exactly the ones whose dangerous precondition
is fully encapsulable (SPSC-shaped) — while the rest force a predictable, measurable price set by
*what the missing conjunct is*: a runtime witness that spends zero-cost (slab), a viral generative
brand that spends API-locality (branded index), a legible predicate that spends solver-freedom
(refined index), a refcount manager that spends bare-metal (shared), an external TCB axiom (MMIO), or
a genuine horn drop (intrusive), an immovability that has no clean coordinate at all (self-ref cell), or
a bit-validity that projects onto a coordinate *already on the map* (validity) — and the trilemma itself
turns out to be a **3-of-N projection of a *closed* object (N ≥ 9)**: its three horns a shadow of **nine
clean axes** (zero-cost #6, API-locality, AI-legibility #7, bare-metal #5, unwind-safety, leak-freedom,
plus the original sound/decidable/concentration), an **inseparable category** (move-stability, validity),
and **two outside-the-axes** (the MMIO TCB axiom, the intrusive horn-drop) — each surfacing only when a
primitive becomes the witness that two previously-identical configurations differ (and one candidate,
checker-tractability, was **rejected** as confounded; the basis shown **closed** when a brand-new danger
class projected cleanly onto it — the method discriminates in nine modes). Honest coda: those axes are
each a *pre-existing* named PL boundary; the result is their unification under one "what does the witness
cost?" rubric — a question with **finitely many answers** — not their discovery. **And two more
generalizations**: that closed object is the **memory-safety** domain's; a constant-time secret showed
information-flow is a *different* domain with its *own* basis, so the Seam Trilemma is
**domain-parameterized** — the same shape, a different basis, in every soundness domain; and a
capability-gated declassification showed that *which seams close* depends on the **floor-domain match** —
Rust's affine floor closes memory-safety seams but *no* information-flow seam (it bounds resource *uses*,
not information *bits*; the IFC coordinate needs a quantitative/effect floor — Jif, FlowCaml, QIF); and a
third domain — type-level lock ordering (**liveness**) — showed that match is **graded, not binary**: it
breaks two ways (a per-token-vs-per-thread *granularity* gap, and a *static-vs-dynamic* coverage gap, the
dynamic-acyclicity coordinate escaping the static floor as IFC's quantitative one escaped the affine
floor), so liveness on a static-order floor is a **partial** match; and a sixteenth — a *runtime* lock
registry on an **upgraded** floor — showed that grade is **not monotonically liftable**: upgrading the
floor fixed coverage but **granularity recurred one rung finer** (thread-keyed vs task-keyed control), so
the granularity axis is a **containment-scope ladder** (token < thread < task) no ambient floor tops out,
and the lift was *bought* with cost-basis axes (#5, #6) plus a prevention→detection modality; and a
seventeenth — a *must-run cleanup guard*, probing **breadth-wise** — showed the floor-match unit is the
**precondition, not the domain** (the *same* liveness domain grades deadlock-freedom *partial* but
must-run-cleanup *none*) and that a floor can be the wrong *structural logic* entirely (affine "at most
once" vs linear "exactly once": a *safe* `mem::forget` skips the finalizer, no decidable fix buys it back —
the ninth, *wrong-structural-logic* mode); and an eighteenth — a *session-typed channel* — was the
**first** out-of-(memory-safety) probe to *hold with a cost* (SEAM-COSTS [horn 3]), showing the floor
matches a *composite* precondition **component-wise** (protocol order *closes* via affine typestate;
completion is a horn-3 *concentration* cost — the #17 linearity gap), and that the *same* `mem::forget` gap
that **broke** #17's horn 1 only **costs** here, because completion is one component beside a sound safety
core: *which horn a danger threatens is set by how the rule packages it*. So the final shape of the result:
**the Seam Trilemma is a domain-general tension; each soundness domain has its own cost-basis; a floor
closes a domain's seams only to the degree it carries that *precondition* — a graded match, full (memory
safety) / partial-to-none (liveness, by precondition) / none (information flow) — that grade cannot be
freely lifted (upgrading the floor trades one gap for a finer one and spends the closed cost-basis), the
floor can fail a precondition not only by domain but by *structural logic* (affine vs linear), and a single
*composite* precondition can be matched *component-wise* — one horn closing while another costs, with which
horn a danger threatens set by how the re-import rule packages it.**
