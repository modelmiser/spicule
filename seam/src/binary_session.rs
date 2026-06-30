//! # Twenty-third seam — a REAL binary session: the two-endpoint liveness deepening.
//!
//! *(Project ordinal: the twenty-third primitive. NOT a fourth-domain bid — both fourth-domain candidates
//! (`crdt` convergence, `wcet` real-time) deflated, so this probe stays inside the **third confirmed domain,
//! liveness**, and DEEPENS it. It removes the single most-cited limitation of the eighteenth seam
//! (`session`): the r25 panel's deflation #1 was "**not a binary session — a single-handle typestate walk**",
//! because one `&'a mut Transport` cannot split into two concurrently-live dual endpoints (`E0499`). So #18's
//! motivating "a peer waits forever" hazard was **not expressible** — its abandonment was *self*-abandonment.
//! This seam builds the real thing: two dual endpoints, **both live at once**, sharing the wire through
//! `Rc<RefCell<Transport>>` — the `alloc` + interior-mutability the frontier predicted the duality would
//! cost.)*
//!
//! ## The primitive — two dual endpoints over a shared wire
//!
//! [`session::<P>()`] returns a pair `(Chan<P>, Chan<P::Dual>)`: two affine handles on **one** shared
//! [`Transport`] (`Rc<RefCell<…>>`), at *dual* protocols. [`HasDual`] maps [`Tx`]`<C>` ↔ [`Rx`]`<C::Dual>`
//! and [`End`] ↔ `End`, so if one endpoint must send, its peer must receive. Each operation **consumes** the
//! handle and returns its successor state, exactly as #18 — but now the value a `recv` collects was deposited
//! by *the other party*, concurrently, not by an earlier step of the same handle.
//!
//! ## What changes when the duality is REAL (the three structural shifts)
//!
//! 1. **`recv` is forced FALLIBLE.** #18's `recv` was soundly *infallible* (`.expect`): the same handle did
//!    the preceding `send`, so a value was always pending. With two real endpoints the value comes from a
//!    *peer who may not have sent yet — or ever*. So [`Chan::recv`] returns `Result<(i32, Chan<C>), Self>` —
//!    handing the handle **back** when nothing is ready. *That `Err` branch is the wait point.* The completion
//!    gap, invisible in #18's signature, surfaces **in the type of `recv` itself**.
//! 2. **Mutual-wait deadlock is FORECLOSED by duality typing.** Two peers cannot both block waiting: if one is
//!    `Tx`, its dual is `Rx`. The classic two-party "both wait for each other" deadlock is a **type error you
//!    cannot write** — duality carries *relative* progress (the binary-session progress theorem, Honda 1993).
//! 3. **Abandonment starvation is NOT foreclosed.** A peer can `mem::forget` its endpoint (safe — the
//!    leakpocalypse / #17 / #18 linearity gap) and **never send**. Its dual's `recv` returns `Err(self)`
//!    forever: *the peer waits forever.* Now genuinely **peer-observable** (a second party starves), where
//!    #18's non-completion was only *self*-observable (`transport.completed() == false`).
//!
//! So the precondition decomposes exactly as #18 — but each component is now tested against a *real* second
//! party:
//! - **Safety (order)** — carried by affine typestate (`send` only at `Tx`, `recv` only at `Rx`, `close`
//!   only at `End`). Unchanged from #18; an out-of-turn op is a type error.
//! - **Relative progress (no mutual wait)** — carried by `HasDual` (NEW: the single-handle walk had no peer to
//!   deadlock against, so it could not exhibit this *positive* result).
//! - **Mutual completion (no abandonment)** — **NOT carried**: a peer's `mem::forget` is safe, starving its
//!   dual. The #17/#18 linearity gap, now across the two-party trust boundary.
//!
//! ### Safety IS carried — out-of-turn ops are type errors (no method at the wrong state):
//! ```compile_fail
//! # use seam_spsc::binary_session::{session, Tx, Rx, End};
//! let (a, _b) = session::<Tx<End>>();
//! let _ = a.recv(); // ERROR: no method `recv` on Chan<Tx<End>> — receiving when the protocol says send
//! ```
//!
//! ### Duality is carried — `session` only builds dual pairs (a non-dual pair is a type error):
//! ```compile_fail
//! # use seam_spsc::binary_session::{session, Chan, Tx, Rx, End};
//! // The peer of `Tx<End>` is `Rx<End>`, not `Tx<End>`: you cannot bind the dual to the wrong protocol.
//! let (_a, _b): (Chan<Tx<End>>, Chan<Tx<End>>) = session::<Tx<End>>(); // ERROR: dual is Rx<End>
//! ```
//!
//! ### Completion is NOT carried — a peer's safe `mem::forget` starves its dual forever (compiles and runs):
//! ```
//! # use seam_spsc::binary_session::{session, Tx, Rx, End};
//! let (a, b) = session::<Tx<End>>();      // a: send-then-end ; b: recv-then-end (dual)
//! core::mem::forget(a);                    // SAFE — peer abandons: it will never send
//! // b is at Rx<End>; its peer is gone, so recv never becomes ready:
//! match b.recv() {
//!     Ok(_) => unreachable!("the peer was forgotten — nothing was ever sent"),
//!     Err(_b) => { /* `_b` handed back: in a blocking transport this is the forever-wait */ }
//! }
//! ```
//!
//! ## THE HYPOTHESIS THIS SPEC PUTS TO THE COLD PANEL (to be replaced by the r30 verdict)
//!
//! `design_seam/SEAM_SPEC_binary_session.md` asks: removing #18's single-handle limitation — building the
//! REAL binary session #18 could not — does the **component-wise grading survive**? The predicted verdict is
//! **SEAM-COSTS [horn 3]**, a SURVIVAL that **deepens liveness** (not a fourth domain, not a new mode):
//! - order **closes** (typestate, as #18); relative progress **closes** (duality — a *new positive*);
//! - mutual completion **costs horn 3**, now peer-observable — the same `mem::forget` gap, across a real
//!   two-party boundary, recurring at every `?`/panic/early-return on either side.
//! - No `unsafe` anywhere (the danger is liveness/semantic, like `secret`/`crdt`/`wcet`); `Rc`/`RefCell`
//!   mediate sharing with runtime borrow checks, so there is no UB path — the worst case is a starved peer,
//!   which is **not UB**.
//!
//! Deflation pre-registered (so the seam SURVIVES honestly, the `crdt` discipline, not the `wcet` over-claim):
//! real binary sessions in Rust are **documented folklore** — Honda 1993 / Honda–Vasconcelos–Kubo 1998
//! (binary session types + the progress theorem), the `session_types` Rust crate (Jespersen–Munksgaard–Larsen
//! 2015, real two-endpoint sessions over channels), and ECOOP 2022 "Stay Safe Under Panic"
//! (Cutner–Yoshida–Vassor), which *names* the abandonment gap. The contribution is **locating the real
//! binary session on the realizability lattice** and showing #18's grading was **not an artifact** of the
//! single-handle limitation. The open question the spec poses: does forcing `recv` fallible, foreclosing
//! mutual-wait by duality, and exposing peer-observable starvation change the **horn** or merely make the #18
//! cost **sharper**? And — the meta-question — does liveness stay the **third domain** (N stays 9), confirming
//! the frontier's expectation that, after two deflated fourth-domain candidates, the three domains hold?
//!
//! The sound two-party session runs in order, both peers complete, and abandonment starves the dual — see
//! `tests/binary_session_use.rs`.

use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`. (Named `Tx`, not `Send`, to avoid
/// shadowing the `core::marker::Send` auto-trait — as in the eighteenth seam.)
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: the **terminal** state — the only state from which an endpoint may `close`.
pub struct End;

/// The duality relation: the protocol the *peer* must follow. `Tx` on one side ⇔ `Rx` on the other; `End`
/// is self-dual. This is what carries **relative progress** (no mutual-wait deadlock): the two endpoints of a
/// [`session`] are typed at dual protocols, so they can never both block waiting to receive.
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
impl HasDual for End {
    type Dual = End;
}

/// The trusted wire (the "transport"): a shared single-slot loopback plus a completion counter. Its
/// byte-moving is the trusted core (like SPSC's happens-before or #18's `Transport`) — *out of scope* for
/// the attack; the seam's claim is about the **two-endpoint typestate protocol** re-import. `closes` is
/// bumped **only** by [`Chan::<End>::close`], so a session where a peer abandons mid-protocol never reaches
/// `closes == 2`.
pub struct Transport {
    slot: Option<i32>,
    closes: u32,
}

impl Transport {
    const fn new() -> Self {
        Transport { slot: None, closes: 0 }
    }
}

/// One affine endpoint of a binary session. `P` is *this endpoint's remaining* protocol; the handle is
/// `!Copy`/`!Clone`, and each operation consumes it and returns the successor state. The wire is shared with
/// the dual endpoint through `Rc<RefCell<Transport>>` — the cost of making two endpoints concurrently live.
pub struct Chan<P> {
    t: Rc<RefCell<Transport>>,
    _p: PhantomData<P>,
}

impl<P> Chan<P> {
    /// Private state transition — the only way to change `P`, available solely to the protocol methods.
    #[inline]
    fn step<Q>(self) -> Chan<Q> {
        Chan { t: self.t, _p: PhantomData }
    }
}

/// Open a binary session at protocol `P`, returning **both** dual endpoints, concurrently live, sharing one
/// transport. The caller side follows `P`; the peer side follows `P::Dual`. *This is the construction #18
/// structurally could not express* (its single `&mut Transport` forbade two live endpoints).
#[inline]
pub fn session<P: HasDual>() -> (Chan<P>, Chan<P::Dual>) {
    let t = Rc::new(RefCell::new(Transport::new()));
    (Chan { t: t.clone(), _p: PhantomData }, Chan { t, _p: PhantomData })
}

impl<Cont> Chan<Tx<Cont>> {
    /// Transmit `x` to the peer and advance to `Cont`. Exists **only** at a `Tx` state — sending out of turn
    /// is a type error elsewhere. (Infallible: the single-slot loopback always accepts; a bounded wire is an
    /// orthogonal backpressure concern, out of scope like #18's transport.)
    #[inline]
    pub fn send(self, x: i32) -> Chan<Cont> {
        self.t.borrow_mut().slot = Some(x);
        self.step()
    }
}

impl<Cont> Chan<Rx<Cont>> {
    /// Try to receive the peer's value and advance to `Cont`. Exists **only** at an `Rx` state (order safety).
    ///
    /// **Fallible — this is the structural shift from #18.** The value comes from the *peer*, who may not have
    /// sent yet, or may have abandoned its endpoint (safe `mem::forget`). On `Ok` you get the value and the
    /// continuation; on `Err(self)` the handle is handed **back** — *in a blocking transport this is the point
    /// at which the caller waits.* If the peer is gone, it stays `Err` forever: **the peer waits forever.**
    /// The type guarantees you may only `recv` *in order* (safety); it does **not** guarantee a value *arrives*
    /// (completion) — that is the peer's uncompellable linearity obligation.
    #[inline]
    pub fn recv(self) -> Result<(i32, Chan<Cont>), Self> {
        let pending = self.t.borrow_mut().slot.take();
        match pending {
            Some(x) => Ok((x, self.step())),
            None => Err(self),
        }
    }
}

impl Chan<End> {
    /// Close this endpoint — bumps the transport's completion counter. Exists only at `End`, so an endpoint
    /// that has not reached its terminal state cannot count toward completion. A session where **both** peers
    /// `close` reaches `closes == 2`; abandonment leaves it short.
    #[inline]
    pub fn close(self) {
        self.t.borrow_mut().closes += 1;
    }

    /// How many endpoints of this session have closed (0, 1, or 2). `2` iff both peers ran to completion;
    /// `< 2` witnesses an abandoned session — the peer-observable liveness failure.
    #[inline]
    pub fn closes(&self) -> u32 {
        self.t.borrow().closes
    }
}
