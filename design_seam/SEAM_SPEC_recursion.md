# Seam spec — recursive session (`Rec`/`Var`: can the floor force a *loop* to ever leave?) (re-import rule under attack)

This is a **design proposal** for a thirty-first *seam* — the last unbuilt **structural** session-type feature,
**recursion** (`μX.T`), the looping connective orthogonal to sequencing (#18/#23), choice (#28), and completion
(#29/#30). The session arc is otherwise closed. This seam asks what an **unbounded-depth** protocol does to the
findings the arc settled at finite depth — especially **#30's completion-forcing**, which mints a `Done` only at
`End`. It is **NOT a fourth-domain bid**; it stays inside the **third confirmed domain (liveness)**, and it
deliberately tests the **boundary** of #30 rather than opening new ground.

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/recursion.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, self-contained over the #23/#29/#30 two-endpoint model). Tests
pass (default), including two `compile_fail` doctests (CF1 a top-level `Var` has no `recur`; CF2 a `Rec` head must
be `enter`-ed before its body is walked).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing + private fields; trait resolution (`HasDual`); higher-kinded
*marker* recursion via type constructors (`Rec<P>`, `Var`); default type parameters (the recursion environment
`E = NoRec`). `Rc`/`RefCell`/`VecDeque` with their runtime checks. The transport's byte/queue moving is the trusted
core (out of scope, like #23/#29/#30's). Do not attack `Rc`/`RefCell`/`VecDeque` or the transport. Attack the
**recursive re-import rule** and the §6 deliverable: *is the `Var`/environment encoding sound, and is conceding
termination/productivity as undecidable NONEs the right ruling?*

## 1. The primitive

**Single-level recursive sessions.** `Rec<Cont>` binds a loop point with body `Cont`; `Var` is the de-Bruijn-0 loop
variable. Every endpoint `Session<P, E>` threads a **type-level recursion environment** `E` (default `NoRec`):
[`enter`] on `Rec<Cont>` unfolds once — continue as `Cont`, set `E := Rec<Cont>`; [`recur`] exists **only** for
`Session<Var, Rec<Cont>>` — re-unfold, continue as `Cont`. `send`/`recv` walk the body and thread `E` unchanged;
[`finish`] exits at `End`. [`start`] vends two dual endpoints (as #23). `HasDual`: `Rec<P> ↔ Rec<P::Dual>`,
`Var ↔ Var`, `Tx ↔ Rx`, `End` self-dual — so `Rec<Tx<Var>>` (infinite producer) ↔ `Rec<Rx<Var>>` (infinite
consumer). No `Drop` bomb (this is the bare recursion primitive; the completion findings were #29/#30's job).

## 2. The true precondition (cathedral side)

- **Order (incl. unfolding):** ops occur only at their matching state; `enter`/`recur` advance the typestate
  soundly; a `Var` is only re-unfoldable under a real enclosing `Rec` (no jump to a non-existent binder).
- **Termination (the thing in question, part 1):** a recursive session that *should* terminate eventually reaches
  `End` (the peer is not looped forever).
- **Productivity / guardedness (part 2):** every `Var` sits under at least one communication action — no
  unguarded `Rec<Var>` spin loop.

## 3. The witness — environment-threaded unfolding; `Var` trapped under its `Rec`

The affine typestate carries **order** for free across unfolding (an out-of-state op is a type error; `enter`/`recur`
consume the handle). The **soundness of `Var`** is carried by the environment discipline: `recur` is impl'd only for
`Session<Var, Rec<Cont>>`, so a top-level `Var` (env `NoRec`) is inert — there is no way to unfold a loop variable
that has no binder. This is the recursion analogue of branded #5: `Var` is trapped under its `Rec`. **Termination
and productivity have NO witness on this floor** — and (the claim) none on any decidable floor.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `start`-ed recursive session vends `enter`/`recur`/`send`/`recv`/`finish`. Safe code composes
> them with no `unsafe`. The type system guarantees, **exactly**:
> 1. **Order through unfolding** — an out-of-state op is a type error; `enter`/`recur` thread the protocol soundly.
> 2. **`Var` soundness** — a `Var` is re-unfoldable only under a real enclosing `Rec` (the environment records it);
>    a top-level `Var` has no `recur`. No ill-formed unfolding, no UB.
> 3. **Duality through recursion** — `Rec<P> ↔ Rec<P::Dual>`, so a recursive protocol's peer is its recursive dual.
>
> It does **NOT** guarantee (and this spec **concedes** these as clean undecidable NONEs, not floor mismatches):
> 4. **Termination** — a recursive protocol need not reach `End`; without a choice connective inside the loop
>    (`Rec<Offer<End, …Var>>`, recursion ⊗ #28) it **cannot** — `Rec<Tx<Var>>` has no terminal. Forcing a loop to
>    exit is the **halting problem**.
> 5. **Productivity / guardedness** — `Rec<Var>` (unguarded) type-checks but is a no-op spin loop; the floor cannot
>    require a communication action between binder and variable.

## 5. Threat model

In scope: any **safe** path that (a) performs an op out of state (incl. at a `Rec`/`Var` head) and type-checks; (b)
obtains a `Session<Var, _>` that re-unfolds to a **wrong or absent** `Rec` body (an ill-formed unfolding — the
soundness crux of `Var`); (c) forges a `Session` at a wrong state/environment via the public surface; (d) argues
that conceding termination/productivity as NONEs is **wrong** — that the floor in fact carries or *breaks* one of
them, or that the encoding admits a UB (not merely a starved peer). Out of scope: `Rc`/`RefCell`/`VecDeque`
internals; the trusted transport; async/threaded scheduling (single-threaded witness, as #23–#30); **nested**
recursion (multi-level de Bruijn — the environment is single-slot; out of scope, a minimal-core limitation); the
choice connective itself (#28 — its composition with recursion is discussed, not built here).

## 6. The scoring rubric — soundness, then the recursion deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the worst case is a **starved/abandoned** (memory-safe) session — a
  non-terminating or dropped loop — not UB. Confirm. (A break here would be an **unsound `Var`** — an ill-formed
  unfolding reachable in safe code — a genuine type-safety hole, not a mere liveness matter.)
- **Horn 2 — decidable & proof-free.** The typestate is structural — Θ(type-size); the *type* is finite even though
  runtime depth is unbounded; the environment threading is compile-time, no SMT. Confirm.
- **Horn 3 — concentration.** Order + duality concentrate in the typestate, per unfolding. The **termination** and
  **productivity** obligations are the question — are they (as claimed) clean undecidable NONEs that no floor
  carries, or does the floor carry/break one?

Then **the recursion deliverable** — the point of the phase. Rule on each:

1. **Is the `Var`/environment encoding SOUND?** The claim: yes — `recur` only under a real `Rec` env; a top-level
   `Var` is inert; no ill-formed unfolding, no UB. Confirm — or exhibit a **safe** path to a `Var` that unfolds to a
   wrong/absent `Rec` body (a type-safety break, tally 8 → 9).
2. **Is conceding TERMINATION a clean NONE the right ruling?** The claim: yes — a recursive session need not reach
   `End`, and without choice (#28) cannot; forcing a loop to exit is the halting problem, undecidable, so no floor
   carries it — a clean NONE, not a liveness-floor *mismatch* (distinct from the #15/#16/#24/#25/#26 progress
   breaks, which were floor-carries-a-sub-fragment, not undecidability). Confirm — or argue the floor *could* carry
   a useful sub-fragment (e.g. structural/bounded recursion), so it is a cost, not a clean NONE.
3. **Does recursion BOUND #30's completion-forcing (does the finding earn its keep)?** The claim: yes — #30 forces a
   `Done` mintable only at `End`; a protocol with no reachable `End` (`Rec<Tx<Var>>`) can never mint one, so #30's
   forcing is **vacuous** here, and its "modulo divergence" caveat becomes the whole protocol — recursion reveals
   #30's unstated precondition (a reachable terminal). Confirm — or argue this is **trivial** (of course an infinite
   protocol doesn't terminate), adding nothing to #30.
4. **Prior-art deflation (pre-registered).** `Rec`/`Var` session encodings (`session_types`/`dejavu`;
   Honda–Yoshida–Carbone `μ`-session-types; Lindley–Morris GV recursion); the **halting problem** / Rice for
   termination; **guarded corecursion** (Coq/Agda productivity) for `Rec<Var>`. Confirm the contribution is
   **locating** recursion as the *boundary of #30's forcing* and the completion story's further decomposition
   (per-unfolding order closes / whole-loop termination is an undecidable NONE) — **not** a new mechanism. Or rule
   it a **redundant re-run** (recursion = the #17 completion residue spread over iterations + the halting problem,
   adding nothing).

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **is the `Var`/environment encoding sound (no safe ill-formed unfolding) or does
it break Horn 1; is conceding termination + productivity as undecidable NONEs correct or does the floor carry/break
one; does recursion bound #30's completion-forcing (a reachable `End` is #30's unstated precondition) or is that
trivial; and does it deflate onto the standard `Rec`/`Var` encoding + halting + guardedness folklore as a *locating*
contribution (per-unfolding order closes / whole-loop termination is an undecidable NONE) — or is it a redundant
re-run of #17's residue, with no new domain, coordinate, or mode (N = 9; the FAILED broken-seam tally stays 8 unless
the `Var` encoding is unsound, → 9)?**
