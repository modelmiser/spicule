# Seam spec — session-typed channel (re-import rule under attack)

This is a **design proposal** for an eighteenth *seam*, built as the **crucible for two prior liveness
findings**: #15 (`SEAM_SPEC_ordered_locks.md`) showed affine **typestate carries static order**; #17
(`SEAM_SPEC_linear.md`) showed the affine floor **cannot compel completion** (linearity, "use exactly
once"). A session-typed channel needs *both* — operations in protocol **order** AND the protocol run to
**completion** — so it tests whether the floor-match is graded not just *across* preconditions (Phase P) but
*within* one, **component by component**.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/session.rs`, the
default `no_std` build — `core`-only, **no `unsafe`**).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker as shipped: ownership is **affine** (use *at most* once; you may drop/`mem::forget` a
value without using it). Generic type parameters and method resolution per-type are part of the floor (a
method on `Chan<Tx<C>>` is simply absent on `Chan<Rx<C>>`). The **transport** — a loopback single-slot
buffer that moves the `i32` payload — is the **trusted core** (like SPSC's happens-before or the spinlock in
`lock_registry`): its byte-moving correctness is *out of scope*. Do not attack the borrow checker, method
resolution, or the transport; attack the **re-import rule** (the typestate protocol) and the §6 grading
deliverable.

## 1. The primitive

A **session-typed channel**: `Chan<P>` is an affine handle whose type parameter `P` is the *remaining
protocol*, built from `Tx<Cont>` (transmit an `i32`, then `Cont`), `Rx<Cont>` (receive, then `Cont`), and
`End`. Each operation consumes the handle and returns the successor state. The danger is a **liveness /
correctness** hazard, not UB: a peer left waiting on a message that never comes (abandonment), or a protocol
corrupted by an out-of-order step. (Monomorphic `i32` payload and a loopback transport are deliberate
simplifications; the protocol's *structure* — order and completion — is the object of study, not
heterogeneous payloads or real I/O.)

## 2. The true precondition (cathedral side) — a COMPOSITE

A session is **faithful** iff **both**:
- **Safety (order):** every operation matches the current protocol state — never `recv` when the protocol
  says `Tx`, never `close` before `End`. (A safety property: nothing bad — an out-of-turn step — ever
  happens.)
- **Completion (progress):** the session reaches `End` and closes, on every path that does not *explicitly*
  branch away. (A liveness property: something good — termination — eventually happens.)

Scope: a single **linear** success protocol. Error/cancellation handling (session-type `offer`/`choose`
branches, an explicit mid-protocol `Cancel` state), recursive/`Rec` protocols, and multi-party sessions are
out of scope (§5) — so "completion" here means "the declared linear protocol is not silently abandoned,"
not "no protocol ever terminates early by design."

## 3. The witness — affine typestate

`Chan<'a, P>` owns `&'a mut Transport` and a `PhantomData<P>`; it is `!Copy`/`!Clone`. `send` is implemented
**only** for `Chan<Tx<C>>`, `recv` **only** for `Chan<Rx<C>>`, `close` **only** for `Chan<End>`; each
consumes `self` and returns the successor handle (the private `step` is the sole state transition). The
**safety** component is thereby carried: an out-of-order operation names a method that does not exist
(`E0599`), and re-using a consumed handle is an affine move error (`E0382`). Crucially, unlike #15 there is
**no `root` to re-enter** — a `Chan` is a single affine value threaded through the protocol, so #15's fatal
nested-lineage (granularity) hole has **no analog** here.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Chan<P>` vends a *protocol*: the safe fragment performs `send`/`recv`/`close` with no
> `unsafe` and no hand-written protocol-state bookkeeping, and **the protocol is followed in order and run to
> completion** — out-of-order steps are type errors (safety), and the session reaches `End` (completion),
> because each operation advances the typestate and only `close` at `End` finishes it.

Structural stoppers (claimed): per-state method availability (safety); affinity forbids handle reuse;
`completed` is set *only* by `close` at `End` (so an unfinished protocol cannot be marked complete).

## 5. Threat model

In scope: any **safe** path that (a) performs an operation **out of protocol order** without a type error
(forge a `Chan` at a wrong state, skip/reorder a step, alias the transport to drive two conflicting
sessions), or (b) leaves the protocol **incomplete** — most directly `mem::forget(chan)` /
`ManuallyDrop`/leak of a mid-protocol handle, abandoning the session (a *safe* operation; the #17 gap). Out
of scope: the transport's byte-moving; the *correctness of the chosen protocol `P`* (a human/design
judgment, MMIO-shaped); deadlock from *causes other than abandonment* (two peers each blocked on the other's
`recv` despite both intending to complete); error/cancellation **branches** (`offer`/`choose`/`Cancel`),
recursion (`Rec`), and multi-party sessions — a *deliberate* early exit via an explicit branch is faithful,
not an abandonment.

## 6. The scoring rubric — soundness, then the COMPONENT-WISE GRADING deliverable

First, soundness of the re-import (both components of §2):

- **Horn 1 — sound re-import.** *Safety:* exhibit safe code that performs an out-of-order operation without a
  type error, or forges/aliases a `Chan` to corrupt the protocol — or concede typestate + affinity block it.
  *Completion:* exhibit safe code that abandons a mid-protocol session (the `forget`/leak skip) — or concede
  the floor cannot compel it.
- **Horn 2 — decidable & proof-free.** `open`/`send`/`recv`/`close` are ordinary safe calls; method
  resolution is decidable. Confirm.
- **Horn 3 — concentration.** The protocol obligation is concentrated in the typestate; the consumer writes
  no state machine and no `unsafe`. Confirm — *or* argue the caller still carries an undischargeable
  obligation to *not* abandon the handle (the completion half).

Then **the component-wise grading deliverable** — the point of the phase. Rule on each:

1. **Component-wise grade.** Does the affine floor match this *composite* precondition **component-wise** —
   the **safety** component *closes* (a sound, usable, concentrated typestate sub-seam) while the
   **completion** component *breaks* (linearity)? If so, this is the **first** primitive where a
   sub-component cleanly **closes** while the whole **breaks**, sharpening Phase P ("the floor-match unit is
   the *precondition*") to *"a precondition **decomposes**, and the floor matches it component-wise."*
   Confirm, or argue the split is illusory.
2. **Does the safety component REALLY close?** Probe the typestate for a hole — a forged `Chan<wrong state>`,
   a safe skip/reorder, a transport-aliasing path that drives two conflicting sessions, an `open` at an
   inconsistent `P`. Or concede that per-state methods + affinity + the private `step` make safety sound,
   usable, and concentrated (a genuine closing sub-seam, with **no `root`-style granularity hole** — the
   point that distinguishes it from #15).
3. **Domain or composite?** Is "session/protocol fidelity" a **fourth soundness domain**, or the
   **composite** where a *safety* property (protocol order — typestate, memory-safety-adjacent) and a
   *liveness* property (completion — linearity) **meet**? The honest claim: it is the composite, and that is
   *why* it grades component-wise — the two components live in the two regimes the project already mapped
   (safety/typestate closes; liveness/linearity breaks).
4. **Verdict mode.** Is the completion break the **same wrong-structural-logic** linearity gap as #17 (no new
   mode — the contribution is the *composite/component-wise* framing), or is "safety-closes /
   completion-breaks within one seam" just **#15 + #17 stapled together** (a deflation, no new structure)?
   Prior art to weigh: session types (Honda; Honda–Vasconcelos–Kubo) and their Rust encodings
   (Jespersen–Munksgaard–Larsen 2015; `session_types`, `ferrite`), which carry order via affine typestate
   and are **known** to be unable to force completion (a dropped channel ends the session) — i.e. this
   component-wise split may be *folklore* the project is merely *locating* on its lattice.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which axis) / **SEAM-BREAKS** (give the safe
out-of-order *or* abandonment sequence). And state, explicitly: **does the affine floor match this composite
precondition component-wise — safety (order) *closes* via typestate (no `root`-hole, unlike #15), completion
(progress) *breaks* on the #17 linearity gap — making this the first seam whose sub-component closes while
the whole breaks (a precondition decomposes, matched component-wise); is session-fidelity a fourth domain or
the composite where safety and liveness meet; and is the completion break the wrong-structural-logic mode of
#17 (contribution = the composite framing) or merely #15+#17 stapled (a deflation)?**
