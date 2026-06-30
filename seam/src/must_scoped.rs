//! # Thirtieth seam — SCOPED SESSION: does a *scoped-closure* combinator **concentrate** completion?
//!
//! *(Project ordinal: the thirtieth primitive. The **direct salvage of #29** (`must_complete`), mirroring the
//! #26→#27 break→salvage rhythm. #29 built a **free-handle** `Session<P>` with a `Drop` **completion bomb** and
//! over-claimed it forced run-to-`End`; the r36 panel **reversed** that — `mem::forget` skips the bomb (the free
//! handle is forgettable), yet completion is **NOT irreducible**: a **scoped-closure / CPS combinator** — the
//! `std::thread::scope` analogue — compels it *modulo divergence*, by moving the obligation to the re-import
//! boundary so the consumer **never holds a forgettable handle**. This seam builds that combinator concretely and
//! asks the panel to rule whether the #29 reversal **closes**: is completion genuinely concentrable, at a horn-3
//! **API-locality** cost, with no new domain/coordinate/mode? NOT a fourth-domain bid; it stays in the third
//! domain (liveness), on the completion coordinate.)*
//!
//! ## The primitive — a session you can only run *inside a scope*, and a completion certificate
//!
//! There is **no free `start`**. The only way to obtain a [`Session`] is to call [`session_scope`], which hands
//! each of two **dual** role closures a freshly-branded endpoint and **requires each to return a [`Done`]**. A
//! `Done<'scope>` is a *completion certificate*: its fields are private and the **only** mint is
//! [`Session::finish`], available **only** at `End`. So a role closure has exactly three fates:
//!
//! 1. **Complete** — walk the protocol to `End`, `finish()` it, return the `Done`. The sanctioned exit.
//! 2. **Diverge** — `loop {}` / `panic!` / block forever: the closure never returns, so `session_scope` never
//!    returns either. Observable (the program hangs or unwinds); it cannot *silently* terminate having abandoned
//!    the peer.
//! 3. **There is no third fate.** You cannot return *without* a `Done`, and you cannot *fabricate* one:
//!    `core::mem::forget(session)` destroys nothing the forcing depends on (there is **no `Drop` bomb** here — the
//!    obligation is a **return type**, not a destructor), and the invariant `'scope` brand (lifted verbatim from
//!    the branded-index seam #5) means a `Done` minted in any *other* scope will not unify.
//!
//! That is the whole move: #29 forced completion with a **destructor** (forgettable — `mem::forget` is safe);
//! this seam forces it with a **return-type obligation** (not forgettable — you cannot conjure a `Done`). The cost
//! is that the session is **trapped in the closure**: it cannot be stored, returned, or moved past the scope — the
//! `thread::scope` ergonomic tax, which is exactly branded #5's "the API-locality tax **is** the soundness
//! mechanism."
//!
//! ## What it does *not* claim (the honest residue)
//!
//! - **Modulo divergence.** Forcing applies to *terminating* closures. A body may `loop {}` forever and never
//!   `finish` — the peer still starves, but the program now **hangs** (observable) rather than terminating with a
//!   silent `mem::forget` (invisible). The residue shrinks from *silent abandonment* to *non-termination*; it does
//!   not vanish.
//! - **Per-endpoint.** `session_scope` forces *both* endpoints because it owns both closures. A session whose peer
//!   lives outside any scope is only as forced as its own side — global completion needs *every* endpoint scoped
//!   (the per-session-vs-global theme of #23/#27).
//!
//! ## HYPOTHESIS (pre-registered; the cold panel adjudicates, not the builder)
//!
//! **SEAM-COSTS [horn3-concentration / API-locality], that HOLDS** — completion *is* concentrable: the scoped
//! combinator carries the completion obligation that every free-handle session seam (#17/#18/#23/#27/#28/#29) left
//! on the consumer, at the cost of surrendering the free movable handle (the brand traps the session in the
//! closure). This is the concrete witness the r36 reversal promised, closing the #29 finding: completion was the
//! price of the **free-handle API**, and paying the **API-locality** price buys it back. Horn 1 holds (no
//! `unsafe`; the worst case is a hung/leaked session, not UB). Horn 2 holds (the typestate is Θ(depth); the brand
//! is resolved by the compiler, no SMT). The open rulings I ask the panel to make: (a) is the forcing **real** or
//! is there a safe escape I have not seen (a way to mint or smuggle a `Done` without finishing)? (b) is "modulo
//! divergence" the *honest* boundary, or does a diverging body re-open the same NONE #29 had? (c) does this
//! **deflate** onto `thread::scope` / generativity / branded #5 (a re-application, not a new mechanism — which I
//! concede), so the contribution is *locating* completion as concentrable-at-API-locality-cost, not a new device?
//! (d) is the broken-seam tally unchanged (this HOLDS, tally stays 8) and N still 9?
//!
//! ### CF1 — the forcing: a body that does **not** `finish` cannot return a `Done` (so cannot terminate normally).
//! `core::mem::forget` is safe and skips any destructor — but it conjures no `Done`, so the closure's return type
//! is unsatisfiable except by `finish` (or divergence). This is the exact #29 bypass made *harmless*:
//! ```compile_fail
//! # use seam_spsc::must_scoped::{session_scope, Tx, Rx, End};
//! session_scope::<Tx<End>, _, _>(
//!     |s| { core::mem::forget(s); },                       // ERROR: returns `()`, a `Done<'a>` is required
//!     |s| { let (_x, s) = s.recv().unwrap(); s.finish() },
//! );
//! ```
//!
//! ### CF2 — generativity (the horn-3 API-locality tax IS the soundness mechanism, as branded #5): the session
//! cannot escape its scope, so it cannot be stored, returned, or completed later out of band:
//! ```compile_fail
//! # use seam_spsc::must_scoped::{session_scope, Session, Tx, End};
//! let mut escaped: Option<Session<'_, Tx<End>>> = None;
//! session_scope::<Tx<End>, _, _>(
//!     |s| { escaped = Some(s); panic!() },                 // ERROR: 'scope escapes the for<'a> binder
//!     |s| { let (_x, s) = s.recv().unwrap(); s.finish() },
//! );
//! ```

use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// An invariant brand: a lifetime appearing both co- and contra-variantly, so two distinct
/// [`session_scope`] calls' brands never unify. Identical mechanism to branded #5's `Invariant<'id>`
/// and `intrusive`'s `GhostToken` — reused on purpose: the #29 reversal named the witness as the
/// `std::thread::scope` analogue, and `thread::scope` is exactly this generativity trick.
type Invariant<'scope> = PhantomData<fn(&'scope ()) -> &'scope ()>;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`.
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: the **terminal** state — the only state a session may be *completed* from.
pub struct End;

/// The duality relation: the protocol the *peer* must follow. `Tx ↔ Rx`, `End` self-dual (as #23/#29).
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

/// The trusted wire (out of scope for the attack, like #23/#29's `Transport`): a FIFO of in-flight
/// values and a `completed` flag the test can observe. A queue (vs #29's single slot) lets a
/// straight-line role send several values before its peer drains them, so multi-step protocols are
/// demonstrable under the single-threaded run-`a`-then-`b` witness. The seam's claim is about whether
/// **completion** can be *compelled* by the scoped combinator, not about the transport.
pub struct Transport {
    vals: VecDeque<i32>,
    completed: bool,
}

impl Transport {
    fn new() -> Self {
        Transport { vals: VecDeque::new(), completed: false }
    }
}

/// One affine session endpoint, **branded** with the enclosing scope. `P` is this endpoint's remaining
/// protocol; the handle is `!Copy`/`!Clone`. The invariant `'scope` brand traps it inside the
/// [`session_scope`] closure that minted it — it cannot be stored, returned, or moved past the scope.
/// There is **no `Drop` bomb** (unlike #29): the completion obligation is carried by the *return type*
/// of the role closure (it must yield a [`Done`]), not by a destructor.
pub struct Session<'scope, P> {
    t: Rc<RefCell<Transport>>,
    _scope: Invariant<'scope>,
    _p: PhantomData<P>,
}

/// A **completion certificate**, branded with the scope it was earned in. Fields are private and the
/// **only** mint is [`Session::finish`] (available only at `End`), so possessing a `Done<'scope>`
/// *proves* the brand-`'scope` session ran to `End`. The invariant brand means a `Done` from another
/// scope will not unify — you cannot smuggle a foreign certificate in to discharge this scope's
/// obligation. This is the keystone the #29 free handle lacked.
pub struct Done<'scope> {
    _scope: Invariant<'scope>,
    _priv: (),
}

impl<'scope, P> Session<'scope, P> {
    /// Move the wire to the successor protocol `Q`, preserving the brand. No `unsafe`: we clone the
    /// `Rc` and drop the old handle (plain drop — no bomb), so the refcount is conserved.
    #[inline]
    fn advance<Q>(self) -> Session<'scope, Q> {
        Session { t: self.t.clone(), _scope: PhantomData, _p: PhantomData }
    }

    /// Test/observation helper: was the shared session ever marked completed (by either side's
    /// `finish`)? Available at any protocol state.
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.t.borrow().completed
    }
}

impl<'scope, Cont> Session<'scope, Tx<Cont>> {
    /// Transmit `x` and advance. Exists only at a `Tx` state.
    #[inline]
    pub fn send(self, x: i32) -> Session<'scope, Cont> {
        self.t.borrow_mut().vals.push_back(x);
        self.advance()
    }
}

impl<'scope, Cont> Session<'scope, Rx<Cont>> {
    /// Try to receive an `i32` and advance (fallible, as #23/#29 — the value comes from a peer who may
    /// never send). On `Err(self)` the still-live handle is handed back; since the closure **must**
    /// ultimately return a [`Done`], a body that cannot receive has only two options — retry (e.g.
    /// loop until the peer sends) or **diverge**. It can never *silently* abandon. This is exactly the
    /// "modulo divergence" boundary: starvation becomes a hang, not a silent skip.
    #[inline]
    pub fn recv(self) -> Result<(i32, Session<'scope, Cont>), Self> {
        let got = self.t.borrow_mut().vals.pop_front();
        match got {
            Some(x) => Ok((x, self.advance())),
            None => Err(self),
        }
    }
}

impl<'scope> Session<'scope, End> {
    /// The **one clean exit**: complete the session and **mint the certificate**. Available only at
    /// `End`. This is the sole constructor of [`Done`], so a role closure can return a `Done<'scope>`
    /// *if and only if* it ran its session to `End` and finished it.
    #[inline]
    pub fn finish(self) -> Done<'scope> {
        self.t.borrow_mut().completed = true;
        Done { _scope: PhantomData, _priv: () }
    }
}

/// **The scoped-session combinator** — the concrete `std::thread::scope` analogue the #29 reversal named
/// (`with_session`, here in its self-contained two-endpoint form). Opens one session, hands its two
/// **dual** endpoints to the two role closures, and **requires each to return a [`Done`]** branded with
/// the scope. Because `Done` is mintable only by `finish` at `End`, neither closure can return without
/// running its endpoint to completion — `mem::forget` cannot help (it conjures no `Done`) and the brand
/// blocks smuggling one in. The endpoints cannot escape (the `for<'a>`/`for<'b>` binders trap the
/// brands), which **is** the horn-3 API-locality cost.
///
/// Single-threaded witness (as #23–#29): role `a` runs first to completion, then role `b`; pick a
/// straight-line protocol (`a` sends, `b` receives) so no interleaving is needed.
#[inline]
pub fn session_scope<P, FA, FB>(a: FA, b: FB)
where
    P: HasDual,
    FA: for<'a> FnOnce(Session<'a, P>) -> Done<'a>,
    FB: for<'b> FnOnce(Session<'b, P::Dual>) -> Done<'b>,
{
    let t = Rc::new(RefCell::new(Transport::new()));
    let sa: Session<'_, P> = Session { t: t.clone(), _scope: PhantomData, _p: PhantomData };
    // `a` must return a `Done<'a>` — proof it ran `sa` to `End`. The certificate is consumed here;
    // nothing branded escapes the scope.
    let _done_a: Done = a(sa);

    let sb: Session<'_, P::Dual> = Session { t, _scope: PhantomData, _p: PhantomData };
    let _done_b: Done = b(sb);
}
