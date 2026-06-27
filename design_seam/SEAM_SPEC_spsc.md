# Seam spec — SPSC lock-free ring buffer (re-import rule under attack)

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
element type `T`. One thread pushes; one (different) thread pops; no locks. Backing store
is a fixed array of `N` slots plus two atomic indices `head` (next slot to pop) and `tail`
(next slot to push), each a `usize`.

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
- **P4 (cardinality).** There is **at most one** producer thread and **at most one**
  consumer thread, ever. (SPSC is *unsound* with two producers: two `Release` stores to
  `tail` race and two writers can claim the same slot.)
- **P5 (payload thread-safety).** Moving a `T` from the producer thread to the consumer
  thread is itself sound — i.e. `T` may be sent across a thread boundary.

P1–P3 are the standard ring-buffer protocol; P2 is a weak-memory happens-before fact; P4
is a *cardinality/protocol* fact; P5 is a thread-safety fact about `T`. None of P1–P5 is
structurally checkable by the NLL floor on its own.

## 3. The token (cheap, floor-checkable — and strictly weaker)

The seam emits exactly this, and nothing more:

- A constructor `fn split<T>(buf: RingBuffer<T, N>) -> (Producer<T>, Consumer<T>)` that
  **consumes** the buffer by value and returns two endpoint handles.
- `Producer<T>` and `Consumer<T>` are **affine** (move-only): neither is `Copy` nor
  `Clone`. There is no other way to obtain either handle.
- Interface methods, both **safe** (no `unsafe`, no proof obligation on the caller):
  - `Producer::push(&mut self, x: T) -> Result<(), T>`  (`Err(x)` returns `x` if full)
  - `Consumer::pop(&mut self) -> Option<T>`             (`None` if empty)
- Auto-trait derivations the foundation author asserts: `Producer<T>: Send`,
  `Consumer<T>: Send`, and **neither is `Sync`**.

**What the token carries vs. what it omits.** Affinity (move-only, no `Clone`) statically
discharges **P4**: because each handle is unique and unforgeable, there is exactly one
producer and one consumer. That conjunct, and only that one, is expressible in the floor.
**P1, P2, P3 are *not* carried by the token** — they are discharged *once*, inside the
seam's `unsafe` block, by the foundation author writing the correct atomics, and thereafter
*trusted*. The token is therefore **provably strictly weaker** than the precondition of §2:
it certifies cardinality structurally and asks the floor to take P1–P3 (and P5) on trust.

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
