# Seam spec — SPSC lock-free ring buffer (re-import rule under attack) — v3

> **Amendment provenance (2026-08-12, v3).** Lineage: v1 (frozen) → v2 (P4 restated as
> role exclusivity after the foreign-model replication exposed the literal-"ever"
> wording defect; P4-v2 then HELD under all five attacks in its gauntlet) → v3 (this
> file), repairing the two completeness gaps the v2 gauntlet surfaced instead:
> **(a)** `N` was safe-chosen, unbounded, and erased from the handle types — at `N=0`
> P3's empty/full discipline has no model, making "holds by construction" vacuous;
> repaired by a const-decidable `where N >= 2` bound at `split`. **(b)** §2 stated no
> teardown obligation; repaired by P6 (drop/teardown), discharged once inside the seam.
> The v1/v2 specs and reports remain frozen; this file re-enters the gauntlet fresh.

This is a **design proposal** for one *seam* in a hypothetical systems language. A seam
is the boundary where a dangerous primitive — whose true safety precondition lives in a
rich, undecidable concurrent separation logic — is implemented once by a *foundation
author* behind an `unsafe` block, then **re-exported as a checked safe interface** that
ordinary code uses with no proofs and no `unsafe`. Your job is to attack the re-export
(the "re-import rule"): can ordinary safe code, using only this interface, be made to
exhibit undefined behavior (UB)?

You are attacking a **text specification**, not source code. Reason about the rules as
stated.

---

## 0. The floor (stipulated, not under attack)

The safe fragment is **Rust's borrow checker exactly as shipped** (Non-Lexical Lifetimes:
moves, shared `&T` vs. unique `&mut T` borrows, lifetimes as region constraints, affine
ownership, `Drop` glue, the `Send`/`Sync` auto-trait system, trait resolution). Assume it
is sound for the single-threaded safe fragment — that is *given*, proven by the fact that
hundreds of thousands of programs rely on it. **Do not attack the borrow checker itself.**
Attack only what the seam *adds on top of it*: the handle types, their auto-trait
derivations, the re-import rule, and the interface methods.

## 1. The primitive

A **single-producer / single-consumer (SPSC) lock-free ring buffer** of capacity `N` over
element type `T`. At any given time one thread pushes; one (different) thread pops; no
locks. Backing store is a fixed array of `N` slots plus two atomic indices `head` (next
slot to pop) and `tail` (next slot to push), each a `usize`.

## 2. The true precondition (cathedral side — rich, undecidable)

The primitive is memory-safe only if **all** of the following hold. These are stated in
the rich logic the foundation author reasons in; collectively they are an Iris/RustBelt-
grade concurrent-separation-logic predicate (no decidable certificate captures them):

- **P1 (per-slot exclusion).** At every instant each slot is owned by exactly one role:
  producer-writable, published (consumer-readable), or empty. No slot is simultaneously
  written by the producer and read by the consumer.
- **P2 (happens-before / publication).** The producer's write of slot `i`'s payload
  *happens-before* the consumer's read of slot `i`. Concretely: producer does
  `store(tail, …, Release)` after writing the slot; consumer does `load(tail, Acquire)`
  before reading it. The Release/Acquire pair is the only thing making the payload write
  visible to the consumer thread.
- **P3 (index discipline).** `head` and `tail` advance modulo `N`; the buffer is empty iff
  `head == tail` and full iff `tail + 1 == head (mod N)`. The producer never overruns a
  full buffer; the consumer never reads an empty one.
- **P4 (role exclusivity — v2).** Producer-role operations (`push`) never overlap in time:
  at every instant at most one thread is executing a producer-role operation, and all
  producer-role operations are **totally ordered by happens-before**. Likewise, mutatis
  mutandis, for consumer-role operations (`pop`). A handle MAY migrate between threads
  sequentially, provided each transfer itself establishes happens-before from the last
  role operation on the old thread to the first on the new one. (SPSC remains *unsound*
  with two **overlapping** producers: two `Release` stores to `tail` race and two writers
  can claim the same slot. It is also unsound under a non-synchronizing migration: the
  new thread could write a slot without observing the old thread's prior slot writes.)
- **P5 (payload thread-safety).** Moving a `T` from the producer thread to the consumer
  thread is itself sound — i.e. `T` may be sent across a thread boundary.
- **P6 (teardown).** When either handle is dropped, the buffer enters a closed regime;
  when the *second* handle is dropped, every published-but-unpopped payload is dropped
  **exactly once** and the backing store is freed exactly once, after a happens-before
  edge from the last role operation on the other handle (the seam's drop glue holds the
  synchronization). No payload is dropped twice, leaked, or read after the store is
  freed. This obligation is discharged once, inside the seam, like P1–P3.

P1–P3 are the standard ring-buffer protocol; P2 is a weak-memory happens-before fact; P4
is a *protocol/exclusion* fact over role operations; P5 is a thread-safety fact about `T`;
P6 is the drop/teardown protocol. None of P1–P6 is structurally checkable by the NLL
floor on its own.

## 3. The token (cheap, floor-checkable — and strictly weaker)

The seam emits exactly this, and nothing more:

- A constructor `fn split<T, const N: usize>(buf: RingBuffer<T, N>) -> (Producer<T>,
  Consumer<T>) where [(); N - 2]:` — i.e. **`N >= 2` is enforced as a const bound at
  `split`**, decidably, once per buffer (rejecting `N = 0`, where P3's empty/full
  discipline has no model, and `N = 1`, which the full test `tail + 1 == head (mod N)`
  cannot distinguish from empty). The constructor **consumes** the buffer by value and
  returns two endpoint handles; `N` may be erased from the handle types because every
  index computation lives inside the seam.
- `Producer<T>` and `Consumer<T>` are **affine** (move-only): neither is `Copy` nor
  `Clone`. There is no other way to obtain either handle.
- Interface methods, both **safe** (no `unsafe`, no proof obligation on the caller):
  - `Producer::push(&mut self, x: T) -> Result<(), T>`  (`Err(x)` returns `x` if full)
  - `Consumer::pop(&mut self) -> Option<T>`             (`None` if empty)
- Auto-trait derivations the foundation author asserts: `Producer<T>: Send`,
  `Consumer<T>: Send`, and **neither is `Sync`**.

**What the token carries vs. what it omits.** Affinity (move-only, no `Clone`) plus the
`&mut self` receivers statically discharge **P4 as stated in v2**: there is exactly one
producer handle, `&mut` exclusivity forbids overlapping `push` calls through it, and every
*safe* mechanism that moves the handle to another thread (channel send, `Mutex`,
`thread::spawn` closure capture) synchronizes — establishing the required happens-before
edge on each migration. That conjunct, and only that one, is expressible in the floor.
**P1, P2, P3, and P6 are *not* carried by the token** — they are discharged *once*,
inside the seam's `unsafe` block (P6 in its drop glue), by the foundation author writing
the correct atomics, and thereafter *trusted*. The token is therefore **provably strictly weaker** than the precondition of §2:
it certifies role exclusivity structurally and asks the floor to take P1–P3 (and P5) on
trust.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A value of type `Producer<T>` (resp. `Consumer<T>`) is a *certificate*
> that its holder is the unique producer (resp. consumer) of one ring buffer whose
> protocol invariants P1–P3 were discharged inside the seam. The safe fragment may
> therefore type `push`/`pop` as **ordinary safe methods** — calling them requires no
> `unsafe` and no proof — and may treat the returned/accepted `T` as an ordinary owned
> value. The foundation author's internal `unsafe` is trusted; the consumer re-imports its
> guarantees through the affine handle.

The structural stoppers the rule relies on:
- **No duplication.** `Producer`/`Consumer` are not `Copy`/`Clone`, so safe code can never
  manufacture a second producer or consumer ⇒ P4 cannot be violated from safe code.
- **`!Sync`.** Neither handle is `Sync`, so safe code cannot share `&Producer` across
  threads; the only way to move a handle to another thread is to `Send` the whole handle
  (moving it away), preserving "one producer thread at a time."
- **`Send` requires `T: Send`** (the foundation author writes `impl<T: Send> Send for
  Producer<T>`), so P5 is discharged by ordinary decidable trait resolution.

## 5. Threat model (state it, don't hide it)

In scope: any UB reachable by **safe** consumer code (no `unsafe` keyword in consumer
code) that holds `Producer`/`Consumer` handles and calls only the §3 interface. Also in
scope: a `T` chosen adversarially (any `Drop`, any interior type), and the handles being
moved between threads. **Out of scope:** a bug *inside* the foundation author's `unsafe`
block (assume the atomics are written correctly — P1–P3 hold by construction); and the
correctness of the compiler / Rust abstract machine honoring atomic orderings.

## 6. The scoring rubric — which horn does your attack hit?

This seam claims all three of the following at once. The design conjecture is that for
*this* primitive all three can hold. **Classify your strongest finding by the horn it
threatens:**

- **Horn 1 — sound re-import.** Crossing back launders no UB into safe code. *Break this*
  by exhibiting safe consumer code (only §3 methods, no `unsafe`) that causes a data race,
  use-after-free, double-free, or other UB. **Give the exact sequence.** This is the
  highest-value attack: a concrete laundering path.
- **Horn 2 — decidable & proof-free safe fragment.** Ordinary code type-checks with no
  user proofs and no SMT. *Break this* by showing the interface can only be made sound if
  the *consumer* must write a proof, carry a ghost/branded token threaded through its code,
  or invoke an external prover.
- **Horn 3 — seam concentration.** Ordinary code touches the seam rarely — ideally once at
  `split`, then `push`/`pop` are seam-free. *Break this* by showing realistic consumer
  code must cross the seam (write `unsafe`, or re-discharge an obligation) repeatedly, on
  the order of once per use rather than once per buffer.

A finding that fits none of these but still kills the design is also valuable — say so.
