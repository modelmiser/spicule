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

## All eleven primitives — and why seam width misses *several* axes, not one

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

**Three more rows (plus a fourth) sharpen the same point from the opposite direction — and are a caution against both
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
`Box::pin` spends bare-metal). So a primitive that aces every column the instrument can see may be paying
on a hidden cost that is *a real clean axis* (take, drain), a *confounded non-axis* (type-level index), or
*a real danger with no clean coordinate at all* (self-ref cell) — and only a richer adjudication (the
cold panel) tells the three apart. See `REALIZABILITY.md` for the full face lattice (the confirmed
eighth and ninth axes, the rejected probe, and the inseparable category) and `SYNTHESIS.md` for the
cross-primitive map.
