# Seam spec — a self-referential cell (re-import rule under attack)

This is a **design proposal** for an eleventh *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, refined-index,
shared-ownership, type-level-index, take-and-replace, and drain-and-yield seams. A seam re-exports a
dangerous primitive as a checked safe interface ordinary code uses with no `unsafe` and no proofs.
Attack the re-export: can ordinary safe code, using only this interface, reach undefined behavior (UB)?
**And** adjudicate the axis question this primitive was built to settle (§6).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/selfref.rs`).

> **Context the builder is being honest about.** The lattice has **nine** confirmed axes. The probes
> past seven have gone: checker-tractability **rejected** (confounded with expressiveness), unwind-safety
> **confirmed** (clean), leak-freedom **confirmed** (clean, distinct from unwind-safety). This primitive
> probes a tenth — **move-stability** — and the builder's honest expectation is that it may *not* be
> cleanly isolable: every Rust witness for address-stability appears to spend another axis. The §6
> deliverable is precisely to rule which of three outcomes holds (clean / reduces / inseparable).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution) **including `Pin` and the `Unpin` auto-trait**: a value behind
`Pin<P>` whose pointee is `!Unpin` cannot be moved out or `&mut`-accessed in safe code. `core::pin::pin!`
pins a value on the stack (`core`-only, no allocator). `Box::pin` would pin on the heap. Do not attack
the borrow checker or the `Pin` model; attack the re-import rule and the axis claims.

## 1. The primitive

**A self-referential struct** — `SelfRef` holds a `u64 value` and a `*const u64 ptr` that, after
`init`, points at its own `value` field. The dangerous primitive is the **raw self-pointer deref**
(`*self.ptr`), re-exported as a safe `get()`. It is memory-safe **iff the struct has not moved since
`init`**: a move relocates `value` but leaves `ptr` aimed at the old address — a dangling read, instant
UB. The danger is *neither* aliasing, bounds, ownership, destructor-timing, nor any of the nine
recovered axes; it is **address stability**.

## 2. The true precondition (cathedral side)

`*self.ptr` is memory-safe iff:

- **M1 (pointer established).** `init` has run, so `ptr == &self.value` (else `ptr` is null — `get`
  guards this with an assert: a caught logic error, never UB).
- **M2 (no move since init).** `self` has not been moved since `init` captured its address. This is the
  hard conjunct: the floor carries it **only via `Pin`** — a `Pin<&mut Self>`/`Pin<&Self>` over a
  `!Unpin` type (the cell is `!Unpin` via `PhantomPinned`) statically forbids the move (and forbids the
  `&mut Self` that would let you `mem::swap`/`replace` it out).

## 3. The token — a pinned reference, and the confound being disclosed

The token is a **`Pin<&mut SelfRef>`** (from `pin!`) to set up the self-reference, then **`Pin<&SelfRef>`**
to read it. `init(self: Pin<&mut Self>)` discharges M1; the `Pin`+`!Unpin` types discharge M2.

**The disclosed confound (this is the whole §6 question).** Every floor witness for M2 spends another
recovered axis:
- `Pin<&mut Self>` / `Pin<&Self>` are **viral in every signature** that touches the cell ⇒ this looks
  like the **API-locality** cost (a viral, restrictive wrapper, exactly like the branded index's `'id`).
- `Box::pin` (a heap address that is stable by construction) would spend **bare-metal**. This module
  deliberately uses **stack pinning** (`pin!`) to stay `core`-only, so the confound — if it is one — is
  isolated to **API-locality alone**, making the adjudication crisp.
- a `with_pinned(value, |p| …)` scope would be a **closure** ⇒ API-locality again (forced scope-closure).

So unlike the drain seam — which dodged its API-locality confound by owning its data (a *free-standing*
guard) — move-stability appears to have **no** free-standing, non-viral, `core`-only witness.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `SelfRef`/`init`/`get` interface vends a self-referential cell and a safe read
> through its internal pointer. The safe fragment may call `get()` with no `unsafe` and no proof
> because M1 is discharged by `init` and M2 by `Pin` over the `!Unpin` cell — so `ptr` is provably
> still valid at the read.

Structural stoppers: `init` requires `Pin<&mut Self>` (you cannot set up the self-pointer on an
unpinned, movable cell and then read it — the read needs `Pin<&Self>`); `SelfRef: !Unpin` makes the
dangling-move unconstructible in safe code (`Pin::get_mut` requires `Unpin` — see CF1).

## 5. Threat model

In scope: any AM-level UB reachable by **safe** consumer code. Consider specifically: (a) any safe path
to *move* a `SelfRef` after `init` (mem::swap/replace/return-by-value through the `Pin`?); (b) calling
`get` before `init` (null deref? — guarded by assert); (c) constructing a `SelfRef` with a pre-set
`ptr` (fields are private; only `new` mints one, with `ptr = null`); (d) `init` twice, or after a
(somehow) move; (e) getting a `&mut SelfRef` out of the `Pin` (needs `Unpin`); (f) a `'static` or
escaping `Pin<&Self>` outliving the stack frame. Out of scope: bugs inside the foundation `unsafe`
(`get_unchecked_mut`, the deref); the soundness of `Pin` itself / Stacked-Borrows provenance of the
self-pointer (foundation-internal, like SPSC's P1–P3 — flag it if you think it is load-bearing here).

## 6. The scoring rubric — soundness, then the axis adjudication this primitive was built for

First, soundness:

- **Horn 1 — sound re-import.** Exhibit safe consumer code that moves a `SelfRef` after `init` (or
  otherwise dangles `ptr`) and reaches a dangling read. Give the exact sequence — or concede `Pin`+
  `!Unpin` forecloses it.
- **Horn 2 — decidable & proof-free.** `init`/`get` are ordinary safe calls. Confirm.
- **Horn 3 — concentration.** One `unsafe` foundation region; O(1) consumer crossings. Confirm.

Then **the axis adjudication this primitive exists to settle** — be precise, this is the deliverable.
Rule which **one** of three outcomes holds for **move-stability**:

1. **A clean tenth axis (N ≥ 10)?** This requires the seam to **ace all nine prior axes** and pay on
   move-stability *alone*. The builder doubts this: the `Pin` wrapper is in every signature. Does the
   seam ace **API-locality**, or does the viral `Pin<&…>` flip it? (Be precise: is `Pin<&Self>` a
   "plain, non-viral wrapper" like `Shared`/`Refined`, or a viral, restrictive one like the branded
   `'id`? It restricts `&mut` access and infects every method receiver.)

2. **Reduces to API-locality (not a new axis)?** If the *only* cost the move-stability seam pays, on
   this floor, is the viral/restrictive `Pin` type surface, then "move-stability" is just **API-locality
   wearing a different hat** — the danger is distinct but the *cost* is one already on the map. This is
   the `typed_nat` outcome (a confound), and would leave N = 9.

3. **Real but structurally inseparable (a new kind of verdict)?** The move-stability *danger* is
   genuinely distinct from all nine axes (no other axis is about a value's address), **yet** it may have
   *no* witness that isolates it: stack-pin ⇒ API-locality, heap-pin ⇒ bare-metal, scope ⇒ API-locality.
   If so, move-stability is **real but not a clean lattice coordinate** — reachable only by *spending*
   API-locality or bare-metal. That is neither "clean axis" nor "confounded non-axis": it is a distinct
   danger with no separable cost. Is this the honest ruling? If so, say so plainly — it is a genuinely
   new category for the meta-pattern (the lattice's first *inseparable* axis), and arguably the most
   interesting outcome.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (say which axis) / **SEAM-BREAKS** (give the UB
sequence). And state, separately and explicitly, **which of (1)/(2)/(3) holds for move-stability**, with
your reasoning — and if (1), what is move-stability's prior art (`Pin` RFC 2349; async/`Future`;
`generator`/`gen`; intrusive collections); if (3), justify why no free-standing witness can exist on
this floor.
