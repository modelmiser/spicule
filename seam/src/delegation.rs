//! # Twenty-fourth seam — DELEGATION (channel-over-channel): the higher-order session step.
//!
//! *(Project ordinal: the twenty-fourth primitive. The r30 panel's expressiveness attacker named
//! **delegation** — sending a channel endpoint as a payload over another channel — as "*the* feature that
//! distinguishes session types from coupled typestate machines," the one #23's straight-line `i32` witness
//! lacked. This seam adds it. Still NOT a fourth-domain bid: it stays in the **third confirmed domain
//! (liveness)** and tests whether the higher-order step opens any new ground — or whether its hazards deflate
//! onto the existing basis, a Phase-K-style closed-basis confirmation in the session domain.)*
//!
//! ## The primitive — a channel as a payload
//!
//! Built on the twenty-third seam's shape ([`Rc<RefCell<Transport>>`]-backed dual endpoints), but the wire's
//! slot now carries **either** an `i32` **or** a whole delegated endpoint (`Slot`). Two new protocol
//! markers extend `Tx`/`Rx`/`End`:
//! - [`TxChan`]`<Q, Cont>` — **delegate** a [`Chan`]`<Q>` (hand off a live session at protocol `Q`), then
//!   continue as `Cont`. [`Chan::send_chan`] consumes the delegated handle and advances.
//! - [`RxChan`]`<Q, Cont>` — **receive** a delegated `Chan<Q>`, then continue as `Cont`. [`Chan::recv_chan`]
//!   re-attaches the protocol `Q` (statically known, pinned by duality) to the received wire.
//!
//! [`HasDual`] passes the delegated protocol `Q` through **unchanged** (`TxChan<Q, C> ↔ RxChan<Q, Dual C>`):
//! the two parties to the *carrier* session agree on `Q`, while `Q`'s own duality is between the delegatee and
//! the *third* party. Delegation is inherently **three-party** — A delegates its session-with-C to B, and now
//! B drives that session with C.
//!
//! ## What the floor carries — and the new hazard it does not
//!
//! 1. **Order (delegation in turn)** — carried by affine typestate, exactly as #23. `send_chan` exists only at
//!    a `TxChan` state, `recv_chan` only at `RxChan`.
//! 2. **Don't-use-after-delegate** — carried *for free by the affine move*. `send_chan(self, c)` **consumes**
//!    `c`; using `c` afterward is `use of moved value`. The floor's affinity is exactly the linearity
//!    delegation needs *here* (you handed the session off — you must not keep driving it).
//! 3. **Completion** — **NOT** carried, and now **three-party**: the delegatee B can `mem::forget` the
//!    received `Chan<Q>` (the #17/#18/#23 linearity gap), starving the *third* party C, who never delegated to
//!    nor heard of B. The horn-3 completion obligation now crosses a party B never contracted with.
//! 4. **Leak-freedom** — **NOT** carried, and delegation makes it *worse* (the genuinely new hazard). The
//!    payload is now an `Rc<RefCell<Transport>>`, so **abandoning a carrier session with a delegated channel
//!    in flight LEAKS the payload session**: `send_chan` parks the payload's `Rc` in the carrier's slot, and a
//!    `mem::forget` of the carrier skips its `Drop`, so that `Rc` is never released — the canonical
//!    `Rc<RefCell>` leak (leakpocalypse / #9 `take` / #10 `drain`), now *amplified* through delegation (one
//!    abandoned carrier leaks a whole second session, transitively anything it holds).
//!
//! ### Order + don't-use-after-delegate ARE carried — these are type / move errors:
//! ```compile_fail
//! # use seam_spsc::delegation::{session, Tx, Rx, End, TxChan};
//! let (a, _b) = session::<TxChan<Rx<End>, End>>();
//! let (w_a, _w_c) = session::<Tx<End>>();        // a worker session to delegate
//! let a = a.send_chan(w_a);                        // delegate w_a away — `w_a` is moved
//! let _ = w_a.send(1);                             // ERROR: use of moved value `w_a` — can't drive a delegated channel
//! # let _ = a;
//! ```
//!
//! ### Sound three-party delegation runs and completes — see `tests/delegation_use.rs`.
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_delegation, r31): SOUND, but the gauntlet FAILED on the §6
//! ## DELIVERABLE — the REJECTION mode (2nd after `typed_nat` #8). My "no new coordinate" claim was REFUTED:
//! ## delegation opens **progress / deadlock-freedom** as a coordinate distinct from completion AND leak-freedom.
//!
//! The **re-import rule is sound** — the horn-classifier ruled **SEAM-COSTS [horn 3]** (decidable, no UB; the
//! soundness attacker confirmed no wrong-`Q` re-typing — duality-by-construction + generic invariance pin `Q`
//! and block any Gay–Hole subtyping smuggle; the worst case is a memory-safe leaked/starved session). But the
//! **judge ruled FAILED** on the in-scope §6 deliverable, and I accept it: my central pre-registered claim —
//! *"delegation adds no new coordinate; its hazards project onto leak-freedom (#9/#10), a Phase-K closed-basis
//! confirmation"* — is **refuted from my own cited lineage**. This is the method's **rejection mode** (the seam
//! is sound, like `typed_nat` #8, but its deliverable is rejected), firing for the **second time**.
//!
//! **The category error (the genuine flaw).** Delegation's three-party hazard — B `mem::forget`s the received
//! `Chan<Q>`, **starving C** — is **not** a completion gap of the carrier and **not** a leak. In the exact
//! words of the `session_types` Rust crate I cited (Jespersen–Munksgaard–Larsen, WGP 2015): an affine session
//! "*means an agent can drop the connection anytime without finishing the protocol, which leads to **deadlock
//! at the other end**.*" That is **progress** — a third, recognized property. An entire literature exists
//! *because delegation is the feature that breaks first-order binary sessions' automatic deadlock-freedom*:
//! Coppo–Dezani–Yoshida–Padovani (MSCS 2016, "*the communication topology changes because of delegation*"),
//! Kobayashi-style deadlock/lock-freedom typing, and deadlock-free GV (Lindley–Morris; Padovani, "Progress as
//! Compositional Lock-Freedom"; Multiparty GV, PACMPL 2022; Rusty Variation, 2019). I saw the `Rc`
//! hazard's **memory-leak face** and filed it under leak-freedom (#9/#10), missing that the **signature** danger
//! is a **progress** phenomenon that merely *also* leaks under `Rc`. The new danger does **not** land on an
//! existing coordinate — it is a **distinct coordinate (progress / deadlock-freedom)**, projected onto the
//! wrong one because the witness cannot see its true axis.
//!
//! **Why the single-threaded scoping doesn't save the claim.** Progress/deadlock only manifests under
//! interleaving; a single-threaded synchronous witness *definitionally cannot deadlock*, so "delegation adds no
//! new coordinate" is an **artifact of the witness**, not a property of delegation. My one real defense — the
//! symmetric two-session **delegation cycle is type-obstructed** (it is an inexpressible infinite type,
//! confirmed by the expressiveness and implementation attackers) — rescues *behavior*, not *classification*: it
//! means the correct deliverable answer is "delegation's signature hazard **is** a distinct coordinate
//! (progress), which this witness avoids by obstructing the cyclic topology and excluding concurrency."
//!
//! **Recorded tradeoffs (accepted, carried into the prose):**
//! 1. **No mid-protocol abort verb; `Result<_, Self>` bans `?`.** On an I/O error you hold a non-`End` channel
//!    you can neither close nor propagate; dropping it starves the peer — so the completion obligation is
//!    undischargeable **by the programmer**, not merely by the type system (a real cost against horn-3's
//!    "concentration is cheap").
//! 2. **Horn-2 complexity is Θ(depth²), not the spec's Θ(depth)** — the `HasDual` involution forces
//!    normalizing `dual(dual(carrier))`. Still polynomial, no SMT — but the stated complexity was wrong.
//! 3. **The finite-type, `!Send`, single-thread witness cannot express the recursive accept-loop server**
//!    (delegation's canonical use case) nor cross-thread hand-off; Horn-2 decidability depends on that
//!    restriction (`Rec`/`Var` would restore it, as `session_types` did).
//!
//! **The net finding (the probe earned its keep by being instructively WRONG).** I predicted a closed-basis
//! confirmation; the method **refuted it with my own citations**, revealing that the higher-order step **opens
//! the progress / deadlock-freedom coordinate** — distinct from completion (linearity) and leak-freedom — which
//! delegation is the *recognized* feature to introduce. So the liveness domain is **not** two coordinates
//! (completion, leak-freedom) but at least **three** (… + progress/deadlock-freedom); the earlier lock-ordering
//! seams (#15/#16) were the *lock* face of progress, and delegation is its *session-topology* face. The seam is
//! sound (SEAM-COSTS [horn 3]), so the **FAILED broken-seam tally STAYS 7** (this is a deliverable rejection,
//! not a soundness break); **N stays 9** (rejection is one of the nine modes — its 2nd firing after `typed_nat`
//! #8); 24 primitives; Lean `delegation_handoff_core_closes` (the sound order + affine-handoff core closes,
//! like `typed_nat`). Folklore deflation confirmed (Gay & Hole 2005; Jespersen et al. 2015; the `Rc<RefCell>`
//! leak) — but the deflation's own sources are what refuted the headline.
//!
//! Sound three-party delegation runs and completes; abandonment leaks the in-flight payload — see
//! `tests/delegation_use.rs`.

use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`.
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: **delegate** (send) a `Chan<Q>`, then continue as `Cont`. The higher-order extension.
pub struct TxChan<Q, Cont>(PhantomData<(Q, Cont)>);
/// Protocol marker: **receive** a delegated `Chan<Q>`, then continue as `Cont`.
pub struct RxChan<Q, Cont>(PhantomData<(Q, Cont)>);
/// Protocol marker: the **terminal** state.
pub struct End;

/// The duality relation: the protocol the *peer* must follow. `Tx ↔ Rx`, `TxChan ↔ RxChan` (the delegated
/// protocol `Q` passes through **unchanged** — both carrier peers agree on `Q`; `Q`'s duality is with the
/// third party), `End` self-dual.
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
impl<Q, Cont: HasDual> HasDual for TxChan<Q, Cont> {
    type Dual = RxChan<Q, Cont::Dual>;
}
impl<Q, Cont: HasDual> HasDual for RxChan<Q, Cont> {
    type Dual = TxChan<Q, Cont::Dual>;
}
impl HasDual for End {
    type Dual = End;
}

/// What the wire's single slot can hold: nothing, an `i32`, or a **delegated endpoint** — type-erased to a
/// bare `Rc<RefCell<Transport>>` (its protocol lives in the receiver's `PhantomData`, pinned by duality, so
/// re-attaching `Q` on `recv_chan` is sound). The `Chan` variant is the whole point: a channel is a payload.
enum Slot {
    Empty,
    Int(i32),
    Chan(Rc<RefCell<Transport>>),
}

/// The trusted wire (the "transport"): a shared single slot plus a completion counter. Out of scope for the
/// attack (like #23's `Transport`); the seam's claim is about the **delegation** re-import.
pub struct Transport {
    slot: Slot,
    closes: u32,
}

impl Transport {
    const fn new() -> Self {
        Transport { slot: Slot::Empty, closes: 0 }
    }
}

/// One affine endpoint. `P` is this endpoint's remaining protocol; the handle is `!Copy`/`!Clone`. The wire is
/// shared with the dual endpoint through `Rc<RefCell<Transport>>`.
pub struct Chan<P> {
    t: Rc<RefCell<Transport>>,
    _p: PhantomData<P>,
}

impl<P> Chan<P> {
    #[inline]
    fn step<Q>(self) -> Chan<Q> {
        Chan { t: self.t, _p: PhantomData }
    }

    /// Diagnostic: the number of live strong references to this session's wire — its endpoints plus any slot
    /// that has parked this channel as a delegated payload. A delegated channel stranded in an *abandoned*
    /// carrier shows up here as a strand with no usable owner: that gap between "strands" and "reachable
    /// handles" is exactly the delegation leak. (A real session library exposes such liveness diagnostics; it
    /// reveals no `unsafe` and cannot mint or move a handle.)
    #[inline]
    pub fn wire_strong_count(&self) -> usize {
        Rc::strong_count(&self.t)
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
        self.t.borrow_mut().slot = Slot::Int(x);
        self.step()
    }
}

impl<Cont> Chan<Rx<Cont>> {
    /// Try to receive an `i32` and advance (fallible, as #23 — the value comes from a peer who may never send).
    #[inline]
    pub fn recv(self) -> Result<(i32, Chan<Cont>), Self> {
        let got = {
            let mut t = self.t.borrow_mut();
            match core::mem::replace(&mut t.slot, Slot::Empty) {
                Slot::Int(x) => Some(x),
                other => {
                    t.slot = other; // not an int (empty / a delegated chan) — leave it
                    None
                }
            }
        };
        match got {
            Some(x) => Ok((x, self.step())),
            None => Err(self),
        }
    }
}

impl<Q, Cont> Chan<TxChan<Q, Cont>> {
    /// **Delegate** `c`: hand off the whole `Chan<Q>` session to the peer, then advance. Consumes `c` (affine)
    /// — so the floor forbids using `c` after delegation. The payload's `Rc` is parked in the wire's slot,
    /// type-erased (its `Q` is re-attached by the peer's `recv_chan`).
    #[inline]
    pub fn send_chan(self, c: Chan<Q>) -> Chan<Cont> {
        self.t.borrow_mut().slot = Slot::Chan(c.t);
        self.step()
    }
}

impl<Q, Cont> Chan<RxChan<Q, Cont>> {
    /// **Receive** a delegated channel, re-attaching protocol `Q` (sound: duality pinned `Q` to what the peer
    /// sent). Fallible, as `recv`. On `Ok` you own the delegated `Chan<Q>` and may now drive that session.
    #[inline]
    pub fn recv_chan(self) -> Result<(Chan<Q>, Chan<Cont>), Self> {
        let got = {
            let mut t = self.t.borrow_mut();
            match core::mem::replace(&mut t.slot, Slot::Empty) {
                Slot::Chan(rc) => Some(rc),
                other => {
                    t.slot = other;
                    None
                }
            }
        };
        match got {
            Some(rc) => Ok((Chan { t: rc, _p: PhantomData }, self.step())),
            None => Err(self),
        }
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
