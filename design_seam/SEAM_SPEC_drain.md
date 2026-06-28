# Seam spec — drain-and-yield with a leaked guard (re-import rule under attack)

This is a **design proposal** for a tenth *seam* in the same hypothetical systems language (same floor,
same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, refined-index, shared-ownership,
type-level-index, and take-and-replace seams. A seam re-exports a dangerous primitive as a checked safe
interface ordinary code uses with no `unsafe` and no proofs. Attack the re-export: can ordinary safe
code, using only this interface, reach undefined behavior (UB)? **And** adjudicate the axis question
this primitive was built to settle (§6).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/drain.rs`).

> **Context the builder is being honest about.** The lattice already has **eight** confirmed axes; the
> eighth (unwind-safety) was confirmed by the take-and-replace seam. The previous *checker-tractability*
> probe was **rejected** because its off-diagonal silently flipped *expressiveness*. This primitive was
> chosen specifically to avoid that failure mode — it is a *free-standing* guard (no closure, no viral
> type), so it should ace API-locality, the axis the obvious leak-freedom witness (scoped threads)
> would have confounded with. The load-bearing §6 questions: (a) is leak-freedom a real **ninth** axis,
> or does it reduce to API-locality / unwind-safety / a plain horn-1 break; (b) is it *distinct from*
> the eighth axis (unwind-safety)?

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution) **including the fact that leaking is safe**: `mem::forget`,
`Box::leak`, `ManuallyDrop`, and reference cycles all skip `Drop` with no `unsafe`. There is **no
linear-type discipline** — the floor is *affine* (use at most once; you may always drop *or leak*
instead of consuming). Do not attack the borrow checker; attack the re-import rule and the axis claims.

## 1. The primitive

**Drain-and-yield** — `drain(&mut buf) -> Drain<'_>`, an iterator that moves each element of a
fixed-capacity buffer out *by value* (`assume_init_read`). During iteration the buffer's already-yielded
slots are **logically moved-from** (bitwise live, but must not be dropped or read again). The danger is
the canonical Rust **leakpocalypse / leak amplification** (the pre-1.0 `Vec::drain` bug): if the
`Drain` guard is leaked after yielding some elements, and the buffer still counts those slots as live,
the buffer's `Drop` will drop the moved-from slots — a **double free**.

## 2. The true precondition (cathedral side)

`drain` is memory-safe iff **no slot is dropped twice and no moved-from slot is read**. Two conjuncts:

- **D1 (no aliasing the half-moved buffer).** No other access to the buffer exists while the `Drain`
  lives. Carried by the floor: `Drain` holds the `&mut` borrow, so the borrow checker forbids touching
  the buffer during iteration (see CF1). This conjunct the floor *does* carry.
- **D2 (every slot dropped at most once, regardless of whether the guard's `Drop` runs).** The
  moved-from slots must never be dropped; the un-yielded slots must be dropped exactly once. The hard
  part is **"regardless of whether the guard's `Drop` runs"** — because the floor permits the guard to
  be *leaked*, the soundness argument may not rely on `Drain::drop` executing. This is the conjunct
  that must be paid for.

## 3. The token — a free-standing guard with pre-pessimization

The token is a **`Drain<'a>`**: a plain `&mut`-borrowing iterator. **No brand, no closure** — it is a
value you hold and iterate, exactly like a normal iterator. The leak-safety mechanism is
**pre-pessimization**: `drain` sets `buf.len = 0` **before** yielding anything, and the `Drain` tracks
the un-yielded range `[idx, end)` to drop on its own `Drop`. Consequences:

- **Guard dropped normally:** `Drain::drop` drops `[idx, end)`; the buffer (len 0) drops nothing. Each
  element dropped exactly once. Sound.
- **Guard leaked (`mem::forget`):** `Drain::drop` never runs, so `[idx, end)` is **leaked** (a memory
  leak, *not* UB); the buffer (len 0) drops nothing, so the moved-from `[0, idx)` slots are **not**
  double-dropped. **Sound under leak** — D2 holds without relying on `Drain::drop`.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `drain` interface vends a safe by-value iterator over a buffer's elements. The safe
> fragment may iterate it with no `unsafe` and no proof because D1 is carried by the `&mut` borrow and
> D2 is enforced by pre-pessimization (the buffer is emptied up front, so a leaked guard loses the tail
> but never double-frees).

Structural stoppers: the `&mut` forbids aliasing the half-moved buffer (D1, CF1); `buf.len = 0` up
front makes D2 hold *whether or not* `Drain::drop` runs (so `mem::forget(drain)` is sound).

**The disclosed cost.** Pre-pessimization means a leak **silently drops (leaks) the un-yielded tail**:
the buffer is emptied before the work is done, so forgetting the guard loses data rather than corrupting
memory. That guarantee downgrade is the price of leak-freedom on a floor that permits leaks.

## 5. Threat model

In scope: any AM-level UB reachable by **safe** consumer code. Consider specifically: (a) `mem::forget`
the `Drain` after partial iteration — double-free of moved-from slots? (the keystone — and note
`mem::forget` is *safe*, so this is squarely in scope); (b) leak the `Drain` and then drop the buffer,
or push to it later (does the borrow end? does len stay 0?); (c) a `T` whose `Drop` panics during
`Drain::drop`'s remainder cleanup (does it leave the rest un-dropped / double-dropped?); (d) calling
`drain` twice / re-entrantly (borrow held?); (e) yielding past `end`, or an `idx`/`end` overflow; (f)
forgetting the *buffer* instead of the guard. Out of scope: bugs inside the foundation `unsafe`; whether
*leaking memory* is itself acceptable (it is safe by the floor's definition — a leak is not UB).

## 6. The scoring rubric — soundness, then the axis adjudication this primitive was built for

First, soundness:

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reaches a double-free or a read of a
  moved-from slot, despite D1∧D2. The obvious attack is `mem::forget(drain)` mid-iteration — show it
  double-frees, or concede it does not. Give the exact sequence.
- **Horn 2 — decidable & proof-free.** `drain`/`next` are ordinary safe calls. Confirm.
- **Horn 3 — concentration.** One `unsafe` foundation region; O(1) consumer crossings. Confirm.

Then **the axis adjudication this primitive exists to settle** — be precise, this is the deliverable:

1. **Is "leak-freedom" a real axis, independent of the eight already recovered?** The claim: this seam
   **aces all eight** — sound; decidable & proof-free; O(1)-concentrated; **brand-free** (a plain
   `Drain<'a>` — no viral type, *no closure*, so it aces API-locality, unlike a scoped-thread guard);
   zero-cost; legible; `core`-only (bare-metal); and **unwind-safe** (a panic mid-iteration just runs
   `Drain::drop`, which pre-pessimization already made safe). It pays on **exactly one** new thing: D2's
   "regardless of whether `Drop` runs" — leak-freedom, the pre-pessimization / guarantee-downgrade cost.
   If that holds, it is a **slab-grade off-diagonal** (identical to SPSC on every named axis, split on
   leak-freedom alone), making leak-freedom a **ninth** axis (N ≥ 9). Adjudicate each "ace"; the result
   turns on whether even one is *also* flipped (the way the `typed_nat` probe secretly flipped
   expressiveness).

2. **Is leak-freedom *distinct from* unwind-safety (the eighth axis), or the same axis?** They are
   siblings — both about a `Drop` not running *normally*. The proposed dissociation: the take-and-replace
   seam is **unwind-sensitive** (a panic mid-take must abort) yet **leak-safe** (forgetting its
   abort-bomb is fine); this drain seam is **leak-sensitive** (a forgotten guard must not double-free)
   yet **unwind-safe** (a panic mid-iteration is handled by the guard's `Drop`). A leak is "`Drop`
   *never* runs"; an unwind is "`Drop` runs at a *bad time*." Is that a genuine off-diagonal (⟹ two
   distinct axes), or should "leak-freedom" and "unwind-safety" be merged into one
   *destructor-execution-robustness* axis (⟹ the eighth axis was broader than stated, N stays 8)?

3. **Or does it confound / reduce?** Candidate objections to rule on: (a) does it reduce to **API-locality**
   — is the `&mut`-borrowing guard "really" a forced-scope cost? (The proposal says no: it is a
   free-standing value, no closure, the borrow is an ordinary `'a` like SPSC's.) (b) Is it just a
   **horn-1 break** dressed up — i.e. the naive version is unsound and pre-pessimization is "concede the
   tail", a usability cost, not a new axis? (c) Is leak-freedom not a property of *this seam* but of the
   floor's decision that **leaking is safe** (so the seam merely copes with a floor choice)?

4. **Novelty — deflate it.** Name leak-freedom's prior art if it has any: Rust's **leakpocalypse**
   (the 1.0 `mem::forget`-is-safe decision; RFC 1066 / the `JoinGuard`/`thread::scoped` removal), **leak
   amplification** (the `Vec::drain` term), linear-vs-affine types (Wadler 1990), C++ RAII vs
   "you can always `longjmp`/leak". Rule whether this is **real but pre-existing** (the expected
   outcome, growing N to ≥ 9 as a *named* axis like bare-metal/unwind-safety did) or not a clean axis.

State your verdict as **SEAM-HOLDS** (sound; all known horns kept; confirm the corner), **SEAM-COSTS**
(sound but names a horn or recovered axis paid — say which), or **SEAM-BREAKS** (AM-level UB; give the
sequence). And state, separately and explicitly: **is leak-freedom a real ninth axis, N ≥ 9? (yes — and
its prior art is … / no — it merges with unwind-safety / no — it confounds with or reduces to … / needs
more)**, and **is it distinct from unwind-safety? (yes / no)**.
