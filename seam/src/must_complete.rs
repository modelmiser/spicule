//! # Twenty-ninth seam — MUST-COMPLETE SESSION: can the affine floor *compel* a session to run to `End`?
//!
//! *(Project ordinal: the twenty-ninth primitive. The session connective set is exhausted (#18/#23 order,
//! #24 delegation, #25 multiparty, #26 priority, #27 projected, #28 choice); every one HOLDS or BREAKS with the
//! **same** leftover — a **completion residue** the affine floor never carries, which I over-claimed as carried
//! **five** times. This seam stops probing new features and attacks the residue itself: it builds the discipline
//! that *tries to force* a session to reach `End`, and asks whether the floor has **any** way — even a
//! cost-paying abort — to compel it. NOT a fourth-domain bid; it stays in the third domain (liveness).)*
//!
//! ## The primitive — a session handle with a completion bomb
//!
//! Self-contained over `Rc<RefCell<Transport>>` two-endpoint sessions (as #23–#28). [`start`] vends two dual
//! [`Session`] handles; `send`/`recv` walk the protocol; [`Session::finish`] (only at `End`) is the **one clean
//! exit**. Each `Session` carries a runtime `live` flag and a `Drop` **completion bomb** (cf. `take` #9's
//! abort-bomb): advancing hands `live` to the successor, `finish` clears it, but if a `live` handle is *dropped*
//! — scope exit, `?`, panic — the bomb fires (here a `panic!`, standing for `process::abort`). So "drop a
//! mid-protocol session" is a **hard error**, and the floor *appears* to compel completion: you cannot just walk
//! away from a half-run session, because its `Drop` aborts.
//!
//! The danger addressed is the **completion** half of liveness (a peer **starved** by an abandoned session),
//! not UB. This is the *remote* analogue of `linear` #17's *local* must-run-cleanup: there, `mem::forget`
//! skipped your own cleanup; here it skips a `Drop` that was the **peer's** only guarantee of being answered.
//!
//! ## The two facts a "must-complete" discipline needs
//!
//! 1. **Drop ⇒ abort.** A `live` `Session` that is dropped before `End` fires the bomb — so the *only* way past
//!    a `Session` is to advance it (`send`/`recv`) or `finish` it. The abort makes abandonment observable.
//! 2. **No silent skip.** For "must complete" to hold, there must be **no safe way** to make a `Session`'s
//!    `Drop` *not run* while also not completing it. THIS is the load-bearing claim — and the one #17 already
//!    falsified for closures: `core::mem::forget` is **safe**, consumes the value by one affine move, and skips
//!    `Drop` (the leakpocalypse, RFC 1066). `ManuallyDrop`, `Box::leak`, and `Rc`-cycles do the same.
//!
//! ## VERDICT (cold panel, REPORT_SEAM_SPEC_must_complete.md, r36): SEAM-COSTS [horn3-concentration], FAILED
//!
//! Horn-classifier: **SEAM-COSTS [horn3-concentration]** — *"No UB; the free-handle Drop-bomb can't force
//! completion, but a decidable scoped-closure restores it at the re-import boundary."* Judge: **FAILED** on the §6
//! deliverable — *"the universal NONE is refuted in-scope; downgrade NONE to SEAM-COSTS/partial-forcing."* The
//! **rejection mode, 6th firing** — and the **most instructive yet**, because it does not relabel my finding, it
//! **reverses** it.
//!
//! What is confirmed, and what the panel overturned:
//! - **The mechanism break is real but is NOT the finding** (soundness attacker, BREAKS/fatal): my §4 clause-2 is
//!   false — `core::mem::forget` skips the bomb in safe code, peer starved. Self-confirmed by the test. But this
//!   only shows the **free-handle** witness is wrong, not that completion is unforceable.
//! - **My "universal NONE" is REFUTED** (expressiveness attacker, decisive): a **scoped-closure / CPS combinator**
//!   — `with_session(|s| { …; s.finish() })`, the **`std::thread::scope` analogue** — **compels completion modulo
//!   divergence, no `unsafe`**, by moving the obligation to the **re-import / construction boundary** so the
//!   consumer never holds a forgettable free handle ("return `End` or diverge"). THE STING (6th refutation from my
//!   own lineage): I cited `thread::scoped`'s *removal* (RFC 1084) as proof completion can't be forced — and
//!   ignored its *actual fix*, **`thread::scope`** (stable 1.63), which does exactly this.
//! - **So completion is NOT an irreducible linear gap — it is CONCENTRABLE**, at a horn-3 **API-locality** cost
//!   (surrender the free movable handle; run the session inside a scope). The whole #17–#28 completion residue was
//!   the price of the **free-handle API**, never an inherent gap: every prior session seam vended a forgettable
//!   `Chan`, so every one carried it.
//! - **Sessions ≠ cleanup closures — "session face of #17" was WRONG.** #17's open-ended cleanup *cannot* be
//!   scoped (a true NONE/break); a **delimited** session *can* — so the peer/interaction structure **does** change
//!   the outcome (refuting pre-registered claim (a)). My claim (b) holds only for the *free-handle* bomb.
//! - **Ergonomics cost (real):** holding a `live` `Session` across any fallible op turns `?` into a **process-abort
//!   operator** — the bomb discipline is unusable on fallible/cancellable code (every real network loop), worse
//!   than the clean #9 abort-discharge I claimed.
//! - **Prior art: peer starvation is NOT irreducible** — affine-session work (Mostrous–Vasconcelos *Affine
//!   Sessions*; ECOOP 2022 *Stay Safe Under Panic*; MultiCrusty; Exceptional GV) resolves it via **cancellation
//!   propagation** (given the timeout/cancellation channel §5 scoped out), not by forcing completion.
//!
//! What survives: **decidability** (the typestate is Θ(depth); the undecidable static leak-detection is correctly
//! *conceded* as a NONE, not claimed). META (the 6th refutation **inverts** my usual error): across the rejections
//! I overclaimed concentration is *total*; here I overclaimed it is *irreducible*. The lesson: **I overstate the
//! extremes — fully-carried or wholly-uncarried — when the truth is a cost in between.** No new domain (liveness),
//! coordinate (completion), or mode (N = 9); broken-seam tally **STAYS 8** (SEAM-COSTS, not the BREAK I predicted —
//! the danger is concentrable). Lean `must_complete_scoped_completion_closes` (renamed from
//! `..._typed_discipline_factors`): the **scoped-closure** witness *closes* completion at an API-locality cost;
//! the free-handle witness only factored.

use alloc::rc::Rc;
use core::cell::RefCell;
use core::marker::PhantomData;

/// Protocol marker: **transmit** an `i32`, then continue as `Cont`.
pub struct Tx<Cont>(PhantomData<Cont>);
/// Protocol marker: **receive** an `i32`, then continue as `Cont`.
pub struct Rx<Cont>(PhantomData<Cont>);
/// Protocol marker: the **terminal** state — the only state a session may be *completed* from.
pub struct End;

/// The duality relation: the protocol the *peer* must follow. `Tx ↔ Rx`, `End` self-dual.
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

/// The trusted wire: a value slot and a `completed` flag. Out of scope for the attack (like #23's `Transport`);
/// the seam's claim is about whether the **completion** discipline can be compelled.
pub struct Transport {
    val: Option<i32>,
    completed: bool,
}

impl Transport {
    const fn new() -> Self {
        Transport { val: None, completed: false }
    }
}

/// One affine session endpoint **with a completion bomb**. `P` is this endpoint's remaining protocol; the handle
/// is `!Copy`/`!Clone`. `live` is `true` while this handle is the current, unfinished frontier of the session:
/// advancing (`send`/`recv`) defuses *this* handle and arms its successor; [`Session::finish`] clears it. If a
/// `live` handle reaches [`Drop`] without being advanced or finished, the **bomb** fires.
pub struct Session<P> {
    t: Rc<RefCell<Transport>>,
    live: bool,
    _p: PhantomData<P>,
}

impl<P> Session<P> {
    /// Move the wire to the successor protocol `Q`, defusing *this* handle (advancing is not abandoning) and
    /// arming the successor. No `unsafe`: we clone the `Rc` and let `self` drop with `live = false`, so the
    /// refcount is conserved and this handle's bomb does not fire.
    #[inline]
    fn advance<Q>(mut self) -> Session<Q> {
        self.live = false;
        Session { t: self.t.clone(), live: true, _p: PhantomData }
    }

    /// Test/observation helper: was the shared session ever marked completed (by either side's `finish`)?
    /// Available at any protocol state so a starved peer can confirm it never completed.
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.t.borrow().completed
    }
}

impl<P> Drop for Session<P> {
    #[inline]
    fn drop(&mut self) {
        if self.live {
            // The completion bomb (cf. `take` #9's abort-bomb). A `live` session dropped before `End` was
            // *abandoned* — its peer is starved. Stands for `process::abort`; in this `no_std` crate a `panic!`,
            // which the (`panic = unwind`) tests observe, and which on a real `?`/double-panic path aborts.
            //
            // The whole seam turns on what this `Drop` CANNOT reach: `core::mem::forget` (and `ManuallyDrop`,
            // `Box::leak`, `Rc`-cycles) skip `Drop` entirely, in *safe* code — so the bomb never fires and the
            // session is abandoned silently. That is the break (`tests/must_complete_use.rs`).
            panic!("must_complete: session abandoned before End — completion bomb (peer starved)");
        }
    }
}

/// Open a session at protocol `P`, returning both dual endpoints (as #23). Both start `live`: each must be run to
/// `End` and `finish`-ed, or its bomb fires.
#[inline]
pub fn start<P: HasDual>() -> (Session<P>, Session<P::Dual>) {
    let t = Rc::new(RefCell::new(Transport::new()));
    (
        Session { t: t.clone(), live: true, _p: PhantomData },
        Session { t, live: true, _p: PhantomData },
    )
}

impl<Cont> Session<Tx<Cont>> {
    /// Transmit `x` and advance. Exists only at a `Tx` state.
    #[inline]
    pub fn send(self, x: i32) -> Session<Cont> {
        self.t.borrow_mut().val = Some(x);
        self.advance()
    }
}

impl<Cont> Session<Rx<Cont>> {
    /// Try to receive an `i32` and advance (fallible, as #23 — the value comes from a peer who may never send).
    /// On `Err(self)` the still-`live` handle is handed back; dropping *it* fires the bomb.
    #[inline]
    pub fn recv(self) -> Result<(i32, Session<Cont>), Self> {
        let got = self.t.borrow_mut().val.take();
        match got {
            Some(x) => Ok((x, self.advance())),
            None => Err(self),
        }
    }
}

impl Session<End> {
    /// The **one clean exit**: complete the session. Available only at `End`. Defuses the bomb (`live = false`),
    /// so the handle drops without aborting. This is the *sanctioned* terminator — the answer that is **not**
    /// `mem::forget`.
    #[inline]
    pub fn finish(mut self) {
        self.t.borrow_mut().completed = true;
        self.live = false;
    }
}
