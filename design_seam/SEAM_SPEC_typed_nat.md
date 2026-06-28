# Seam spec — a type-level proof of in-bounds indexing (re-import rule under attack)

This is a **design proposal** for an eighth *seam* in the same hypothetical systems language (same
floor, same rules) as the SPSC, MMIO, intrusive-list, slab, branded-index, refined-index, and
shared-ownership seams. A seam re-exports a dangerous primitive as a checked safe interface ordinary
code uses with no `unsafe` and no proofs. Attack the re-export: can ordinary safe code, using only
this interface, reach undefined behavior (UB)? **And** adjudicate the axis question this primitive was
built to settle (§6).

You are attacking a **text specification** backed by real `rustc`-checked code (`seam/src/typed_nat.rs`).

---

## 0. The floor (stipulated, not under attack)

Rust's borrow checker exactly as shipped (NLL: moves, `&T`/`&mut T`, lifetimes, affinity, `Drop`,
`Send`/`Sync`, **trait resolution**, `#![recursion_limit]`). The floor has trait resolution but **no
SMT solver, no refinement types, and no dependent const-generic naturals**. This proposal uses the
*trait resolver itself* as the proof engine — which is exactly why it pays where it pays. Do not
attack the borrow checker; attack the re-import rule and the axis claims.

## 1. The primitive

**Unchecked array indexing** — `slice.get_unchecked(i)`, no bounds check; out of bounds is instant
UB. The *same* dangerous primitive as the branded-index and refined-index seams, deliberately: same
danger, a *third* witness, so the three can be compared on equal terms.

The witness here is a **type-level proof**. Index `I` and length `Len` are type-level natural numbers
(`Z`, `S<Z>`, `S<S<Z>>`, …). The in-bounds fact `I < Len` is the trait bound `I: Lt<Len>`, discharged
by the compiler's trait resolver via two recursive impls:

```
impl<M>    Lt<S<M>> for Z    {}                      // 0 < n+1
impl<I, M> Lt<S<M>> for S<I> where I: Lt<M> {}       // i+1 < n+1  ⟺  i < n   (nothing is < 0)
```

The proof carries **no runtime data** (the index types are zero-sized; `Bound` is exactly a `&[T]` in
size) and needs **no external solver** (it is native trait resolution). It **erases**: a proven access
compiles to a bare `get_unchecked`.

## 2. The true precondition (cathedral side)

`get_unchecked(i)` on `slice` is memory-safe iff `i < slice.len()` at the access. Carried as two
conjuncts:

- **T1 (proof holds).** `I: Lt<Len>` is derivable — i.e. `I::VALUE < Len::VALUE`. The two `Lt` impls
  encode `<` exactly, so derivability ⟺ the numeric fact.
- **T2 (length binding).** The type-level `Len` equals the indexed slice's runtime length. On a
  dependently-typed floor this is definitional (`Len` *is* the slice's length index); the bare-NLL
  model checks it once, in `Bound::new` (see §3).

## 3. The token — a bound buffer, and the honest floor gap

The token is a **`Bound<'a, T, Len>`**: a `&'a [T]` whose length has been bound to the type-level
numeral `Len`. Introduced only by `Bound::new(data) -> Option<Self>` (which discharges T2 by checking
`data.len() == Len::VALUE`) and consumed by `get::<I, …>(&Bound) -> &T` (which requires `I: Nat +
Lt<Len>` — the T1 proof — and does the unchecked read). `Bound` is an ordinary value; there is **no
brand and no closure**, and `I` is a turbofish type argument, not a value.

**The disclosed floor gap.** The bare-NLL floor cannot relate a runtime length to a type-level numeral
in types, so `Bound::new` performs *one* runtime `data.len() == Len::VALUE` compare, standing in for
what a dependently-typed floor proves statically. Consequences, stated rather than hidden:

- The witness **form** is faithful and real: the per-access proof `I: Lt<Len>` is genuinely a
  compile-time trait obligation that **erases** (no runtime residue, no solver). T1 is demonstrated,
  not modelled.
- Only **T2's binding** is modelled at run time (one compare per buffer, like the refined index's
  `bound <= len`). Per-access indexing carries **no** check.

## 4. The re-import rule (THE THING UNDER ATTACK)

> **Re-import.** A `Bound`/`get` interface vends length-bound buffers and a safe unchecked-index
> operator. The safe fragment may type `get::<I>(&b)` as an ordinary safe index — no `unsafe`, no hand
> proof — because the type system has established T1 (`I: Lt<Len>`, by trait resolution) and T2
> (`Len::VALUE == len`, by `new`), so `I::VALUE < len` holds and the bounds check is elided.

Structural stoppers:
- **`Nat` is sealed** (`Nat: sealed::Sealed`, `Sealed` implemented only in-crate for `Z`/`S<N>`). A
  consumer can impl the public `Lt` trait for their *own local* type (orphan rule), but that type can
  never be `Nat`, so `get`'s `I: Nat + Lt<Len>` bound rejects it, and there is no other supplier of
  `I::VALUE`. Forging `impl Nat for Evil { const VALUE = 999; }` is a compile error (the sealed
  supertrait is unsatisfiable). *(Verified: E0277 `Evil: Sealed` not satisfied.)*
- The two `Lt` impls are the only ones reachable for `Nat` indices and encode `<` exactly, so no false
  `I: Lt<Len>` is derivable (`get::<N3>` on an `N2` buffer fails: E0277 `S<Z>: Lt<Z>`, i.e. `1 < 0`).

## 5. Threat model

In scope: any AM-level UB (out-of-bounds `get_unchecked`) reachable by **safe** consumer code using
only §3. Consider, specifically: (a) forging a `Nat` with a lying `VALUE` (sealed?); (b) impl-ing `Lt`
for a custom type to forge a proof (orphan rule + the `Nat` bound?); (c) a `Bound` built over one
slice then used to index a *different, shorter* slice (is there any path? `get` takes only `&Bound`,
which owns its `&[T]` — is there a swap/replace?); (d) `Len::VALUE` overflow for huge numerals; (e)
the length binding in `new` being bypassable; (f) a `Bound` whose `Len::VALUE` disagrees with the
`Bound`'s own stored slice after construction. Out of scope: bugs inside the foundation's single
`unsafe` (`get_unchecked`); the soundness of `rustc`'s trait resolver (the floor's TCB).

## 6. The scoring rubric — soundness, then the axis adjudication this primitive was built for

First, soundness (the real attack surface):

- **Horn 1 — sound re-import.** Exhibit safe consumer code that reaches an out-of-bounds
  `get_unchecked` despite T1∧T2 — a forged `Nat`/`Lt`, a slice-swap defeating T2, an overflowed
  `VALUE`, a length-binding bypass. Give the exact sequence.
- **Horn 2 — decidable & proof-free.** The subtle one, and the deliverable. This witness is
  **decidable** (Peano `Lt` is well-founded — resolution terminates for any concrete numerals) and the
  consumer writes **no** hand proof. It needs **no external solver** (unlike the refined index — this
  is native trait resolution). *But* resolving `I: Lt<Len>` costs `I::VALUE` resolution steps, and the
  resolver only halts because of an **arbitrary fuel cap**, `#![recursion_limit]`: a deep-but-in-bounds
  index fails to compile with E0275 "overflow evaluating the requirement" until you raise the knob
  (*verified at `recursion_limit = "4"`, index 6 into a length-7 buffer*). The access is in bounds; the
  checker merely ran out of fuel.
- **Horn 3 — concentration.** `Bound::new`/`get` are O(1) safe calls; seam width O(1)/buffer. Confirm.

Then **the axis adjudication this primitive exists to settle** — be precise, this is the deliverable:

1. **Is "checker-tractability" a real face of horn 2, distinct from BOTH decidability and
   solver-freedom?** The refined-index phase already un-bundled horn 2 into **decidability** ⊗
   **solver-freedom**. The claim now: there is a *third* face — **does the native checker halt in
   bounded resource without an arbitrary fuel knob?** — and these three are pairwise distinct, shown by
   an off-diagonal over three primitives:

   | Primitive | decidable | solver-free | checker-tractable |
   |---|:--:|:--:|:--:|
   | SPSC (`T: Send`) | ✓ | ✓ | ✓ (resolves in one step) |
   | refined (`{v < n}`) | ✓ | ✗ (needs Z3) | ✓ (QF-LIA efficiently decidable) |
   | **typed_nat (`I: Lt<Len>`)** | ✓ | ✓ | **✗ (depth = index; halts only under `recursion_limit`)** |

   typed_nat vs SPSC holds {decidable, solver-free, sound, zero-cost} fixed and flips **only**
   checker-tractability ⇒ it is independent of the rest. typed_nat vs refined shows checker-tractability
   ⟂ solver-freedom (one has each, lacks the other). Agree or dispute. Is this a genuine third face, or
   is it the *same* thing as decidability (i.e. "decidable" should always have meant "tractably
   decidable" and the refined/SPSC checkers just happen to be cheap)?

2. **Does it extend N, or merely sharpen horn 2 (like solver-freedom did)?** Solver-freedom was ruled
   **not** a new axis — it is the `unified` result's "keep the invariant faithfully → external prover"
   horn reappearing. Is checker-tractability the same kind of non-axis (a re-description of horn 2), or
   is it a genuinely separable dimension that takes the lattice to **N ≥ 8**? Be precise about the
   difference between "an answer exists" and "the checker affords the answer cheaply."

3. **Novelty — deflate it.** The honest meta-pattern of this project is that *every* recovered axis is
   an already-named PL concern (zero-cost abstraction, generativity, refinement-legibility, the
   `core`/`alloc` split). Name checker-tractability's prior art if it has any — e.g. type-checking
   complexity (ML let-polymorphism is DEXPTIME-complete, Mairson 1990), Rust trait resolution being
   Turing-complete (hence `recursion_limit`), Haskell's `UndecidableInstances`, typenum. Rule whether
   this is **real but pre-existing** (the expected outcome) or, implausibly, novel.

State your verdict as **SEAM-HOLDS** (sound; all known horns kept; confirm the corner), **SEAM-COSTS**
(sound but names a horn or recovered axis paid — say which), or **SEAM-BREAKS** (AM-level UB; give the
sequence). And state, separately and explicitly: **is checker-tractability a real eighth axis,
N ≥ 8? (yes / no — sharpens horn 2 only / needs more)** — and if yes, **what is its prior art?**
