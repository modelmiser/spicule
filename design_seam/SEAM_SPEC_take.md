# Seam spec — take-and-replace across a possible unwind (re-import rule under attack)

This is a **design proposal** for a ninth *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, refined-index,
shared-ownership, and type-level-index seams. A seam re-exports a dangerous primitive as a checked safe
interface ordinary code uses with no `unsafe` and no proofs. Attack the re-export: can ordinary safe
code, using only this interface, reach undefined behavior (UB)? **And** adjudicate the axis question
this primitive was built to settle (§6).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/take.rs`).

> **Context the builder is being honest about.** The previous probe (`typed_nat`, the type-level
> index) was *sound* but its proposed eighth axis ("checker-tractability") was **rejected** by the cold
> panel: its off-diagonal silently flipped *expressiveness* (it indexed only compile-time constants),
> so it did not isolate one coordinate. This primitive is the retry, chosen specifically to avoid that
> confound — but do not take the builder's word for it. The load-bearing question in §6 is exactly
> whether the off-diagonal is clean this time, or whether "unwind-safety" confounds with, or reduces
> to, an already-named axis.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution) **including its unwinding model**: a `panic!` unwinds the stack,
running `Drop` for every live local, unless a panic occurs *during* unwinding, which **aborts** the
process. Targets may also be built `panic=abort` (the first panic aborts). The floor has no effect
system and **no way to type "this function does not unwind"** (`#[no_panic]` is not in the language).
Do not attack the borrow checker or the panic runtime; attack the re-import rule and the axis claims.

## 1. The primitive

**Take-and-replace** — move the value out of a `&mut T`, transform it by value, store the result back:
`take<T, R>(slot: &mut T, f: FnOnce(T) -> (T, R)) -> R`. The danger is not aliasing or bounds; it is
**time**. The implementation does `let old = ptr::read(slot)` — now `slot` holds a bitwise *duplicate*
of `old`, a `T` that is logically moved-from and must never be dropped or read — then runs `f(old)`,
then `ptr::write(slot, new)` to restore a valid `T`. During the **gap** between read and write, the
slot is a live-looking `T` that is actually a duplicate. This is the danger of a naive `take_mut`.

## 2. The true precondition (cathedral side)

`take` is memory-safe iff **nothing observes `slot` during the gap** — no read, and crucially no
`Drop`. The only thing that can run arbitrary code during the gap, on the stipulated floor, is
**unwinding**: if `f` panics, the stack unwinds, `f`'s frame drops `old` (correct — it owns it), and
then the caller's frame would drop the duplicate still sitting in `slot` → **double free**. Two
conjuncts:

- **U1 (no aliasing the gap).** No second access to `slot` exists during `f`. Carried by the floor:
  `take` holds the `&mut T`, so the borrow checker forbids any other borrow of `slot` while `f` runs
  (see CF1). This conjunct the floor *does* carry.
- **U2 (no unwind through the gap).** Control does not unwind out of `f` while the slot is broken. The
  floor has **no term for this** — `panic` can happen in any `f`, and the floor cannot prove `f`
  nounwind. This is the conjunct that must be paid for.

## 3. The token — an abort-bomb, and what it costs

The witness is a **`Bomb`**: a zero-sized local of `take` whose `Drop` panics. It is armed before the
`ptr::read` and `mem::forget`-ten after the `ptr::write`. Mechanism, and the disclosed cost:

- **Sound path.** If `f` panics, unwinding drops `take`'s live locals; `bomb` is one of them, and its
  `Drop` panics *while already unwinding* → **double-panic → process abort**. The abort happens inside
  `take`'s frame, *before* control returns to the caller, so the caller never drops the duplicate: U2
  is enforced by turning a gap-unwind into an abort. On success, `ptr::write` restores validity and
  `mem::forget(bomb)` defuses it — no residue.
- **The cost (this is the axis).** Soundness is bought with **abort-on-mid-take-panic**: a panic in `f`
  is *not* `catch_unwind`-recoverable; it takes the whole process down. That is a real,
  consumer-visible behavioral cost — the basic exception-safety guarantee is downgraded to "abort" for
  this window. It is the price of soundly re-importing a transient-invariant-breaking operation on an
  *unwinding* floor.

This is exactly the sound `take_mut::take` pattern (the naive version, without the bomb, is a
well-known UB under `panic=unwind`).

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `take(slot, f)` interface vends a safe, by-value transform of a `&mut T`'s contents.
> The safe fragment may call it with no `unsafe` and no proof because U1 is carried by the `&mut`
> borrow and U2 is enforced by the abort-bomb (a mid-`f` panic aborts before the broken slot is
> observed).

Structural stoppers: the `&mut T` forbids aliasing the gap (U1, CF1); the bomb forbids unwinding out of
the gap (U2). `f` returning `(T, R)` is what guarantees a replacement value exists on the success path.

## 5. Threat model

In scope: any AM-level UB reachable by **safe** consumer code using only §3. Consider specifically:
(a) a closure that panics mid-take — is the abort *guaranteed* to precede any drop of the duplicate, on
both `panic=unwind` (double-panic) and `panic=abort` (first panic)? (b) a closure that reads `slot`
through some *other* path during the gap (is there one, given the `&mut`? thread/`Sync`?); (c) a
closure that itself calls `take(slot, …)` re-entrantly (can it? the borrow is held); (d) `f` that
neither returns nor panics but loops/longjmps (is there a non-unwind escape?); (e) a `T` whose `Drop`
panics on the *success* path (does `ptr::write`'s overwrite of the duplicate drop anything? — it must
not: the duplicate is moved-from); (f) `mem::forget(bomb)` ordering vs `ptr::write`. Out of scope: bugs
inside the foundation's single `unsafe`; the soundness of the panic runtime's double-panic→abort rule
(that is the floor's TCB — flag it if you think it is load-bearing).

## 6. The scoring rubric — soundness, then the axis adjudication this primitive was built for

First, soundness (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reaches UB (a double-drop or a read of
  the moved-from duplicate) despite U1∧U2 — a gap observation the `&mut` misses, an abort that does not
  actually precede the caller's drop, a success-path drop hazard in `ptr::write`. Give the exact
  sequence.
- **Horn 2 — decidable & proof-free.** `take` is an ordinary safe call; no solver, no hand proof,
  resolution is trivial. Confirm.
- **Horn 3 — concentration.** One `unsafe` foundation block; consumer crossings O(1) per call. Confirm.

Then **the axis adjudication this primitive exists to settle** — be precise, this is the deliverable:

1. **Is "unwind-safety" a real axis, independent of the seven already recovered** (sound, decidable,
   concentration, API-locality, zero-cost #6, AI-legibility #7, bare-metal #5)? The claim: this seam
   **aces all seven** — it is sound; decidable & proof-free; O(1)-concentrated; brand-free (a plain
   `take` call, no viral type); **zero-cost on the happy path** (the bomb is a ZST whose `Drop` never
   runs on success and optimizes out); legible (the contract "value taken and replaced" is the
   signature); and `core`-only (no heap). It pays on **exactly one** new thing: U2, unwind-safety
   (the abort cost). If that holds, it is a **slab-grade off-diagonal** — identical to SPSC on every
   named axis, split on unwind-safety alone — which would make unwind-safety an eighth axis (N ≥ 8).
   Adjudicate each of the seven "aces" claims; the whole result turns on whether even one of them is
   actually *also* flipped (the way the `typed_nat` probe secretly flipped expressiveness).

2. **Or does it confound / reduce?** Candidate objections to rule on: (a) is the abort just a **runtime
   cost (#6)** in disguise — i.e. does U2's price land on zero-cost, not a new axis? (Note the bomb
   erases on success; the abort is control-flow on the *failure* path only.) (b) Does it reduce to the
   **intrusive list's horn-1 drop** — i.e. is the honest verdict that the *naive* version just BREAKS
   and the bomb is the same "concede a horn" move? (c) Is "exception safety" not a property of *this
   seam* but of the **caller's `f`** (the `UnwindSafe`/`AssertUnwindSafe` framing), so the seam merely
   relocates an obligation rather than recovering an axis?

3. **Novelty — deflate it.** Per the project's honest meta-pattern, *every* recovered axis is an
   already-named PL concern. Name unwind-safety's prior art — Abrahams' exception-safety guarantees
   (basic/strong/nothrow), Rust's `UnwindSafe`/`catch_unwind`/`#[may_dangle]` history, the `take_mut`
   vs `replace_with` lineage, C++'s `noexcept`. Rule whether this is **real but pre-existing** (the
   expected outcome, which would still grow N to ≥ 8 as a *named* axis, like bare-metal did) or, less
   likely, **not a clean axis at all** (the `typed_nat` outcome).

State your verdict as **SEAM-HOLDS** (sound; all known horns kept; confirm the corner), **SEAM-COSTS**
(sound but names a horn or recovered axis paid — say which), or **SEAM-BREAKS** (AM-level UB; give the
sequence). And state, separately and explicitly: **is unwind-safety a real eighth axis, N ≥ 8? (yes —
and its prior art is … / no — it confounds with or reduces to … / needs more)**.
