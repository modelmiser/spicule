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
//! a protocol corrupted by an out-of-order step, or a session abandoned before it completes — not UB.
//! *(The two-peer "a dual peer waits forever" framing is the **motivation**; as the r25 panel noted, this
//! `core`/`no_std`/no-`unsafe` witness has no duality — one `&'a mut Transport` cannot split into two
//! concurrently-live endpoints — so the demonstrated object is a **single-handle typestate walk** and the
//! abandonment is *self*-abandonment. See the verdict below.)*
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
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_session, r25): SEAM-COSTS [horn 3] — it HOLDS with a cost.
//! ## The composite is matched COMPONENT-WISE; the same linearity gap that BROKE #17 only COSTS here.
//!
//! Unlike the four prior out-of-(memory-safety)-domain probes, this seam **did not break** — it **survived
//! with a recorded cost** (TRADEOFF, 1/1). The panel confirmed the core structural claim and deflated three
//! over-statements; all accepted.
//!
//! **Confirmed — the component-wise match is real, not illusory.** The soundness attacker exhausted the
//! surface and found no defect: forging a `Chan` at a wrong state is blocked by nominal non-subtyping +
//! private fields, skip/reorder is `E0599`, transport-aliasing into two conflicting sessions needs two
//! `&mut Transport` (`E0499`), and the sharp cross-component move — can `mem::forget` release `&'a mut
//! Transport` early to start a second session? — fails, because **leaking a value holding a `&'a mut` does
//! not shorten `'a`**. So the completion break **cannot infect safety**; the two components are orthogonal.
//! - **Safety (order) CLOSES** — sound, decidable, concentrated; and unlike #15 there is **no `root`-hole**.
//! - **Completion (progress) COSTS horn 3** — *not* a horn-1 break. The "don't abandon" obligation is
//!   undischargeable by the type system and **recurs at every `?`/panic/early-return**, de-concentrating the
//!   completion half. The hazard is liveness, not UB, and §6 routed the concession through horn 3.
//!
//! **The illuminating contrast with #17 (the real yield).** The *same* `mem::forget` linearity gap lands on
//! a **different horn**: #17 (`linear`) **broke** horn 1, because completion *was* its whole soundness
//! promise; #18 **costs** horn 3, because completion is *one component beside an independently-sound safety
//! core*, so the gap **de-concentrates** rather than **unsounds**. Which horn a linearity gap threatens is
//! not a property of the gap but of **how the re-import rule packages it**.
//!
//! **Three deflations (recorded costs, accepted — they did not break the seam):**
//! 1. **Not a binary session — a single-handle typestate *walk*.** `core`/`no_std`/no-`unsafe` structurally
//!    forecloses two concurrently-live dual endpoints (one `&'a mut Transport`, `E0499`), so the motivating
//!    "a peer waits forever" hazard is *not expressible* here; the demonstrated abandonment is *self*-
//!    abandonment of one handle. (The grading result holds for the single-handle walk regardless.)
//! 2. **"Usable" only for infallible straight-line protocols.** `send`/`recv`/`close` are total/infallible,
//!    so over a *fallible* transport the programmer reaches for `?` — and `?`/panic/early-return *is* the
//!    conceded abandonment. With `offer`/`choose`/`Cancel` scoped out, there is no faithful failure path.
//! 3. **Not "first" — documented folklore.** The affine-typestate-order / cannot-force-completion split is
//!    Jespersen–Munksgaard–Larsen 2015 (`session_types`), RFC 1066 / the leakpocalypse, and especially
//!    **ECOOP 2022 "Stay Safe Under Panic" (Cutner–Yoshida–Vassor)**, which *names* it. The contribution is
//!    **locating** this known split on the realizability lattice (with #15-minus-`root` typestate and the
//!    #17 linearity gap as its only structural content), not discovering it.
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
