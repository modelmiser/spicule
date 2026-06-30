# Seam spec — delegation (channel-over-channel) (re-import rule under attack)

This is a **design proposal** for a twenty-fourth *seam*. It is **NOT a fourth-domain bid** — it stays inside
the **third confirmed domain (liveness)** and adds the higher-order session feature the twenty-third seam
(`SEAM_SPEC_binary_session.md`) lacked. The r30 expressiveness attacker named **delegation** — sending a
channel endpoint as a payload over another channel — as "*the* feature that distinguishes session types from
coupled typestate machines." This seam adds it, and asks whether the higher-order step opens new ground or
**deflates onto the existing basis** (a Phase-K-style closed-basis confirmation, one domain over).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/delegation.rs`,
default `no_std` build — `core` + `alloc`, **no `unsafe`**, like the `binary_session`/`crdt`/`secret` seams).
143 tests pass (default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership (handles `!Copy`/`!Clone`, consumed by each op — crucially, **the affine
move carries "don't use a channel after you delegate it"**), nominal typing + private fields, trait resolution
(for `HasDual`, now recursing through the *delegated* protocol `Q`), `Rc`/`RefCell` with their runtime borrow
checks, and the leakpocalypse fact (`mem::forget` is safe and skips `Drop`, RFC 1066). The transport's
byte-moving is the trusted core (out of scope, like #23's). Do not attack `Rc`/`RefCell` or the transport.
Attack the **re-import rule** (the delegation claim) and the §6 deliverable.

## 1. The primitive

A **delegating session**: on top of #23's `Rc<RefCell<Transport>>`-backed dual endpoints, the wire's slot now
carries **either** an `i32` **or** a whole delegated endpoint. Two markers extend `Tx`/`Rx`/`End`:
`Chan<TxChan<Q, C>>::send_chan(c: Chan<Q>) -> Chan<C>` hands off a live `Chan<Q>` session; its dual
`Chan<RxChan<Q, C>>::recv_chan() -> Result<(Chan<Q>, Chan<C>), Self>` receives it, re-attaching `Q`. `HasDual`
passes `Q` through unchanged. Delegation is inherently **three-party**: A delegates its session-with-C to B, so
B now drives that session with C. The danger is a **liveness** hazard (a starved or **leaked** session), not UB.

## 2. The true precondition (cathedral side) — a composite, now higher-order and three-party

- **Order (delegation in turn):** neither carrier endpoint performs an op out of state.
- **Linearity of the delegated handle (don't-use-after-delegate):** once A delegates `c`, A must not keep
  driving `c`.
- **Mutual completion (no abandonment):** every session — carrier *and* delegated — runs to `End`; nobody is
  dropped/forgotten mid-protocol.
- **Leak-freedom:** no session's wire is stranded alive-but-unreachable.

## 3. The witness — affine typestate + a duality relation that recurses through the payload protocol

`send_chan` exists only at `TxChan`, `recv_chan` only at `RxChan` (order). `send_chan(self, c)` **consumes** `c`
(don't-use-after-delegate, by the affine move — the one linearity the floor *does* enforce). The delegated
`Chan<Q>` is type-erased to a bare `Rc` in the slot and re-typed `Q` on receipt; this is sound because
`HasDual` pins `Q` identically on both carrier endpoints.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A delegating session vends **channel-over-channel handoff**. Safe code composes
> `send`/`recv`/`send_chan`/`recv_chan`/`close` with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Order** — an out-of-turn op is a type error.
> 2. **Don't-use-after-delegate** — `send_chan` consumes the delegated handle (affine move), so driving a
>    delegated channel afterward is `use of moved value`. *This is the linearity delegation needs here, and the
>    floor has it for free.*
>
> The type system does **NOT** guarantee **mutual completion** — now **three-party**: the delegatee B can
> `mem::forget` the received `Chan<Q>`, starving the third party C (a session B never contracted to complete).
> Nor does it guarantee **leak-freedom**: the delegated payload is an `Rc<RefCell<Transport>>`, so **abandoning
> a carrier with a delegated channel in flight LEAKS the payload session** (`send_chan` parks the payload's
> `Rc` in the carrier's slot; a `mem::forget` of the carrier skips its `Drop`, stranding that `Rc` — the
> canonical `Rc<RefCell>` leak, amplified through delegation). Both are **liveness** failures (a leaked or
> starved session is memory-safe), not UB.

Structural stoppers (claimed): nominal/private typing forbids forging a `Chan` at a wrong state; affinity
forbids reusing a delegated handle; `HasDual` pins `Q` so the slot's type-erasure is sound; the value escapes
`recv_chan` only on `Ok`.

## 5. Threat model

In scope: any **safe** path that (a) performs a delegation op out of order and type-checks; (b) drives a
delegated channel after handing it off (the affine-move claim); (c) shows the re-import guarantees **completion**
or **leak-freedom** — exhibit the three-party abandonment (a forgotten delegatee starves C) and the leak
amplification (a forgotten carrier strands an in-flight delegation, `tests/delegation_use.rs` shows it via
`wire_strong_count` against a dropped-carrier control). Also: can safe code build an `Rc` **cycle** of mutual
delegations (does the protocol-type recursion obstruct it)? Out of scope: `Rc`/`RefCell` internals; the trusted
transport; bounded-wire backpressure; async scheduling (single-threaded witness, as #23).

## 6. The scoring rubric — soundness, then the closed-basis deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; `Rc`/`RefCell` mediate sharing; the type-erased slot is re-typed
  only via duality-pinned `Q`. Confirm the worst case is a **leaked or starved** (memory-safe) session, not UB
  — or exhibit a real UB path (an order violation that type-checks; a `recv_chan` that re-types a wire to the
  wrong `Q`; a `RefCell` unsoundness rather than a documented panic).
- **Horn 2 — decidable & proof-free.** `HasDual` now recurses through `Q` but stays structural (finite types,
  Θ(depth), no SMT). Confirm — but note whether Horn 2 passes only because completion/leak-freedom are never
  checked (the `crdt`/`binary_session` pattern).
- **Horn 3 — concentration.** Order + don't-use-after-delegate are concentrated in the types. Confirm — or note
  the caller still carries the undischargeable completion obligation (now spanning a third party) and the
  leak-freedom obligation.

Then **the closed-basis deliverable** — the point of the phase. Rule on each:

1. **Does the higher-order step add a NEW coordinate, or PROJECT onto the existing basis?** The hypothesis:
   delegation adds **no** new axis. Its hazards are (a) #23's completion gap, now **three-party**; and (b) a
   **leak** — abandoning a carrier with an in-flight delegation strands the payload session — which **projects
   onto the leak-freedom axis (#9 `take` / #10 `drain`)**. If so this is the **Phase-K closed-basis pattern**
   recurring in the session domain: a genuinely new danger class lands on an existing coordinate. Confirm or
   refute (is leak-amplification a distinct coordinate from leak-freedom?).
2. **Is the leak a horn-3 COST or a floor-mismatch?** A stranded session is not UB (leakpocalypse: leaks are
   safe). Is the right verdict SEAM-COSTS [horn 3] (the re-import never promised leak-freedom), or
   SEAM-BREAKS [floor-mismatch] (the affine floor cannot carry the no-strand precondition over `Rc` payloads)?
   And: does the protocol-type recursion **obstruct** the symmetric two-session delegation cycle (a minor
   positive the floor provides), or is a cycle constructible via the type-erased slot?
3. **Does liveness stay the THIRD domain (N stays 9)?** This is a higher-order *deepening*, not a new domain or
   mode. Confirm N stays 9 — or argue three-party / higher-order delegation is categorically new.
4. **Prior-art deflation (pre-registered).** Higher-order session types with delegation are deep folklore:
   Honda 1993; **Gay & Hole 2005** (subtyping for session types, with delegation); the `session_types` Rust
   crate (delegation over real channels); and the `Rc<RefCell>` abandonment/cycle leak is the canonical Rust
   footgun (RFC 1066 leakpocalypse). Confirm the contribution is **locating** delegation on the lattice and
   showing the higher-order step adds **no new basis element** — its content is #23's grading (order closes,
   completion costs horn 3) plus the leak-freedom axis, now three-party and higher-order.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does delegation add a new coordinate or project onto the leak-freedom axis
(#9/#10) — the Phase-K closed-basis pattern in the session domain; is the in-flight-delegation leak a horn-3
cost or a floor-mismatch; does the protocol-type recursion obstruct the symmetric delegation cycle; does
liveness stay the third domain with N = 9; and does delegation deflate onto known folklore (Gay & Hole 2005,
the `session_types` crate, the `Rc<RefCell>` leak), confirming the higher-order step opens no new ground?**
