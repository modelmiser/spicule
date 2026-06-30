# Seam spec — priority-typed sessions (rung-(b)) (re-import rule under attack)

This is a **design proposal** for a twenty-sixth *seam*. It is **NOT a fourth-domain bid** — it stays inside the
**third confirmed domain (liveness)** and builds the exact rung the twenty-fifth seam's rejection handed back.
`REPORT_SEAM_SPEC_multiparty.md` (r32) ruled that `SEAM_SPEC_multiparty.md`'s deliverable staked a **false
dichotomy** — that global progress lives *only* whole-protocol in a *generative* global-type projection — and
named the omitted middle rung: **per-channel priority/obligation typing** (Kobayashi linear-π; Padovani;
**Dardha–Gay, FoSSaCS 2018**; Kokke–Dardha), which rejects the source-less ring *compositionally, with no global
type*. This seam **builds that rung** and asks where on the trilemma it sits.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/priority.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, built on `binary_session` #23 and `typed_nat` #8). 150 tests
pass (default).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing + private fields; **const trait-bound resolution** (this is
the load-bearing capability here — `Pi: Lt<Po>` discharged by recursive impls); `Rc`/`RefCell`. The sealed
`typed_nat::Lt` is #8's machinery, already attacked and survived (Peano `<`: `Z: Lt<S<M>>`; `S<I>: Lt<S<M>>` iff
`I: Lt<M>`; **no `Lt<Z>`, no reflexive `Lt<N>`**). The binary edges' transport is #23's trusted core (out of
scope). Do not attack `typed_nat`'s soundness, `Rc`/`RefCell`, or the transport. Attack the **priority re-import
rule** (the deadlock-freedom claim) and the §6 deliverable.

## 1. The primitive

A **priority-typed session**: over #23's binary edges, a *role* receives on its inbound edge at priority `Pi`
then sends on its outbound edge at priority `Po`, under the discipline **`Pi: Lt<Po>`** (Kobayashi/Dardha–Gay: an
action may not block on one of equal-or-higher priority). Three role shapes: `Source<Po>` (pure send),
`Sink<Pi>` (pure recv), `Relay<Pi, Po>` (recv-then-send, `Pi: Lt<Po>`). `relay()` mints a coherent `A→B→C` with
an acyclic assignment (`p_AB=N0 < p_BC=N1`) that completes; the deadlocking ring `A→B→C→A` is **untypable**
(below). The danger addressed is a **liveness** hazard (a deadlock), not UB.

## 2. The true precondition (cathedral side)

- **Role-local order:** recv before send (affine typestate, as #18/#23).
- **Priority monotonicity (per role):** `Pi: Lt<Po>` — receive at a strictly lower priority than you send.
- **Global acyclicity (deadlock-freedom):** a single, globally-consistent priority assignment exists making all
  roles' `Lt` bounds simultaneously satisfiable — equivalently, the wait-for graph is acyclic.

## 3. The witness — a const `Lt` bound on each forwarder

`Relay<Pi, Po>::recv` and the `role::<Pi, Po>(..)` constructor carry `where Pi: Lt<Po>`. The ring forces
`p_CA: Lt<p_AB>`, `p_AB: Lt<p_BC>`, `p_BC: Lt<p_CA>`, chaining to `p_CA: Lt<p_CA>` — no instance, for **any**
concrete assignment — so the ring cannot be constructed (a `compile_fail`). The relay picks `N0 < N1` and runs.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A priority-typed session vends roles whose recv-then-send is gated by `Pi: Lt<Po>`. Safe code
> composes them with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Role-local order** — out-of-order ops are type errors (as #23).
> 2. **Per-channel priority monotonicity** — a role with `Pi` not `Lt<Po>` has no `recv` and cannot be built.
> 3. **Deadlock-freedom of any well-typed assembly** — *because* the cyclic priority chain is untypable, a
>    *fully priority-typed* assembly that compiles has an acyclic wait-for order, hence a terminating schedule:
>    the source-less ring #25's bare floor accepted is here a `compile_fail`.
>
> The type system does **NOT** discharge the **global acyclic assignment** for you: the `Lt` bounds are *local*
> and *per-call*, but their joint satisfiability — that *some* consistent acyclic assignment exists — is a
> **whole-program** obligation the compiler only *checks* (against the concrete `Nat`s the programmer wrote),
> never *searches*. Edge-priority **consistency across the two endpoints** of each edge is the **constructor's**
> invariant (`relay` threads the same `Nat`), not a per-endpoint type fact. And a programmer may always drop to
> the bare floor (#25) and rebuild the deadlock — the guarantee is conditional on opting into the discipline.

Structural stoppers (claimed): no reflexive `Lt` ⇒ no self-loop; the cyclic chain is unsatisfiable ⇒ ring
untypable; `recv` exists only when `Pi: Lt<Po>`.

## 5. Threat model

In scope: any **safe** path that (a) builds a priority-typed cycle that type-checks (defeating the `Lt`
discipline); (b) shows the re-import discharges the **global assignment** for the programmer (rather than merely
checking a supplied one); (c) shows edge-priority consistency is enforced **per-endpoint** rather than by the
constructor; (d) argues the discipline is decidable-but-impractical (assignment search, recursion). Out of
scope: `typed_nat`/`Rc`/`RefCell` internals; the transport; async/threaded scheduling (single-threaded witness,
as #23–#25); implementing priority *inference* (Scribble-style — the resolution is argued, not built).

## 6. The scoring rubric — soundness, then the rung-(b) deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a memory-safe starved/leaked session; the
  priority discipline only *adds* rejections, never UB. Confirm — or exhibit a typed cyclic assembly that
  deadlocks, or a `Lt`-bound bypass.
- **Horn 2 — decidable & proof-free.** `Pi: Lt<Po>` is #8's decidable Peano `<` (Θ(depth), no SMT). Confirm —
  but note whether decidability of the *check* hides the cost of the *assignment search*.
- **Horn 3 — concentration.** Role-local order + per-channel priority monotonicity are concentrated in the
  types. Confirm — or note the caller still carries the **whole-program acyclic-assignment** obligation and the
  **edge-consistency** invariant.

Then **the rung-(b) deliverable** — the point of the phase. Rule on each:

1. **Does priority typing carry deadlock-freedom per-call, or whole-program?** The hypothesis (built to
   **confirm** #25, not extend it): the `Lt` bounds are local/per-call and *do* make the ring untypable (where
   #25's bare floor accepted it), but the deadlock-freedom *guarantee* still rests on a **whole-program**
   obligation — a globally-consistent acyclic assignment the type system checks but does not search, plus
   constructor-maintained edge consistency. Confirm or refute: is the guarantee genuinely per-call, or does it
   reduce to a whole-program assignment + a trusted constructor (a mini-projection)?
2. **SEAM-COSTS [horn 3] or SEAM-HOLDS?** Is the assignment obligation a real horn-3 cost (assignment search is
   the hard part for non-trivial protocols; Scribble exists to automate it), or is it "just part of writing the
   program" (HOLDS)? And: is the ring *really* untypable for all assignments, or can equal/!-typed priorities or
   a non-priority-typed sub-assembly evade it?
3. **Does this CONFIRM #25's weaker thesis and UNIFY progress's faces?** #25's surviving claim: global progress
   is not concentrable per-call; needs strictly more than the per-edge floor. Does rung (b) confirm it (local
   bounds + whole-program assignment)? Does the structure match `ordered_locks` #15/#16 (type-level rank carries
   local order; whole-program lock-acyclicity escapes — the *lock* face of the same shape) and reuse `typed_nat`
   #8? Or is the "unification" overclaimed?
4. **Does liveness stay the THIRD domain (N stays 9)?** No new domain, coordinate (progress, #24), or mode.
   Confirm N stays 9 and the FAILED broken-seam (soundness) tally stays 7 — or argue rung (b) is categorically
   new.
5. **Prior-art deflation (pre-registered).** Priority/obligation typing for deadlock-freedom is deep folklore:
   Kobayashi (linear-π levels/obligations); Padovani; **Dardha–Gay FoSSaCS 2018** (priorities for session
   types); Kokke–Dardha (Linear Haskell); Scribble/inference for assignment. Confirm the contribution is
   **locating** rung (b) on the lattice and showing Rust's const-bound floor carries the local discipline while
   the assignment is whole-program — its content is #25's correction made concrete, plus the #15/#16 unification.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does priority typing carry deadlock-freedom per-call or does it rest on a
whole-program acyclic assignment + a trusted constructor (confirming #25's weaker thesis); is the verdict
SEAM-COSTS [horn 3] (the assignment is the cost) or SEAM-HOLDS; is the ring genuinely untypable for all
assignments; does this unify progress's lock face (#15/#16) and session face (#24/#25) and reuse `typed_nat` #8;
does liveness stay the third domain with N = 9 and the FAILED tally at 7; and does priority typing deflate onto
known folklore (Kobayashi; Dardha–Gay 2018; Scribble), confirming the contribution is locating rung (b)?**
