# Seam-width metric (horn-3 instrument)

**Definition.** A *seam crossing* in consumer code is a site that either (a) writes the
`unsafe` keyword, or (b) crosses the foundation boundary (calls the seam constructor that
mints the affine handles — `split` / `take`). **Seam width** = number of crossings the
ordinary consumer must perform to *use* the primitive, as a function of how much it uses it.
Horn 3 (concentration) holds iff width is bounded by the number of *primitives instantiated*,
not the number of *operations performed*.

This is the greenfield instrument the phase prompt asks for (the orchestrator had no such
metric). It is computed from the real Rust in `../seam/`, not asserted.

## SPSC ring buffer — measured

| Side | what | count |
|---|---|---|
| **Foundation** (`seam/src/lib.rs`) | `unsafe` tokens total | 12 |
| | `unsafe impl Send`/`Sync` (auto-trait bounds) | 3 |
| | `unsafe {}` protocol blocks (P1–P3) + `unsafe fn take`-style | 4 |
| **Consumer** (`seam/tests/sound_use.rs`) | `unsafe` in *code* | **0** (compile-enforced by `#![forbid(unsafe_code)]`) |
| | `unsafe` in comments (prose only) | 2 |
| | seam crossings (`.split`) | 3 — **one per buffer** (3 buffers across 3 tests) |
| | free uses (`.push`/`.pop`) | 12 static call sites |
| | runtime element ops in the 100k stress test | ~200,000, **all seam-free** |

**Result: seam width = O(1) per buffer, O(0) per element.** The consumer crosses the seam
exactly once, at construction; every subsequent operation is an ordinary safe method call.
All `unsafe` is concentrated in the foundation. **Concentration (horn 3) holds.**

## Why — the structural reason

SPSC's rich precondition factors so that *no hard conjunct escapes to the caller*:
- The dangerous facts (P1 per-slot exclusion, P2 happens-before, P3 index discipline) are
  fully internal to the atomics — they leave no caller-visible proposition.
- The conjuncts that *do* concern the caller are all floor-expressible: cardinality (P4) →
  affinity, storage lifetime → NLL `'a` on `split`, payload thread-safety (P5) → a decidable
  `T: Send` bound resolved by the compiler, never written by the consumer.

A primitive whose precondition has a conjunct that *cannot* be internalized (one that names
caller-owned state) would force the caller to carry it — pushing seam width from O(buffers)
toward O(operations). That is the Phase-C contrast (MMIO; intrusive list).

## All twelve primitives — and why seam width misses *several* axes, not one

| Primitive | Consumer seam width | Runtime cost (zero-cost #6) | Type surface (API-locality) |
|---|---|---|---|
| SPSC ring buffer | **O(1)/buffer** (`split` once) | none — fully erased | plain `'a` — non-viral |
| MMIO read-to-clear | **O(1)** with a foundation `Send` opt-in, else O(ISRs) | none — fully erased | plain — non-viral |
| Generational slab | **O(1)** — `insert`/`get`/`remove` are all safe calls | **non-zero** — a generation word/slot + a compare branch/access, never erased | plain `Key` — non-viral |
| Branded index | **O(1)/slice** (`scope` once) | erases, but **deferred** — single-index `try_index` re-checks per element; the elision win needs range-branding | **viral `'id`** — infects every index/signature, traps indices in a `scope` closure |
| Refined index | **O(1)/index** (`refine` once) | erases on a real Flux floor; the cost moves to an **SMT solver in the TCB** (horn 2) | plain `Refined` — non-viral; the value *escapes* |
| Shared (Rc) | **O(1)** — `new`/`clone`/deref/drop are safe calls | **non-zero** — a refcount inc/dec, never erased — *and* a **global allocator** (bare-metal #5) | plain `Shared` — non-viral |
| Type-level index | **O(1)/buffer** (`Bound::new` once) | **erases fully** — the proof is a trait bound; cost moves to the **resolver's fuel** (`recursion_limit`, horn 2) — but that cost is encoding-relative + confounded, *not a clean axis* | plain turbofish `I` — non-viral; **but static-index-only** (a hidden *expressiveness* cost no width/cost/type column captures) |
| Take-and-replace | **O(1)/call** (`take(slot,f)` once) | erases on the happy path (the abort-bomb is a ZST `mem::forget`-ten); mild codegen dent (an optimization barrier) | plain `take` — non-viral; **but a mid-take panic aborts the process** (the 8th axis, **unwind-safety** — a hidden cost no width/cost/type column captures) |
| Drain-and-yield | **O(1)/drain** (`drain()` once) | erases — ordinary iteration, no proof residue | plain `Drain<'a>` — non-viral, **no closure**; **but a leaked guard silently loses the un-yielded tail** (the 9th axis, **leak-freedom** — a hidden cost no width/cost/type column captures) |
| Self-ref cell | **O(1)/cell** (`init` once) | erases — `Pin` is a marker, no runtime residue | **viral, `&mut`-revoking `Pin`** — flips API-locality; *and* the deeper cost is **immovability** (move-stability), which is **inseparable** — *not* a clean column at all (N stays 9) |
| Validity (bytes→enum) | **O(1)/read** (`tag_at`) | **non-zero** — a set-membership tag check per read (zero-cost #6, the slab's place via a different predicate) | plain `Tag`; **but the zero-copy `&[u8]→&[Tag]` view needs a sealed ctor (API-locality)** — so the cost is **inseparable**, and it adds **no new axis** (the closed-basis proof) |
| Intrusive list (branded) | **O(accesses)** — the token threads through every access | (design A erases; the cost is seam width / lost reclaim) | **viral `'brand`** |

**Two instructive rows now show seam width is necessary but not sufficient.** The slab *aces* seam
width (horn 3) yet pays on **zero-cost (#6)** — concentration asks "how often does ordinary *code*
touch the seam," erasure asks "does the witness survive to run time," and they are independent (the
slab's witness is read inside the seam — width O(1) — but is a runtime load-and-branch). The
**branded index** *also* aces seam width (one `scope` per slice) yet pays on **API-locality (#5)**:
its witness never appears in consumer *code* as a crossing, but it appears in every consumer *type*
as a viral `'id`. So "concentration" was itself a bundle of two faces — a *code* face (seam-width,
horn 3) and a *type* face (API-locality) — that the branded index pulls apart, exactly as the slab
pulled zero-cost out of "concentrated & proof-free."

That a primitive can be O(1) on this instrument and *still* impose a real consumer tax on two
distinct hidden axes is precisely why the Seam Trilemma is a **3-of-N projection (N ≥ 9)**, not
3-of-3. Seam-width measures one axis well and is blind to the rest.

**Four more rows sharpen the same point from the opposite direction — and are a caution against both
over- and under-counting.** The **type-level index** is O(1) on width, *fully erases*, and is non-viral
— a near-perfect corner — yet its real tax, **static-index-only expressiveness** (it cannot index a
runtime `usize`), is invisible to *all three* columns. That invisibility is what let the builder
mistake it for a clean "checker-tractability" axis, and what the cold panel caught: an instrument blind
to expressiveness reads a confounded off-diagonal as clean, so the eighth-axis claim there was
**rejected**. The **take-and-replace** and **drain-and-yield** rows are the mirror image: both ace
width, erase on the happy path, and are non-viral — yet their real taxes, **unwind-safety** (a mid-take
panic aborts) and **leak-freedom** (a leaked guard loses the tail), are *also* invisible to all three
columns. But this time the hidden costs were **clean, and distinct, axes**: the panel **confirmed** them
as the eighth and ninth (N ≥ 9), and ruled them dissociable (leak = `Drop` never runs; unwind = `Drop`
runs at a bad time). The **self-ref cell** completes the lesson with a *third* outcome: its real tax —
**move-stability / immovability** — is *also* a danger no width/cost column captures, but it is *neither*
a clean axis *nor* a confounded non-axis. The panel ruled it **real but structurally inseparable**: a
distinct danger with no isolable cost (immovability is non-uniform — stack-pin spends API-locality,
`Box::pin` spends bare-metal). And the **validity** seam adds a *fourth* outcome — the one that closes
the basis: its hidden cost (bit-level validity) is *also* invisible to the width column, but it lands on
a coordinate the instrument *already has* (#6 per read, API-locality for the zero-copy view), adding **no
new axis**. So a primitive that aces every column the instrument can see may be paying on a hidden cost
that is *a real clean axis* (take, drain), a *confounded non-axis* (type-level index), *a real danger
with no clean coordinate at all* (self-ref cell), or *a real danger that projects onto a coordinate
already on the map* (validity) — and only a richer adjudication (the cold panel) tells the four apart.
That last outcome — a brand-new danger class adding nothing new — is what shows the cost-basis is
**closed**. See `REALIZABILITY.md` for the full face lattice (the confirmed axes, the rejected probe, the
inseparable category, and the closed-basis capstone) and `SYNTHESIS.md` for the cross-primitive map.

**A note on the five out-of-domain primitives — and the instrument's domain.** The constant-time secret
(`seam/src/secret.rs`), the capability-gated declassification (`seam/src/labeled.rs`), the type-level
lock ordering (`seam/src/ordered_locks.rs`), the runtime lock-order registry
(`seam/src/lock_registry.rs`, feature `lockdep`), and the must-run cleanup guard (`seam/src/linear.rs`,
`core`-only) are deliberately *not* in the table above, because this instrument cannot measure them: the
two information-flow primitives have **no `unsafe` keyword** (the "seam crossing" the metric counts) and
their danger is an *information leak*; the three liveness primitives' danger is a *deadlock* or a
*skipped finalizer* (liveness/correctness hazards) — none is UB. That is the point. Seam-width — and the
whole nine-axis basis — is a **memory-safety** instrument; these five belong to **other soundness domains**
(information-flow / non-interference; liveness / progress), and **all five break** there: the secret seam
(declassification is etiquette), the labeled seam (`map` hands the closure bare plaintext; the affine
"budget" counts *uses*, not *bits*), the static lock-ordering seam (re-entrant `Floor::root` mints a second
token lineage → a typable lock-order inversion; and the dynamic-acyclicity fragment escapes the static
rank floor), the runtime registry (the *floor-upgrade* probe: it fixes coverage but `HELD` is keyed on
the OS *thread* while async control is per-*task*, so work-stealing migration launders an undetected
inversion), and the linearity seam (a *safe* `mem::forget` skips the finalizer — affine "at most once"
cannot compel "exactly once"). The deeper reason the metric can't see them is the deeper result: **a
closing seam — and a measuring instrument — exists only when the *floor* matches the *domain*, and that
match is GRADED — upgrading the floor is NON-MONOTONE — and the floor can even be the wrong *structural
logic* (the linearity seam: affine vs linear) rather than merely an unmatched domain.** Rust's affine
floor is **fully** matched to memory safety (where seam-width is well-defined and the basis is closed),
**not** matched to information flow (no seam closes;
the real coordinate, *quantitative leakage*, is unmeasurable by an affine/borrow type system), and only
**partially** matched to liveness — and the registry shows that match cannot be lifted by simply picking a
stronger floor. A *static* type-level-order floor carries the static distinct-rank single-stack fragment
but leaks the *dynamic* acyclicity coordinate (runtime-ordered/same-rank locks, what `lockdep` checks at
runtime) and leaks on *granularity* (per-*value* affinity where the precondition is per-*thread*).
*Upgrading* to a runtime `thread_local!` registry buys back the dynamic coordinate — at the cost of
**bare-metal #5** (it needs OS thread-local, so it **cannot be `no_std`**: the `lockdep` feature gate makes
that cost *structural and visible in the build graph*, not measurable by seam-width either), a per-acquire
runtime check (**#6**), and a *prevention→detection* modality shift — yet **granularity recurs one rung
finer** (the held-set is per-*thread*, but tasks are the control unit). The "granularity" coordinate is a
containment-scope **ladder** — token < thread < task < … — and no ambient Rust floor tops it out. Each
domain needs its own floor and its own instruments — a quantitative/effect type system (Jif, FlowCaml, QIF)
for information flow; a runtime, learned acquires-before graph (FreeBSD WITNESS, `lockdep`, Abseil — all of
which abandoned static ranks) or a *task-local* discipline for liveness — for domains whose axes have no
memory-safety analog. (A *sixth* out-of-memory-safety primitive — the session-typed channel below — is the
exception that proves the rule: it does **not** break, because its *order-safety* component genuinely
**closes** and only its *completion* component costs, so the broken count stays **five**.)

**A fifth primitive breaks — and it sharpens what the *match* is a match *of*.** The must-run cleanup
guard (`seam/src/linear.rs`, a `MustRun<F: FnOnce()>` RAII finalizer — core-only, *in* the default
`no_std` build) is the seventeenth seam and the **fifth to break**. It is *not* a new domain — its
danger is **must-run-cleanup** (a liveness/correctness hazard — the finalizer never runs — not UB), the
*same* liveness domain as the two lock seams (so "out-of-domain" above means *outside memory safety*, not
a fourth domain). It earns no width row for the usual reason (it breaks:
`mem::forget` is safe, so `Drop` is not mandatory — the leakpocalypse), and it refines the graded-match
result two ways. **(i) The unit of the match is the *precondition*, not the *domain*:** within one liveness
domain, deadlock-freedom grades *partial* (the lock seams) while must-run-cleanup grades a clean *none* —
so "which domain" was too coarse; it is "which precondition." **(ii) A new verdict mode —
*wrong-structural-logic*, not a graded domain-mismatch:** the affine floor (use *at most* once) is the wrong
*structural logic* for a *linear* precondition (use *exactly* once); no decidable boundary price buys it
back (every sound fix is either undecidable reachability or a viral `Leak`/linear bound — Boats' scoped-task
trilemma, whose linear horn Rust rejected). It is the **enforcement** face of the ninth axis (leak-freedom)
— `drain` (#10) is the **tolerance** face (a leaked guard is *harmless*), while `linear` needs a leak to be
*impossible* and cannot get it — so **N is unchanged**.

**A sixth out-of-domain primitive HOLDS — and shows which *horn* a linearity gap hits depends on how the
rule packages it.** The session-typed channel (`seam/src/session.rs`, a `Chan<P>` affine typestate over
`Tx`/`Rx`/`End` — core-only, no `unsafe`) is the eighteenth seam, and the **first out-of-(memory-safety)
primitive to hold with a cost rather than break** (cold panel r25: **SEAM-COSTS [horn 3]**, survived). Its
precondition — *protocol fidelity* — is a **composite**, and the affine floor matches it **component-wise**,
the two components *orthogonal* (the soundness attacker confirmed a `mem::forget` cannot shorten the `&'a
mut Transport`'s `'a` to start a second session, so completion cannot infect safety): **order-safety
closes** (sound, concentrated, and — unlike #15 — with no `root`-hole to re-enter), while **completion costs
horn 3** — an undischargeable "don't abandon" obligation that recurs at every `?`/panic/early-return. The
illumination is the contrast with the *fifth* break: the **same** safe `mem::forget` gap that **broke**
#17's horn 1 only **costs** here, because completion was #17's *whole* soundness promise but is *one
component beside a sound safety core* here — so **which horn a linearity gap threatens is not a property of
the gap but of how the re-import rule *packages* it.** Honest deflations the panel recorded: this
`core`/no-`unsafe` witness has no duality (a single-handle typestate *walk*, not a binary session — `E0499`
forecloses two concurrently-live endpoints), "usable" holds only for *infallible straight-line* protocols (a
fallible step forces `?`, which *is* the abandonment), and the order-closes/completion-breaks split is
documented folklore (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 *"Stay Safe Under Panic"*) — the
contribution is *locating* it on the lattice, not discovering it.

**A seventh probe HOLDS — and turns "the horn depends on packaging" from an observation into a (bounded)
experiment.** The value-owning scope-restore guard (`seam/src/restore.rs`, a `Restore<T, F: FnOnce(T)>` —
the real `scopeguard::guard(value, f)` + `into_inner`, core-only, no `unsafe`) is the nineteenth seam and
the **controlled-packaging probe**: it carries the **identical** un-compellable-`Drop` danger as the *fifth*
break (`mem::forget` skips the restore closure — bit-for-bit #17's gap) but wraps it in the *sixth* probe's
structure (a sound affine-ownership core, beside which the restore is a side-obligation). The cold panel
(r26) ruled **SEAM-COSTS [horn 3]**, survived — confirming the dial: the **same** `mem::forget` gap that
**broke** #17's horn 1 only **costs** horn 3 here. On the **seam-width** instrument it earns no row, for the
*holding* reason rather than the breaking one: width is **O(1) per guard, O(0) per element** — it is a
`scopeguard`, the consumer writes no `unsafe` and no manual restore bookkeeping — so, exactly as with the
session channel, the horn-3 cost is **not a per-element width crossing** but a *de-concentration of an
obligation* (the un-compellable "don't `forget` the guard," recurring at every `?`/panic/early-return),
invisible to a width metric. The panel **bounded** the thesis three ways, all accepted: **(i) confounded** —
to package the danger as a side-obligation, #19 also adds the value-ownership core #17 lacked, so the
value-ownership model co-varies (a better-controlled probe than the incidental #17-vs-#18 pair, not a clean
single-variable manipulation); **(ii) bounded, not universal** — the horn is *forceable*: the same
skippable-`Drop` danger in a **borrowed-slot / `take_mut`** packaging forces **horn-1 UB** (the ninth-axis
`take` danger — a mid-restore panic double-drops the borrowed slot), and `Pin`'s drop guarantee and the
post-leakpocalypse `thread::scope` restructure exhibit horns that **cannot** be repackaged — so *which* horn
depends on **where safety rests**, partly danger-intrinsic; **(iii) not novel** — `JoinGuard`/`thread::scoped`
broke horn 1 and `scopeguard` costs horn 3 already in 2015 (RFC 1066, the leakpocalypse), so the
contribution is the *method* and *locating* the dial, **no new verdict mode** (N stays nine modes; the broken
count stays **five** — this one holds). The refined finding: **which horn a danger threatens is a design
choice *within a structural envelope* set by whether soundness can be *decoupled* from the un-compellable
closure** — owned value ⇒ it *costs* (this seam); borrowed-across-unwind / `Pin` / `thread::scope` ⇒ it
*forces*. Phase Q said the horn moves; this says it moves *within a fence*, and names the fence.

**The twentieth seam falls off the width edge — and that is the point.** The take-and-reseat guard over a
**borrowed slot** (`seam/src/reseat.rs`, a `Reseat<'a, T>` — `take_mut` in guard form) is the *controlled
counterpart* to #19: it keeps #19's packaging **byte-identical** (a guard with `Deref`/`DerefMut`/
`into_inner`/Drop-restore) and changes **only** the decoupling property, owned value → borrowed slot — the
clean single-variable experiment #19's confound forbade. The cold panel (r27) ruled **SEAM-BREAKS [horn 1],
FAILED — the *sixth* broken seam.** On the **seam-width** instrument it earns no row for the *breaking*
reason: the re-import *launders UB* (a 4-line safe `String` program — `new`/`push_str`/`into_inner` — leaves
two owners of overlapping heap and **double-frees**, no `unsafe` in user code), and width is only defined for
seams that *hold or cost*. #19 sat at the edge (O(1)/guard, holding with a horn-3 obligation); #20, same
packaging but a borrowed slot, falls **off** it. Two findings: **(1)** the horn flip is a **clean single
variable** — all five attackers agreed #19 (owned + `into_inner`) *cost* horn 3 and #20 (borrowed +
`into_inner`) *breaks* horn 1, packaging byte-identical, so Phase R's **confound is repaired** and the flip
is attributable to the decoupling property alone; **(2)** the fence is **FORCED, not movable** — the
`into_inner`-less "sound" subset is *degenerate* (it deletes the by-value move-out that defines `take_mut`,
strictly dominated by plain `&mut T`), so every *usable* packaging breaks, and the fence sits exactly at the
**owned-vs-borrowed** line. Deflation (pre-registered): no novel mechanism (`take_mut` + `scopeguard`-
`into_inner` + RFC 1066); the contribution is *analytical* — the controlled pair and *locating* the fence.
The broken count is now **six**; N stays **nine** verdict modes. The trilogy's refined finding: *the horn is
movable iff the value-ownership model lets soundness **decouple** from the un-compellable closure* — owned
**can** (⇒ cost), a borrowed slot **cannot**, for any usable packaging (⇒ forced break).

**The twenty-first seam: a fourth-domain probe that earns no width row — because the domain DEFLATED.** The
state-based CRDT merge (`seam/src/crdt.rs`, a `Replica<T: JoinSemilattice>` + blessed `GCounter<N>`, core-only,
no `unsafe`) is the first primitive aimed at a *fourth* soundness domain — distributed/replicated
**convergence** (eventual consistency), its danger *replica divergence* (two replicas that saw the same updates
disagree). Like the five out-of-domain primitives above it carries no finite seam width: the cold panel (r28)
ruled **SEAM-BREAKS [floor-mismatch]** *floor-relatively* — the `T: JoinSemilattice` bound is a **syntactic
proxy, not a witness** (a safe non-commutative/non-idempotent `join` type-checks and **diverges replicas**;
closing it is undecidable by Rice), so there is no sound width to measure. But it was scored a **TRADEOFF
survival** (the break pre-registered), and the headline is the **deflation**: the candidate fourth domain
**collapsed**, three corrections accepted. **(1)** The blessed-CRDT TCB-axiom rescue is **not MMIO-shaped** —
unlike MMIO's finite, foundation-owned registers, the lattice space is **open-ended**, so the law-trust
**recurs per user-defined CRDT** rather than amortizing into one axiom. **(2)** 'Dischargeable' holds only at
the *model* level — Liquid-Haskell/Coq proofs are about a math model; no Rust pass checks model-to-shipped-code,
so the trust stays irreducible at the compilation boundary (subtler than `secret`'s constant-time, which
CompCert-CT *does* preserve). **(3)** The decisive deflation — the gap is the **generic unchecked-typeclass-law
phenomenon** (Rust checks the *signatures* of `Ord`/`Eq`/`Hash`/`Monoid`/semilattice, **none** of their laws),
so convergence is **not** a fourth domain: it **projects onto `restore`'s application-correctness basis** (#19).
The refined finding: **no clean fourth domain here** — the soundness-domain map stays at **three** (memory
safety / information flow / liveness) plus the application-correctness basis, into which convergence folds.
The broken count **stays six** (this one *survived*, floor-relative break and all); N stays **nine** modes.

**The twenty-second seam: the second fourth-domain probe — and it FAILS off the width edge.** The cost-typed
computation (`seam/src/wcet.rs`, a `Timed<C, T>` graded cost monad reusing `typed_nat`'s sealed `Nat`/`Lt`,
core-only, no `unsafe`) tried the cleaner candidate the convergence post-mortem named: **real-time / worst-case
execution time**, whose danger is a *deadline miss* and whose bound is a **quantitative resource** (cycles), not
an algebraic law. It earns no width row for the breaking reason: the cold panel (r29) ruled **SEAM-BREAKS
[horn 1], FAILED — the *seventh* broken seam**, all five attackers landing, because §4 over-claimed — `within::<D>`
was promised to prove the *deadline* but the type tracks the *op-count*, and `step` charges `+1` decoupled from
its closure, so `pure(()).step(|_| for _ in 0..1_000_000 {…}).within::<1>()` type-checks, admitting a
million-cycle computation under a one-unit deadline. The op-count is an **unsound** proxy — *sharper* than
`secret`'s, whose proxy foreclosed *some* leaks (sound but incomplete); this one can be **arbitrarily far** from
the truth. Four breaks compound it: Horn 3 is **false** (non-unifying `if`-branch `Timed` types force manual
step-padding); the unary cost is **uncompilable** at real 10³–10⁷-cycle budgets (`typed_nat`'s `O(C)` blow-up,
now fatal — and the only compilable path, folding work into one `step`, *is* the break); no type-level
**addition**, so nothing composes; and the primitive is **Danielsson's 2008 cost monad**, for which *op-count ≠
cycles* is the **founding premise**, not a finding — *pick a lane*: abstract cost (sound, not real time) **xor**
physical deadline (needs a machine the floor lacks). So real-time **does not open a fourth domain** either: it
deflates onto the cost monad / `typed_nat` (#8), an **MMIO-shaped-but-uninstalled** axiom (#2 — worse than MMIO,
whose axiom is faithful *and* present), and liveness's partial-match. The FAILED broken count rises to **seven**
(unlike `crdt`, which *survived*); N stays **nine** modes. **The meta-finding:** *two* fourth-domain candidates
probed, **neither opens a fourth domain** — convergence by an unchecked algebraic law, real-time by an unsound
physical-time proxy — and the three confirmed domains (memory / IFC / liveness) look increasingly **special**.

**The twenty-third seam: the only one of the last three with a width row — and removing the limitation *widened*
it.** The real binary session (`seam/src/binary_session.rs`, two dual affine endpoints `Chan<P>`/`Chan<P::Dual>`
both live over `Rc<RefCell<Transport>>`, `alloc` + interior mutability, no `unsafe`) is **not** a fourth-domain
bid — it stays in the **liveness** domain and *deepens* it, removing #18's most-cited limitation (one `&mut
Transport` could not hold two concurrently-live endpoints, `E0499`). Unlike `crdt`/`wcet` it **holds**: the
cold panel (r30) ruled **SEAM-COSTS [horn 3], TRADEOFF, survived** — every attacker granted no-UB — so it *does*
sit on the width map, and its core profile mirrors #18: each op (`send`/`recv`/`close`) is **O(1)**, order and
*per-session* relative progress are **fully concentrated in the type** (the affine typestate + the `HasDual`
duality relation, `Θ(depth)` to resolve, no SMT), so the order/progress core is **zero seam width**. As in #18
the **completion** half is the width: an undischargeable "don't abandon" obligation recurring at every
`?`/panic/early-return on *either* side — now **peer-observable** (a real second endpoint starves), where #18's
was self-observable. **The new width finding — removing the single-handle limitation *increased* consumer
width.** Forcing a real peer forces `recv` fallible (`Result<…, Self>`), and its `Err` is **non-actionable**:
`mem::forget` *leaks* the `Rc` (skips `Drop`, strong count not decremented), so a starved dual cannot
distinguish "not-yet" from "abandoned" (an actionable signal, like `mpsc`'s disconnect-`Err`, would need `Drop`
to fire). So the consumer now writes *more* protocol bookkeeping than #18's infallible `recv` had — (a) a manual
two-endpoint interleaving/scheduler (the witness is single-threaded, no runtime) and (b) a dead-or-spinning
`match` at every `recv` site — the **leakpocalypse double-bind**: the same gap that *costs* completion also
*denies* an actionable `Err`. And the width is measured over a **narrow fragment**: relative progress closes
only **per session** (two interleaved `session()` calls type-check a mutual-wait deadlock — Kobayashi/Padovani;
global deadlock-freedom needs multiparty typing), and only straight-line `i32` protocols are expressible (no
choice / delegation / recursion). The probe **held** (FAILED count stays **seven**; N stays **nine** modes; Lean
`binary_session_progress_core_closes`, a `..._closes`) yet **refuted its own headline**: the "real binary
session" is #18 with a duplicated handle and a `RefCell` between the copies — the order/progress core's width is
still zero, but the completion cost is now wider, not narrower.

**The twenty-fourth seam: delegation holds at zero core-width, but the gauntlet REJECTS the width framing —
the new hazard is off the map entirely.** Delegation (`seam/src/delegation.rs`, channel-over-channel: a
`Chan<TxChan<Q, C>>::send_chan(c: Chan<Q>)` hands off a whole live session, `recv_chan` receives it) adds the
higher-order session feature #23 lacked, still in the **liveness** domain. The carried core has the **same
zero-width profile** as #18/#23: each op (`send`/`recv`/`send_chan`/`recv_chan`/`close`) is **O(1)**, and the
two carried preconditions are fully concentrated — order (affine typestate) and, the elegant part,
**don't-use-after-delegate for free** (the affine *move* consumes the delegated handle; the one linearity
delegation needs, the floor has at no consumer cost). So the order + handoff core is **zero seam width**, and
the cold panel (r31) confirmed the *mechanism* sound: horn-classifier **SEAM-COSTS [horn 3]**, no UB, decidable.
Two width/complexity corrections recorded: **Horn-2 well-formedness is `Θ(depth²)`, not `Θ(depth)`** (the
`HasDual` involution forces normalizing `dual(dual(carrier))`); and there is **no mid-protocol abort verb** —
`Result<_, Self>` bans `?`, so on an I/O error the consumer holds a non-`End` channel it can neither close nor
propagate, making the completion obligation undischargeable **by the programmer**, extra consumer width over the
advertised "concentration is cheap." **But the headline result is a width *non-event*:** the judge ruled the §6
deliverable **FAILED** (the **rejection mode**, 2nd after `typed_nat` #8) because delegation's signature hazard
is **not a width cost at all**. I had filed it as a leak projecting onto the leak-freedom axis (#9/#10); the
panel refuted that from my own cited lineage — the three-party hazard (B forgets `Chan<Q>`, starving C) is
"deadlock at the other end" (Jespersen et al. 2015), i.e. **progress / deadlock-freedom**, a coordinate
*distinct* from completion and leak-freedom (Coppo–Dezani–Yoshida–Padovani: "the communication topology changes
because of delegation"; deadlock-free GV). I saw the `Rc` hazard's memory-leak *face* and mislabeled the
progress *axis*; the single-threaded witness hides deadlock by construction (the symmetric delegation cycle is
type-obstructed — an inexpressible infinite type — which rescues *behavior*, not *classification*). So the width
map gains **no new column** (the carried core is still zero-width), and the new danger lands **off the width map
entirely** — a progress coordinate, like the lock seams' (#15/#16) deadlock, not a seam-width cost. The probe
**held** at the mechanism level (FAILED *broken-seam* count stays **seven** — this is a deliverable rejection,
not a soundness break; N stays **nine** modes; Lean `delegation_handoff_core_closes`, a `..._closes`, like
`typed_nat`) yet earned its keep by being **instructively wrong**: it opened the progress coordinate I had tried
to fold into leak-freedom.

**The twenty-fifth seam: the order core is zero-width again, but the *global*-progress obligation is off the
per-call width map — it is a whole-program proof obligation.** Multiparty (`seam/src/multiparty.rs`, an n-party
session assembled from #23's binary edges; a `RecvThenSend` role typestate enforces recv-before-send) builds the
very feature #23's deflation #1 named — *global deadlock-freedom needs multiparty typing*. `relay()` mints a
coherent `A→B→C` (a source role exists → it completes); `ring()` mints an incoherent `A→B→C→A` where every role
is `RecvThenSend`, so there is **no source** — and it **type-checks a globally-deadlocked system**, the floor
unable to range over the global topology (#23's asserted interleaved mutual-wait deadlock, now a compiled
three-party artifact). The carried core has the **same zero-width profile** as #18/#23/#24: each op is O(1), and
role-local order + linearity are fully concentrated in affine typestate (Lean `multiparty_role_order_core_closes`,
a `..._closes`). The cold panel (r32) confirmed the *mechanism* sound — horn-classifier **SEAM-COSTS [horn 3]**,
no UB (the worst case is a memory-safe recv-`Err` hang), decidable. **But the headline is again a width
non-event:** the judge ruled the §6 deliverable **FAILED** — the **rejection mode**, 3rd after `typed_nat` #8 and
`delegation` #24 — because global progress is **not a per-call width cost at all**. I had staked a **false
dichotomy**: that global progress lives *only* whole-protocol in a *generative* global-type projection, no
per-role discipline able to reject the ring. The panel refuted it from my own cited lineage — **per-channel
priority/obligation typing** (Kobayashi linear-π; Padovani; **Dardha–Gay, FoSSaCS 2018**; Kokke–Dardha) rejects
the source-less ring *compositionally, with no global type*, via `prio(recv) < prio(send)` chaining to a
contradictory `p < p` on the cycle — and I had **mis-filed Kobayashi as *support* when he is the
*counterexample***. The honest taxonomy has **three rungs**: (a) the per-edge `HasDual` floor [doesn't carry];
(b) per-channel priority typing + a whole-program acyclicity check [no global type]; (c) generative MPST
projection. What survives is the *weaker* truth: global progress is **not concentrable per-call** and needs
strictly more than the per-edge floor — but it is a **whole-program** obligation, *not* necessarily a generative
one. So the width map gains **no new column** (the carried core is still zero-width), and the new obligation lands
**off the per-call width map entirely** — a whole-program proof obligation, like the lock seams' (#15/#16)
deadlock and delegation's (#24) progress, not a seam-width cost. The probe **held** at the mechanism level
(FAILED *broken-seam* count stays **seven** — a deliverable rejection, not a soundness break; N stays **nine**
modes) yet earned its keep by being **instructively wrong** a third time: it restored the three-rung taxonomy I
had collapsed to two.
