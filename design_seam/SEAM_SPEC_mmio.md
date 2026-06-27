# Seam spec — MMIO read-to-clear device register (re-import rule under attack)

This is a **design proposal** for a second *seam* in the same hypothetical systems language
(same floor, same rules) as the SPSC ring-buffer seam. A seam re-exports a dangerous
primitive — whose true safety precondition lives outside the decidable safe fragment — as a
checked safe interface ordinary code uses with no `unsafe` and no proofs. Attack the
re-export: can ordinary safe code, using only this interface, reach undefined behavior (UB)?

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity,
`Drop`, `Send`/`Sync`, trait resolution). Assume it is sound for the single-threaded safe
fragment. **Do not attack the borrow checker.** Attack only what the seam adds: the handle
type, the `take` constructor, and the read/write interface.

## 1. The primitive

A **memory-mapped I/O (MMIO) "read-to-clear" status register** on a microcontroller. A
fixed physical address `ADDR` (e.g. `0x4000_4400`) is wired to a peripheral. Reading it
returns the current status bits **and has a side effect**: the act of reading *clears* the
"pending" bits in hardware (read-to-clear semantics, common for interrupt-status registers).
Writes configure the peripheral. There is no RAM at `ADDR`; the address denotes hardware.

## 2. The true precondition (cathedral side — and note where it lives)

The primitive is sound only if **all** of the following hold:

- **M1 (it is really a register).** `ADDR` is a valid, mapped MMIO location backed by the
  peripheral — not unmapped (a fault), not ordinary RAM, not aliasing another peripheral.
- **M2 (volatile access).** Every access compiles to exactly one real load/store of the
  right width; the compiler must NOT elide, duplicate, reorder across, fuse, or cache it.
  A read-to-clear register read by the optimizer twice clears twice; elided, it never clears.
- **M3 (exclusive ownership of the peripheral).** At most one part of the program drives
  this peripheral; two unsynchronized owners issuing read-to-clear reads lose interrupts.
- **M4 (datasheet bit-semantics).** Which bits clear on read, which are write-1-to-clear,
  reserved bits, and access width are defined **only by the chip's datasheet** — an external
  physical document, not anything in the program text.

**Where these live.** M3 is a cardinality fact (like SPSC's P4). M2 is a compiler-contract
fact (volatile). But **M1 and M4 are facts about the physical world** — they are not
propositions about the program at all, and *no* certificate, decidable or undecidable, can
discharge them from inside the language. They are axioms asserted by fiat.

## 3. The token (cheap, floor-checkable — and strictly weaker)

The seam emits exactly:

- An affine handle `StatusReg` obtained once via `unsafe fn Peripheral::take() -> StatusReg`
  (the single `unsafe` the *application* author writes, at startup, asserting M1/M4 — "I have
  read the datasheet and this address is this register"). `take` returns `None`/panics if
  called twice (run-time singleton), so at most one `StatusReg` exists.
- `StatusReg` is **affine** (no `Copy`/`Clone`) and **`!Sync`** (and `!Send` unless the HAL
  opts in), so it cannot be duplicated or shared across threads from safe code.
- Safe interface methods (no `unsafe`, no proof obligation on the caller):
  - `StatusReg::read_status(&mut self) -> Status`   (performs the volatile, read-to-clear read)
  - `StatusReg::configure(&mut self, cfg: Config)`  (volatile write)

The token carries **M3** (affinity + run-time singleton ⇒ one owner) and **M2** (the methods
are *defined* to emit volatile accesses — the foundation author writes `read_volatile`
inside). It **cannot carry M1 or M4** — those are discharged by the `unsafe take()` axiom and
thereafter trusted.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A value of type `StatusReg` is a certificate that its holder is the unique
> owner of a real, datasheet-correct MMIO register. The safe fragment may therefore type
> `read_status`/`configure` as ordinary safe methods — no `unsafe`, no proof — trusting that
> the `take()` axiom (M1/M4) and the foundation author's `read_volatile` (M2) are correct.

Structural stoppers: affinity + the run-time singleton stop a second owner (M3); the methods
encapsulate the volatile intrinsics so the consumer never names a raw pointer (M2).

## 5. Threat model (state it, don't hide it)

In scope: any UB reachable by **safe** consumer code holding a `StatusReg` and calling only
§3 methods. **Out of scope:** a bug inside the foundation author's `read_volatile` glue; and
the correctness of the `take()` axiom itself (M1/M4 — whether `ADDR` really is this register
with these bit-semantics). The honest question this seam forces: *what is the status of a
seam whose soundness rests on an axiom that is true or false about the physical world, and
that no checker — in this language or any — can verify?*

## 6. The scoring rubric — which horn does your attack hit?

Same three horns as the SPSC seam:

- **Horn 1 — sound re-import.** Safe consumer code reaches UB *in the Rust abstract-machine
  sense* (data race, UAF, read of uninit, etc.). Note: a volatile read of a *mapped* address
  that returns the "wrong" value is **not** UB — it is a functional bug. Distinguish (a) a
  genuine AM-level UB path in safe code from (b) functional incorrectness downstream of a
  false M1/M4 axiom. Which did you find?
- **Horn 2 — decidable & proof-free safe fragment.** Show the interface can only be made
  sound if the *consumer* must write a proof or thread a token through ordinary code.
- **Horn 3 — seam concentration.** Show realistic consumer code must cross the seam (write
  `unsafe`) repeatedly rather than once at `take`.

Additionally — and this is the point of this second primitive — **does this seam introduce
an obligation class that is neither "decidable witness" nor "undecidable logic," but simply
*outside the formal system entirely* (a physical-world axiom)?** If so, name it: is the seam
"sound relative to a trusted external axiom," and is that the same thing as the trilemma or a
different shape? Be precise.
