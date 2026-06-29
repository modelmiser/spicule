//! # Eighteenth seam — a session-typed channel: the COMPOSITE-PRECONDITION probe.
//!
//! *(Project ordinal: the eighteenth primitive. Built as the natural crucible for the two prior liveness
//! findings: #15 (`ordered_locks`) showed affine **typestate carries static order**; #17 (`linear`) showed
//! the affine floor **cannot compel completion** (linearity). A session type needs *both* — operations in
//! protocol **order** AND the protocol run to **completion** — so it asks whether the floor-match is graded
//! not just *across* preconditions (Phase P) but *within* one, **component by component**.)*
//!
//! ## The primitive — a protocol as a type-level sequence
//!
//! A [`Chan<P>`] is an affine handle whose type parameter `P` is the *remaining protocol*, built from the
//! markers [`Tx`] (transmit an `i32`, then continue), [`Rx`] (receive, then continue), and [`End`]. Each
//! operation **consumes** the handle and returns one at the successor state: `Chan<Tx<C>>::send` → `Chan<C>`,
//! `Chan<Rx<C>>::recv` → `(i32, Chan<C>)`, `Chan<End>::close`. The re-import promise under attack: *"the
//! protocol is followed in order and run to completion."* The danger is a **liveness/correctness** hazard —
//! a dual peer left waiting forever, or a protocol corrupted by an out-of-order step — not UB.
//!
//! ## Why this decomposes — and the floor is suspected to match it *component-wise*
//!
//! "Protocol fidelity" is a **composite** precondition:
//! - **Safety (order):** no operation out of turn. Carried by **affine typestate** — `send` exists *only* on
//!   `Chan<Tx<_>>`, `recv` *only* on `Chan<Rx<_>>`, `close` *only* on `Chan<End>`, so an out-of-order call is
//!   a *type error* (no such method). This is #15's static-order discipline, here with **no `root` to
//!   re-enter** (the granularity hole that sank #15 has no analog — `Chan` is a single affine value).
//! - **Completion (progress):** the protocol reaches `End`. **Not** carried — a `Chan<P>` mid-protocol can be
//!   `mem::forget`/`ManuallyDrop`/leak-ed in *safe* code, abandoning the session (the #17 linearity gap).
//!
//! ### Safety IS carried — these are type errors (no method at the wrong state):
//! ```compile_fail
//! # use seam_spsc::session::{open, Transport, Tx, End};
//! let mut t = Transport::new();
//! let c = open::<Tx<End>>(&mut t);
//! let _ = c.recv(); // ERROR: no method `recv` on Chan<'_, Tx<End>> — receiving when the protocol says send
//! ```
//! ```compile_fail
//! # use seam_spsc::session::{open, Transport, Tx, End};
//! let mut t = Transport::new();
//! let c = open::<Tx<End>>(&mut t);
//! c.close(); // ERROR: no method `close` on Chan<'_, Tx<End>> — closing before the protocol reaches End
//! ```
//! ```compile_fail
//! # use seam_spsc::session::{open, Transport, Tx, End};
//! let mut t = Transport::new();
//! let c = open::<Tx<Tx<End>>>(&mut t);
//! let _c2 = c.send(1);
//! let _ = c.send(2); // ERROR: use of moved value `c` — the handle is affine, one step consumes it
//! ```
//!
//! ### Completion is NOT carried — a safe `mem::forget` abandons the protocol (it compiles and runs):
//! ```
//! # use seam_spsc::session::{open, Transport, Tx, Rx, End};
//! let mut t = Transport::new();
//! {
//!     let c = open::<Tx<Rx<End>>>(&mut t);
//!     let c = c.send(7);        // now mid-protocol at Rx<End>
//!     core::mem::forget(c);     // SAFE — abandon: never recv, never close
//! }
//! assert_eq!(t.completed(), false); // the protocol never reached End — a dual peer would wait forever
//! ```
//!
//! ## The deliverable the spec poses (verdict PENDING the cold panel — `SEAM_SPEC_session.md`)
//!
//! The completion break is compiler-settled (it is #17's gap). What the gauntlet adjudicates:
//! 1. **Component-wise grade.** Does the affine floor match this *composite* precondition component-wise —
//!    *safety closes* (the typestate fragment is a sound, usable, concentrated sub-seam) while *completion
//!    breaks* (linearity)? If so, this is the first primitive where a sub-component cleanly **closes** while
//!    the whole **breaks**, sharpening Phase P ("the unit is the precondition") to *"a precondition
//!    decomposes, matched component-wise."*
//! 2. **Does the safety component REALLY close, or is there a typestate hole?** (forge a `Chan` at a wrong
//!    state, skip a step via some safe path, alias the transport) — or concede sealing/affinity blocks it.
//! 3. **Domain or composite?** Is "session/protocol fidelity" a *fourth domain*, or a **composite** of a
//!    safety property (typestate, memory-safety-adjacent) and a liveness property (completion) — i.e. the
//!    place where domains *compose* rather than a new one?
//! 4. **Verdict mode.** Completion breaks on the same **wrong-structural-logic** linearity gap as #17 (no
//!    new mode), OR the *composite* framing is itself the new contribution (component-wise grading), OR the
//!    safety-closes/completion-breaks split is just "#15 + #17 stapled together" (a deflation).
//!
//! And the sound use runs in order and completes — see `tests/session_use.rs`.

use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`. (Named `Tx`, not `Send`, to avoid
/// shadowing the `core::marker::Send` auto-trait.)
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: the **terminal** state — the only state from which the session may `close`.
pub struct End;

/// The trusted transport (the "wire"): a loopback single-slot buffer. Its byte-moving is the trusted core
/// (like SPSC's happens-before or the spinlock in `lock_registry`) — *out of scope* for the attack; the
/// seam's claim is about the **typestate protocol** re-import, not the transport. `completed` is set **only**
/// by [`Chan::<End>::close`], so a session abandoned mid-protocol leaves it `false`.
pub struct Transport {
    slot: Option<i32>,
    completed: bool,
}

impl Transport {
    /// A fresh, empty transport.
    pub const fn new() -> Self {
        Transport { slot: None, completed: false }
    }
    /// Did a session on this transport run all the way to `End` and `close`? (False if abandoned.)
    pub fn completed(&self) -> bool {
        self.completed
    }
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}

/// The affine session handle. `P` is the *remaining* protocol; the handle is `!Copy`/`!Clone`, and each
/// operation consumes it and returns the successor state — so the type tracks where in the protocol you are.
pub struct Chan<'a, P> {
    t: &'a mut Transport,
    _p: PhantomData<P>,
}

impl<'a, P> Chan<'a, P> {
    /// Private state transition — the only way to change `P`, available solely to the protocol methods.
    #[inline]
    fn step<Q>(self) -> Chan<'a, Q> {
        Chan { t: self.t, _p: PhantomData }
    }
}

/// Open a session on `t` at protocol `P`. The caller picks `P` (its peer would hold the dual protocol).
#[inline]
pub fn open<P>(t: &mut Transport) -> Chan<'_, P> {
    Chan { t, _p: PhantomData }
}

impl<'a, Cont> Chan<'a, Tx<Cont>> {
    /// Transmit `x` and advance to `Cont`. Exists **only** at a `Tx` state — sending out of turn is a type
    /// error elsewhere.
    #[inline]
    pub fn send(self, x: i32) -> Chan<'a, Cont> {
        self.t.slot = Some(x);
        self.step()
    }
}

impl<'a, Cont> Chan<'a, Rx<Cont>> {
    /// Receive the pending value and advance to `Cont`. Exists **only** at an `Rx` state.
    #[inline]
    pub fn recv(self) -> (i32, Chan<'a, Cont>) {
        // The typestate guarantees a value is pending here (a `Tx` on the peer/loopback preceded this `Rx`);
        // that guarantee is the trusted-core invariant, analogous to SPSC's published-slot invariant.
        let x = self.t.slot.take().expect("protocol guarantees a pending value at an Rx state");
        (x, self.step())
    }
}

impl<'a> Chan<'a, End> {
    /// Close the session — the **only** way `Transport::completed` becomes `true`. Exists only at `End`,
    /// so a protocol that has not reached its terminal state cannot be marked complete.
    #[inline]
    pub fn close(self) {
        self.t.completed = true;
    }
}
