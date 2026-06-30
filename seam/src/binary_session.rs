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
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_binary_session, r30): SEAM-COSTS [horn 3], TRADEOFF — it
//! ## HOLDS with a cost, and #18's component-wise grading SURVIVES — but the "real binary session" framing was
//! ## substantially DEFLATED (single-threaded, straight-line, single-session) and five costs were recorded.
//!
//! The seam **survived** (TRADEOFF, 1/1) — its *exact, scoped* soundness claims held and **every attacker
//! granted Horn 1 / no-UB** (no out-of-turn typed op, no UB path; the worst case is a starved, memory-safe
//! handle). Order **closes** (method availability + nominal/affine typing), *per-session* relative progress
//! **closes** (duality preserved under reduction; `HasDual` sealed by coherence), decidability holds
//! (`HasDual` projection is Θ(depth), single-occurrence RHS — no SMT, no higher-order unification), and
//! completion is honestly conceded a **horn-3 cost**. But three "BREAKS" attackers converged on one thing:
//! the **deliverable framing overclaimed** relative to the witness actually shipped. All five corrections
//! accepted:
//!
//! 1. **Relative progress closes only PER SESSION — the sharpest finding (expressiveness).** Two interleaved
//!    `session()` calls produce a **type-checking mutual-wait deadlock** (the Kobayashi/Padovani fact:
//!    interleaved binary sessions deadlock under plain duality — each session is individually dual, the
//!    *composition* has no terminating schedule, both endpoints permanently `Err`). So the re-import rule's
//!    *flat* sentence "mutual-wait deadlock is a type error you cannot write" is **false** and is narrowed to
//!    *the two endpoints of one session*. Worse: within one session two endpoints trivially can't both be
//!    `Rx` (one is the `Tx` dual), so the "new positive" is true by a **triviality** — binary duality's
//!    progress theorem only has *teeth* in the interleaved-session case, which the spec scoped out. **Global
//!    deadlock-freedom is out of reach of binary session types — it needs multiparty/Kobayashi typing.** This
//!    is the real refinement of liveness's partial-match: even "relative progress closes" is bounded to a
//!    single session.
//! 2. **The single-threaded witness makes "peer waits forever" self-abandonment-flavored (prior-art).** One
//!    thread owns *both* `Chan` handles and chooses the interleaving, so there is no genuine second party —
//!    "the verbatim r25 deflation #1 of #18, recurring as a two-handle walk in one thread." The contribution
//!    is therefore **real but modest**: the judge confirmed the **static** limitation *is* lifted (#18 could
//!    not hold two concurrently-live endpoints *at all*, `E0499`; this vends two live dual affine handles) —
//!    but *not* more than the `session_types` crate (2015), which had genuine two-thread concurrency a decade
//!    earlier. The honest contribution is **locating** the binary session on the lattice + lifting the static
//!    two-live-endpoint limitation, **not** "built a more-real session than prior art."
//! 3. **The fallible `recv`'s `Err` is NON-ACTIONABLE — and the same `mem::forget` gap forecloses the fix
//!    (ergonomics, the decisive sharpening).** Because `mem::forget` **leaks the `Rc`** (skips `Drop`, strong
//!    count *not* decremented), a starved dual cannot distinguish "not-yet" from "abandoned": `strong_count`
//!    still sees the peer's reference alive, and no closed-flag is set. So `Err` **conflates "not ready" with
//!    "never,"** and the design forecloses any fix — an *actionable* signal (like `mpsc::Recv`'s disconnect
//!    `Err`) would need `Drop` to fire, which `forget` skips. This dents deliverable #2's "honest surfacing"
//!    and horn-3's "consumer writes no protocol bookkeeping" (the consumer writes the scheduler **and** a
//!    non-actionable `match` at every `recv`). The judge's resolution: it does **not** kill the rule (the rule
//!    never claims `Err` is actionable — `recv` is fallible *precisely because* completion is unguaranteed),
//!    so honesty-at-the-type-level is real (vs #18's hidden `.expect`) while **actionability is absent**. The
//!    sharp liveness finding: *the leakpocalypse gap that costs completion ALSO forecloses detection of the
//!    abandonment* — a double bind; the actionable version requires the linear/`Drop`-guaranteed floor Rust
//!    lacks.
//! 4. **Only straight-line `i32` protocols (expressiveness).** No `offer`/`choose` (`&`/`⊕`), no **delegation**
//!    (channel-over-channel — *the* feature distinguishing session types from coupled typestate; the payload
//!    is `i32`, a `Chan` is not), no recursion. Without delegation a "binary session" collapses to "two
//!    coupled typestate walks — #18 doubled," so "real binary session" overclaims breadth.
//! 5. **Citation fix (prior-art).** ECOOP 2022 "Stay Safe Under Panic" is **Lagaillardie–Neykova–Yoshida**,
//!    not Cutner–Yoshida–Vassor (the attribution #18's doc also carried — corrected in both).
//!
//! **The net finding.** The seam HELD (horn-3 cost, no UB, **N stays 9**, **liveness stays the third domain**,
//! the FAILED broken-seam tally **stays 7**) and **confirms #18's component-wise grading transfers** — order
//! and *per-session* progress close, completion costs horn 3. But the genuinely new content, once the "real
//! binary session" framing is deflated to "#18 with a duplicated handle and a `RefCell` between the copies,"
//! is **two sharpenings of the liveness map**: (a) binary duality gives only **per-session** progress —
//! global deadlock-freedom needs multiparty session types (Kobayashi/Padovani); and (b) the **leakpocalypse
//! double-bind** — the same `mem::forget` gap that costs completion *also* denies the consumer an actionable
//! abandonment signal, deepening the #17/#18 linearity gap. The probe earned its keep by **refuting its own
//! headline** while confirming the underlying grading.
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
