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

/- ──────────────────────────────────────────────────────────────────────────
   THE CHARACTERIZATION (added in the constructive "build-and-break" phase).

   The trilemma above is the negative half: when P does not factor through V, no
   decision is sound+usable+concentrated. The constructive phase found the result
   is primitive-dependent — some seams (SPSC) CLOSE, others (intrusive list) drop a
   horn — and the dividing line is *exactly* whether the soundness precondition
   factors through the safe witness. We mechanize that iff, parameterised over an
   arbitrary precondition `Pr` and witness `Wt`, then instantiate the two regimes.

   Honest bound (carrying forward proof/REVIEW.md): this upgrades the checked claim
   from "P is not a function of V" to "the seam closes IFF P is a function of V" —
   a genuine characterization whose CLOSING direction must construct the decision.
   But what makes `Pr`/`Wt` faithful to a real Rust primitive (affinity carries
   SPSC's cardinality; GhostCell's brand is the only carrier for intrusive aliasing)
   still lives in the prose modeling dictionary, not in the kernel. The math is now
   a true characterization; the PL faithfulness is still asserted, not verified.
   ────────────────────────────────────────────────────────────────────────── -/

section Characterization
variable (Pr Wt : Prog → Bool)

/-- Sound / Usable / Concentrated, parameterised over an arbitrary (precondition,
    witness). `Sound`/`Usable`/`Concentrated` above are the `Pr=P, Wt=V` instances. -/
def SoundW (d : Prog → Bool) : Prop := ∀ p, d p = true → Pr p = true
def UsableW (d : Prog → Bool) : Prop := ∀ p, Pr p = true → d p = true
def ConcW (d : Prog → Bool) : Prop := ∃ cert : Bool → Bool, ∀ p, d p = cert (Wt p)

/-- A seam CLOSES (vends a sound, usable, proof-free/concentrated interface) **iff**
    its soundness precondition factors through the safe witness. This is the exact
    dividing line between SPSC (closes) and the intrusive list (drops a horn). -/
theorem seam_closes_iff_factors :
    (∃ d : Prog → Bool, SoundW Pr d ∧ UsableW Pr d ∧ ConcW Wt d) ↔
    (∃ cert : Bool → Bool, ∀ p, Pr p = cert (Wt p)) := by
  constructor
  · rintro ⟨d, hsound, husable, cert, hcent⟩
    exact ⟨cert, fun p => by
      have : d p = Pr p := bool_eq_of_iff (hsound p) (husable p)
      rw [← this]; exact hcent p⟩
  · rintro ⟨cert, hfac⟩
    exact ⟨Pr, fun _ hp => hp, fun _ hp => hp, cert, hfac⟩

end Characterization

/- ── The two regimes, instantiated as machine-checked verdicts. ── -/

/-- SPSC shape: the soundness-critical fact IS carried by the safe witness (affinity
    discharges cardinality; happens-before leaves no caller-visible residue), so the
    precondition is literally `Wt`. The seam CLOSES — a sound, usable, concentrated
    decision exists. -/
def Pspsc (p : Prog) : Bool := V p

theorem spsc_seam_closes :
    ∃ d : Prog → Bool, SoundW Pspsc d ∧ UsableW Pspsc d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Intrusive shape: the soundness-critical fact (no aliasing `&mut` over caller-owned
    nodes) is NOT carried by the witness — it is the very gap `P_not_V_measurable`
    names. The seam does NOT close: any decision must drop sound-, usable-, or
    concentrated-ness (the GhostCell token = giving up concentration). -/
theorem intrusive_seam_does_not_close :
    ¬ ∃ d : Prog → Bool, SoundW P d ∧ UsableW P d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  intro h
  apply P_not_V_measurable
  rcases h with ⟨cert, hfac⟩
  exact ⟨cert, fun p => (hfac p).symm⟩

/-- Slab shape (the `Copy`-key / `RefCell` corner): the soundness fact is NOT a function of the
    STATIC witness `V` — a `Copy` key, like an interior-mutability handle, defeats `V`, so by
    `intrusive_seam_does_not_close` the seam cannot close through `V`. But unlike the intrusive
    list, it DOES factor through a RUNTIME witness `Vrt` that reads the live exclusivity fact
    (the generation check). Modelling that richer witness, the seam closes.

    The whole zero-cost story lives in the *difference* between `V` and `Vrt`: `V` is static and
    erasable, `Vrt` is computed at run time and does NOT erase. That distinction (which witness
    is free) is the property #6 the trilemma folded away — and it is modeling-dictionary, not
    kernel: the Lean only sees two Boolean witnesses, not their compile-time cost. -/
def Vrt (p : Prog) : Bool := p.region_ok && (!p.aliased)  -- runtime witness: reads live state

theorem slab_closes_through_runtime_witness :
    ∃ d : Prog → Bool, SoundW P d ∧ UsableW P d ∧ ConcW Vrt d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩   -- P factors through Vrt (definitionally Vrt = P): closes at runtime cost

/-- The pairing that IS the slab's lesson, machine-checked: the SAME precondition `P` does not
    close through the static (erasable) witness, but does through the runtime (non-erasable) one.
    Seam-width/concentration is aced either way; the price is which witness you had to read. -/
theorem slab_runtime_closes_but_static_does_not :
    (¬ ∃ d : Prog → Bool, SoundW P d ∧ UsableW P d ∧ ConcW V d) ∧
    (∃ d : Prog → Bool, SoundW P d ∧ UsableW P d ∧ ConcW Vrt d) :=
  ⟨intrusive_seam_does_not_close, slab_closes_through_runtime_witness⟩

/-- Branded-index shape (sound unchecked indexing — Beingessner/Gankra Ch. 6.3; bluss's `indexing`).
    Like SPSC, the soundness-critical fact (B1∧B2∧B3: in-bounds ∧ length-frozen ∧ index-belongs-
    to-THIS-slice) IS carried by the safe witness — here a *generative brand*: an invariant `'id`
    that ties a proof-carrying index to one frozen-length slice. So the precondition factors through
    the witness and the seam CLOSES, exactly as SPSC does: sound + usable + concentrated all hold.

    `Pbrand` is definitionally `V`, just like `Pspsc` — and that identity is the POINT, not a
    redundancy: in the kernel the branded index closes *indistinguishably* from SPSC. Their whole
    difference lives where the kernel cannot see — the cold panel (REPORT_SEAM_SPEC_branded.md)
    located it precisely: `V` here is a *viral generative type parameter* (the #5 API-locality cost,
    attested across the `runST`/lightweight-capabilities lineage in languages with no seam at all,
    hence provably NOT seam-width), the zero-cost win is *deferred* to out-of-scope range machinery
    (#6, conditional on access pattern), and the brand is a *nominal* witness (#7 illegibility,
    dissociable from #5 by refinement types). Same modeling-dictionary caveat as the slab's `Vrt`:
    the math says "closes"; *which axes it spends to close* is prose, because the kernel sees only a
    Boolean witness, never whether that witness erases, infects types, or carries reconstructible
    meaning. This instance is what makes the trilemma a 3-of-N (N≥6), not 3-of-4, projection. -/
def Pbrand (p : Prog) : Bool := V p

theorem branded_seam_closes :
    ∃ d : Prog → Bool, SoundW Pbrand d ∧ UsableW Pbrand d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Refinement-typed shape (Flux — Lehmann et al., PLDI 2023; Liquid Haskell — Vazou et al., ICFP
    2014). The soundness fact (R1∧R2: `value < bound ≤ len`) is carried by the safe witness — here a
    *legible predicate* rather than a brand — so the seam CLOSES, again indistinguishably from SPSC
    in the kernel (`Prefined` is definitionally `V`).

    This instance is the one that CLOSES the #5⟂#7 cell, and the closure is what proves the axes
    independent — but, like every column past "concentration", the proof lives in the modeling
    dictionary, not here: the kernel cannot see that this witness is *brand-free* (a `Refined` escapes
    its constructor, unlike `Idx<'id>`) and *legible* (the predicate `value < bound` is the type's
    contract), while the slab's witness is *brand-free yet illegible*. Holding #5 fixed (refined and
    slab both brand-free) and flipping #7 is the off-diagonal the cold panel (REPORT_SEAM_SPEC_refined)
    confirmed. The kernel sees only that all three close; that they close on *different recovered
    axes* — and that horn 2 itself un-bundles into decidability (✓) vs solver-freedom (✗, the
    external SMT in the TCB — the original `unified` "keep" horn, NOT a new axis) — is prose. -/
def Prefined (p : Prog) : Bool := V p

theorem refined_seam_closes :
    ∃ d : Prog → Bool, SoundW Prefined d ∧ UsableW Prefined d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Shared-ownership shape (a minimal `Rc` — std `rc::Rc` minus `Weak`/`RefCell`). The soundness
    fact (C1∧C2: the strong count equals the live-owner set, freed on the last drop) is carried by a
    *runtime reference-count manager*, so the seam CLOSES, once more indistinguishably from SPSC in
    the kernel (`Pshared` is definitionally `V`).

    What the kernel cannot see — and the cold panel (REPORT_SEAM_SPEC_shared) made precise — is that
    this witness needs a *runtime memory-management subsystem* (a global allocator), where every
    prior seam needed only `core`. That is the seventh axis, **bare-metal / runtime-freedom (seed
    #5)**, confirmed independent of zero-cost (#6) by the off-diagonal against the slab: both pay a
    runtime cost, but the slab's witness is a fixed `core`-only pool (bare-metal ✓) while this one is
    a global-heap manager (bare-metal ✗). Honest deflation the panel insisted on: the axis is real and
    independent but NOT novel — it is Rust's own `core`/`alloc` boundary (RFC 2480), best named
    *unbounded-dynamic-allocation vs caller-provided-bounded-storage* (a `heapless`-pool `Rc` is
    bare-metal but capacity-bounded). Like every column past "concentration", whether the witness
    needs a heap is modeling-dictionary; the kernel sees only that it suffices. -/
def Pshared (p : Prog) : Bool := V p

theorem shared_seam_closes :
    ∃ d : Prog → Bool, SoundW Pshared d ∧ UsableW Pshared d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Type-level-proof shape (the index `I: Lt<Len>` discharged by the trait resolver — the typenum /
    Peano-in-types lineage). The soundness fact (T1∧T2: `I::VALUE < Len::VALUE = len`) is carried by
    the safe witness — a *compile-time trait proof* that erases entirely — so the seam CLOSES, once
    more indistinguishably from SPSC in the kernel (`Ptypednat` is definitionally `V`). Soundness was
    conceded unanimously by the cold panel (REPORT_SEAM_SPEC_typed_nat): sealing + the orphan rule +
    `new`'s conservative length check block every forgery; no UB path.

    This instance is the project's HONEST NEGATIVE RESULT, and it is recorded here precisely because the
    kernel certifies only *closure*, never the axis interpretation the prose tried and FAILED to attach.
    The eighth-axis probe ("checker-tractability" as a third face of horn 2) was REJECTED by the panel
    on three convergent grounds: (1) the off-diagonal is *confounded* — `get::<I>` takes a type, not a
    runtime index, so this seam is static-index-only, a strict expressive subset of branded/refined; the
    claimed one-coordinate flip silently also flips expressiveness; (2) the decisive cost is
    *encoding-relative* — unary Peano hits `recursion_limit` (E0275), but binary type-level naturals
    (typenum `UInt`) are O(log N) and do not, so the cell measures the numeral library, not the
    primitive; (3) it is *mislabeled* — `Lt` resolution is linear, strictly cheaper than refined's QF-LIA,
    so the real concern is the absence of a *terminating-resolution* fragment (Paterson conditions / GHC
    `UndecidableInstances` / `-freduction-depth` ≅ `recursion_limit`), a pre-existing named concern. So
    the seam is sound and closes; it pays a real `recursion_limit` cost INSIDE horn 2; but that cost is
    NOT a clean recovered axis. The lattice stays N ≥ 7. The method's off-diagonal discipline has a STOP,
    and the eighth probe is where it fired. -/
def Ptypednat (p : Prog) : Bool := V p

theorem typed_nat_seam_closes :
    ∃ d : Prog → Bool, SoundW Ptypednat d ∧ UsableW Ptypednat d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Take-and-replace shape (`take_mut`'s sound abort-bomb pattern). The soundness fact (U1∧U2: no
    aliasing the gap ∧ no unwind through the gap) is carried — U1 by the `&mut` borrow, U2 by an
    abort-bomb that converts a mid-take panic into a process abort before the broken slot is observed —
    so the seam CLOSES, once more indistinguishably from SPSC in the kernel (`Ptake` is definitionally
    `V`). Soundness is the load-bearing claim, adjudicated by the cold panel (REPORT_SEAM_SPEC_take):
    the only UB window is the gap, and the bomb forecloses the only gap-observer the floor admits
    (unwinding).

    The axis this primitive probes — **exception-safety-guarantee-level** (Abrahams' basic/strong/nothrow;
    Rust's `UnwindSafe`/`catch_unwind`) — is, like every column past "concentration", invisible to the
    kernel: the Lean sees only that a Boolean witness suffices, never that the witness is an abort-guard
    whose price is a non-recoverable panic. The cold panel (REPORT_SEAM_SPEC_take) ruled it a **real,
    independent eighth axis (N ≥ 8)**, deflated to pre-existing (C++ `noexcept`, `UnwindSafe`, the
    `take_mut`↔`replace_with` lineage) — and crucially a *clean* off-diagonal (the seam splits from SPSC
    on the guarantee-level alone), unlike the rejected `typed_nat` probe. The panel's one mandatory
    correction: the axis is the guarantee-level, and the abort is merely *this seam's* discharge of U2;
    a `replace_with` sentinel discharges the same U2 soundly-and-recoverably by paying API-surface
    instead — so the axis carries a discharge menu, not a fixed cost. All modeling dictionary; the
    kernel sees only that V suffices. -/
def Ptake (p : Prog) : Bool := V p

theorem take_seam_closes :
    ∃ d : Prog → Bool, SoundW Ptake d ∧ UsableW Ptake d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Drain-and-yield shape (`Vec::drain`'s sound pre-pessimization pattern). The soundness fact (D1∧D2:
    no aliasing the half-moved buffer ∧ every slot dropped at most once *regardless of whether the
    guard's `Drop` runs*) is carried — D1 by the `&mut` borrow, D2 by emptying the buffer up front so a
    leaked guard leaves a valid empty buffer — so the seam CLOSES, once more indistinguishably from SPSC
    in the kernel (`Pdrain` is definitionally `V`). Soundness is the load-bearing claim, adjudicated by
    the cold panel (REPORT_SEAM_SPEC_drain): the keystone is that `mem::forget`-ing the guard (a *safe*
    operation) double-frees nothing.

    The axis this primitive probes — **leak-freedom** (Rust's leakpocalypse / `mem::forget`-is-safe; the
    `Vec::drain` leak-amplification lineage; linear-vs-affine types) — is, like every column past
    "concentration", invisible to the kernel: the Lean sees only that a Boolean witness suffices, never
    that the witness leaks the un-yielded tail when forgotten. The cold panel (REPORT_SEAM_SPEC_drain)
    ruled it a **real, independent ninth axis (N ≥ 9)**, **distinct from the eighth (unwind-safety)** —
    the off-diagonal is populated both ways: take-and-replace is unwind-sensitive / leak-safe, this drain
    is leak-sensitive / unwind-safe ("`Drop` never runs" vs "`Drop` runs at a bad time", codified
    separately in Rust as *Leaking* vs *Exception Safety*). Deflated to pre-existing (leakpocalypse / PPYP
    / leak amplification / RFC 1066 / `thread::scoped` removal), and noted the *thinnest, most-deflatable*
    axis in the lattice (its cost is invisible in all non-leak code) — but a genuinely populated
    off-diagonal, not a confound: it aces API-locality precisely *because* the guard owns its data (a
    free-standing `Drain<'a>`, no closure), so it need not pay the scoped-thread closure cost. All
    modeling dictionary; the kernel sees only that V suffices. -/
def Pdrain (p : Prog) : Bool := V p

theorem drain_seam_closes :
    ∃ d : Prog → Bool, SoundW Pdrain d ∧ UsableW Pdrain d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Self-referential-cell shape (a `Pin`-guarded self-pointer; `Pin` — RFC 2349; async/`Future`;
    intrusive collections). The soundness fact (M1∧M2: the self-pointer is established ∧ the cell has
    not moved since) is carried by the safe witness — here `Pin` over a `!Unpin` cell, which statically
    forbids the move — so the seam CLOSES, once more indistinguishably from SPSC in the kernel
    (`Pselfref` is definitionally `V`). Soundness adjudicated by the cold panel (REPORT_SEAM_SPEC_selfref):
    the keystone is that no safe path can move the cell after `init`.

    The axis this primitive probes — **move-stability** (a value whose *address* is load-bearing) — is,
    like every column past "concentration", invisible to the kernel: the Lean sees only that a Boolean
    witness suffices, never that the witness is a viral, `&mut`-revoking `Pin`. The cold panel
    (REPORT_SEAM_SPEC_selfref) ruled **outcome (3): real but STRUCTURALLY INSEPARABLE** — a *new verdict
    category*. Move-stability is a distinct danger (no other axis concerns a value's address) yet has no
    isolable cost: its price is *immovability* (revocation of move/return/store, heavier than
    API-locality), non-uniform across witnesses (stack-pin ⇒ API-locality, `Box::pin` ⇒ bare-metal). So
    it is neither a clean tenth axis nor a confounded non-axis (the `typed_nat` outcome): **N stays 9**,
    with an "inseparable" category added. Honest caveats the panel extracted: the inseparability is
    *contingent* (RFC 2349's rejected `?Move` immovable types were a free-standing witness Rust chose not
    to adopt), and the crisp `core`-only reading holds only for this *non-composable* single-frame cell.
    All modeling dictionary; the kernel sees only that V suffices. -/
def Pselfref (p : Prog) : Bool := V p

theorem selfref_seam_closes :
    ∃ d : Prog → Bool, SoundW Pselfref d ∧ UsableW Pselfref d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Validity-invariant shape (a `transmute`-after-check of a `#[repr(u8)]` enum byte). The soundness
    fact (the byte is a valid discriminant) is carried by the safe witness — a runtime range check before
    the unsafe reinterpretation — so the seam CLOSES, once more indistinguishably from SPSC in the kernel
    (`Pvalidity` is definitionally `V`). Soundness adjudicated by the cold panel (REPORT_SEAM_SPEC_validity):
    the keystone is that no byte `> 2` ever becomes a `Tag`.

    This primitive was built to test a *meta*-question — is the cost-basis **closed**? The cold panel's
    verdict (after fixing two spec bugs — a *range* check unsound for non-contiguous discriminants, then a
    residual stale `b<=2`): the closed-basis meta-claim is **unanimously upheld** — a genuinely new danger
    class (bit-level validity) added **no tenth coordinate**; every witness spends a pre-existing axis;
    *"what does the safe witness cost?" has finitely many answers.* On the *classification* of validity's
    own cost the panels split, and the more careful reading is **(3) INSEPARABLE**, a *second* inseparable
    danger beside move-stability: the zero-copy `&[u8]→&[Tag]` view (the real systems use) admits two sound
    witnesses — eager runtime validation (spends #6) or a sealed constructor (spends API-locality) — same
    danger, two axes, the move-stability signature. Caveats: sound only in the *fieldless single-byte
    corner* (data-carrying/`repr(C)` enums need a recursive `is_bit_valid` witness), and wholly non-novel
    (`num_enum`/`bytemuck`/`zerocopy` + refinement types). Like every column past "concentration", which
    answer holds is modeling-dictionary; the kernel sees only that V suffices. -/
def Pvalidity (p : Prog) : Bool := V p

theorem validity_seam_closes :
    ∃ d : Prog → Bool, SoundW Pvalidity d ∧ UsableW Pvalidity d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Constant-time-secret shape (`subtle`-style timing-safe ops). This is the first instance from a
    **different soundness domain** — information flow / non-interference, not memory safety — so read
    `Pr`/`Wt` by analogy: the precondition is *timing-safety* (running time independent of the secret)
    and the witness is the constant-time API (no `PartialEq`/`Index` to branch on; branchless `ct_eq`).
    The precondition factors through that witness (source-level data-dependent branching is foreclosed),
    so the seam CLOSES, indistinguishably from SPSC in the kernel (`Psecret` is definitionally `V`).

    The kernel's blindness is *deeper* here than for any prior seam, and that depth is the §6 point: the
    Lean does not model time at all. So this `..._closes` theorem certifies only the trivial sub-fact — the
    *typed* discipline (no `PartialEq`/`Index` to branch on `Secret`) factors as a Boolean witness. The
    cold panel (REPORT_SEAM_SPEC_secret) ruled the actual seam **SEAM-BREAKS** in the information-flow
    sense: `ct_eq(a,b).expose_bool()` then `if b` type-checks with only vended ops and leaks a secret
    relationship — declassification is *etiquette, not type-structure*, and closing it needs whole-program
    undecidable IFC. The intrusive list's horn-drop, re-run in a new domain: a concentrated decidable seam
    cannot soundly enforce non-interference. The §6 **domain ruling was vindicated**: information-flow is
    *out of frame* — a different soundness domain with its **own basis** (declassification discipline,
    termination/concurrency channels), so the closed memory-safety basis is **closed per domain**, and the
    Seam Trilemma is domain-parameterized. (`Psecret = V` certifies none of this — it is the sharpest
    illustration of the modeling-dictionary caveat: the kernel models neither time nor non-interference.) -/
def Psecret (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors as a Boolean witness. The
    real seam BREAKS in the IFC sense — non-interference is not a concentrated decidable property. -/
theorem secret_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Psecret d ∧ UsableW Psecret d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Capability-gated-declassification shape — the *second* information-flow seam, and (unlike the secret
    seam, which broke) a **closing** one. Read `Pr`/`Wt` by analogy in the IFC domain: the precondition is
    *bounded/concentrated declassification* (release only at an audited site, ≤ the issued cap budget) and
    the witness is an **affine declassification capability** (`Secret<T>` has no exit but `declassify(cap)`;
    `Cap` is `!Copy`/`!Clone`, so release is type-structurally bounded). The precondition factors through
    that witness — *where/when/how-often* a secret is released is enforced by the type, not convention — so
    the seam CLOSES, indistinguishably from SPSC in the kernel (`Plabeled` is definitionally `V`).

    But the cold panel (REPORT_SEAM_SPEC_labeled) ruled the real seam **SEAM-BREAKS** — the *second* IFC
    horn-drop, unanimously. `map`/`zip` must hand the closure the bare plaintext (an uncapped `Secret→T`
    exit; a captured-`&mut` closure exfiltrates it), and the cap "budget" counts *events* not *bits*
    (`issue()` in a loop is unbounded; one cap leaks a megabyte). So `..._factors` (renamed from
    `..._closes`) certifies *only* the trivial typed sub-fact, exactly as the secret seam's does. The
    deepest finding it records: **a closing seam exists only when the floor matches the domain.** Rust's
    affine floor is matched to memory safety (SPSC closes free, basis closed) and *mismatched* to
    information flow — *no* IFC seam closes; the one thing the cap enforces (release *location*) projects
    onto the memory-safety affine axis, and the genuinely-IFC coordinate (the *amount* — quantitative
    leakage, Smith min-entropy) needs a quantitative/effect-tracking floor (Jif/FlowCaml/QIF) the kernel
    and the Rust floor alike cannot express. Pure modeling-dictionary; the kernel sees none of it. -/
def Plabeled (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors. The real seam BREAKS in
    the IFC sense — Rust's affine floor cannot bound information *amount*, only resource *uses*. -/
theorem labeled_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Plabeled d ∧ UsableW Plabeled d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Type-level-lock-ordering shape — the *third* soundness domain (**liveness / progress**), and the
    *third* seam to break. Read `Pr`/`Wt` by analogy: the precondition is *lock-order-inversion freedom*
    (the per-thread held set is a strictly-decreasing chain in a global rank order) and the witness is a
    type-level rank `L` plus an affine `Floor<F>` capability with `with` bounded by `L: Below<F>`. In the
    kernel the typed discipline factors (`Pordlocks` is definitionally `V`), as always.

    But the cold panel (REPORT_SEAM_SPEC_ordered_locks, r21) ruled **SEAM-BREAKS [horn 1]**, and the break
    *grades* the floor-domain match. Two independent failures, both "the floor under-carries the
    precondition": (1) **granularity** — `Floor::root` is re-entrant, so two sibling roots mint two
    `Floor<TopRank>` lineages and safe code types a low-then-high *inversion* (the discipline is per-token;
    the precondition is per-thread-global, and Rust's floor has no ambient per-thread one-shot resource);
    (2) **coverage** — even patched, the dynamic-acyclicity fragment (runtime-ordered / same-rank locks:
    bank-transfer `min/max`, hand-over-hand coupling, IRQ ordering) escapes the *static* type-level-order
    floor, exactly as IFC's quantitative coordinate escaped the affine floor. So the floor-domain match is
    **graded**: full (memory safety / affine floor), partial (liveness / static-order floor — this), none
    (information flow / affine floor). Documented precedent: Boyapati–Lee–Rinard OOPSLA'02 *minus* their
    dynamic tree-order extension, and Kobayashi's natural-number levels. `..._factors` (not `..._closes`)
    certifies only the trivial typed sub-fact, as the two IFC seams' do; the kernel sees none of the
    liveness, the granularity gap, or the static/dynamic coverage gap. -/
def Pordlocks (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors. The real seam BREAKS — the
    static type-level-order floor carries only a sub-fragment of the deadlock-freedom precondition (wrong
    granularity per-token, wrong coverage static-only), so the floor-domain match here is *partial*. -/
theorem ordered_locks_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Pordlocks d ∧ UsableW Pordlocks d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Runtime-lock-registry shape — the *floor-upgrade* probe (feature `lockdep`), the *second* liveness seam
    and the *fourth* seam to break. Same domain and danger as `Pordlocks`, but the floor is upgraded from a
    static type-level order to a **runtime** registry: a `thread_local!` held-set of runtime ranks, checked
    on each acquire. In the kernel the discipline factors (`Pregistry` is definitionally `V`), as always.

    The cold panel (REPORT_SEAM_SPEC_lock_registry, r22) ruled **SEAM-BREAKS [horn 1]**, and the break shows
    the floor-upgrade is **non-monotone**. It fixed #15's *coverage* gap (runtime-chosen-fixed ranks close —
    bank-transfer, hand-over-hand) and paid the predicted price (#5 bare-metal — it cannot be `no_std`; #6 a
    per-acquire runtime check; a prevention→detection modality, soundness now path-coverage-dependent). But
    **granularity recurred one rung finer**: `HELD` is keyed on the OS *thread*, while the unit of control
    in async is the *task*. Work-stealing migration resumes a task (holding a rank) on a fresh thread with an
    empty `HELD`, so a lower rank is accepted with no panic — an executed, undetected inversion (verified by
    a two-thread reproduction; the registry is thread-blind). So the **granularity** axis is a
    containment-scope **ladder** — token < thread < task < … — and each ambient Rust floor reaches one rung
    (affine token #15, thread-local #16); there is no ambient *task-local*. Prior art forecloses the design:
    FreeBSD WITNESS / lockdep / Abseil all abandoned static ranks for dynamically-learned acquires-before
    graphs. `..._factors` (not `..._closes`) certifies only the trivial typed sub-fact; the kernel sees none
    of the liveness, the thread/task granularity mismatch, the coverage limit, or the modality. -/
def Pregistry (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors. The real seam BREAKS — the
    floor-upgrade is non-monotone: it fixed coverage but granularity recurred (thread-keyed vs task-keyed
    control), so the liveness floor-match stays *partial* and the granularity gap is an unbounded ladder. -/
theorem lock_registry_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Pregistry d ∧ UsableW Pregistry d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Must-run-cleanup shape — the *linearity* probe (a `MustRun<F: FnOnce()>` RAII guard), built
    breadth-wise: a new precondition rather than a new domain, and the *fifth* seam to break. Read
    `Pr`/`Wt` by analogy: the precondition is *must-run-cleanup* (the finalizer runs exactly once before
    the scope is abandoned) and the witness is the affine ownership of the guard plus its `Drop`. In the
    kernel the typed discipline factors (`Plinear` is definitionally `V`), as always — the closure is run
    *at most* once (the `Option<F>` one-shot), which is exactly the AFFINE half the floor does carry.

    The real seam BREAKS, compiler-settled (not a panel judgment call): `core::mem::forget` is a **safe**
    function — its unbounded `fn<T>(T)` consumes the guard by one legal affine move — so no destructor is
    mandatory; the *leakpocalypse* (RFC 1066; the removal of `thread::scoped`, RFC 1084). "Must run
    *exactly* once" is a **linear** requirement; Rust's floor is **affine** (*at most* once), so it cannot
    forbid the skip. The cold panel (REPORT_SEAM_SPEC_linear) confirmed this across **two independent
    rounds** (r23 + r24), the breaking attackers included, and ruled: (1) the affine floor earns a **clean
    NONE** at the type level — the `#[must_use]` lint is a cosmetic non-type advisory (as `scopeguard`
    ships), not a partial fragment; (2) the floor-match unit is the **precondition**, not the *domain* —
    the same liveness domain grades deadlock-freedom *partial* (#15/#16) and must-run-cleanup *none*; (3)
    this is the **enforcement** face of the ninth axis (leak-freedom), inverse to the tenth seam `drain`
    (which *closed* by leak-**tolerance**), so N is unchanged; (4) the verdict mode is the NEW
    **wrong-structural-logic / out-of-frame** — every sound *decidable* fix collapses to a viral
    `Leak`/linear bound (the alternative, heap-reachability, is undecidable ≅ halting), so the floor is the
    wrong *structural logic* (affine vs linear), not the wrong *domain*; sharpened to Boats' *scoped-task
    trilemma* (Rust rejected the linear horn), not a binary swap. `..._factors` (not `..._closes`)
    certifies only the trivial affine sub-fact; the kernel sees neither the linearity gap nor the
    `forget`/`ManuallyDrop` escape. -/
def Plinear (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *affine* discipline factors — the closure runs at most
    once. The real seam BREAKS — "runs *exactly* once" is linear, and Rust's affine floor permits the skip
    (`mem::forget` is safe), so the must-run-cleanup floor-match is a clean *none* at the type level (the
    `#[must_use]` lint is a cosmetic advisory, not a floor fragment), never the type-level closure memory
    safety enjoys. The verdict mode is *wrong-structural-logic* (affine vs linear), not domain-mismatch. -/
theorem linear_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Plinear d ∧ UsableW Plinear d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Session-typed-channel shape — the *composite-precondition* probe (a `Chan<P>` affine typestate handle),
    and the FIRST out-of-(memory-safety) probe to **hold with a cost** rather than break. Built as the
    crucible for #15 (typestate carries static order) and #17 (the floor cannot compel completion). Read
    `Pr`/`Wt` by analogy, but note the kernel models only the component the seam actually CLOSES: the
    *order-safety* precondition (no out-of-order operation) factors through the affine typestate witness
    (`Psession` is definitionally `V`), so `send`/`recv`/`close` exist only at the matching state, an
    out-of-order step is a *type error*, and — unlike #15 — there is no `root` to re-enter, so the safety
    sub-seam is sound, usable, and concentrated. It CLOSES.

    The cold panel (REPORT_SEAM_SPEC_session, r25) ruled the whole seam **SEAM-COSTS [horn 3]** (TRADEOFF,
    survived) — it HOLDS with a recorded cost, the first such in this arc. The composite precondition is
    matched **component-wise**, and the components are *orthogonal* (the soundness attacker confirmed
    `mem::forget` cannot shorten `'a` to start a second session, so the completion gap cannot infect safety):
    order **closes** (above); **completion** is an undischargeable horn-3 *concentration* cost (the #17
    linearity gap — a mid-protocol `Chan` is `mem::forget`-able — recurring at every `?`/panic/early-return),
    NOT a horn-1 break. The illuminating contrast: the *same* `mem::forget` gap that **broke** #17's horn 1
    only **costs** here, because completion was #17's *whole* soundness promise but is *one component beside a
    sound safety core* here — so which horn a linearity gap threatens depends on how the rule **packages** it.
    Like every column past concentration, the horn-3 cost is modeling-dictionary; the kernel sees only that
    the order-safety witness suffices. Panel deflations recorded in the prose: this `core`/no-`unsafe` witness
    has no duality (a single-handle typestate *walk*, not a binary session — E0499 forecloses two endpoints),
    "usable" holds only for infallible straight-line protocols, and the order/completion split is documented
    folklore (Jespersen–Munksgaard–Larsen 2015; RFC 1066; ECOOP 2022 "Stay Safe Under Panic") — the
    contribution is *locating* it on the lattice. -/
def Psession (p : Prog) : Bool := V p

/-- The seam HOLDS with a cost (SEAM-COSTS [horn 3]): its *order-safety* component closes via affine typestate
    (sound, usable, concentrated, no `root`-hole), exactly as a closing seam does in the kernel. The completion
    component is a horn-3 concentration cost (the #17 linearity gap), invisible to the kernel — the same gap
    that BROKE #17's horn 1 only COSTS here, because order-safety is independently sound. -/
theorem session_order_safety_closes :
    ∃ d : Prog → Bool, SoundW Psession d ∧ UsableW Psession d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Value-owning-scope-restore shape — the *controlled-packaging* probe (a `Restore<T, F: FnOnce(T)>` guard,
    the real `scopeguard::guard(value, f)` + `into_inner`), built to turn Phase Q's packaging finding from an
    observation into a *deliberate* experiment: it carries the **identical** un-compellable-`Drop` danger as
    #17 (`mem::forget` skips the restore closure) but wraps it in #18's structure — a sound affine-ownership
    *core* beside which the restore is a side-obligation. Read `Pr`/`Wt` by analogy: the precondition the seam
    actually CLOSES is the *memory-safety / ownership core* (own a `T`, vend scoped `DerefMut` access,
    `Option::take` so the value is committed-xor-restored exactly once — no double-free, no alias, no
    double-consume), which factors through the affine ownership witness (`Prestore` is definitionally `V`). It
    CLOSES.

    The cold panel (REPORT_SEAM_SPEC_restore, r26) ruled **SEAM-COSTS [horn 3]** (TRADEOFF, survived) — it
    HOLDS with a recorded cost, confirming the packaging dial: the *same* `mem::forget` gap that BROKE #17's
    horn 1 only **costs** horn 3 here (the restore is an undischargeable "don't forget the guard" obligation,
    recurring at every `?`/panic/return — never UB, since memory safety never rested on the closure). But the
    panel **bounded** the thesis, all accepted in the prose: (1) the move is **confounded** — to package the
    danger as a side-obligation, #19 also adds the value-ownership core #17 lacked, so the value-ownership
    model co-varies (a better-controlled probe, not a single-variable manipulation); (2) the thesis is
    **bounded, not universal** — the horn is *forceable*: the same skippable-`Drop` danger in a borrowed-slot/
    `take_mut` packaging forces horn-1 UB (the ninth-axis `take` danger), and `Pin`'s drop guarantee and the
    `thread::scope` restructure exhibit un-repackageable horns, so *which* horn depends on **where safety
    rests** — partly danger-intrinsic; (3) **not novel** — `JoinGuard` broke horn 1 and `scopeguard` costs
    horn 3 already in 2015 (RFC 1066, the leakpocalypse), so the contribution is the *method* and *locating*
    the dial, not a new verdict structure (N stays 9 modes; the 5 broken seams unchanged). The refined
    finding: which horn a danger threatens is a design choice **within a structural envelope** set by whether
    soundness can be decoupled from the un-compellable closure. Like every column past concentration, the
    horn-3 cost is modeling-dictionary; the kernel sees only that the ownership-core witness suffices. -/
def Prestore (p : Prog) : Bool := V p

/-- The seam HOLDS with a cost (SEAM-COSTS [horn 3]): its *memory-safety / ownership core* closes via affine
    ownership (`!Copy`/`!Clone`, `Option::take` one-shot, borrow-tied `Deref`), sound/usable/concentrated,
    exactly as a closing seam does in the kernel. The restore obligation is a horn-3 concentration cost (the
    same #17 `mem::forget` gap), invisible to the kernel — but the gap only COSTS here, because soundness is
    *decoupled* from the closure (an owned value), the controlled-packaging move that #17 lacked. -/
theorem restore_ownership_core_closes :
    ∃ d : Prog → Bool, SoundW Prestore d ∧ UsableW Prestore d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Take-and-reseat-over-a-borrowed-slot shape — the *forcing-the-horn* probe (a `Reseat<'a, T>` guard,
    `take_mut` in guard form), the SIXTH seam to break, and the controlled counterpart to #19. It keeps #19's
    packaging byte-identical (a guard with `Deref`/`DerefMut`/`into_inner`/Drop-restore) and changes only the
    decoupling property — owned value → borrowed slot — the single-variable experiment #19's confound forbade.
    Read `Pr`/`Wt` by analogy: the precondition is *single ownership across the transient `ptr::read`
    duplicate*. In the kernel the typed discipline factors (`Preseat` is definitionally `V`), as always — the
    affine guard, the unwind-safe `Drop`-reseat, and the `mem::forget`-is-a-leak fact are all the parts the
    floor DOES carry.

    The real seam BREAKS, compiler-settled (REPORT_SEAM_SPEC_reseat, r27 — SEAM-BREAKS [horn 1], FAILED): a
    4-line **safe** witness — `let mut s = String::from("hello"); let mut g = Reseat::new(&mut s);
    g.push_str(" world"); g.into_inner()` — returns the owned `String` while the slot keeps the `ptr::read`
    duplicate (`E0509` forces the `mem::forget(self)` that suppresses the reseat), so both own overlapping
    heap and **double-free**, with no `unsafe` in user code. The cold panel ruled: (1) the single-variable
    horn flip is **clean** — #19 (owned + `into_inner`) *cost* horn 3, this (borrowed + `into_inner`) *breaks*
    horn 1, packaging byte-identical — so Phase R's confound is **repaired** and the flip is attributable to
    the decoupling property alone; (2) the fence is **FORCED, not movable** — the `into_inner`-less "sound"
    subset is *degenerate* (it deletes the by-value move-out that defines `take_mut`, strictly dominated by
    `&mut T`), so every *usable* packaging breaks horn 1, and the fence sits at the **owned-vs-borrowed** line.
    Deflation (pre-registered): no novel mechanism (`take_mut` + `scopeguard`-`into_inner` + RFC 1066);
    contribution is analytical. The refined finding closing the packaging trilogy: *the horn a danger
    threatens is movable iff the value-ownership model lets soundness DECOUPLE from the un-compellable closure*
    (owned can ⇒ cost; borrowed cannot, for any usable packaging ⇒ forced break). `..._factors` (not
    `..._closes`) certifies only the trivial typed sub-fact; the kernel sees neither the `into_inner`
    double-free nor the duplicate window. -/
def Preseat (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors — the affine guard, the
    unwind-safe reseat, and the forget-is-a-leak fact are what the floor carries. The real seam BREAKS:
    `into_inner` over a borrowed slot hands out the value while the slot keeps the `ptr::read` duplicate — a
    safe-code double-free (horn 1). It is the FORCED-horn counterpart to #19's held owned-value `restore`:
    same packaging, only the decoupling property flipped, so the verdict flips cost-3 → break-1. -/
theorem reseat_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Preseat d ∧ UsableW Preseat d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- State-based-CRDT-merge shape — the *fourth-domain* probe (a `Replica<T: JoinSemilattice>` with a
    `merge` that applies the semilattice `join`). Like the `secret`/`labeled` IFC seams it has **no `unsafe`**
    — the danger is **semantic** (replica *divergence*, not UB). Read `Pr`/`Wt` by analogy: the precondition
    is *convergence* (the semilattice laws — commutative / associative / idempotent). In the kernel the typed
    discipline factors (`Pcrdt` is definitionally `V`), as always — the `T: JoinSemilattice` trait bound and
    same-`T` typing are the parts the floor DOES carry.

    The real seam BREAKS *floor-relatively* (REPORT_SEAM_SPEC_crdt, r28 — horn-classifier SEAM-BREAKS
    [floor-mismatch], scored a TRADEOFF survival): the trait bound is a **syntactic proxy, not a witness** —
    `merge` applies whatever `join` the impl supplies, the bound checks only the *signature*, so a safe
    non-commutative/non-idempotent `impl` type-checks and **diverges replicas that saw the same updates**
    (Horn 1, in safe code; closing it is undecidable by Rice). But the candidate fourth domain **DEFLATED**,
    three corrections accepted: (1) the blessed-CRDT TCB rescue is **not MMIO-shaped** — the lattice space is
    open-ended, so the law-trust recurs per user CRDT, never amortizing into one foundation axiom; (2)
    'dischargeable' holds only at the *model* level (Liquid-Haskell/Coq proofs are about a math model; no Rust
    pass checks model-to-shipped-code, so the trust is irreducible at the compilation boundary); (3) the
    decisive deflation — the gap is the **generic unchecked-typeclass-law phenomenon** (Rust checks the
    *signatures* of `Ord`/`Eq`/`Hash`/`Monoid`/semilattice, none of their *laws*), so convergence **projects
    onto `restore`'s application-correctness basis** rather than opening a fourth soundness domain. The map
    stays at three domains (memory/IFC/liveness) + the application-correctness basis; N stays 9 modes; the
    FAILED broken-seam tally stays 6 (this survived). `..._factors` (not `..._closes`) certifies only the
    trivial typed sub-fact; the kernel sees neither the lawless-`join` divergence nor the deflation. -/
def Pcrdt (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors — the `JoinSemilattice` trait
    bound and same-`T` typing are what the floor carries. The real seam BREAKS floor-relatively: a safe
    lawless `join` type-checks and diverges replicas (the semilattice laws are unenforceable, undecidable by
    Rice). But the candidate fourth domain DEFLATED — convergence is the generic unchecked-typeclass-law
    phenomenon, projecting onto `restore`'s application-correctness basis, not a new soundness domain. -/
theorem crdt_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Pcrdt d ∧ UsableW Pcrdt d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Cost-typed-computation shape — the *second fourth-domain* probe (a `Timed<C, T>` graded cost monad,
    reusing `typed_nat`'s sealed `Nat`/`Lt`), targeting **real-time / WCET** after the convergence probe
    (`crdt`) deflated. Like `crdt`/`secret` it has **no `unsafe`** — the danger is the **semantic** mismatch
    between a static op-count and real elapsed time (a *deadline miss*). Read `Pr`/`Wt` by analogy: the
    precondition is *the computation meets its cycle deadline*. In the kernel the typed discipline factors
    (`Pwcet` is definitionally `V`), as always — the static op-count accounting (`within::<D>` proving `C ≤ D`)
    is the part the floor DOES carry.

    The real seam BREAKS, and FAILED (REPORT_SEAM_SPEC_wcet, r29 — SEAM-BREAKS [horn 1], the seventh broken
    seam): `step` charges `C := S<C>` *decoupled from its closure*, so a million-iteration loop is cost `1` and
    `within::<1>` admits it — `C ≤ D` proves nothing about real cycles (my §4 over-claimed the deadline
    guarantee). All five attackers landed: (1) soundness — op-count is no cycle upper bound; (2) ergonomics —
    Horn 3 FALSE (non-unifying `if`-branches force manual step-padding); (3) implementation — unary Peano cost
    is O(C), uncompilable at real 10³–10⁷-cycle budgets, and the only compilable path *is* the break; (4)
    expressiveness — no type-level addition, so no compositional sequencing; (5) prior-art — this is
    Danielsson's 2008 cost monad, and "op-count ≠ cycles" is WCET's *founding premise*, so "pick a lane"
    (abstract cost xor physical deadline — the latter needs a defining machine the floor lacks). The refined
    finding: WCET does NOT open a fourth domain — it deflates (cost monad / `typed_nat` #8 for the static
    part — which does not even *scale*; an `mmio`-shaped-but-*uninstalled* axiom #2 for the physical claim;
    liveness's partial-match for data-dependent loops). The op-count is an **unsound** proxy — *sharper* than
    `secret`'s sound-but-incomplete one. META: two fourth-domain candidates (`crdt`, `wcet`), neither opens a
    fourth domain — the three domains (memory/IFC/liveness) look special. N stays 9; FAILED tally rises to 7.
    `..._factors` (not `..._closes`) certifies only the trivial typed sub-fact; the kernel sees neither the
    op-count/cycle gap nor the deflation. -/
def Pwcet (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* discipline factors — the static op-count accounting
    (`within::<D>` proving `C ≤ D`) is what the floor carries. The real seam BREAKS (and FAILED): `step`
    charges one unit decoupled from its closure's work, so `within::<1>` admits a million-cycle computation —
    the op-count is an unsound proxy for real time. WCET does not open a fourth domain; it deflates (Danielsson
    2008 cost monad / `typed_nat` #8 + an uninstalled `mmio`-shaped axiom + liveness's partial-match). -/
theorem wcet_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Pwcet d ∧ UsableW Pwcet d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Real-binary-session shape — the *two-endpoint liveness-deepening* probe (a `session::<P>()` vending two
    dual affine `Chan<P>`/`Chan<P::Dual>` handles, both live, over `Rc<RefCell<Transport>>`). NOT a
    fourth-domain bid — after `crdt`/`wcet` deflated, this stays in the third domain (liveness) and removes
    #18's most-cited limitation: one `&mut Transport` could not hold two concurrently-live endpoints (`E0499`),
    so #18's "peer waits forever" hazard was inexpressible. Read `Pr`/`Wt` by analogy: like #18 the kernel
    models only the component the seam CLOSES — the *order-safety* + *per-session relative-progress* core
    factors through the affine-typestate + `HasDual` witness (`Pbinsession` is definitionally `V`): `send`/
    `recv`/`close` exist only at the matching state (order), and the two endpoints hold dual protocols so they
    cannot both block within one session (relative progress). It CLOSES.

    The cold panel (REPORT_SEAM_SPEC_binary_session, r30) ruled the whole seam **SEAM-COSTS [horn 3]**
    (TRADEOFF, survived) — it HOLDS with a cost, and **#18's component-wise grading SURVIVES** the removal of
    the single-handle limitation: order closes, per-session progress closes, **completion** is an
    undischargeable horn-3 *concentration* cost (the #17/#18 linearity gap — a peer's `mem::forget` starves
    its dual — now peer-observable), NOT a horn-1 break (no `unsafe`; the worst case is a starved memory-safe
    handle). But the panel substantially **deflated the framing** (five recorded costs, all accepted): (1) the
    sharpest — relative progress closes only **per session**, since two interleaved `session()` calls
    type-check a mutual-wait deadlock (Kobayashi/Padovani), so global deadlock-freedom is out of reach of
    binary session types (needs multiparty typing); (2) the single-threaded witness makes "peer waits forever"
    self-abandonment-flavored — only the *static* two-live-endpoint limitation is lifted, not `session_types`
    2015's real two-thread concurrency; (3) the fallible `recv`'s `Err` is **non-actionable** because
    `mem::forget` leaks the `Rc` (the leakpocalypse double-bind: the gap that costs completion also denies an
    actionable abandonment signal); (4) only straight-line `i32` protocols (no choice/delegation/recursion);
    (5) citation fix (ECOOP 2022 = Lagaillardie–Neykova–Yoshida). Like every column past concentration, the
    horn-3 cost is modeling-dictionary; the kernel sees only that the order+progress witness suffices. N stays
    9 modes; liveness stays the third domain; the FAILED broken-seam tally stays 7 (this survived). -/
def Pbinsession (p : Prog) : Bool := V p

/-- The seam HOLDS with a cost (SEAM-COSTS [horn 3]): its *order-safety* + *per-session relative-progress* core
    closes via affine typestate + the `HasDual` duality relation (sound, usable, concentrated), exactly as a
    closing seam does in the kernel. The completion component is a horn-3 concentration cost (the #17/#18
    linearity gap, now peer-observable via two real endpoints), invisible to the kernel — and the panel's
    deflation (per-session-only progress; non-actionable `Err` from the leaked `Rc`; single-threaded,
    straight-line witness) lives in the prose, not the checked sub-fact. -/
theorem binary_session_progress_core_closes :
    ∃ d : Prog → Bool, SoundW Pbinsession d ∧ UsableW Pbinsession d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Delegation (channel-over-channel) shape — the *higher-order session* probe (`send_chan`/`recv_chan` over
    `Rc<RefCell<Transport>>`, the feature #23 lacked). Read `Pr`/`Wt` by analogy: the kernel models only the
    component the seam CLOSES — the *order* + *don't-use-after-delegate* core (`Pdeleg` is definitionally `V`):
    `send_chan`/`recv_chan` exist only at the matching state (order), and `send_chan` **consumes** the delegated
    handle, so the affine MOVE carries don't-use-after-delegate — the one linearity delegation needs here, which
    the floor has for free. It CLOSES, and the soundness is intact (no UB, decidable, duality-pinned re-typing).

    But the cold panel (REPORT_SEAM_SPEC_delegation, r31) ruled the whole gauntlet **FAILED — on the §6
    DELIVERABLE, not on soundness** (the horn-classifier still read the *mechanism* as SEAM-COSTS [horn 3]).
    This is the method's **REJECTION mode**, firing the **second time** (after `typed_nat` #8, also sound +
    rejected): my pre-registered claim — "delegation adds no new coordinate; its hazards project onto
    leak-freedom (#9/#10), a Phase-K closed-basis confirmation" — was **refuted from my own cited lineage**.
    Delegation's signature hazard (B forgets the received `Chan<Q>`, **starving C**) is "deadlock at the other
    end" (Jespersen–Munksgaard–Larsen 2015) — **progress / deadlock-freedom**, a coordinate *distinct* from
    completion (linearity) and leak-freedom. Delegation is the *recognized* feature that breaks first-order
    binary sessions' automatic deadlock-freedom (Coppo–Dezani–Yoshida–Padovani, "the communication topology
    changes because of delegation"; Kobayashi; deadlock-free GV — Lindley–Morris, Padovani, Multiparty GV).
    I saw the `Rc` hazard's leak-FACE and mislabeled the progress AXIS; the single-threaded witness hides
    deadlock by construction, so "no new coordinate" was an artifact of the scoping. The symmetric delegation
    cycle is type-obstructed (an inexpressible infinite type) — but that rescues *behavior*, not *classification*.
    So the liveness domain has **progress** as a third coordinate (the lock seams #15/#16 were its *lock* face;
    delegation its *session* face). The seam is sound, so the FAILED broken-seam tally STAYS 7 (a deliverable
    rejection, not a soundness break); N stays 9 (rejection is one of the nine modes). Like every column past
    concentration, the horn-3 cost and the rejected classification are modeling-dictionary; the kernel sees only
    that the order + affine-handoff witness suffices. -/
def Pdeleg (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn 3]; the gauntlet FAILED on the *deliverable*, not on soundness): its
    *order* + *don't-use-after-delegate* core closes via affine typestate + the affine move (which carries the
    one linearity delegation needs), exactly as a closing seam does in the kernel — like `typed_nat` (sound, its
    deliverable rejected). The uncarried components — completion (horn-3, now three-party) and the newly-named
    **progress / deadlock-freedom** coordinate delegation introduces — live in the prose, invisible to the
    kernel; the rejection corrected my mislabeling of progress as a leak-freedom projection. -/
theorem delegation_handoff_core_closes :
    ∃ d : Prog → Bool, SoundW Pdeleg d ∧ UsableW Pdeleg d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Multiparty session (n-party progress) shape — the *global-coherence* probe (`relay`/`ring` assembled from
    #23's binary edges; a `RecvThenSend` role enforces recv-before-send). Read `Pr`/`Wt` by analogy: the kernel
    models only the component the seam CLOSES — the *role-local order* + *linearity* core (`Pmulti` is
    definitionally `V`): each role's ops are available only in role-local order (affine typestate), exactly as
    #18/#23. It CLOSES; soundness is intact (no UB — a deadlocked multiparty system is memory-safe, leakpocalypse
    — decidable per-role `HasDual`, Θ(depth)).

    What the kernel does NOT see is the deliverable's subject: **global progress / deadlock-freedom**, the
    coordinate #24 named, now at n-party scale. The witness `ring()` — three perfectly-dual binary edges wired
    into a source-less cycle — *type-checks a system with no terminating schedule*: every role is `RecvThenSend`,
    so no role can send before it receives and no role can receive before someone sends. The per-edge floor
    accepts the globally-deadlocked topology because deadlock-freedom is a property of the *whole composition*,
    which the per-edge floor cannot range over.

    The cold panel (REPORT_SEAM_SPEC_multiparty, r32) ruled the gauntlet **FAILED — on the §6 DELIVERABLE, not on
    soundness** (the horn-classifier still read the *mechanism* as SEAM-COSTS [horn 3]). This is the method's
    **REJECTION mode**, firing the **third time** (after `typed_nat` #8 and `delegation` #24, both sound +
    rejected): my pre-registered claim — "global progress can be concentrated *only* whole-protocol, in a
    *generative* projection of a coherent global type; no per-role discipline can reject the ring" — was a **false
    dichotomy**, refuted (again) from my own cited lineage. Three seats named the middle rung I omitted:
    **per-channel priority/obligation typing** (Kobayashi linear-π; Padovani; Dardha–Gay FoSSaCS 2018;
    Kokke–Dardha) rejects the source-less ring *compositionally, with no global type*, via `prio(recv) <
    prio(send)` chaining to a contradictory `p < p` on the cycle — and I had **mis-filed Kobayashi as *support***
    for the whole-protocol thesis when he is the *counterexample*. The honest taxonomy has **three rungs**: (a)
    per-edge `HasDual` floor [doesn't carry]; (b) per-channel priority typing + a whole-program acyclicity check
    [no global type]; (c) generative MPST projection. The surviving weaker truth (credited by the soundness
    seat): global progress is **not concentrable per-call** and needs strictly more than the per-edge floor — but
    it is a **whole-program** obligation, not necessarily a *generative* one. No new domain (liveness), coordinate
    (progress, from #24), or mode (N stays 9); the seam is sound, so the broken-seam tally is unchanged. Like
    every column past concentration, the horn-3 cost and the three-rung relocation are modeling-dictionary; the
    kernel sees only that the role-local order + linearity witness suffices. -/
def Pmulti (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn 3]; the gauntlet FAILED on the *deliverable*, not soundness): the
    *role-local order* + *linearity* core closes via affine typestate, exactly as #18/#23 — `relay` and `ring`
    are built from the identical per-edge floor, and that floor's per-role discipline is sound and decidable, like
    `typed_nat`/`delegation` (sound, deliverable rejected). The uncarried component — **global progress /
    deadlock-freedom** — lives in the prose: it is a whole-program (not per-call) obligation strictly above the
    per-edge floor, dischargeable by per-channel priority typing (no global type) *or* a generative global-type
    projection — a three-rung taxonomy, not the two-rung dichotomy I staked. That relocation is invisible to the
    kernel; the rejection corrected my collapse of the priority-typing rung into the generative one. -/
theorem multiparty_role_order_core_closes :
    ∃ d : Prog → Bool, SoundW Pmulti d ∧ UsableW Pmulti d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Priority-typed session (rung-(b)) shape — built to *confirm* #25's weaker thesis, but it **BREAKS**: the
    *eighth* seam to break, and the **session face** of `ordered_locks` #15's *lock* face (same shape, one
    soundness sub-domain over). Read `Pr`/`Wt` by analogy: the precondition is *deadlock-freedom of a
    priority-typed assembly* and the witness is a per-channel const bound `Pi: Lt<Po>` (Kobayashi/Dardha–Gay) on
    `typed_nat`'s sealed `Lt`. In the kernel the typed discipline factors (`Pprio` is definitionally `V`), as
    always.

    But the cold panel (REPORT_SEAM_SPEC_priority, r33) ruled **SEAM-BREAKS [horn 1]** — the re-import rule's own
    §6 horn-1 falsifier ("a typed cyclic assembly that deadlocks") is met by **safe code**. The §3 untypability
    argument (the cyclic chain collapses to `p: Lt<p>`) silently assumes **edge-priority consistency** — that the
    `Nat` A *sends* on edge AB equals the one B *receives* on AB — which the proposal itself concedes is "the
    constructor's invariant, **not a per-endpoint type fact**." Rust does no transitive reasoning; it checks each
    `Pi: Lt<Po>` in isolation. So the public `role::<Pi, Po>` constructor lets three `role::<N1, N2>` relays
    (every bound `N1: Lt<N2>`, 1 < 2, discharges) wire into the ring `A→B→C→A`: it **compiles, no `unsafe`, and
    deadlocks**. Deadlock-freedom therefore rests **entirely on the trusted `relay()` constructor** threading one
    `Nat` per edge (a mini-projection in the TCB), not on the `Lt` discipline — so §4.3's "deadlock-freedom of
    *any* well-typed assembly" is unsound as written.

    This is the same break as `ordered_locks` #15/#16, one domain over: a type-level order carries the **local**
    bound (per-token rank there; per-role priority here) while the **whole-program** consistency/acyclicity
    escapes the types (the wrong-granularity gap). It thus **confirms** #25's weaker thesis — global progress
    needs strictly more than the per-channel floor; the local discipline alone carries nothing global — but by
    BREAKING (the advertised guarantee is false) rather than merely costing. The contribution salvages to
    SEAM-COSTS [horn 3] **only** if `role()` is hidden and roles are minted solely by a trusted
    edge-consistent constructor (`relay`). `..._factors` (not `..._closes`) certifies only the trivial typed
    sub-fact, as `ordered_locks`/`secret`/`labeled` do; the kernel sees none of the liveness or the
    edge-consistency gap. No new domain (liveness), coordinate (progress, from #24), or mode (SEAM-BREAKS
    [horn 1] is one of the nine); the broken-seam tally rises to 8. -/
def Pprio (p : Prog) : Bool := V p

/-- Trivial sub-fact only (see the docstring): the *typed* per-channel priority discipline factors. The real
    seam BREAKS [horn 1] — the public `role::<Pi, Po>` constructor accepts a fully priority-typed cyclic
    assembly (three `role::<N1, N2>` relays) that compiles and deadlocks, because edge-priority consistency is a
    trusted-constructor invariant, not a per-endpoint type fact. The session face of `ordered_locks` #15's lock
    face; confirms #25's weaker thesis by breaking, not costing. -/
theorem priority_typed_discipline_factors :
    ∃ d : Prog → Bool, SoundW Pprio d ∧ UsableW Pprio d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Projected session (schedulability by construction) shape — the **#26 salvage**, SOUND, and the seam that
    CLOSES where `priority` #26 broke. Read `Pr`/`Wt` by analogy: the precondition is *schedulability of the
    assembly* (a terminating schedule EXISTS) and the witness is a trusted, edge-consistent, monotone-priority
    builder over #23's edges and #8's `Lt` (`Pproj` is definitionally `V`). #26 broke because its per-role
    constructor `role()` was **public**, so a caller wired locally-valid roles into a deadlocking ring. This seam
    **seals** the per-role constructor (`worker()` is module-private) and vends only trusted builders (`relay`,
    `relay4`) that thread strictly-increasing priorities. Two mechanisms make the danger unconstructible: (1)
    access — the #26 exploit is uncompilable (no public `worker()`); (2) structure — a monotone chain cannot close
    into a ring (the closing edge needs a decreasing `Lt` step, no instance). So the sound schedulability core
    CLOSES — the #26 break is genuinely resolved.

    The cold panel (REPORT_SEAM_SPEC_projected, r34): the horn-classifier ruled **SEAM-COSTS [horn 3]** (SOUND —
    horn 1 conceded: the ring is unconstructible, no role inputs, no decreasing `Lt`; horn 2 Θ(depth)), but the
    JUDGE ruled **FAILED on the §6 DELIVERABLE** — the **rejection mode, 4th firing** (after `typed_nat` #8,
    `delegation` #24, `multiparty` #25; #26 was a break). My §4 overclaimed: "deadlock-freedom by construction"
    and "the consumer carries nothing global." The genuine price (attacker-soundness): the monotone-priority
    builder proves a deadlock-free schedule *exists*, but affine Rust **never forces the runtime to follow it**,
    so a **progress residue** stays on the untrusted caller — *run-all, in-order, no-drop*. So
    "deadlock-freedom by construction" downgrades to "**schedulability by construction, given all roles
    execute**"; concentration is **partial**: the constructor carries the **topology** obligation (no cycle — the
    #26 fix), but the **completion/execution** obligation never left the consumer (the #17/#18/#23 horn-3
    completion gap). The giveaway: Scribble/`mpstthree` endpoints are deadlock-free *only* under the assumption
    every participant executes its endpoint — I imported that ("single-threaded witness") while claiming the
    consumer carries nothing; the execution assumption IS the leftover global obligation. (Secondary deflation:
    the expressiveness cost was *overstated* — acyclic generality is recoverable via one generic trusted
    combinator, as `mpstthree` proves.) The refined finding: the **progress coordinate decomposes** —
    *deadlock-freedom/topology* (the trusted constructor can carry) vs *completion/execution* (the consumer always
    carries). Sound, so the broken-seam tally STAYS 8; N stays 9 (rejection mode). Like every column past
    concentration, the cost is modeling-dictionary; the kernel sees only that the trusted-builder witness
    suffices for schedulability. -/
def Pproj (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn 3]; the seam that closes where #26 broke): the *schedulability*-by-
    construction core closes via the sealed per-role constructor + the trusted monotone-priority builders — the
    #26 exploit is uncompilable and a monotone chain cannot close into a ring, so a terminating schedule provably
    EXISTS. The uncarried components — that the consumer must still *follow* that schedule (run-all/in-order/
    no-drop: the progress/completion residue affine Rust never forces) and that concentration is therefore
    *partial*, not total — live in the prose, invisible to the kernel. The first `..._closes` since #25; SOUND
    like `typed_nat`/`delegation`/`multiparty` (their deliverables rejected too). It resolves the #26 break's
    topology half while confirming the completion half never left the consumer. -/
theorem projected_schedulability_closes :
    ∃ d : Prog → Bool, SoundW Pproj d ∧ UsableW Pproj d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Choice (offer/select, `&`/`⊕`) shape — the *branching* connective, the last unprobed core session-type
    feature. Read `Pr`/`Wt` by analogy: the kernel models the component the seam CLOSES — the *order + branch-
    typing* core (`Pchoice` is definitionally `V`): each `Branch<L, R>` variant carries the correctly-typed
    continuation (label–continuation correlation — can't take one label and continue as the other), and `HasDual`
    threads choice so the selector's options are exactly the offerer's branches (branch-set duality). It CLOSES;
    soundness is intact (no UB — the worst case is a starved/leaked memory-safe session).

    The cold panel (REPORT_SEAM_SPEC_choice, r35) ruled **SEAM-COSTS [horn3-concentration]** (horn-classifier),
    and the JUDGE ruled **FAILED on the §6 DELIVERABLE** — the method's **REJECTION mode, 5th firing** (after
    `typed_nat` #8, `delegation` #24, `multiparty` #25, `projected` #27; all sound + rejected). My §4 over-claimed
    concentration as TOTAL again — the arc's recurring trap, a 5th time — and the panel imposed FOUR corrections:
    (1) the §4 rule's four guarantees reduce to **two-legged branch-safety** (label–continuation correlation +
    branch-set duality); the false one is **#2 "handle every label"** — Rust exhaustiveness gives *scrutinee
    coverage, not run-to-`End`*, so a safe `match offer() { Ok(Branch::Left(c)) => .., _ => {} }` type-checks, no
    `unsafe`, and **drops** `Chan<R>`, starving that branch (order is the generic typestate leg since #18, not
    branch-specific). (2) The deflation is onto **binary `Either`**, NOT n-ary *named* sum types — labeled n-ary
    choice needs nesting that drops label names, or `offer!`/`choose!` macros. (3) Horn 2 is **Θ(type-size)**, not
    Θ(depth) (a balanced `Offer` tree has 2^depth leaves; interning keeps it linear-in-source, so costless but
    mislabeled). (4) Completion is **amplified** per-branch and **peer-choice-reachable** (the dual's `select`
    triggers it), not "unchanged by branching." The core thesis survives, sharpened: choice adds **no new
    coordinate** (the branching connective opens no new ground); **completion stays the residue** — now fed by
    **two** safe paths (`mem::forget` AND the dropped wildcard branch), the #17/#18/#23/#27 gap. No new domain
    (liveness), coordinate (progress, from #24), or mode (N stays 9); the seam is sound, so the broken-seam tally
    stays 8. Prior art CONFIRMS (Honda 1993's `&`/`⊕`; the `session_types`/Jespersen–Munksgaard–Larsen 2015
    affine-vs-linear gap IS this completion residue). Like every column past concentration, the horn-3 completion
    cost is modeling-dictionary; the kernel sees only that the order + branch-typing witness suffices. -/
def Pchoice (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn3-concentration]; the gauntlet FAILED on the *deliverable*, not
    soundness): only the *order + branch-TYPING* core closes via the floor's sum types — each `Branch<L, R>`
    variant pins the correctly-typed continuation (label–continuation correlation), with `HasDual` threading the
    branch set (branch-set duality). The renamed theorem (`..._branch_typing_closes`, from `..._branch_safety_`)
    records the r35 correction: **"handle every label" does NOT close** — exhaustiveness gives scrutinee coverage,
    not run-to-`End`, so a safe wildcard `match` drops a branch. That uncarried component IS *completion* (the
    #17/#18/#23/#27 residue, now fed by `mem::forget` AND the dropped branch) — it lives in the prose, invisible to
    the kernel. Choice carries branch-*typing*, not branch-*completion*; the branching connective adds no new
    coordinate, deflating onto the floor's sum types. -/
theorem choice_branch_typing_closes :
    ∃ d : Prog → Bool, SoundW Pchoice d ∧ UsableW Pchoice d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Must-complete-session shape — the *completion-residue* probe (a `Session<P>` affine handle with a `Drop`
    completion bomb), built to ask whether the affine floor can *compel* run-to-`End`. Read `Pr`/`Wt` by analogy:
    the precondition is *the session runs to `End`* (completion). In the kernel the typed discipline factors
    (`Pmustcomplete` is definitionally `V`) — the affine *walk* (send/recv advance only at the matching state) is
    the part the floor carries for free, as always.

    The cold panel (REPORT_SEAM_SPEC_must_complete, r36) ruled **SEAM-COSTS [horn3-concentration]**
    (horn-classifier) and the JUDGE ruled **FAILED on the §6 DELIVERABLE** — the **rejection mode, 6th firing**,
    and the most instructive: it **reverses** my finding rather than relabelling it. The free-handle `Drop`-bomb IS
    bypassable (`mem::forget` skips it, peer starved — self-confirmed). But my "universal NONE" was **REFUTED**: a
    **scoped-closure / CPS combinator** — `with_session(|s| { …; s.finish() })`, the **`std::thread::scope`
    analogue** — **compels completion modulo divergence, no `unsafe`**, by moving the obligation to the re-import
    boundary so the consumer never holds a forgettable handle ("return `End` or diverge"). STING (6th refutation
    from my own lineage): I cited `thread::scoped`'s removal (RFC 1084) but ignored its fix, `thread::scope`
    (stable 1.63). So completion is **NOT an irreducible linear gap — it is CONCENTRABLE** via the scoped witness,
    at a horn-3 **API-locality** cost (surrender the free movable handle): the whole #17–#28 completion residue was
    the price of the **free-handle API**, not an inherent gap. And sessions ≠ `linear` #17's cleanup — a
    *delimited* session CAN be scoped to force completion, an *open-ended* cleanup cannot (refuting my "session
    face of #17" and claim (a)). META: the refutation **inverts** my usual error — across the rejections I
    overclaimed concentration is *total*; here I overclaimed it is *irreducible*; the truth is a cost between. No
    new domain (liveness), coordinate (completion), or mode (N stays 9); the danger is concentrable, so the
    broken-seam tally STAYS 8 (SEAM-COSTS, not the BREAK I predicted). Like every column past concentration, the
    API-locality cost is modeling-dictionary; the kernel sees only that the scoped witness suffices. -/
def Pmustcomplete (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn3-concentration]; the gauntlet FAILED the *deliverable*, not soundness):
    completion *does* close — through a **scoped-closure / CPS** witness (`with_session`, the `thread::scope`
    analogue) that moves the obligation to the re-import boundary, exactly as a holding seam closes in the kernel
    (like `session`/`projected`). The renamed theorem (`..._scoped_completion_closes`, from
    `..._typed_discipline_factors`) records the r36 reversal: the **free-handle** witness only *factored* (the bomb
    is bypassable by `mem::forget`), but the **scoped** witness *closes* completion at an API-locality cost the
    kernel cannot see. So completion is concentrable, not the irreducible NONE I staked; the #17–#28 residue was
    the free-handle API's price, and the danger is COST, not a break (tally stays 8). -/
theorem must_complete_scoped_completion_closes :
    ∃ d : Prog → Bool, SoundW Pmustcomplete d ∧ UsableW Pmustcomplete d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Scoped-session shape — the **thirtieth** seam, the concrete **salvage of #29** (mirroring #26→#27), and (under
    the pre-registered HYPOTHESIS) a *holding* seam. Read `Pr`/`Wt` by analogy: the precondition is again
    *completion* (the session runs to `End`), but the witness is now the **scoped-closure combinator** #29's r36
    reversal named — `session_scope`/`with_session`, the `std::thread::scope` analogue. In the kernel the discipline
    factors (`Pmustscoped` is definitionally `V`), as always.

    The point #29 conceded and this seam discharges: #29's **free-handle** `Drop`-bomb only *factored* (it is
    `mem::forget`-able), but a **scoped** witness that moves the completion obligation to the re-import boundary —
    where the consumer never holds a forgettable handle, and the only normal exit is a `Done` certificate mintable
    solely by `finish` at `End` — *closes* completion. The destructor obligation (forgettable) becomes a
    return-type obligation (not forgettable); the invariant `'scope` brand (branded #5's generativity, reused on
    purpose) traps the session in the closure, which **is** the horn-3 **API-locality** cost.

    The cold panel (REPORT_SEAM_SPEC_must_scoped, r37) ruled **SEAM-COSTS [horn3-concentration]** and the JUDGE
    ruled **TRADEOFF — HELD (survival 1/1)**, like `restore` #19 / `binary_session` #23 (NOT the rejection mode #29
    was). **This closes the #29 reversal as predicted:** every attacker conceded **no safe `Done` is forged,
    smuggled, or forgotten** (the return-type obligation defangs `mem::forget`), so completion *is* concentrable —
    the #17–#28 residue was the price of the free-handle API, recoverable by surrendering the free handle. Horn 1
    (no UB) and Horn 2 (Θ(depth), no SMT) intact. The panel **sharpened** it (merits-strongest, soundness):
    concentration is **per-endpoint, not whole-session** — `session_scope` forces only each *returning* closure's
    own endpoint, so under run-`a`-then-`b` a finishes while a diverging peer is starved, and **global completion
    escapes via peer divergence** (the #23/#27 per-session-vs-global gap, one step on). The headline closes
    *per-endpoint*; the *global* obligation stays uncarried, exactly the caveat the builder hedged. A `..._closes`
    (like `session`/`projected`/`must_complete`'s scoped reading): the scoped witness suffices to close completion
    per-endpoint; the API-locality + synchronous-trap cost, and the global escape, are modeling-dictionary the
    kernel cannot see. Tally stays 8 (this HELD); N stays 9. -/
def Pmustscoped (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn3-concentration]; the gauntlet ruled TRADEOFF/HELD, not a break):
    completion closes **per-endpoint** through the **scoped-closure** witness (`session_scope`, the `thread::scope`
    analogue) — the consumer never holds a forgettable handle, the only normal exit is a `finish`-minted `Done`, so
    the (local) completion obligation is carried at an API-locality cost the kernel cannot see. The concrete witness
    the #29 reversal promised; it closes the #29 finding that completion is concentrable, not the irreducible NONE
    the free-handle bomb suggested — while the *global* (all-endpoints) obligation escapes via peer divergence,
    living in the prose, invisible to the kernel. -/
theorem must_scoped_completion_closes :
    ∃ d : Prog → Bool, SoundW Pmustscoped d ∧ UsableW Pmustscoped d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Recursive-session shape — the **thirty-first** seam, the last unbuilt structural session feature
    (`Rec<P>`/`Var`, `μX.T`). Read `Pr`/`Wt` by analogy: the precondition that the floor *does* carry is
    *order-through-unfolding* (each `enter`/`recur` advances the affine typestate soundly, the loop variable `Var`
    kept sound by a type-level recursion environment so an unbound `Var` is inert) together with duality
    (`Rec<P> ↔ Rec<P::Dual>`). In the kernel the discipline factors (`Precursion` is definitionally `V`), as always.

    The cold panel (REPORT_SEAM_SPEC_recursion, r38) ruled **SEAM-COSTS [horn3-concentration]** and the JUDGE ruled
    **FAILED on the §6 DELIVERABLE** — the **rejection mode, 7th firing** (after `typed_nat` #8, `delegation` #24,
    `multiparty` #25, `projected` #27, `choice` #28, `must_complete` #29). Sound, so the broken-seam tally STAYS 8.
    What HELD: Horn 1 (the `Var`/environment encoding is sound — no safe ill-formed unfolding, worst case a starved
    peer, not UB), Horn 2 (decidable, Θ(type-size)), and the #30-bounding finding (`Rec<Tx<Var>>` has no reachable
    `End`, so #30's `Done`-forcing is VACUOUS — recursion exposes #30's unstated precondition, a reachable terminal).
    THE REJECTION: **I OVER-CONCEDED.** I filed **productivity/guardedness** as an undecidable NONE alongside
    termination — but it is a **decidable cost**. `Rec<Var>` = `µX.X` is the **non-contractive** type every system I
    cited (Coq/Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**; a sealed
    `Guarded` marker (impl for `Tx`/`Rx`/`End`/`Rec`, **not** `Var`) bounding `enter`/`Rec`-formation carries it in
    Θ(type-size), no SMT — the same machinery as order. So guardedness CONCENTRATES (horn 3); **only termination is
    the genuine undecidable NONE** (halting). Refuted from my own cited lineage again (#24/#25/#29). META — the
    *inverse* of the arc's usual over-claim, same shape: I fled the middle (a decidable, partial cost) toward the
    *undecidable* extreme, as the rejections fled toward *total* concentration. Corrected: order + duality +
    **guardedness** all close; termination alone is the NONE. So the order/duality/guardedness core closes
    (`..._closes`, like `session`/`projected`/`must_scoped`); the termination NONE and the #30-vacuity finding live
    in the prose, invisible to the kernel. Tally stays 8 (sound — a deliverable rejection, not a soundness break);
    N stays 9. The natural #32 salvage: build the sealed `Guarded` marker so `Rec<Var>` will not form. -/
def Precursion (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [horn3-concentration]; the gauntlet FAILED the *deliverable*, not soundness):
    order-through-unfolding, duality, AND **guardedness** close per-unfolding through the affine typestate
    (`enter`/`recur` advance soundly; `Var` is sound by the recursion-environment discipline; and — the r38
    correction — guardedness is *decidable*, a sealed `Guarded` marker rejecting the non-contractive `Rec<Var>` in
    Θ(type-size), the same machinery as order, NOT the undecidable NONE I over-conceded). Only **termination** is the
    genuine undecidable NONE (halting), and recursion bounds #30's completion-forcing (it needs a reachable `End`) —
    both live in the prose, invisible to the kernel. The renamed theorem (`recursion_guarded_order_closes`, from
    `recursion_order_unfolding_closes`) records that guardedness joined the closing core. -/
theorem recursion_guarded_order_closes :
    ∃ d : Prog → Bool, SoundW Precursion d ∧ UsableW Precursion d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

/-- Guarded-recursion shape — the **thirty-second** seam, the **salvage of #31** (`recursion`), mirroring
    #26→#27 and #29→#30. Read `Pr`/`Wt` by analogy: the precondition the floor now *carries* is **guardedness /
    productivity** (every recursion `Var` sits under a communication action — no non-contractive `µX.X` spin),
    on top of #31's order + duality. In the kernel the discipline factors (`Pguarded` is definitionally `V`).

    The cold panel (REPORT_SEAM_SPEC_guarded, r39) ruled **SEAM-COSTS [none]** and the JUDGE ruled **SURVIVED
    (1/1)** — the salvage HELD, and `[none]` is the cleanest result in the arc (all three horns hold, no horn
    dropped, stronger than #30's `[horn3]`). The over-concession #31 was rejected for is **discharged**:
    guardedness/productivity is concentrable, the decidable cost #31 owed — a sealed `Guarded` marker (impl for
    `Tx`/`Rx`/`End`, not `Var`) bounds `enter`, so the unguarded `Rec<Var>` cannot be unfolded, rejected in
    Θ(type-size), no SMT, the *same machinery as order*. And — the phase's point — I avoided the #27
    salvage-over-claim trap by claiming only productivity and conceding termination up front. What the panel MARKED
    (tradeoffs, NOT horn-drops, so the seam still survived): **completeness is bought by restriction** — the shallow
    head-check equals full contractivity *only because* the single-level, choice-free grammar collapses it to
    "head ≠ `Var`"; a richer grammar (nested `Rec`, or choice-in-recursion) needs the real recursive contractivity
    walk, which this seam *scopes out, not escapes*; and my "folklore" deflation slightly overstated generality. The
    marker buys **productivity**, NOT **termination** (`Rec<Tx<Var>>` is guarded and still infinite — the halting
    problem). Final split: order + duality + guardedness/productivity all CLOSE (for the single-level fragment);
    termination alone is the conceded undecidable NONE. A `..._closes` (the guarded core closes, like
    `session`/`projected`/`must_scoped`); the restriction caveat and termination live in the prose, invisible to the
    kernel. Tally stays 8 (this SURVIVED); N stays 9. -/
def Pguarded (p : Prog) : Bool := V p

/-- The sound core CLOSES (SEAM-COSTS [none], SURVIVED — the salvage held, all three horns intact):
    guardedness/productivity concentrates through the sealed `Guarded` marker (rejecting the non-contractive
    `Rec<Var>` at the unfolding boundary in Θ(type-size), no SMT — the same trait-resolution machinery as order),
    discharging the decidable cost #31 was rejected for over-conceding. Order + duality + guardedness all close (for
    the single-level fragment — completeness bought by restriction, a marked tradeoff not a horn-drop); only
    termination stays the conceded undecidable NONE (the halting problem), living in the prose, invisible to the
    kernel. -/
theorem guarded_recursion_productivity_closes :
    ∃ d : Prog → Bool, SoundW Pguarded d ∧ UsableW Pguarded d ∧ ConcW V d := by
  rw [seam_closes_iff_factors]
  exact ⟨id, fun _ => rfl⟩

end SeamGap
