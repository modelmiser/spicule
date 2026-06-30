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
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_recursion.md, r38): SEAM-COSTS [horn3-concentration], FAILED — I OVER-CONCEDED
//!
//! Horn-classifier: **SEAM-COSTS [horn3-concentration]** — *"`Var` sound, no UB; but guardedness is decidable — a
//! `Guarded` bound owed, not an undecidable NONE."* Judge: **FAILED** on the §6 deliverable — the **rejection mode,
//! 7th firing** (after `typed_nat` #8, `delegation` #24, `multiparty` #25, `projected` #27, `choice` #28,
//! `must_complete` #29). Sound — so the broken-seam tally **STAYS 8** (a deliverable rejection, not a soundness
//! break) — but my central concession was **wrong**, and (the recurring sting) it was refuted **from my own cited
//! prior art** again.
//!
//! What HELD (every attacker confirmed): **Horn 1 — the `Var`/environment encoding is sound.** No safe path
//! produces an ill-formed unfolding; the worst case is a starved peer (a non-terminating or dropped loop), not UB —
//! the `recur`-only-under-`Rec<P>` discipline genuinely traps `Var`. **Horn 2 — decidable** (Θ(type-size), no SMT).
//! And the **#30-bounding finding HELD**: a protocol with no reachable `End` (`Rec<Tx<Var>>`) makes #30's
//! `Done`-forcing vacuous — recursion does expose #30's unstated precondition (a reachable terminal).
//!
//! What was REJECTED (the killer — soundness + prior-art): **I conflated two obligations and over-conceded one.** I
//! filed **productivity / guardedness** as an *undecidable NONE* alongside termination. It is **not** — it is a
//! **decidable cost** at the re-import boundary. `Rec<Var>` = `µX.X` is the **non-contractive** type that *every
//! system I cited* (Coq/Agda **guarded corecursion**, contractive recursive types) rejects **syntactically**. A
//! sealed **`Guarded` marker** (implemented for `Tx`/`Rx`/`End`/`Rec`, **not** for `Var`) required by
//! `enter`/`Rec`-formation carries guardedness in **Θ(type-size), no SMT — the same machinery as order**. So
//! productivity **CONCENTRATES (horn 3)**; it is a cost, not a NONE. **Only *termination* is the genuine undecidable
//! NONE** (forcing a loop to take an `End` branch is the halting problem). I cited guarded corecursion as prior art
//! and then mis-ruled the very thing it *decides* — the #24/#25/#29 pattern (refuted from my own lineage), 7th time.
//!
//! META — the inverse of the arc's usual over-claim, but the **same shape**: where the rejections #8/#24/#25/#27/#28
//! over-claimed concentration is *total* and #29/#30 over-claimed the danger is *irreducible/global*, here I
//! **over-conceded** — put guardedness at the *undecidable* extreme when the truth is a **decidable cost in
//! between**. The unifying lesson sharpens: **I flee the middle (a decidable, partial cost) toward whichever
//! extreme is rhetorically cleaner** — total concentration, or total impossibility. The corrected ruling: order +
//! duality + **guardedness** all CLOSE (concentrable, decidable); **termination alone** is the undecidable NONE; the
//! completion story decomposes once more (per-unfolding order + guardedness close / whole-loop termination is the
//! NONE). No new domain (liveness), coordinate, or mode (N stays 9); tally STAYS 8. Lean `recursion_guarded_order_
//! closes` (renamed from `recursion_order_unfolding_closes`: guardedness joins the closing core, the over-conceded
//! part the kernel still cannot see). The natural **#32** is the concrete salvage — add the sealed `Guarded` marker
//! so `Rec<Var>` will not form — mirroring #26→#27 and #29→#30.
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
