# Seam spec — guarded recursion (concentrating the cost I over-conceded in #31) (re-import rule under attack)

This is a **design proposal** for a thirty-second *seam*, the **direct salvage of the thirty-first** (`recursion`),
mirroring the #26→#27 and #29→#30 reject→salvage rhythm. #31 was SOUND but its §6 deliverable was **REJECTED**
(r38, the rejection mode's 7th firing): I **over-conceded** — I filed **productivity/guardedness** as an
*undecidable NONE* alongside termination. The panel's correction: `Rec<Var>` = `µX.X` is the **non-contractive**
type every cited system (Coq/Agda guarded corecursion; contractive recursive types) rejects **syntactically**; a
sealed `Guarded` marker carries it in `Θ(type-size)`, no SMT — the same machinery as order. This seam **builds that
marker** and asks the panel to confirm guardedness genuinely concentrates, leaving **termination** as the *only*
conceded NONE. It is **NOT a fourth-domain bid**; it stays inside the **third confirmed domain (liveness)**, on the
**progress** coordinate (its productivity face).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/guarded.rs`, default
`no_std` build — `core` + `alloc`, **no `unsafe`**, self-contained over the #31 single-level recursion model). Tests
pass (default), including a `compile_fail` doctest (CF1: the unguarded `Rec<Var>` cannot be unfolded).

---

## 0. The floor (stipulated, not under attack)

Rust as shipped: affine ownership; nominal typing; **sealed traits** (a private supertrait, no downstream impls);
trait resolution (`HasDual`, `Guarded`); marker recursion (`Rec<Cont>`, `Var`); default type parameters (`E = NoRec`).
`Rc`/`RefCell`/`VecDeque` with their runtime checks. The transport is the trusted core (out of scope, as #31). Do not
attack `Rc`/`RefCell`/`VecDeque` or the transport, or the soundness of the #31 `Var`/environment encoding (already
adjudicated sound at r38). Attack the **`Guarded` re-import rule** and the §6 deliverable: *does the sealed marker
genuinely concentrate guardedness while admitting every productive loop, and is "productivity concentrates,
termination stays a NONE" the right final split?*

## 1. The primitive

**Guarded single-level recursive sessions.** Identical `Rec<Cont>`/`Var`/`Session<P, E>` machinery as #31
(`enter`/`recur`/`send`/`recv`/`finish`, `HasDual` threading recursion). The one addition is the salvage: a
**sealed [`Guarded`] marker** implemented for `Tx<C>`/`Rx<C>`/`End` and **not** for `Var`, with [`enter`] now
requiring the recursion **body** `Cont: Guarded`. So an unguarded `Rec<Var>` = `µX.X` has no `enter` — the
non-contractive recursion is rejected at compile time, `Θ(type-size)`, no SMT.

## 2. The true precondition (cathedral side)

- **Order + `Var` soundness + duality:** as #31 (already ruled sound — not re-litigated here).
- **Productivity / guardedness (the thing this seam carries):** every recursion `Var` sits under at least one
  communication action — no silent `µX.X` spin loop. `Rec<Var>` is ill-formed; `Rec<Tx<Var>>` is well-formed.
- **Termination (still NOT carried, conceded NONE):** a guarded loop may still run forever (`Rec<Tx<Var>>` is
  guarded and infinite). Forcing a loop to reach `End` is the halting problem.

## 3. The witness — a sealed `Guarded` marker, shallow head-check = full contractivity

`Guarded` is a sealed trait impl'd for `Tx<C>`/`Rx<C>` (an action head guards everything inside, incl. a bare `Var`)
and `End` (no `Var`, vacuously guarded), and **not** for `Var` (a bare loop variable is unguarded) or `Rec<_>`
(nested recursion, out of scope for the single-level core — conservatively rejected). `enter` (the only operation on
a `Rec` head) carries `Cont: Guarded`. **Why the shallow head-check is full contractivity:** for single-level
recursion, if the body's head is an action then *every* `Var` inside it is under that action ⇒ guarded; if `End`, no
`Var` ⇒ vacuously guarded; only a bare-`Var` head is unguarded. So one constructor test per `Rec` decides
guardedness — no recursive trait walk, the same trait-resolution machinery that carries order.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `start`-ed guarded recursive session vends `enter`/`recur`/`send`/`recv`/`finish`. Safe code
> composes them with no `unsafe`. The type system guarantees, **exactly** (additionally to #31's order/`Var`-
> soundness/duality, which are stipulated sound):
> 1. **Guardedness / productivity** — `enter` (and `recur`) require the body `Cont: Guarded`; `Var` is not
>    `Guarded`; so `Rec<Var>` = `µX.X` cannot be unfolded. Every admitted loop has a communication action before its
>    variable — no silent spin. Decidable in `Θ(type-size)`, no SMT, the same machinery as order.
>
> It does **NOT** guarantee (conceded NONE, deliberately — the one obligation #31 correctly isolated as
> undecidable): **termination** — a guarded loop may run forever; forcing it to reach `End` is the halting problem.

## 5. Threat model

In scope: any **safe** path that (a) **unfolds an unguarded recursion** — gets `enter`/`recur` on a `Rec<Var>` (or
any body whose head is a bare `Var`) to type-check, defeating the marker; (b) **forges `Guarded`** for `Var` (the
seal must forbid downstream impls); (c) exhibits a **productive loop the marker wrongly rejects** (a false
positive — a guarded body that fails `Guarded`), i.e. argues the shallow head-check is *not* full contractivity;
(d) argues the seam **now over-claims** — that "productivity concentrates, termination stays a NONE" is wrong, e.g.
guardedness is not actually the carriable line, or the marker secretly buys/misses something. Out of scope:
`Rc`/`RefCell`/`VecDeque`; the trusted transport; the #31 `Var`/environment soundness (ruled sound); **nested**
recursion (multi-level — the single-slot environment; conservatively rejected); termination itself (conceded NONE).

## 6. The scoring rubric — soundness, then the salvage deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** No `unsafe`; the `Guarded` marker only *removes* a degenerate program (`Rec<Var>`),
  never admits UB. The worst case remains a starved/non-terminating (memory-safe) session. Confirm.
- **Horn 2 — decidable & proof-free.** Guardedness is a single sealed-trait bound per `Rec`, `Θ(type-size)`, no SMT.
  Confirm.
- **Horn 3 — concentration.** Guardedness/productivity concentrates in the `Guarded` bound on `enter` — the cost
  #31 over-conceded as an undecidable NONE. Confirm it genuinely concentrates (this is the salvage's whole point).

Then **the salvage deliverable** — the point of the phase. Rule on each:

1. **Does the `Guarded` marker genuinely reject `Rec<Var>` while admitting every productive loop?** The claim: yes —
   `Var` is not `Guarded` so `Rec<Var>` has no `enter`; `Tx<…>`/`Rx<…>`/`End` are `Guarded` so `Rec<Tx<Var>>`,
   `Rec<Rx<Tx<Var>>>`, `Rec<End>`, etc. all unfold. Confirm — or exhibit a **safe** path that unfolds an unguarded
   recursion (marker defeated, Horn-1-style break of the *advertised* guardedness), or a **productive** loop the
   marker wrongly rejects (a false positive — the head-check is not full contractivity).
2. **Is the shallow head-check really full single-level contractivity?** The claim: yes — an action head guards all
   interior `Var`s; `End` is vacuous; only a bare-`Var` head is unguarded. Confirm — or find a single-level body the
   head-check mis-classifies.
3. **Is "productivity concentrates, termination stays a NONE" the right FINAL split — or do I now OVER-claim?** The
   claim: the marker buys productivity (no silent `µX.X` spin), **not** termination (`Rec<Tx<Var>>` is guarded and
   infinite). Confirm the split is honest — or argue I over-claim the marker's reach (it secretly fails to ensure
   productivity, or it implicitly buys/needs more), the inverse of #31's under-claim.
4. **Prior-art deflation (pre-registered).** Guarded corecursion (Coq's guardedness checker; Agda's productivity /
   sized types); contractive/equi-recursive types (Gay–Hole session types; `session_types`/`dejavu` well-formedness;
   Pierce TAPL contractiveness). Confirm the contribution is **building** the carriable half the #31 verdict
   identified — a standard contractivity check realized as a sealed Rust marker — **not** a new mechanism, and that
   it pointedly does **not** touch termination. Or rule it a **redundant re-statement** of #31 (the verdict already
   named the fix; building it adds nothing).

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
sequence). And state, explicitly: **does the sealed `Guarded` marker concentrate guardedness (reject `Rec<Var>`,
admit every productive loop) with no soundness regression and no false positive; is the shallow head-check full
single-level contractivity; is "productivity concentrates, termination stays the one undecidable NONE" the honest
final split or do I now over-claim the marker's reach; and does it deflate onto textbook guarded-corecursion /
contractivity folklore as the *built* carriable half of #31 (closing the over-concession) — with no new domain,
coordinate, or mode (N = 9; the FAILED broken-seam tally stays 8; this is the salvage that should HOLD)?**
