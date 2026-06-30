# Seam spec — multiparty session (n-party progress) (re-import rule under attack)

This is a **design proposal** for a twenty-fifth *seam*. It is **NOT a fourth-domain bid** — it stays inside the
**third confirmed domain (liveness)** and answers the question the twenty-third seam left open and the
twenty-fourth named. `SEAM_SPEC_binary_session.md`'s deflation #1 conceded it outright: "*two interleaved
`session()` calls type-check a mutual-wait deadlock; **global deadlock-freedom is out of reach of binary session
types — it needs multiparty/Kobayashi typing**.*" `SEAM_SPEC_delegation.md` then *named the coordinate* —
**progress / deadlock-freedom** — as distinct from completion (linearity) and leak-freedom. This seam builds the
multiparty thing and asks **where on the trilemma the progress coordinate lives at n-party scale**.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/multiparty.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, built *on* `binary_session`). 148 tests pass (default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership (handles `!Copy`/`!Clone`, consumed by each op), nominal typing + private
fields, trait resolution (`HasDual` on each binary edge), `Rc`/`RefCell` with their runtime borrow checks, and
the leakpocalypse fact (`mem::forget` is safe and skips `Drop`, RFC 1066). The binary edges' transport is #23's
trusted core (out of scope). Do not attack `Rc`/`RefCell`, the transport, or the binary-session re-import (it
survived as #23). Attack **the multiparty re-import rule** (the global-progress claim) and the §6 deliverable.

## 1. The primitive

A **multiparty session** assembled from #23's binary edges: each directed edge `X→Y` is one binary `session`,
and a *role* is a bundle of the edges it touches, driven in a role-local order. The assembly is deliberate —
the claim under test is that **multiparty progress is not reducible to the binary duality of the edges it is
built from**. A `RecvThenSend` role typestate enforces **recv-before-send** (`recv(self) -> Result<(i32,
JustSend), Self>`; there is no `send` before the `JustSend` a successful `recv` yields). Two constructors mint
role bundles from the **same** per-edge floor — only the global wiring differs:

- `relay() -> (JustSend, RecvThenSend, JustRecv)` — a **coherent** `A→B→C`: a source, a forwarder, a sink.
- `ring() -> (RecvThenSend, RecvThenSend, RecvThenSend)` — an **incoherent** cycle `A→B→C→A`: every role must
  receive before it sends, so there is **no source**.

The danger is a **liveness** hazard (a globally-stuck session), not UB.

## 2. The true precondition (cathedral side) — now GLOBAL

- **Role-local order:** no role performs an op out of its local state (recv-before-send for a forwarder).
- **Per-edge duality:** each binary edge is dual (carried by `HasDual`, as #23 — no *single edge* deadlocks).
- **Global progress (deadlock-freedom):** the *composition of all roles* has a terminating schedule —
  equivalently (here) the communication topology has a **source** role. This is a property of the **set** of
  roles, not of any one edge or role.
- **Mutual completion / leak-freedom:** as #23/#24 (every session runs to `End`; no wire stranded).

## 3. The witness — per-role affine typestate over binary edges

`RecvThenSend` exposes only `recv`; `send` lives on the `JustSend` it yields (role-local order, type-enforced).
Each binary edge is a #23 `session`, perfectly dual. `relay` has a `JustSend` source, so a terminating schedule
exists and it completes. `ring` is built from the **identical** building block but wired into a source-less
cycle — and it **type-checks**: every role-local type is well-formed, every edge is dual, yet no role can start.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A multiparty session vends typed role endpoints over binary edges. Safe code composes
> `recv`/`send`/`close` with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Role-local order** — an out-of-order op (e.g. a forwarder sending before it receives) is a type error.
> 2. **Per-edge duality** — each binary edge's two peers are at dual protocols (#23).
>
> The type system does **NOT** guarantee **global progress (deadlock-freedom)**: `ring()` type-checks a
> three-party cycle with **no source**, hence **no terminating schedule** — every role's `recv` returns `Err`
> forever, because reaching the `send` that would unblock a successor requires a receive that never arrives.
> Global progress is a property of the **whole composition** (a source must exist), which **no per-edge or
> per-role re-import rule can range over** — `relay` (which progresses) and `ring` (which deadlocks) are built
> from the **identical** per-edge floor; only the global topology differs, and the floor cannot see it. A
> deadlocked multiparty session is **memory-safe** (leakpocalypse), not UB.

Structural stoppers (claimed): nominal/private typing forbids forging a role at a wrong state; the `RecvThenSend
→ JustSend` typestate forbids sending before receiving; per-edge `HasDual` keeps each edge dual.

## 5. Threat model

In scope: any **safe** path that (a) performs a role op out of local order and type-checks; (b) shows the
re-import guarantees **global progress** — exhibit a deadlocking composition the floor accepts (`ring()`, three
dual edges, no source, no terminating schedule — `tests/multiparty_use.rs` drives it to permanent `Err`); (c)
argues the floor *could* range over the global topology (could a per-role type discipline reject the ring
without a whole-protocol global type?). Out of scope: `Rc`/`RefCell` internals; the binary transport;
async/threaded scheduling (single-threaded witness, as #23/#24); implementing a Scribble/`mpstthree`-style
global-type projection (the resolution is *argued from prior art*, not built — the gauntlet should weigh that).

## 6. The scoring rubric — soundness, then the concentration-leg deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a **globally-stuck (memory-safe)** session, not
  UB. Confirm — or exhibit a real UB path (an out-of-local-order op that type-checks; an edge re-typed wrong; a
  `RefCell` unsoundness rather than a documented panic).
- **Horn 2 — decidable & proof-free.** Per-role `HasDual` stays structural (finite types, Θ(depth), no SMT).
  Confirm — but note whether Horn 2 passes only because **global progress is never checked** (the
  `binary_session` pattern, one scale up).
- **Horn 3 — concentration.** Role-local order + linearity are concentrated in the types. Confirm — or note the
  caller still carries the **global-progress** obligation, which (the claim) **cannot be concentrated per-call**.

Then **the concentration-leg deliverable** — the point of the phase. Rule on each:

1. **Where does global progress live?** The hypothesis: a *global* property like deadlock-freedom **cannot be
   concentrated in a per-call re-import rule** — it can only be concentrated **whole-protocol**, in a trusted
   projection of a coherent *global type* to per-role *local types* (Honda–Yoshida–Carbone, POPL 2008;
   Coppo–Dezani–Yoshida–Padovani, MSCS 2016; the `mpstthree` Rust crate, Lagaillardie–Neykova–Yoshida,
   COORDINATION 2020 — deadlock-free *by construction of the generated endpoints*). Confirm or refute: could any
   *per-role* type discipline (no global type) reject `ring` while accepting `relay`? Is the seam's **location**
   genuinely property-scoped (local → per-call; global → whole-protocol / a *generative* seam)?
2. **SEAM-COSTS [horn 3] or SEAM-BREAKS [floor-mismatch]?** A deadlocked session is not UB (leakpocalypse). Is
   the right verdict SEAM-COSTS [horn 3] (the per-call re-import never promised global progress, and the
   per-role core CLOSES as #23), or SEAM-BREAKS [floor-mismatch] (the re-import *shape* — per-call — structurally
   cannot carry a whole-composition property)? Both, scoped how?
3. **Is this new, or #23's deflation #1 at n = 3?** #23 already asserted "interleaved binary sessions deadlock —
   global deadlock-freedom needs multiparty typing." Is multiparty's increment real (it makes that a *compiled*
   artifact and locates the resolution whole-protocol), or a re-description? Does the single-threaded witness
   make the "deadlock" degenerate (no concurrent cyclic-wait *hang*, just "no source → all-recv-`Err`")?
4. **Does liveness stay the THIRD domain (N stays 9)?** No new domain, no new coordinate (progress was #24's),
   no new mode. Confirm N stays 9 and the FAILED broken-seam tally stays 7 (no soundness break) — or argue
   multiparty / global progress is categorically new.
5. **Prior-art deflation (pre-registered).** Multiparty session types are deep folklore: Honda–Yoshida–Carbone
   2008; Coppo–Dezani–Yoshida–Padovani 2016; Kobayashi deadlock-freedom typing; deadlock-free GV
   (Lindley–Morris; Padovani; Multiparty GV PACMPL 2022); the `mpstthree`/Scribble Rust line. Confirm the
   contribution is **locating** multiparty progress on the lattice and showing global progress forces the
   concentration **whole-protocol** — its content is #23/#24's grading (order + linearity close, global progress
   is the whole-protocol concentration the floor cannot supply per-call).

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does global progress live whole-protocol (a generative seam) rather than in
any per-call re-import rule; is the right verdict SEAM-COSTS [horn 3] for the per-role core or SEAM-BREAKS
[floor-mismatch] for the re-import shape; is multiparty a real increment over #23's deflation #1 or a
re-description; does liveness stay the third domain with N = 9 and the FAILED tally at 7; and does multiparty
deflate onto known folklore (Honda–Yoshida–Carbone 2008; the `mpstthree` crate; Kobayashi), confirming the
contribution is locating the global-progress concentration whole-protocol?**
