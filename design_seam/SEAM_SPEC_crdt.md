# Seam spec — state-based CRDT merge (re-import rule under attack)

This is a **design proposal** for a twenty-first *seam*, the **first probe aimed at a fourth soundness
domain**. The arc has mapped three — **memory safety** (full floor-domain match: many seams close),
**information flow** (mismatch: no seam closes, `SEAM_SPEC_secret.md`/`labeled`), and **liveness/progress**
(partial/graded: `ordered_locks`/`lock_registry`). This seam tests whether the floor-domain-match thesis is
genuinely domain-general, on **distributed/replicated convergence** (eventual consistency).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/crdt.rs`, the
default `no_std` build — `core`-only, **no `unsafe`**, like the constant-time `secret` seam).

---

## 0. The floor (stipulated, not under attack)

Rust's type system as shipped: generics + trait resolution (a `Replica<T>::merge` requires `T:
JoinSemilattice`; a `Replica<GCounter<3>>` is a different type from `Replica<GCounter<4>>`), ownership, and
ordinary method dispatch. There is **no `unsafe`** and the abstract machine models no notion of "replica" or
"convergence" — so the project's usual "can safe code reach UB?" is *trivially* no. Do not attack the borrow
checker or trait resolution. Attack the **re-import rule** (the convergence claim) and the §6 deliverable.

## 1. The primitive

A **state-based CRDT** (convergent replicated data type): a `Replica<T>` holds a local `T` and merges remote
replica state via `Replica::merge`, which delegates to `T::join`. The danger is a **consistency** hazard, not
UB: two replicas that have observed the **same set of updates** end in **different states** — *divergence*.
(The blessed `GCounter<N>` — a grow-only counter, `join` = element-wise `max` — is a *correct* CRDT included
as the sound baseline; the object of study is the general `merge` re-import, not this one counter.)

## 2. The true precondition (cathedral side) — the join-semilattice laws

`Replica`s **converge** iff `T::join` is the least-upper-bound of a join-semilattice:
- **Commutative:** `a.join(b)` and `b.join(a)` reach the same state (order-independence).
- **Associative:** grouping of merges is irrelevant.
- **Idempotent:** `a.join(a) == a` — re-delivering the same state is a no-op (safe at-least-once delivery).

These three laws are the **convergence precondition**. They are *semantic* properties of `join`, not
properties of its type.

## 3. The witness — a `JoinSemilattice` trait bound

`Replica<T>::merge` is gated on `T: JoinSemilattice`, and `merge` only accepts another `Replica<T>` (same
`T`). So the floor carries the **type** of a merge: you cannot merge incompatible replica types, and you
cannot merge a `T` that has no `join` at all. The blessed `GCounter::join` (element-wise `max`) satisfies all
three laws. The **open question** is whether the trait bound is a *witness* for convergence or merely a
*syntactic proxy* for it.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Replica<T>` vends an **eventually-consistent replicated value**: safe code calls `merge`
> with no `unsafe` and no hand-written reconciliation, and **all replicas that have merged the same updates
> converge to the same state**, because `merge` applies the semilattice join.

Structural stoppers (claimed): the `T: JoinSemilattice` bound (merge needs a join); same-`T` typing (no
cross-type merge); the blessed `GCounter` as an existence proof that a correct join exists.

## 5. Threat model

In scope: any **safe** path that makes two replicas **diverge** after observing the same updates — most
directly, a safe `impl JoinSemilattice` whose `join` violates a law (overwrite = non-commutative; accumulate
= non-idempotent), which type-checks and diverges replicas. Out of scope: the AM's lack of a time/replica
model (stipulated §0); the *correctness of the chosen `GCounter`* byte-moving; Byzantine replicas, network
partition recovery, and causal-delivery metadata (a state-based CRDT assumes none) — the object is the
**convergence of the merge**, not a full distributed-systems model.

## 6. The scoring rubric — soundness, then the FOURTH-DOMAIN deliverable

First, soundness of the re-import:

- **Horn 1 — sound re-import.** Exhibit safe code that diverges two replicas which saw the same updates — the
  spec's witness: a safe `impl JoinSemilattice { fn join(&mut self, o) { self.0 += o.0 } }` double-counts on
  re-delivery, so a replica that processes one update twice disagrees with one that processed it once. Confirm
  this is reachable in safe code and is a real consistency violation — or argue the trait bound forecloses it.
- **Horn 2 — decidable & proof-free.** `new`/`merge`/`get` are ordinary safe calls. Confirm — *but note
  whether Horn 2 passes only because the strong property (convergence) is never actually checked* (the
  `secret`-seam pattern: a decidable seam that type-checks precisely because it does not discharge the
  semantic property).
- **Horn 3 — concentration.** The merge is concentrated in `merge`; the consumer writes no reconciliation
  logic. Confirm — or note the caller still carries the undischargeable obligation that *their* `join` obeys
  the laws.

Then **the fourth-domain deliverable** — the point of the phase. Rule on each:

1. **Floor-mismatch, like information flow?** Does the affine/ownership floor **fail to carry** the
   convergence precondition — the semilattice laws being semantic, not type-level — so that the convergence
   domain has **no closing seam** (the IFC result, in a new domain)? The honest claim: **SEAM-BREAKS
   [floor-mismatch]**. Confirm, or find a way the floor *does* carry the laws.
2. **Or TCB-axiom-costed, like MMIO?** If the re-import is restricted to a **blessed library of verified
   CRDTs** (the foundation author provides `GCounter`/`OR-Set`, safe code only *uses* them), the laws become a
   **trusted axiom** discharged once — and the seam **holds with a cost** (like `mmio`'s hardware axiom or
   `secret`'s constant-time axiom). Rule which re-import boundary the seam actually offers: safe code *supplies*
   the `join` (→ mismatch/break) or only *uses* blessed ones (→ TCB-axiom cost). Is the laws-trust **MMIO-
   irreducible** or **dischargeable** (mechanized-CRDT-verification — e.g. verified CRDTs in Coq/Isabelle, the
   `secret`-seam correction)?
3. **A genuine fourth domain, or a deflation?** Is **convergence** a fourth soundness domain with its own
   basis (the semilattice laws / monotonicity — the CALM theorem; Shapiro et al. 2011) — parallel to how
   information-flow was established as a third — or does it **deflate** to the *application-correctness
   invariant* the nineteenth seam (`restore`) already located (a logical property the type system never
   promised), i.e. **projects onto an existing finding** rather than opening a new domain?
4. **Verdict mode.** Is this the **floor-mismatch** mode (no new mode — convergence is a new domain reached by
   the existing IFC-style verdict), or **projects-onto-basis** (deflation to application-correctness)? Either
   way, does N stay 9? Prior art to weigh: CRDTs (Shapiro–Preguiça–Baquero–Zawirski 2011), the CALM theorem
   (Hellerstein–Alvaro), lattice-based languages (Bloom^L / LVars / Lasp), and verified-CRDT work.

State your verdict as **SEAM-HOLDS** / **SEAM-COSTS** (which horn / TCB axiom) / **SEAM-BREAKS** (give the safe
divergence sequence). And state, explicitly: **does the affine/ownership floor carry the convergence
precondition (the semilattice laws), or is the convergence domain floor-mismatched with no closing seam (the
IFC result in a fourth domain); does restricting to blessed verified CRDTs convert the break into a TCB-axiom
cost (MMIO-shaped), and is that trust irreducible or dischargeable; is convergence a genuine fourth domain or
a deflation to the application-correctness invariant `restore` located; and does N stay 9?**
