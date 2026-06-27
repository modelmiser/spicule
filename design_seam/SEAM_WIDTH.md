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

## All four primitives — and why seam width ≠ zero-cost

| Primitive | Consumer seam width | Runtime cost (zero-cost #6) |
|---|---|---|
| SPSC ring buffer | **O(1)/buffer** (`split` once) | none — fully erased |
| MMIO read-to-clear | **O(1)** with a foundation `Send` opt-in, else O(ISRs) | none — fully erased |
| Generational slab | **O(1)** — `insert`/`get`/`remove` are all safe calls | **non-zero** — a generation word/slot + a compare branch/access, never erased |
| Intrusive list (branded) | **O(accesses)** — the token threads through every access | (design A erases; the cost is seam width / lost reclaim) |

**The slab is the instructive row: it *aces* seam width (horn 3) yet still pays — on a different
axis (zero-cost / #6).** Concentration asks "how often does ordinary code touch the seam"; erasure
asks "does the witness survive to run time." They are independent. The slab's witness is read once
per access *inside* the seam (so the consumer never crosses — width O(1)) but it is a runtime
load-and-branch (so it does not erase). That is precisely why the Seam Trilemma is a 3-of-4
projection: bundling "concentrated & proof-free" hides that a concentrated seam can still be
non-zero-cost. See `SYNTHESIS.md`.
