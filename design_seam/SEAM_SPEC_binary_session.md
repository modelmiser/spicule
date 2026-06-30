# Seam spec — a real binary session (re-import rule under attack)

This is a **design proposal** for a twenty-third *seam*. It is **NOT a fourth-domain bid** — both
fourth-domain candidates (`SEAM_SPEC_crdt.md` convergence, `SEAM_SPEC_wcet.md` real-time) *deflated*. This
probe stays inside the **third confirmed domain — liveness/progress — and deepens it**, by removing the most
-cited limitation of the eighteenth seam (`SEAM_SPEC_session.md`).

The r25 panel's deflation #1 of the session seam was: *"not a binary session — a single-handle typestate
walk"*. One `&'a mut Transport` cannot split into two concurrently-live dual endpoints (`E0499`), so #18's
motivating **"a peer waits forever"** hazard was *not expressible* — its abandonment was *self*-abandonment of
one handle. This seam builds the **real** binary session: two dual endpoints, **both live at once**, sharing
the wire through `Rc<RefCell<Transport>>` (the `alloc` + interior mutability the duality costs).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/binary_session.rs`,
the default `no_std` build — `core` + `alloc`, **no `unsafe`**, like the `secret`/`crdt`/`wcet` semantic-domain
seams). 139 tests pass (default).

---

## 0. The floor (stipulated, not under attack)

Rust's type system as shipped: affine ownership (`!Copy`/`!Clone` handles consumed by each op), nominal
typing + private fields, trait resolution (for the `HasDual` relation), `Rc`/`RefCell` with their **runtime**
borrow checks, and the leakpocalypse fact — `core::mem::forget` / `ManuallyDrop` / leaking are **safe** and
skip `Drop` (RFC 1066). The transport's byte-moving is the **trusted core** (out of scope, like SPSC's
happens-before or #18's `Transport`). Do not attack `Rc`/`RefCell` soundness or the transport. Attack the
**re-import rule** (the two-party protocol claim) and the §6 deliverable.

## 1. The primitive

A **real binary session**: `session::<P>()` returns `(Chan<P>, Chan<P::Dual>)` — two affine endpoints on one
shared `Rc<RefCell<Transport>>`, at *dual* protocols (`HasDual` maps `Tx<C> ↔ Rx<C::Dual>`, `End ↔ End`). Each
op consumes the handle and returns the successor state: `Chan<Tx<C>>::send → Chan<C>`; `Chan<Rx<C>>::recv →
Result<(i32, Chan<C>), Self>`; `Chan<End>::close`. The danger is a **liveness** hazard — a peer abandons the
session and its dual **waits forever** — not UB.

## 2. The true precondition (cathedral side) — a composite, now tested against a REAL peer

"Protocol fidelity for a two-party session" decomposes into three components:
- **Safety (order):** neither endpoint performs an operation out of turn.
- **Relative progress (no mutual-wait deadlock):** the two peers never both block waiting to receive.
- **Mutual completion (no abandonment):** both endpoints run to `End` — neither is dropped/forgotten
  mid-protocol, leaving its peer waiting for a message that never comes.

## 3. The witness — affine typestate + a duality relation over a shared wire

`Chan<P>` is affine, `P` is *this endpoint's remaining protocol*. `send` exists only at `Tx`, `recv` only at
`Rx`, `close` only at `End` (order). `session::<P>()` types the second endpoint at `P::Dual` via the `HasDual`
trait, so the two protocols are duals by construction (relative progress). The two endpoints share one
`Rc<RefCell<Transport>>`, so a value sent on one is observed by `recv` on the other — a *real* peer-to-peer
transfer, not the same handle's earlier step.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** `session::<P>()` vends a **two-party ordered channel**. Safe code composes `send`/`recv`/
> `close` with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Order** — an out-of-turn operation is a type error (no such method at the wrong state).
> 2. **Relative progress** — the two endpoints hold dual protocols (`HasDual`), so they can never *both* block
>    waiting to receive: if one is `Tx`, its dual is `Rx`. Mutual-wait deadlock is a type error you cannot
>    write.
>
> The type system does **NOT** guarantee **mutual completion**: a `recv` returns `Result<…, Self>` —
> handing the handle **back** when no value is ready — *precisely because* the value depends on a peer that may
> never send. A peer may `mem::forget` its endpoint in safe code, abandoning the session; its dual's `recv`
> then stays `Err` forever (in a blocking transport, it waits forever). This is a **liveness** failure
> (a starved peer is memory-safe), not UB.

Structural stoppers (claimed): nominal non-subtyping + private fields forbid forging a `Chan` at a wrong
state; affinity forbids reusing a consumed handle; `HasDual` forbids binding a non-dual peer; the value escapes
`recv` only on `Ok`, so no out-of-order receive succeeds.

## 5. Threat model

In scope: any **safe** path that (a) performs an operation out of protocol order and type-checks; (b) makes the
two endpoints *both* block waiting (a mutual-wait deadlock the duality claims to foreclose); or (c) any claim
that the seam guarantees **completion** — show that abandonment (`mem::forget` of one endpoint) is safe and
starves the dual, and confirm whether that is a horn-1 break, a horn-3 cost, or a floor-mismatch. Out of scope:
`Rc`/`RefCell` internals; the trusted transport's byte-moving; bounded-wire backpressure (the loopback slot is
single-valued by design); async/runtime scheduling (the witness is single-threaded — "waits forever" is
modelled as `recv` returning `Err` indefinitely).

## 6. The scoring rubric — soundness, then the liveness-deepening deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** There is **no `unsafe`** anywhere, and `Rc`/`RefCell` mediate the shared wire
  with runtime borrow checks, so safe code cannot reach UB. Confirm the worst case is a **starved peer**
  (liveness), not memory corruption — or exhibit a genuine UB path (e.g. a `RefCell` double-borrow that is
  *unsoundness*, not merely a documented panic; or an order violation that type-checks).
- **Horn 2 — decidable & proof-free.** `send`/`recv`/`close`/`session` are ordinary safe calls; order and
  duality resolve by trait selection and method availability. Confirm — *but note whether Horn 2 passes only
  because the strong property (completion) is never actually checked* (the `secret`/`crdt`/`wcet` pattern).
- **Horn 3 — concentration.** Order and duality are concentrated in the types; the consumer writes no protocol
  bookkeeping. Confirm — or note the caller still carries the undischargeable **completion** obligation (don't
  abandon the session), which recurs at every `?`/panic/early-return on **either** side.

Then **the liveness-deepening deliverable** — the point of the phase. Rule on each:

1. **Does #18's component-wise grading SURVIVE removing the single-handle limitation?** #18 ruled: order
   *closes*, completion *costs horn 3*. With a real binary session: does **order still close** (the typestate
   discipline transfers across the duality boundary)? Does **relative progress now close** (a *new positive*
   the single-handle walk could not exhibit — there was no peer to deadlock against)? Does **completion still
   cost horn 3**, now **peer-observable** (a second party starves) rather than self-observable? Or does
   building the real thing change the **horn** — e.g. does forcing `recv` fallible, or the shared `RefCell`,
   introduce a break #18 did not have?
2. **Is the fallible `recv` the honest surfacing of the completion gap, or a defect?** #18's `recv` was
   infallible (`.expect` — sound, because the same handle did the preceding `send`). A real binary session
   forces `recv` fallible (the value comes from a peer who may never send). Rule: is making the completion gap
   visible *in the type of `recv`* an honest improvement over #18 (which hid it), or does the `Err`-handed-back
   design itself weaken the order guarantee (you can call `recv` and get nothing — is that an order hole or
   just the absence of a completion guarantee)?
3. **Does liveness stay the THIRD domain (N stays 9)?** This is a *deepening*, not a new domain or a new mode
   — the danger is still progress/liveness (deadlock/starvation), the same domain as `ordered_locks` (#15) and
   `lock_registry` (#16). Confirm N stays 9 and no new verdict mode is needed — or argue that peer-observable
   two-party starvation is categorically distinct from single-party deadlock and warrants a new coordinate.
4. **Prior-art deflation (pre-registered).** Real binary sessions in Rust are documented folklore: Honda 1993
   / Honda–Vasconcelos–Kubo 1998 (binary session types + the *progress* theorem that duality gives relative
   progress); the `session_types` Rust crate (Jespersen–Munksgaard–Larsen 2015, real two-endpoint sessions
   over channels, which *documents* that Rust cannot force completion — you may drop a channel); ECOOP 2022
   "Stay Safe Under Panic" (Cutner–Yoshida–Vassor), which *names* the abandonment gap and engineers around it.
   Confirm the contribution is **locating** the real binary session on the realizability lattice — and showing
   #18's grading was **not an artifact** of the single-handle limitation — not discovering the mechanism. Does
   it deflate onto #18 (`session`) + #17 (`linear`, the linearity gap) + the `HasDual` progress theorem,
   adding no new basis element?

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does #18's component-wise grading survive (order closes, relative progress
now closes as a new positive, completion still costs horn 3 — now peer-observable); is the forced-fallible
`recv` the honest surfacing of the completion gap or a defect; does liveness stay the third domain with N = 9;
and does the real binary session deflate onto known folklore (#18 + #17 + the binary-session progress theorem),
confirming the frontier's expectation that — after two deflated fourth-domain candidates — the three domains
hold?**
