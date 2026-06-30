//! # Thirty-first seam — RECURSIVE SESSION (`Rec`/`Var`): can the floor force a *loop* to ever leave?
//!
//! *(Project ordinal: the thirty-first primitive. The last unbuilt **structural** session-type feature —
//! recursion (`μX.T`), the looping connective orthogonal to sequencing (#18/#23), choice (#28), and completion
//! (#29/#30). The session arc is otherwise closed; this seam asks what an **unbounded-depth** protocol does to the
//! findings the arc settled at finite depth — especially #30's completion-forcing, which mints a `Done` only at
//! `End`. NOT a fourth-domain bid; it stays in the third domain (liveness), and it deliberately tests the
//! **boundary** of #30 rather than opening new ground.)*
//!
//! ## The primitive — single-level recursion, with the loop variable kept sound by an environment
//!
//! `Rec<P>` binds a recursion point whose body is `P`; `Var` is the de-Bruijn-0 loop variable ("jump back to the
//! nearest enclosing `Rec`"). The soundness problem recursion poses — that a `Var` is meaningless without an
//! enclosing `Rec` — is solved by threading a **type-level recursion environment** `E` on every handle
//! (`Session<P, E>`, default `E = NoRec`):
//!
//! - [`Session::enter`] on `Rec<P>` **unfolds** once: it continues as the body `P` and sets the environment to
//!   `Rec<P>` (remembering the body to substitute for `Var`).
//! - [`Session::recur`] exists **only** for `Session<Var, Rec<P>>` — it re-unfolds, continuing as `P` again. A
//!   **top-level `Var` (environment `NoRec`) has no `recur` method at all**, so an unbound loop variable is
//!   *inert*, never unsound. This is the recursion analogue of branded #5 forbidding an index to escape its scope:
//!   `Var` is trapped under its `Rec`.
//! - `send`/`recv` walk the body; `finish` exits at `End` (as #23/#29/#30); every op **threads `E` unchanged**, so
//!   the environment is always the enclosing `Rec` body throughout the loop.
//!
//! Duality threads through recursion (`HasDual`): `Rec<P> ↔ Rec<P::Dual>`, `Var ↔ Var`. So the infinite producer
//! `Rec<Tx<Var>>` is dual to the infinite consumer `Rec<Rx<Var>>` — the binary-session relative-progress structure
//! (#23) survives unfolding.
//!
//! ## The sharp point — a well-typed session with **no reachable `End`**
//!
//! Without a *choice* connective inside the loop, single-level recursion is **necessarily non-terminating**: the
//! body of `Rec<Tx<Var>>` ends in `Var`, which `recur`s back to the body — **there is no `End` on the recursive
//! path, ever.** The type system happily types `enter; send; recur; send; recur; …` forever. Two consequences the
//! arc has not faced:
//!
//! 1. **#30's completion-forcing is VACUOUS here.** #30 (`must_scoped`) forces completion by requiring a `Done`
//!    mintable only at `End`. A protocol with no reachable `End` can **never** mint a `Done` — so the scoped
//!    salvage's forcing is structurally inapplicable, and #30's honest "modulo divergence" caveat stops being an
//!    edge case and becomes the **entire** protocol. Recursion reveals #30's unstated precondition: *a reachable
//!    terminal*.
//! 2. **To ever leave a loop you need CHOICE (#28).** A terminating loop is `Rec<Offer<End, …Var>>` — recursion ⊗
//!    choice. The floor can express and type that, but it **cannot force the consumer to ever select the `End`
//!    branch** (a livelock: choose "recurse" forever). Forcing termination is the **halting problem** — undecidable
//!    (Rice), so it is a clean **NONE**, not a floor-mismatch break and not concentrable.
//!
//! ## What this seam claims, and what it CONCEDES (stated up front to avoid the arc's recurring over-claim)
//!
//! - **Claims (and asks the panel to confirm):** order-through-unfolding **closes** (affine typestate threads
//!   soundly across `enter`/`recur`; no UB; `Var` is sound by the environment discipline); duality **closes**
//!   (`Rec`/`Var` thread `HasDual`); the typestate stays **decidable** (the *type* is finite even though runtime
//!   depth is not — Θ(type-size), no SMT); and recursion **bounds** #30's completion-forcing (it needs a reachable
//!   `End`).
//! - **Concedes as clean undecidable NONEs (does NOT claim to carry):** **termination** (a recursive session need
//!   not, and without choice cannot, reach `End`; forcing a loop to exit is the halting problem); **productivity /
//!   guardedness** (`Rec<Var>` — unguarded — type-checks but is a no-op spin loop; Rust's floor cannot enforce that
//!   every `Var` sits under a communication action). These are not floor *mismatches* (no decidable checker carries
//!   them either); they are domain-independent undecidability.
//!
//! ## HYPOTHESIS (pre-registered; the cold panel adjudicates, not the builder)
//!
//! **SEAM-COSTS [horn3-concentration]** — order + duality concentrate per-unfolding (closes); the new loop-level
//! obligations (termination, productivity) are **conceded undecidable NONEs**, not breaks. Recursion adds **no new
//! domain** (liveness) and **no new coordinate** — it adds a **termination/fairness FACE** to the existing progress
//! coordinate and **bounds** #30's forcing. The completion story **decomposes once more**, consistent with #27
//! (topology closes / execution residue stays) and #30 (per-endpoint closes / global escapes): **per-unfolding
//! order closes / whole-loop termination is an undecidable NONE.** Deflation (conceded): the encoding is standard
//! (`session_types`/`dejavu` `Rec`/`Var`; Honda–Yoshida–Carbone `μ`-types); "can't force a loop to halt" is the
//! halting problem; guardedness is Coq/Agda folklore — so the contribution is **locating** recursion as the
//! boundary of #30's forcing, not a new mechanism. The open rulings I ask for: (a) is the `Var`/environment
//! encoding **sound** (no UB, no ill-formed unfolding), or is there a safe escape (a `Var` reachable with a wrong
//! or absent `Rec`)? (b) is conceding termination/productivity as NONEs **right**, or does the floor carry/break
//! one of them? (c) does recursion **deflate entirely** onto #17's completion residue + the halting problem (a
//! redundant re-run), or does the #30-vacuity finding earn its keep? (d) tally stays 8, N stays 9?
//!
//! ### CF1 — a top-level `Var` is inert: no enclosing `Rec`, so no `recur` (an unbound loop variable cannot loop):
//! ```compile_fail
//! # use seam_spsc::recursion::{start, Var};
//! let (a, _b) = start::<Var>();   // Session<Var, NoRec> — a Var with no enclosing Rec
//! let _ = a.recur();              // ERROR: `recur` exists only for Session<Var, Rec<P>>, not NoRec
//! ```
//!
//! ### CF2 — you must `enter` a `Rec` before walking its body (the recursion head is not a `Tx`):
//! ```compile_fail
//! # use seam_spsc::recursion::{start, Rec, Tx, Var};
//! let (a, _b) = start::<Rec<Tx<Var>>>();
//! let _ = a.send(1);              // ERROR: `send` exists only at Tx<_>; must `enter()` the Rec first
//! ```

use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`.
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: the **terminal** state — the only state a session may be *finished* from.
pub struct End;
/// Recursion binder: bind a loop point whose body is `Cont`. Unfolded by [`Session::enter`].
pub struct Rec<Cont>(PhantomData<Cont>);
/// The de-Bruijn-0 loop variable: "jump back to the nearest enclosing `Rec`." Re-unfolded by
/// [`Session::recur`], which exists **only** when the environment records a real enclosing `Rec`.
pub struct Var;

/// The top-level recursion environment: **no enclosing `Rec`**. A `Session<Var, NoRec>` has no
/// [`Session::recur`], so an unbound loop variable is inert (sound), not a hole.
pub struct NoRec;

/// The duality relation: the protocol the *peer* must follow. `Tx ↔ Rx`, `End`/`Var` self-dual,
/// `Rec<P> ↔ Rec<P::Dual>` — duality threads through recursion, so a recursive protocol's dual is the
/// recursive dual (the infinite producer `Rec<Tx<Var>>` ↔ the infinite consumer `Rec<Rx<Var>>`).
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
impl<Cont: HasDual> HasDual for Rec<Cont> {
    type Dual = Rec<Cont::Dual>;
}
impl HasDual for Var {
    type Dual = Var;
}

/// The trusted wire (out of scope for the attack, like #23/#29/#30's `Transport`): a FIFO of in-flight
/// values. The seam's claim is about whether the floor can carry the *recursion* obligations, not about
/// the transport.
pub struct Transport {
    vals: VecDeque<i32>,
}

impl Transport {
    fn new() -> Self {
        Transport { vals: VecDeque::new() }
    }
}

/// One affine session endpoint at remaining protocol `P`, carrying a type-level recursion environment
/// `E` (the enclosing `Rec` body, or `NoRec` at the top level). `!Copy`/`!Clone`. No `Drop` bomb (this
/// is the bare recursion primitive, like #29's free handle — the completion findings are #29/#30's job;
/// here the point is that the loop has no `End` to complete to).
pub struct Session<P, E = NoRec> {
    t: Rc<RefCell<Transport>>,
    _p: PhantomData<(P, E)>,
}

impl<P, E> Session<P, E> {
    /// Re-wrap the wire at a new protocol/environment, consuming `self` (affine advance). No `unsafe`.
    #[inline]
    fn step<Q, F>(self) -> Session<Q, F> {
        Session { t: self.t.clone(), _p: PhantomData }
    }
}

impl<Cont, E> Session<Tx<Cont>, E> {
    /// Transmit `x` and advance. Exists only at a `Tx` state. Threads the environment `E` unchanged.
    #[inline]
    pub fn send(self, x: i32) -> Session<Cont, E> {
        self.t.borrow_mut().vals.push_back(x);
        self.step()
    }
}

impl<Cont, E> Session<Rx<Cont>, E> {
    /// Try to receive an `i32` and advance (fallible, as #23/#29/#30). On `Err(self)` the live handle is
    /// returned. Threads the environment `E` unchanged.
    #[inline]
    pub fn recv(self) -> Result<(i32, Session<Cont, E>), Self> {
        let got = self.t.borrow_mut().vals.pop_front();
        match got {
            Some(x) => Ok((x, self.step())),
            None => Err(self),
        }
    }
}

impl<Cont, E> Session<Rec<Cont>, E> {
    /// **Unfold** the recursion once: continue as the body `Cont`, recording `Rec<Cont>` as the
    /// environment so a later `Var` knows which body to re-unfold. This is the only way past a `Rec`
    /// head — you cannot `send`/`recv`/`finish` at `Rec<…>` without entering first.
    #[inline]
    pub fn enter(self) -> Session<Cont, Rec<Cont>> {
        self.step()
    }
}

impl<Cont> Session<Var, Rec<Cont>> {
    /// **Re-unfold** the loop variable: jump back to the body `Cont` of the enclosing `Rec` recorded in
    /// the environment. Available **only** when the environment is a real `Rec<Cont>` — a top-level
    /// `Session<Var, NoRec>` has no `recur`, so an unbound `Var` is inert and sound.
    #[inline]
    pub fn recur(self) -> Session<Cont, Rec<Cont>> {
        self.step()
    }
}

impl<E> Session<End, E> {
    /// Finish at the terminal state (as #23/#29). NOTE the seam's point: on a *recursive* path built
    /// without choice there is **no reachable `End`**, so this method is simply never reachable — the
    /// loop has nothing to complete to. A terminating loop needs `Rec<Offer<End, …Var>>` (recursion ⊗
    /// choice #28), and even then the floor cannot force the `End` branch to be taken (halting).
    #[inline]
    pub fn finish(self) {
        let _ = self;
    }
}

/// Open a recursive session at protocol `P`, returning both dual endpoints (as #23/#29). Both start at
/// the top-level environment `NoRec`; `enter` installs the recursion environment. `P` is typically a
/// `Rec<…>`; a bare top-level `Var` is constructible but inert (no `recur` under `NoRec`).
#[inline]
pub fn start<P: HasDual>() -> (Session<P, NoRec>, Session<P::Dual, NoRec>) {
    let t = Rc::new(RefCell::new(Transport::new()));
    (Session { t: t.clone(), _p: PhantomData }, Session { t, _p: PhantomData })
}
