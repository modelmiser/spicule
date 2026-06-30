//! # Twenty-second seam — a cost-typed computation: the WCET / real-time fourth-domain probe.
//!
//! *(Project ordinal: the twenty-second primitive, and the **second** probe aimed at a fourth soundness
//! domain — after the convergence probe (`crdt`) *deflated*. This one targets **real-time / worst-case
//! execution time (WCET)**: the danger is a **deadline miss** — a computation that should complete within a
//! cycle budget runs too long. That is neither UB, nor a leak, nor a deadlock (the program makes progress,
//! just too slowly), nor a timing side-channel (that is information-flow — leaking *via* time, not *missing a
//! deadline*), nor a consistency violation. Real-time is a cleaner fourth-domain candidate than convergence
//! because it is **not an algebraic law**: a cycle budget is a **quantitative resource bound**, the kind of
//! thing Rust's affine/ownership floor most plainly has no term for.)*
//!
//! Same floor, same rules — and, as with `crdt`/`secret`, there is **no `unsafe`**: the danger is the
//! **semantic** mismatch between a static cost and real elapsed time.
//!
//! ## The primitive — a computation whose cost is tracked in the type
//!
//! A [`Timed<C, T>`] is a value of type `T` produced by a computation whose worst-case cost is the
//! type-level numeral `C`, in a **"one unit per [`Timed::step`]"** cost model. [`pure`] is cost `0`; each
//! `step` charges one unit and advances `C` to its successor; [`Timed::within`] discharges a **deadline**
//! `D`, returning the value **only if** the static cost is within budget (`C ≤ D`, a compile-time proof).
//! The type-level arithmetic is the *exact* sealed `Nat`/`Lt` machinery of the eighth seam (`typed_nat`) —
//! reused on purpose: see the verdict.
//!
//! ## Why the floor MISMATCHES this domain — the op-count is not the time
//!
//! The floor **can** carry the static op-count: `C` accumulates by trait resolution and `within` checks
//! `C ≤ D` at compile time, so an over-budget straight-line computation **fails to type-check** (see the
//! doctests). But the op-count is a **fiction for real time**, in three ways, and that is the whole probe:
//! 1. **A `step` may wrap arbitrarily expensive work.** The model charges one unit per `step`, but the
//!    closure is unconstrained — a single `step` can run a million-iteration loop. The op-count says `1`;
//!    the real cost is enormous. *Demonstrated in `tests/wcet_use.rs` with a counter, no wall-clock needed.*
//! 2. **Even one primitive is not one cycle.** A `u64` division, a dependent load that misses cache, a
//!    mispredicted branch — each is tens to hundreds of cycles. Real WCET needs a **hardware** model
//!    (cache, pipeline, memory contention) the source-level op-count cannot see.
//! 3. **Data-dependent control flow is inexpressible.** `while n > 0 { … }` has no static `C` (the count is
//!    a runtime value), so it falls *out* of the type-level accounting entirely — a partial-match gap.
//!
//! So the hypothesis the spec (`design_seam/SEAM_SPEC_wcet.md`) puts to the panel: real-time/WCET does **not**
//! get a closing seam — **SEAM-BREAKS [floor-mismatch]** — because the floor carries a **proxy that is not the
//! property** (op-count, not cycles), and the cost-model axiom is **unsound** (a fiction), *weaker* than
//! `mmio`'s faithful hardware axiom. And the open deliverable: does WCET **deflate** like convergence — its
//! static fragment onto `typed_nat`'s type-level resource accounting (#8), its trusted cost-model onto an
//! `mmio`-shaped-but-unsound axiom (#2) — rather than opening a genuine fourth domain?
//!
//! ### The floor carries the static op-count — over-budget straight-line code fails to compile:
//! ```compile_fail
//! # use seam_spsc::wcet::{pure, Timed};
//! # use seam_spsc::typed_nat::{S, Z};
//! type N2 = S<S<Z>>;
//! let c = pure(()).step(|_| ()).step(|_| ()).step(|_| ()); // cost 3
//! let _ = c.within::<N2>(); // ERROR: `S<S<S<Z>>>: Lt<S<N2>>` unsatisfied — 3 > 2, over budget
//! ```
//! ```compile_fail
//! # use seam_spsc::wcet::pure;
//! let c = pure(7i32).step(|x| x + 1);
//! let _v = c.value; // ERROR: field `value` is private — the only way out is `within`, which proves a deadline
//! ```
//! ```compile_fail
//! # use seam_spsc::typed_nat::Nat;
//! struct Evil;
//! impl Nat for Evil { const VALUE: usize = 0; } // ERROR: `Evil: Sealed` — a forged zero-cost numeral is rejected
//! ```
//!
//! ### The static accounting discharges for in-budget straight-line code (it compiles and runs):
//! ```
//! # use seam_spsc::wcet::pure;
//! # use seam_spsc::typed_nat::{S, Z};
//! type N3 = S<S<S<Z>>>;
//! let r = pure(2i32).step(|x| x + 1).step(|x| x * 10).within::<N3>(); // cost 2 ≤ deadline 3
//! assert_eq!(r, 30);
//! ```
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_wcet, r29): SEAM-BREAKS [horn 1], FAILED — the SEVENTH
//! ## broken seam (and the second fourth-domain candidate to fail to open a fourth domain).
//!
//! **The seam FAILED** (0/1) — all five attackers ruled BREAKS, from five distinct angles. Unlike the
//! convergence probe (`crdt`), which *deflated and survived* (its break pre-registered), this one **FAILED**,
//! because my §4 re-import rule **over-claimed**: it promised `within` returns the value "only when the
//! computation meets its deadline," but the type tracks the **op-count**, not the time, so the guarantee is
//! falsified in safe code. The honest verdict is the break I predicted (floor-mismatch, unsound proxy) plus a
//! spec defect (the over-claim) and richer corrections:
//!
//! 1. **Soundness (fatal) — the op-count is no upper bound on cycles.** `step` advances `C := S<C>`
//!    *structurally, decoupled from the closure*, so a million-iteration loop is cost `1` and `within::<1>`
//!    returns it through the only sanctioned exit. `C ≤ D` proves nothing about real elapsed time. The break
//!    is structural: `pure(()).step(|_| for i in 0..1_000_000 { black_box(i); }).within::<S<Z>>()`.
//! 2. **Ergonomics — Horn 3 is FALSE.** An `if` whose two arms do different work yields **non-unifying**
//!    `Timed<S<C>>` vs `Timed<S<S<S<C>>>>` types, forcing the programmer to hand-pad branches with dummy
//!    `step`s. "The consumer writes no cost bookkeeping" is refuted — the cost index is *heavy* bookkeeping.
//! 3. **Implementation — the unary cost does not scale.** `C` is unary Peano (the *exact* `typed_nat` #8
//!    machinery), so the result type and `Lt` resolution are **linear in the cycle count**: honest per-cycle
//!    stepping is **uncompilable** at any realistic 10³–10⁷-cycle budget (recursion-limit / O(C) type
//!    blow-up). The only compilable path — bundling many units into one `step` — *is* the soundness break.
//! 4. **Expressiveness — no additive composition.** The seam exposes only `pure(0)` and `step(+1)`, with no
//!    **type-level addition**, so two cost-typed computations cannot be sequenced and summed. Every program
//!    is one flat literal step-chain; factoring real code forces folding work into a lying single `step`.
//! 5. **Prior-art — this is Danielsson's 2008 cost monad, and the gap is WCET's *founding premise*.** The
//!    primitive is the graded/indexed cost monad (Danielsson, POPL 2008). "op-count ≠ cycles" is **not a
//!    finding** — it is the premise WCET analysis *exists to close*. Every cost-indexed precedent (Danielsson,
//!    RaML / amortized resource analysis, gas/fuel metering, aiT) achieved soundness only by **abandoning the
//!    physical-cycle claim** or **installing a defining/microarchitectural machine**. The seam keeps the
//!    physical-time claim while reusing their machinery — exactly the unsound gap. **Pick a lane:** *abstract
//!    cost* (sound, but it is not real time) **xor** *physical deadline* (needs a defining machine the floor
//!    does not provide).
//!
//! **The refined finding — WCET does not open a fourth domain either (the second candidate to fail).** It
//! **deflates** multiply: the static cost monad onto Danielsson 2008 / `typed_nat`'s type-level `Nat` (#8 —
//! which does not even *scale* to real budgets); the *physical-deadline* claim onto an `mmio`-shaped axiom
//! (#2) that this seam **never installs** (so it is *worse* than MMIO — MMIO's hardware axiom is faithful and
//! present; here the cost model is a fiction and absent); and the data-dependent-loop gap onto the liveness
//! domain's **partial-match** (a `while` has no static `C`). So the floor carries a **proxy that is not the
//! property**, and the proxy is **unsound** — *sharper* than `secret`, whose proxy foreclosed *some* leaks
//! (sound but incomplete); the op-count can be **arbitrarily far** from the truth.
//!
//! **The meta-finding (for the record).** **Two** fourth-domain candidates have now been probed — convergence
//! (`crdt`, deflated + survived) and real-time/WCET (`wcet`, deflated + FAILED) — and **neither opens a fourth
//! soundness domain**, by *different* mechanisms (unchecked algebraic law → application-correctness; unsound
//! physical-time proxy → cost-monad + missing machine). The three confirmed domains (memory safety /
//! information flow / liveness) look increasingly **special**. N stays **9** verdict modes (floor-mismatch +
//! projects-onto-basis, both already on the board); 22 primitives; the FAILED broken-seam tally rises to **7**.
//!
//! The static accounting *does* discharge in-budget, and the op-count *does* diverge from real work — see
//! `tests/wcet_use.rs`.

use crate::typed_nat::{Lt, Nat, S, Z};
use core::marker::PhantomData;

/// A value of type `T` produced by a computation whose worst-case cost is the type-level numeral `C`, in a
/// **"one unit per [`step`](Timed::step)"** cost model. The model is a **fiction**: a `step` may wrap
/// arbitrarily expensive work, and real cycles depend on hardware the op-count cannot see (see the module
/// docs). The value is reachable **only** through [`within`](Timed::within), which proves a deadline.
pub struct Timed<C, T> {
    value: T,
    _c: PhantomData<C>,
}

/// A constant — cost `0`.
#[inline]
pub fn pure<T>(value: T) -> Timed<Z, T> {
    Timed { value, _c: PhantomData }
}

impl<C: Nat, T> Timed<C, T> {
    /// Charge **one unit** of cost for `f` and advance the type-level cost from `C` to `S<C>`. The model
    /// *trusts* that `f` is one unit of work — the unenforceable assumption the probe is about: nothing
    /// stops `f` from running a million iterations or a cache-missing division, all charged as one unit.
    #[inline]
    pub fn step<U>(self, f: impl FnOnce(T) -> U) -> Timed<S<C>, U> {
        Timed { value: f(self.value), _c: PhantomData }
    }

    /// Discharge a **deadline** `D`: extract the value, but **only if** the static cost is within budget —
    /// the bound `C: Lt<S<D>>` is the compile-time proof that `C ≤ D` (`C < D+1`). Over-budget straight-line
    /// code fails to type-check here. (Sound *for the op-count model*; the model is the fiction.)
    #[inline]
    pub fn within<D: Nat>(self) -> T
    where
        C: Lt<S<D>>,
    {
        self.value
    }

    /// The static op-count (`C::VALUE`) — what the model *claims* the cost is. Real elapsed cycles may be
    /// arbitrarily larger; `tests/wcet_use.rs` exhibits a cost-`1` computation that does a million units of
    /// real work.
    #[inline]
    pub fn cost(&self) -> usize {
        C::VALUE
    }
}
