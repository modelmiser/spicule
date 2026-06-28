# Seam spec — reference-counted shared ownership (re-import rule under attack)

This is a **design proposal** for a seventh *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, and refined-index seams.
A seam re-exports a dangerous primitive as a checked safe interface ordinary code uses with no
`unsafe` and no proofs. Attack the re-export: can ordinary safe code, using only this interface,
reach undefined behavior (UB)? **And** adjudicate the axis question this primitive was built to settle
(§6).

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution). The floor is **`#![no_std]`-capable**: it targets bare metal with
no runtime, no garbage collector, and **no global allocator unless one is explicitly added**. Do not
attack the borrow checker; attack the re-import rule and the axis claims.

## 1. The primitive

**Manual reference counting** — a heap allocation holding a value plus a `usize` strong count, with
`clone` incrementing the count, `drop` decrementing it, and the allocation freed (and the value
dropped) when the count hits zero. Get any step wrong and you get a **double-free** (free while
owners remain), a **use-after-free** (an owner reads freed storage), or a **leak**. This is the
internals of `std::rc::Rc` / `Arc`, normally hidden behind `unsafe`.

The seam re-exports it as **`Shared<T>`**: `Shared::new(v)` allocates and returns the first owner;
`clone` makes another owner; `Deref` reads the shared value; `drop` releases an owner. The danger is
discharged by a **runtime reference-count manager**, and that manager is what makes this primitive
interesting: it is the first seam whose sound re-import needs a *runtime memory-management subsystem*
rather than only compile-time witnesses or a per-access check.

## 2. The true precondition (cathedral side)

Memory-safe operation requires **all** of:

- **C1 (count = live owners).** The strong count equals the number of live `Shared` handles to the
  allocation at all times — every `clone` increments, every `drop` decrements, exactly once each.
- **C2 (free on last, and only last).** The value is dropped and the storage freed precisely when
  the count transitions to zero — not before (UAF for survivors), not never (leak).
- **C3 (no aliased mutation).** While ≥2 owners share the value, none may obtain `&mut` to it —
  shared-mutable aliasing is UB.
- **C4 (single-threaded refcount).** The count is non-atomic (`Cell`), so the owners must not cross
  threads; concurrent `clone`/`drop` would race the count and corrupt C1.

**Where the hard conjunct lives.** C1/C2 concern a **data-dependent lifetime**: the instant of the
last drop is not statically knowable — it depends on run-time control flow. Affinity expresses
*exactly one* owner; NLL expresses *static* lifetimes; neither can express "N owners, freed when the
last leaves." That is why the witness must be a **runtime count**, and the count needs storage that
outlives every owner and is reclaimed on the last drop — i.e. a **manager** (here, the global heap).

## 3. The token — and the soundness mechanism

The token is a `Shared<T>` handle (a `NonNull<Inner<T>>` into a heap `Inner { strong: Cell<usize>,
value: T }`; `Inner` is private, unforgeable). The discharges:

- **C1** — `clone` is the *only* way to make an owner and it increments; `Drop` decrements. Safe code
  cannot make or destroy an owner any other way.
- **C2** — `Drop` frees iff the pre-decrement count was 1.
- **C3** — `Shared<T>: Deref<Target = T>` but **not** `DerefMut`. Safe code can never obtain `&mut T`
  through a `Shared`, so aliased mutation is a *compile error*. (Interior mutability would require a
  `RefCell` inside — its own runtime borrow flag — which is out of scope and does not bear on the
  axis here.)
- **C4** — `Cell<usize>` is `!Sync` and the handle holds a raw pointer, so `Shared<T>` is `!Send` and
  `!Sync`: the floor's trait resolution forbids sending an owner across threads.

**Honest residual caveat (resource bug, not UB).** Reference cycles (`Shared` A owns B owns A) never
reach count zero, so they **leak** — memory is never reclaimed. This is the documented `Rc` caveat;
it is a *resource* bug, not UB (no freed memory is touched). Breaking cycles needs a `Weak`
back-reference, out of scope for this minimal core.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Shared<T>` vends `Clone` handles and `Deref` read access. The safe fragment may
> type `new`/`clone`/`*shared`/`drop` as ordinary safe operations — no `unsafe`, no proof — because
> the foundation's manual refcount manager maintains C1∧C2 (count tracks owners; free on last drop),
> `Deref`-without-`DerefMut` enforces C3 (no aliased mutation), and `!Send`/`!Sync` enforce C4. The
> consumer never sees the heap pointer, the count arithmetic, or the free.

## 5. Threat model

In scope: any AM-level UB (double-free, use-after-free, reading freed storage, data race on the
count) reachable by **safe** consumer code using only the §3 interface. Consider: dropping the same
logical owner twice; cloning without incrementing (can safe code force it?); obtaining `&mut` to
shared data (C3); sending a `Shared` to another thread (C4); a `T` whose `Drop` re-enters `Shared`
(drops another owner during the free); forging a handle (private `Inner`). Out of scope: bugs inside
the foundation's `unsafe` refcount glue; cycle **leaks** (conceded as a resource bug in §3, not UB —
only flag if you can turn one into actual AM-UB); `usize` count overflow (2⁶⁴ live clones).

## 6. The scoring rubric — which horn, and is BARE-METAL a seventh axis?

First, soundness (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that double-frees, reads freed storage,
  mutates aliased shared data, or races the count — *despite* C1–C4 (`Deref`-not-`DerefMut`,
  `!Send`/`!Sync`, the increment/decrement discipline). Give the exact sequence; name which conjunct
  it defeats. (A re-entrant `Drop` that drops another owner during a free is the sharpest thing to
  try.)
- **Horn 2 — decidable & proof-free.** `new`/`clone`/`drop` are ordinary safe calls; no solver, no
  hand proof. Confirm, or show otherwise.
- **Horn 3 — concentration.** Seam width is O(1): the manager lives entirely in the foundation; the
  consumer crosses only at safe API calls. Confirm.

Then **the point of this seventh primitive** — the axis the first six left uniform. All six prior
seams are **bare-metal**: they need no heap, no allocator, no runtime (the crate is `#![no_std]` and
they are `core`-only). This one is **not** — its sound re-import requires a *runtime memory-management
manager* (a global allocator + a reference-count lifecycle). Adjudicate, precisely:

1. **Is bare-metal / runtime-freedom (seed property #5) a real, independent axis?** The operational
   test offered: *does the seam compile under `#![no_std]` with no `extern crate alloc`?* Seams 1–6
   do; this one cannot (it is the sole reason the crate pulls in `alloc`). Is "needs a runtime memory
   manager" a genuine consumer-visible cost — the embedded/kernel author who has no heap simply
   cannot use this seam — or is it merely a target-environment detail and not a *seam* property?

2. **Is it a NEW axis, or a re-slice of one already on the map?** The honest deflation checks — run
   each, this is the crux:
   - **Is it just zero-cost (#6)?** Argue from the **off-diagonal**: the *slab* pays #6 (a runtime
     generation compare — not zero-cost) yet is *bare-metal* (a fixed `[Slot; N]` pool, `core`-only,
     no heap). This `Shared` *also* pays #6 (the refcount inc/dec) but is *not* bare-metal. They
     **agree on #6 and differ on #5** — so a runtime *check* (no manager) and a runtime *manager* are
     not the same thing. Does that dissociation hold, or is the slab secretly "needs a manager" too
     (its pool is a manager)? Decide — and note the distinction is *global-allocator dependency vs
     caller-provided fixed storage*.
   - **Is it the "keep" horn (solver-in-TCB) from the refined seam?** No — that was a *compile-time*
     prover in the toolchain; this is a *runtime* subsystem in the deployed binary. Different TCB.
     Confirm they are distinct.
   - **Is it the MMIO TCB axiom?** No — an allocator is ordinary code, not an unverifiable
     physical-world assumption. Confirm.
   - So: is **bare-metal / runtime-freedom** a genuine *seventh* axis (the projection is 3-of-≥7), or
     does it reduce to something already counted (the projection stays at 6)?

3. **The trade (note if real).** Shared ownership does not *force* the global heap: a pool-backed
   refcount (an arena `Rc`, `heapless`-style) would be bare-metal but *capacity-bounded*. So #5
   appears to trade against unbounded dynamic count. Is that a genuine exchange (like #5↔#6), or a
   reason to say bare-metal isn't really separable here?

State your verdict as **SEAM-HOLDS** (sound; all known horns kept; name any new-axis cost),
**SEAM-COSTS** (sound but a horn or a named new axis is paid — say which), or **SEAM-BREAKS** (you
found AM-level UB; give the sequence). And state explicitly: **is bare-metal a seventh axis? (yes /
no / reduces-to-X).**
