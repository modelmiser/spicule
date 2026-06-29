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

end SeamGap
