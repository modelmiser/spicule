# Mechanized core of the Seam Trilemma

`SeamGap.lean` — a core-Lean-4 (no Mathlib) machine-checked proof of the load-bearing
facet of the Seam Trilemma (`../design_seamful2/THEOREM.md`). Check it:

```
lean proof/SeamGap.lean      # exit 0, no output = all proofs accepted
```

## What is proved

The adversarial process named this as the trilemma's crux:

> *Sound re-import needs a linearity witness NLL's vocabulary cannot carry.*

(NLL is region **+ affine**, not region-only; the proof's force comes entirely from
`Copy` *disabling affine tracking*, so the affine machinery degenerates and only the
region bit remains observable. See "faithfulness" below.)

Formalized as a **measurability gap**:

- `P_not_V_measurable` — **no function of the NLL/affine witness `V` equals the
  soundness predicate `P`.** Proved by an explicit *indistinguishability pair*
  (`safe`, `ub`) that `V` maps identically but `P` separates. **Depends on no axioms.**
- `no_decision_has_all_three` — **no re-import decision can be simultaneously `Sound`,
  `Usable`, and `Concentrated`.** The trilemma's "pick at most two," as a theorem.
  Depends only on `propext`.

## Why the model is faithful (not rigged)

The one modeling claim that carries weight is: *`V` is blind to the aliasing fact
`aliased`.* This is **forced**, not chosen for convenience:

- The primitive is an interior-mutable cell whose handle is `Copy` (freely
  duplicable). NLL's vocabulary is regions/lifetimes + affinity.
- A region/affine system **cannot** record exclusivity of a duplicable value: you can
  copy the handle without it noticing. So `V`, which sees the NLL verdict, genuinely
  cannot observe whether a second live copy exists.
- This is *why real Rust cannot put interior mutability in the safe checker* — it needs
  `RefCell` (a **runtime** borrow flag — a runtime witness, conceding zero-cost) or
  `UnsafeCell` (drop to the **unsafe seam**). The model reproduces the actual reason the
  escape hatch exists.

## The horns (mapping to the trilemma)

| Lean predicate | Trilemma horn |
|---|---|
| `Sound d` | (1) sound re-import — no UB launders into safe code |
| `Concentrated d` (d factors through `V`) | (2)+(3) decidable/proof-free **and** concentrated — ordinary code decides from the safe witness alone, no use-site proof, no seam crossing |
| `Usable d` | the seam actually vends the primitive (accepts its sound uses) |

`no_decision_has_all_three` ⇒ to keep `Sound`+`Concentrated` you must drop `Usable`
(the seam can't vend the primitive from `V` alone — the verified-Rust reality, where the
obligation surfaces at use sites). The model collapses horns (2) and (3) into one
predicate (`Concentrated`) because in this minimal setting "decide cheaply" and "don't
cross the seam" are the same thing: *decide from `V` alone*. The full trilemma separates
them because, in the richer setting, you may cross the seam to obtain a proof (keep 2,
drop 3) — a distinction this model does not represent.

## What is NOT proved (honest boundary — scope AND depth)

**Depth (the most important caveat, per the adversarial review in `REVIEW.md`):** the
*mechanized lemma is information-theoretically trivial.* Stripped of names,
`P_not_V_measurable` says "no function of one field equals a predicate over two fields" —
it would prove *any* two-field/one-projection setup, and carries **no PL content in the
Lean itself.** All the load-bearing substance lives in the **unverified modeling
dictionary** — the prose claims that `V` = what NLL can see, `P` = the soundness
precondition, and `Copy` *forces* `V`'s blindness. What is machine-checked is the
*logical skeleton*; what you must take on the (adversarially-reviewed, but prose) modeling
argument is the PL substance. This is the normal shape of a minimal formalization, stated
plainly so the "machine-checked" framing transfers no more confidence than it earns.

**Scope:**
- One primitive only (a `Copy`-handle interior-mutable cell). Not a full undecidability
  metatheorem for concurrent separation logic; not a claim that *every* foundational
  primitive has this property (the README does **not** assert it generalizes — that would
  be circular, since it generalizes only to primitives *defined* to have the gap).
- The model abstracts a `borrow_mut` site to two Bools — the *information* `V` sees vs. the
  *information* `P` needs — not the operational semantics of a real queue/MMIO/DMA primitive.
- `no_decision_has_all_three` is a three-line corollary of `P_not_V_measurable` (in this
  model the "trilemma" is really a di-lemma); `Usable` is encoded as totality, the strongest
  reading of usability and hence the weakest theorem.

**Net:** the trilemma as a whole remains "proven by survived adversarial challenge." This
file machine-checks its **logical skeleton** (`P ∉ σ(V)` ⇒ no sound-usable-concentrated
re-import) and — via `REVIEW.md` — forces the faithfulness claim to be stated explicitly
and attacked. It does not, and does not claim to, mechanize the PL substance.
