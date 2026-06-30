//! # Twenty-eighth seam — CHOICE (offer/select, `&`/`⊕`): the branching connective.
//!
//! *(Project ordinal: the twenty-eighth primitive. The last unprobed **core** session-type feature. Every prior
//! session seam (#18, #23–#27) probed the *sequencing / progress* axis — order, delegation, n-party topology,
//! priority, projection. Choice is the orthogonal **branching** connective: **external choice** (`offer`, `&`) —
//! receive a label and branch into a different continuation per label — and **internal choice** (`select`, `⊕`)
//! — send a label, committing to one branch. NOT a fourth-domain bid; it stays in the **third confirmed domain
//! (liveness)** and tests whether the branching connective opens any new ground or simply deflates onto the
//! floor's sum types.)*
//!
//! ## The primitive — labeled branching over a shared wire
//!
//! Self-contained (like `delegation`/`multiparty`'s session machinery — the new methods cannot be added to
//! #23's `Chan` from outside, its transport field is private), over `Rc<RefCell<Transport>>` dual endpoints
//! (#23's two-live-endpoint model). Two new markers extend `Tx`/`Rx`/`End`:
//! - [`Offer`]`<L, R>` — **external choice**: this endpoint *receives* a choice; it must handle **both**
//!   branches. [`Chan::offer`] returns a [`Branch`]`<L, R>` sum the consumer **must `match`**.
//! - [`Select`]`<L, R>` — **internal choice**: this endpoint *sends* a choice. [`Chan::select_left`] →
//!   `Chan<L>`, [`Chan::select_right`] → `Chan<R>`.
//!
//! [`HasDual`] threads choice: **`Offer<L, R>` ↔ `Select<L::Dual, R::Dual>`** — the selector's two options are
//! *exactly* the offerer's two branches, dualized. So one side cannot offer a branch set the other side cannot
//! select, nor select an option the other side does not offer.
//!
//! ## The branch-safety facts — the floor carries branch *typing*, NOT branch *completion* (r35 correction)
//!
//! 1. **Label–continuation correlation.** Each [`Branch`] variant carries the *correctly-typed* continuation
//!    (`Chan<L>` in `Left`, `Chan<R>` in `Right`), so you cannot take the left label and continue as protocol
//!    `R`. The sum type ties the tag to the session state. (Carried — concentrates in the types.)
//! 2. **Handle every label — OVERSTATED (the §6 break).** [`Chan::offer`] yields a [`Branch`] enum and an
//!    exhaustive `match` must *cover* both variants — but coverage is **scrutinee coverage, not run-to-`End`**:
//!    `match offer() { Ok(Branch::Left(c)) => .., _ => {} }` type-checks, holds no `unsafe`, and **drops**
//!    `Chan<R>`, starving that branch. "Handle-all" is therefore **not** a guarantee separate from completion —
//!    it collapses into the #17/#18/#23/#27 completion residue. (NOT carried — stays on the consumer.)
//!
//! ### Duality is carried — the selector's options are exactly the offerer's branches:
//! ```compile_fail
//! # use seam_spsc::choice::{session, Offer, Select, Tx, Rx, End, Chan};
//! // Offer<Rx<End>, End>'s dual is Select<Tx<End>, End>, not Select<Rx<End>, End>:
//! let (_o, _s): (Chan<Offer<Rx<End>, End>>, Chan<Select<Rx<End>, End>>) =
//!     session::<Offer<Rx<End>, End>>(); // ERROR: dual is Select<Tx<End>, End>
//! ```
//!
//! ### Offer + select run; abandoning a branch mid-protocol starves the peer — see `tests/choice_use.rs`.
//!
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_choice.md, r35): SEAM-COSTS [horn3-concentration], gauntlet FAILED
//!
//! Horn-classifier: **SEAM-COSTS [horn3-concentration]** — *"Sound and decidable, but 'handle-all' isn't
//! type-enforced — it collapses into the caller's completion residue."* Judge: **FAILED** on the §6 deliverable —
//! *"`_ => drop(continuation)` is exhaustive, unsafe-free, yet starves a branch — so #2 ('must handle both') is
//! overstated and collapses."* This is the method's **REJECTION mode, firing the 5th time** (after `typed_nat`
//! #8, `delegation` #24, `multiparty` #25, `projected` #27 — all sound, all deliverable-rejected).
//!
//! What survives, and the four corrections the panel imposed:
//! - **Soundness intact** (no UB — the worst case is a starved/leaked memory-safe session); horns 1 and 2 hold,
//!   the cost is **horn 3**. The break is the §4 over-claim of concentration as TOTAL — the arc's recurring trap,
//!   a 5th time.
//! - **(1) Branch-safety is TWO legs, not three.** The §4 rule listed four guarantees; the false one is **#2
//!   "handle every label"** — exhaustiveness gives *scrutinee coverage, not run-to-`End`*, so a safe wildcard
//!   `match offer() { Ok(Branch::Left(c)) => .., _ => {} }` drops `Chan<R>` and starves that branch. What is left
//!   is **two-legged branch-safety** (label–continuation correlation + branch-set duality); **order** is the
//!   *generic* typestate guarantee carried since #18, not branch-specific. "Handle-all" has no content separate
//!   from the conceded completion residue.
//! - **(2) The deflation is onto binary `Either` + exhaustiveness, NOT n-ary *named* sum types.** Recovering real
//!   labeled n-ary choice needs positional nesting that **drops label names**, or macro scaffolding (`offer!`/
//!   `choose!`, exactly what the `session_types` crate needed). "Deflates onto the floor's sum types" holds for
//!   *binary* choice only.
//! - **(3) Horn 2 is Θ(type-size), not Θ(depth).** A balanced `Offer` tree has 2^depth leaves; interning + the
//!   no-recursion restriction keep the projection pass linear-in-source (so it costs nothing), but the metric
//!   label was wrong.
//! - **(4) Completion is AMPLIFIED, not "unchanged by branching".** The obligation multiplies **per-branch** and
//!   becomes **peer-choice-reachable** — the dual's `select` decides which completion obligation fires, so the
//!   residue is now *adversary-triggered*, not merely self-abandonment.
//!
//! Core thesis holds, sharpened: **no new coordinate** (the branching connective opens no new ground); completion
//! stays the #17/#18/#23/#27 residue, now fed by **two** safe paths (`mem::forget` AND the dropped wildcard
//! branch). No new domain (liveness), coordinate (progress, from #24), or mode (N = 9); broken-seam tally stays
//! **8** (sound — a deliverable rejection, not a horn-1 break). Prior art CONFIRMS (Honda 1993's `&`/`⊕`; the
//! `session_types`/Jespersen–Munksgaard–Larsen 2015 affine-vs-linear gap is literally this completion residue).
//! Lean `choice_branch_typing_closes` (renamed from `..._branch_safety_closes`: only branch *typing* closes —
//! the two legs + generic order — not branch *completion*).

use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`.
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: **external choice** (`&`) — receive a label and branch to `L` or `R`. The dual of `Select`.
pub struct Offer<L, R>(PhantomData<(L, R)>);
/// Protocol marker: **internal choice** (`⊕`) — send a label, committing to `L` or `R`. The dual of `Offer`.
pub struct Select<L, R>(PhantomData<(L, R)>);
/// Protocol marker: the **terminal** state.
pub struct End;

/// The duality relation: the protocol the *peer* must follow. `Tx ↔ Rx`, `End` self-dual, and **choice flips**:
/// `Offer<L, R> ↔ Select<L::Dual, R::Dual>` — if one side *offers* a branch set, its peer *selects* over the
/// dualized set, so the option set matches the branch set exactly.
pub trait HasDual {
    /// The dual protocol the peer holds.
    type Dual;
}
impl<Cont: HasDual> HasDual for Tx<Cont> {
    type Dual = Rx<Cont::Dual>;
}
impl<Cont: HasDual> HasDual for Rx<Cont> {
    type Dual = Tx<Cont::Dual>;
}
impl<L: HasDual, R: HasDual> HasDual for Offer<L, R> {
    type Dual = Select<L::Dual, R::Dual>;
}
impl<L: HasDual, R: HasDual> HasDual for Select<L, R> {
    type Dual = Offer<L::Dual, R::Dual>;
}
impl HasDual for End {
    type Dual = End;
}

/// The trusted wire: a value slot, a choice-label slot (`false` = left, `true` = right), and a completion
/// counter. Out of scope for the attack (like #23's `Transport`); the seam's claim is about the **choice**
/// re-import.
pub struct Transport {
    val: Option<i32>,
    label: Option<bool>,
    closes: u32,
}

impl Transport {
    const fn new() -> Self {
        Transport { val: None, label: None, closes: 0 }
    }
}

/// One affine endpoint. `P` is this endpoint's remaining protocol; `!Copy`/`!Clone`. The wire is shared with the
/// dual endpoint through `Rc<RefCell<Transport>>`.
pub struct Chan<P> {
    t: Rc<RefCell<Transport>>,
    _p: PhantomData<P>,
}

impl<P> Chan<P> {
    #[inline]
    fn step<Q>(self) -> Chan<Q> {
        Chan { t: self.t, _p: PhantomData }
    }
}

/// Open a session at protocol `P`, returning both dual endpoints (as #23).
#[inline]
pub fn session<P: HasDual>() -> (Chan<P>, Chan<P::Dual>) {
    let t = Rc::new(RefCell::new(Transport::new()));
    (Chan { t: t.clone(), _p: PhantomData }, Chan { t, _p: PhantomData })
}

impl<Cont> Chan<Tx<Cont>> {
    /// Transmit `x` and advance. Exists only at a `Tx` state.
    #[inline]
    pub fn send(self, x: i32) -> Chan<Cont> {
        self.t.borrow_mut().val = Some(x);
        self.step()
    }
}

impl<Cont> Chan<Rx<Cont>> {
    /// Try to receive an `i32` and advance (fallible, as #23 — the value comes from a peer who may never send).
    #[inline]
    pub fn recv(self) -> Result<(i32, Chan<Cont>), Self> {
        let got = self.t.borrow_mut().val.take();
        match got {
            Some(x) => Ok((x, self.step())),
            None => Err(self),
        }
    }
}

/// The outcome of [`Chan::offer`]: a sum the consumer **must `match`**, each variant carrying the correctly-typed
/// continuation. Rust's exhaustiveness forces handling both branches; the variant types forbid taking one label
/// and continuing as the other branch's protocol.
pub enum Branch<L, R> {
    /// The peer selected the **left** branch; continue as `L`.
    Left(Chan<L>),
    /// The peer selected the **right** branch; continue as `R`.
    Right(Chan<R>),
}

impl<L, R> Chan<Offer<L, R>> {
    /// Receive the peer's choice. Fallible (as `recv`): the label comes from a peer who may not have selected
    /// yet. On `Ok` you get a [`Branch`] you must `match` — handling **both** branches, each at its correct
    /// protocol. On `Err(self)` the handle is handed back, still waiting.
    #[inline]
    pub fn offer(self) -> Result<Branch<L, R>, Self> {
        let got = self.t.borrow_mut().label.take();
        match got {
            Some(false) => Ok(Branch::Left(self.step())),
            Some(true) => Ok(Branch::Right(self.step())),
            None => Err(self),
        }
    }
}

impl<L, R> Chan<Select<L, R>> {
    /// Select the **left** branch (commit to `L`), depositing the label for the peer's [`Chan::offer`].
    #[inline]
    pub fn select_left(self) -> Chan<L> {
        self.t.borrow_mut().label = Some(false);
        self.step()
    }

    /// Select the **right** branch (commit to `R`).
    #[inline]
    pub fn select_right(self) -> Chan<R> {
        self.t.borrow_mut().label = Some(true);
        self.step()
    }
}

impl Chan<End> {
    /// Close this endpoint — bumps the completion counter. Exists only at `End`.
    #[inline]
    pub fn close(self) {
        self.t.borrow_mut().closes += 1;
    }

    /// How many endpoints of this session have closed (0, 1, or 2).
    #[inline]
    pub fn closes(&self) -> u32 {
        self.t.borrow().closes
    }
}
