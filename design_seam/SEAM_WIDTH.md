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
memory-safety analog.

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
