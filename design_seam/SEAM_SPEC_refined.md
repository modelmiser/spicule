# Seam spec — refinement-typed bounds (re-import rule under attack)

This is a **design proposal** for a sixth *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, and branded-index seams. A seam
re-exports a dangerous primitive as a checked safe interface ordinary code uses with no `unsafe` and
no proofs. Attack the re-export: can ordinary safe code, using only this interface, reach undefined
behavior (UB)? **And** adjudicate the axis question this primitive was built to settle (§6).

You are attacking a **text specification**, not source code.

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, trait resolution). **Crucially, the floor has no SMT solver and no refinement types.**
This proposal therefore describes a primitive as it would exist on a *refinement-typed* floor (Flux,
Liquid Haskell, F\*, Dafny), and is explicit about which parts the bare NLL floor can only *model* at
run time (§3). Do not attack the borrow checker; attack the re-import rule and the axis claims.

## 1. The primitive

**Unchecked array indexing** — `slice.get_unchecked(i)`, no bounds check; out of bounds is instant
UB. Same dangerous primitive as the branded-index seam, deliberately: using the *same* danger with a
*different witness* is what lets the two be compared on equal terms.

The witness here is a **refinement type**: a decidable logical predicate attached to an ordinary
value. `i: usize{i < len}` is "a `usize` a checker has *proven* less than `len`." The proof is
discharged by an SMT solver over a decidable arithmetic fragment (QF-LIA / UTVPI) at compile time, so
a real refinement floor erases the bounds check (zero runtime cost) **with no brand and no scope
closure** — the index is a plain value.

## 2. The true precondition (cathedral side)

`get_unchecked(i)` on `slice` is memory-safe iff `i < slice.len()` at the access. The refinement
carries this as a first-class, *legible* fact: the type literally *is* `{i < len}`. Two conjuncts:

- **R1 (predicate holds).** The value satisfies `value < bound` — established once, by the checker
  (solver), when the refined value is introduced.
- **R2 (bound is this slice's length).** The `bound` the predicate was proven against is the length
  of the slice being indexed. On a refinement floor this is *definitional* — the index's type and the
  slice's type share the same length variable `n`. (The bare-NLL model must check it; see §3.)

## 3. The token — a refined value, and the honest floor gap

The token is a **`Refined`**: a `usize` whose contract is the visible predicate `value < bound`,
introduced only by `refine(value, bound) -> Option<Refined>` (which discharges R1) and consumed by
`at(slice, refined) -> &T` (which binds R2 and does the unchecked read). It is an **ordinary value**:
storable, returnable, nestable in other structures. There is **no brand and no closure.**

**The disclosed floor gap.** The stipulated floor has no solver, so the reference model
(`seam/src/refined.rs`) discharges R1 with a *runtime* `value < bound` check inside `refine`, and R2
with a *runtime* `bound == slice.len()` compare inside `at` — both standing in for what a refinement
floor proves statically. Consequences, stated rather than hidden:

- The witness **form** is faithful and real: `Refined` is genuinely brand-free (it escapes its
  constructor — a *compile error* for the branded index's `Idx<'id>`) and genuinely legible (the
  predicate is its contract).
- The static **erasure** (#6 zero-cost) is *not* reproduced by the bare-NLL model (it pays a residual
  `bound == len` compare per access). A real refinement floor erases it — and instead pays the
  **external-solver / one-toolchain** cost: an SMT solver in the trusted compilation loop.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `refine`/`at` interface vends `Refined` values and a safe unchecked-index
> operator. The safe fragment may type `at(slice, r)` as an ordinary safe index — no `unsafe`, no
> hand proof — because the type system (solver) has established R1 (`value < bound`) at introduction
> and R2 (`bound == len`) binds it to this slice, so `value < len` holds and the bounds check is
> elided. `refine` returning `Option` is where R1 is discharged.

Structural stoppers: `Refined` fields are private (no forging a predicate that does not hold); `at`
binds R2 (a foreign-length slice is a caught logic error — panic — never UB).

**Honest residual caveat (memory-safe, not UB).** Because `Refined` binds a *length*, not a slice
*identity*, a `Refined{2,5}` is valid against *any* length-5 slice — memory-safe, but it may
*logically* index the wrong same-length slice. This is the exact analog of the slab's cross-arena
caveat, and it is the price of brand-freedom: the branded index's `'id` *prevented* this by binding
identity, at the cost of virality. Closing it would require re-introducing a brand. The proposal
accepts the caveat and claims only **memory safety**.

## 5. Threat model

In scope: any AM-level UB (out-of-bounds `get_unchecked`) reachable by **safe** consumer code using
only §3. Consider: forging a `Refined` with a false predicate (public fields?); using a `Refined` on
a slice of different length (R2); arithmetic on a `Refined`'s value that escapes the predicate
(`value + 1`); a `Refined` proven against a `bound` larger than any real slice; integer overflow in
`refine`. Out of scope: bugs inside the foundation's single `unsafe` (`get_unchecked`); the soundness
of a real SMT solver (that is the refinement floor's TCB, analogous to MMIO's axiom — flag it if you
think it is load-bearing here). This minimal core refines single indices, not ranges or arithmetic.

## 6. The scoring rubric — soundness, then the axis adjudication this primitive was built for

First, soundness (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reaches an out-of-bounds
  `get_unchecked` despite R1∧R2 — a forged predicate, an R2 bypass, value-arithmetic that escapes the
  refinement, an overflow. Give the exact sequence.
- **Horn 2 — decidable & proof-free.** Here is the subtle one. A refinement type *is* decidable
  (QF-LIA terminates) and the consumer writes *no* hand proof. But it requires an **external SMT
  solver in the trusted toolchain**. Is that still "decidable & proof-free", or does the solver
  dependency mean horn 2 was itself a *bundle* of (a) **decidability** (a terminating checker exists)
  and (b) **solver-freedom / one-toolchain** (no external prover in the trusted loop) — with the
  refinement type acing (a) and paying (b)? If so, this primitive **un-bundles horn 2** the way the
  branded index un-bundled horn 3. Rule on it.
- **Horn 3 — concentration.** `refine`/`at` are O(1) safe calls per index; seam width O(1). Confirm.

Then **the axis adjudication this primitive exists to settle** — be precise, this is the deliverable:

1. **Where does this primitive sit on (#5 API-locality, #7 AI-legibility)?** The claim:
   **brand-free** (#5 ✓ — a `Refined` escapes and is storable, no viral `'id`, no closure) and
   **legible** (#7 ✓ — the predicate `value < bound` is the type's contract; an outside reasoner
   reconstructs the safety invariant *from the signature alone*, with no access to the foundation
   `unsafe`). Agree or dispute each, using that operational test for legibility.

2. **Does this CLOSE the #5⟂#7 independence cell?** The branded-index phase established #5 and #7 are
   *distinct* but left them **co-varying** in every built primitive (branded index = viral ∧
   illegible; everything else = brand-free ∧ legible), which is consistent with their being the
   *same* axis. Independence requires an **off-diagonal**: two primitives that **agree on one axis
   and differ on the other**. The proposal's argument:
   - This refined index is **(brand-free, legible)**.
   - The **slab** should be reclassified **(brand-free, *illegible*)** — its `Copy` `Key`'s validity
     (live slot at matching generation) is a hidden runtime fact *not reconstructible from the `Key`
     type or the `get -> Option` signature*; the earlier table's "legible ✓" was too generous.
   - Then: refined vs slab **holds #5 fixed (both brand-free) and flips #7** ⇒ #7 ⟂ #5. And branded
     vs slab holds #7 fixed (both illegible) and flips #5 ⇒ #5 ⟂ #7 the other way.

   Is this reclassification of the slab honest, and does the pair genuinely close the cell? Or is the
   slab actually legible (in which case the cell is *not* closed and an off-diagonal is still
   missing)? This is the load-bearing judgment.

3. **Any new axis surfaced?** If horn 2 un-bundles (solver-freedom), note that the projection grows
   again (N ≥ 7). Be precise about whether "needs Z3" is a genuine independent axis or just the
   "keep" horn of the original impossibility result reappearing.

State your verdict as **SEAM-HOLDS** (sound; all known horns kept; confirm the corner), **SEAM-COSTS**
(sound but names a horn or recovered axis paid — say which), or **SEAM-BREAKS** (AM-level UB; give the
sequence). And state, separately and explicitly: **does the #5⟂#7 cell close? (yes / no / needs more)**
