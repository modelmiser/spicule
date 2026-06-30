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
//! not UB. This is the *remote* analogue of `linear` #20's *local* must-run-cleanup: there, `mem::forget`
//! skipped your own cleanup; here it skips a `Drop` that was the **peer's** only guarantee of being answered.
//!
//! ## The two facts a "must-complete" discipline needs
//!
//! 1. **Drop ⇒ abort.** A `live` `Session` that is dropped before `End` fires the bomb — so the *only* way past
//!    a `Session` is to advance it (`send`/`recv`) or `finish` it. The abort makes abandonment observable.
//! 2. **No silent skip.** For "must complete" to hold, there must be **no safe way** to make a `Session`'s
//!    `Drop` *not run* while also not completing it. THIS is the load-bearing claim — and the one #20 already
//!    falsified for closures: `core::mem::forget` is **safe**, consumes the value by one affine move, and skips
//!    `Drop` (the leakpocalypse, RFC 1066). `ManuallyDrop`, `Box::leak`, and `Rc`-cycles do the same.
//!
//! ## HYPOTHESIS (pre-gauntlet — the §6 deliverable is what the cold panel adjudicates)
//!
//! **SEAM-BREAKS** — the must-complete guarantee is **false in safe code**, the **session face of `linear` #20's
//! must-run break**, just as `priority` #26 was the session face of `ordered_locks` #15/#16's lock-order break.
//! `core::mem::forget(session)` skips the bomb with no `unsafe`, abandoning the session and **starving the peer**
//! with no abort (`tests/must_complete_use.rs` shows it). The pre-registered claims the panel should test: (a)
//! the **peer / two-endpoint** dimension does **not** change the break — `mem::forget` skips `Drop` whether the
//! effect is local (#20) or remote (here); (b) even the **abort-bomb** (the #9 `take` discharge — sound, paying a
//! non-recoverable abort) **cannot** force completion, because `mem::forget` bypasses `Drop` *before* the bomb
//! can fire (it is not an unwind the bomb can catch). So "must complete" is a **clean NONE** at the type level —
//! "runs to `End`" is a **linear** requirement and Rust's floor is **affine** (*at most* once), the same
//! *wrong-structural-logic* (affine vs linear) verdict as #20, not a domain mismatch. This **unifies** the whole
//! session arc's completion residue (#17/#18/#23/#27/#28) as **one** irreducible linear gap: there is no
//! discipline — not even a cost-paying abort — that compels a session to `End` on an affine floor. No new domain
//! (liveness), coordinate (completion is the #17 residue, not new), or mode (SEAM-BREAKS is one of the nine).
//! Deflates onto deep folklore (the leakpocalypse / RFC 1066; the removal of `thread::scoped`, RFC 1084;
//! `session_types`/Jespersen–Munksgaard–Larsen 2015's affine-vs-linear gap). Lean
//! `must_complete_typed_discipline_factors` (a `..._factors`, like `linear`/`priority` — the affine *walk*
//! factors; the linear *completion* does not).

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
