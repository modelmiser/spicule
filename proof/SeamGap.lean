/-
  SeamGap.lean — a machine-checked core of the Seam Trilemma.

  The adversarial process (design_seamful2/THEOREM.md) named this load-bearing
  fact: "sound re-import needs a linearity witness NLL's region-only vocabulary
  cannot carry." This file mechanizes exactly that, as a MEASURABILITY GAP:

      the soundness precondition P of a Copy-handle interior-mutable cell is NOT
      a function of the NLL/affine witness V.

  Modeling fidelity (why this is not a toy): an interior-mutable cell's handle is
  `Copy` (freely duplicable). NLL's vocabulary — regions/lifetimes + affinity —
  cannot record exclusivity of a duplicable value: you can copy the handle without
  the region/affine system noticing. That is precisely why real Rust cannot put
  interior mutability in the safe checker and instead needs RefCell (a RUNTIME
  borrow flag) or UnsafeCell (drop to the unsafe SEAM). So `V` being blind to the
  aliasing fact is faithful, not stipulated for convenience.

  Core Lean 4 only (no Mathlib). Check with:  lean proof/SeamGap.lean
-/

namespace SeamGap

/-- A program, abstracted at one `borrow_mut` site by the two facts that matter:
    `region_ok` — what NLL's region+affinity check sees (the V-witness);
    `aliased`   — whether a second live access exists (the semantic exclusivity
                  fact). Because the handle is `Copy`, V is blind to `aliased`. -/
structure Prog where
  region_ok : Bool
  aliased   : Bool

/-- The safe-fragment witness. It reads ONLY `region_ok`: the whole point is that
    NLL/affinity cannot observe `aliased` for a duplicable handle. -/
def V (p : Prog) : Bool := p.region_ok

/-- Soundness precondition of `borrow_mut`: the lifetime/region is valid AND no
    other live access exists. A mutable borrow is sound iff both hold. -/
def P (p : Prog) : Bool := p.region_ok && (!p.aliased)

/-- The indistinguishability pair: identical to V, opposite under P. -/
def safe : Prog := ⟨true, false⟩   -- region ok, not aliased → sound
def ub   : Prog := ⟨true, true⟩    -- region ok, aliased     → UB (two live &mut)

theorem same_witness : V safe = V ub := rfl
theorem differ_on_P  : P safe ≠ P ub := by decide

/-- A Bool is determined by the two implication directions (iff ⇒ eq). -/
theorem bool_eq_of_iff {a b : Bool}
    (h1 : a = true → b = true) (h2 : b = true → a = true) : a = b := by
  cases a <;> cases b <;> simp_all

/-- THE GAP. No certifier that reads only the V-witness can equal P: a Copy-handle
    cell's soundness condition is not a function of the NLL/affine vocabulary. -/
theorem P_not_V_measurable :
    ¬ ∃ cert : Bool → Bool, ∀ p, cert (V p) = P p := by
  intro h
  cases h with
  | intro cert hcert =>
    have hs : cert (V ub) = P safe := by rw [← same_witness]; exact hcert safe
    have hu : cert (V ub) = P ub   := hcert ub
    have e  : P safe = P ub        := by rw [← hs, hu]
    exact differ_on_P e

/- ── The three horns of the trilemma, as predicates on a re-import decision `d`. ── -/

/-- Sound re-import: `d` never green-lights a UB program for safe use. -/
def Sound (d : Prog → Bool) : Prop := ∀ p, d p = true → P p = true

/-- Usable: the seam actually vends the primitive — `d` accepts its sound uses.
    (Dropping this is "drop the primitive"; the seam vends nothing useful.) -/
def Usable (d : Prog → Bool) : Prop := ∀ p, P p = true → d p = true

/-- Concentrated & proof-free: ordinary code decides using ONLY the safe witness
    `V` — no use-site proof, no crossing the seam. Bundles horns 2 (decidable,
    proof-free) and 3 (concentration): both mean "d factors through V". -/
def Concentrated (d : Prog → Bool) : Prop := ∃ cert : Bool → Bool, ∀ p, d p = cert (V p)

/-- THE TRILEMMA (mechanized core): no re-import decision can be simultaneously
    sound, usable, and concentrated. You must drop one horn. -/
theorem no_decision_has_all_three :
    ¬ ∃ d : Prog → Bool, Sound d ∧ Usable d ∧ Concentrated d := by
  intro h
  cases h with
  | intro d hd =>
    cases hd with
    | intro hsound hrest =>
      cases hrest with
      | intro husable hconc =>
        -- Sound ∧ Usable ⇒ d agrees with P pointwise.
        have hdP : ∀ p, d p = P p := fun p =>
          bool_eq_of_iff (hsound p) (husable p)
        -- Concentrated ⇒ P factors through V, contradicting the gap.
        cases hconc with
        | intro cert hcent =>
          apply P_not_V_measurable
          exact ⟨cert, fun p => by rw [← hcent p]; exact hdP p⟩

/-- Corollary, horn by horn: any CONCENTRATED, SOUND decision must reject some
    sound program — i.e. give up usability (the verified-Rust reality: the seam's
    obligation surfaces, it cannot be discharged from V alone). -/
theorem concentrated_sound_not_usable :
    ¬ ∃ d : Prog → Bool, Sound d ∧ Concentrated d ∧ Usable d := by
  intro h
  cases h with
  | intro d hd =>
    cases hd with
    | intro hs hr =>
      cases hr with
      | intro hc hu => exact no_decision_has_all_three ⟨d, hs, hu, hc⟩

end SeamGap
