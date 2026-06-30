//! # Twenty-fifth seam — MULTIPARTY session (n-party progress): the global-coherence step.
//!
//! *(Project ordinal: the twenty-fifth primitive. NOT a fourth-domain bid — it stays in the **third confirmed
//! domain (liveness)** and answers the question the twenty-third seam left open and the twenty-fourth seam
//! named. #23's deflation #1 conceded it outright: "*two interleaved `session()` calls type-check a mutual-wait
//! deadlock; **global deadlock-freedom is out of reach of binary session types — it needs
//! multiparty/Kobayashi typing**.*" #24 (`delegation`) then *named the coordinate* — progress/deadlock-freedom
//! — as distinct from completion and leak-freedom. This seam builds the multiparty thing and asks **where on
//! the trilemma the progress coordinate lives at n-party scale**: can the affine floor carry the *global*
//! progress multiparty session types promise, or does that property live entirely above the floor?)*
//!
//! ## The primitive — three parties, assembled from binary edges
//!
//! A multiparty session is built here from the twenty-third seam's binary channels (`crate::binary_session`):
//! each directed edge `X→Y` is one binary [`session`], and a *role* is a bundle of the edges it touches, driven
//! in a role-local order. This is deliberate — the whole argument is that **multiparty progress is NOT
//! reducible to the binary duality of the edges it is built from**. Two role typestates:
//! - [`RecvThenSend`] — a role that must **receive before it may send** (the order is type-enforced: `recv`
//!   consumes the role and yields a [`JustSend`]; there is no `send` method before that).
//! - [`JustSend`] / [`JustRecv`] — a pure source / pure sink (the relay's endpoints).
//!
//! Two constructors mint role bundles from the **same** per-edge floor — only the global wiring differs:
//! - [`relay`] — a **coherent** protocol `A→B→C`: a source ([`JustSend`]), a forwarder ([`RecvThenSend`]), a
//!   sink ([`JustRecv`]). It has a source role, so it can start; it runs to completion.
//! - [`ring`] — an **incoherent** cycle `A→B→C→A` where **every** role is [`RecvThenSend`] (receive from its
//!   predecessor, then send to its successor). Every binary edge is perfectly dual and every role-local type is
//!   a well-formed affine chain — **yet the composition has no source, so it cannot start**: the per-edge floor
//!   *type-checks a globally-deadlocked system*. This is #23's asserted "interleaved mutual-wait deadlock," now
//!   a compiled artifact, at three parties.
//!
//! ## What the floor carries — and the global property it does not
//!
//! 1. **Role-local order** — carried by affine typestate, exactly as #18/#23. `RecvThenSend` exposes only
//!    `recv`; `send` exists only on the `JustSend` it yields. An out-of-order op is a type error.
//! 2. **Per-edge duality / relative progress** — carried by `HasDual` on each binary edge (#23's positive): no
//!    *single edge* has both peers blocked on `recv`.
//! 3. **GLOBAL progress (deadlock-freedom) — NOT carried, and this is the point.** Deadlock-freedom of the whole
//!    composition is the property that *there exists a terminating schedule* — equivalently (here) that the
//!    communication topology has a **source** role. That is a property of the *set* of roles, which **no
//!    per-edge or per-role re-import rule can range over**: [`ring`] and [`relay`] are built from the identical
//!    building block; only the global topology differs, and the floor cannot see it. `ring()` compiles.
//!
//! ### Role-local order IS carried — a `RecvThenSend` cannot send before it receives:
//! ```compile_fail
//! # use seam_spsc::multiparty::ring;
//! let (a, _b, _c) = ring();
//! let _ = a.send(1); // ERROR: no method `send` on RecvThenSend — must recv (from C) before sending (to B)
//! ```
//!
//! ### Global progress is NOT carried — `ring()` type-checks a system with no terminating schedule:
//! ```
//! # use seam_spsc::multiparty::ring;
//! let (a, _b, _c) = ring();
//! // Every role must receive first, but no role can send first — the ring has no source.
//! // A waits on C, who waits on B, who waits on A: the deadlock the floor accepted.
//! assert!(a.recv().is_err());
//! ```
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_multiparty, r32): SOUND, but the gauntlet FAILED on the §6
//! ## DELIVERABLE — the REJECTION mode (3rd, after `typed_nat` #8 and `delegation` #24). My "generative seam is
//! ## the UNIQUE home of global progress" claim was a **false dichotomy**, refuted (again) from my own citations.
//!
//! The **re-import rule is sound** — the horn-classifier ruled **SEAM-COSTS [horn 3]** (no UB: the worst case is
//! a memory-safe recv-`Err` hang, not undefined behavior; structural `HasDual` keeps Horn 2 decidable). But the
//! **judge ruled FAILED** on the in-scope §6 deliverable, and I accept it. This is the method's **rejection
//! mode** (the seam is sound, like `typed_nat` #8 and `delegation` #24, but its deliverable is rejected), firing
//! for the **third time** — and, as with #24, **refuted from my own cited lineage**.
//!
//! **The defect (a false dichotomy).** I staked the deliverable on global progress living **only**
//! whole-protocol, in a *generative* projection of a coherent global type — and explicitly asked the panel to
//! answer "no" to "*could any per-role discipline (no global type) reject `ring`?*" **That is false.** Three
//! seats converged on the middle rung I omitted: **per-channel priority / obligation typing** — Kobayashi
//! (linear-π deadlock-freedom via levels/obligations), Padovani, **Dardha–Gay (FoSSaCS 2018, "A New Approach to
//! Deadlock-Freedom… via priorities")**, Kokke–Dardha — which is **per-channel, compositional, non-generative,
//! and uses no global type**, yet rejects the source-less ring *exactly* via `prio(recv) < prio(send)` chaining
//! to a contradictory `p < p` around the cycle, while admitting the relay (acyclic priorities). The `ring`/
//! `relay` pair is that tradition's own running example. Worse: I **mis-filed Kobayashi in §5 as *supporting*
//! the whole-protocol thesis when he is the *counterexample*** — collapsing rung (b) into rung (c) to manufacture
//! the result. The honest taxonomy has **three rungs**, not two:
//! - **(a) per-edge `HasDual` floor** — does **not** carry global progress (`ring()` type-checks a deadlock).
//! - **(b) per-channel priority/obligation typing** (Kobayashi; Dardha–Gay) — rejects the ring *compositionally,
//!   with no global type*, but needs a **whole-program acyclicity check** on the priorities.
//! - **(c) generative whole-protocol global-type projection** (Honda–Yoshida–Carbone MPST; `mpstthree`) —
//!   deadlock-free *by construction of the generated endpoints*.
//!
//! **What SURVIVES (the weaker, true thesis — credited by the soundness attacker).** Global progress is **not
//! concentrable per-call** and needs **strictly more than the per-edge `HasDual` floor** — `ring()` proves the
//! floor accepts a deadlock. But it is carried **whole-program**, which is *not* the same as a generative
//! global-type projection: rung (b) carries it with no global type at all. So the correct locating claim is
//! "global progress is a whole-program (not per-call) obligation, above the floor" — **not** "only a generative
//! seam." I over-reached from the true weaker claim to a false unique-home claim.
//!
//! **Recorded tradeoffs (accepted, carried into the prose):**
//! 1. **Horn-3 "concentration confirmed" rests on a degenerate witness** — single-shot, fixed `N = 3`,
//!    single-threaded. Recursion / dynamic-`N` / async erode it (the same `Rec`/`Var` and concurrency limits as
//!    #23/#24). The "deadlock" here is also degenerate: not a concurrent cyclic-wait *hang* but "no source →
//!    every `recv` is `Err`," a static starvation the single thread observes immediately.
//! 2. The per-role order + linearity core genuinely **CLOSES** (Lean `multiparty_role_order_core_closes`), like
//!    #23 — that part of the soundness story is uncontested.
//!
//! **The net finding (the probe earned its keep by being instructively WRONG — a third time).** I predicted the
//! concentration leg had exactly two rungs (floor / generative) and that global progress crowned the generative
//! one uniquely; the method **refuted the dichotomy with my own miscited source**, restoring the **three-rung**
//! taxonomy: global progress is a **whole-program obligation strictly above the per-edge floor**, dischargeable
//! by per-channel priority typing (no global type, rung b) *or* a generative projection (rung c). No new domain
//! (liveness), no new coordinate (progress, from #24), no new mode (N stays 9 — rejection's 3rd firing); the
//! seam is sound, so the broken-seam (soundness) tally **stays 7**; 25 primitives. Liveness stays the third
//! domain; progress stays its third coordinate (the lock seams #15/#16 = its *lock* face, delegation #24 = its
//! *session* face, multiparty = its *global-topology* face). Prior-art deflation confirmed (Honda–Yoshida–
//! Carbone 2008; Kobayashi; Dardha–Gay 2018; `mpstthree`) — but, once more, the deflation's own sources are
//! what refuted the headline.
//!
//! The coherent relay completes; the ring type-checks but cannot start — see `tests/multiparty_use.rs`.

use crate::binary_session::{session, Chan, End, Rx, Tx};

/// A role that has nothing to receive: it only **sends** one `i32` and closes. The relay's *source* (role A),
/// and the continuation of a [`RecvThenSend`] after it has received.
pub struct JustSend {
    out: Chan<Tx<End>>,
}

impl JustSend {
    /// Send `v` to the successor and complete this role's outbound edge.
    #[inline]
    pub fn send(self, v: i32) {
        self.out.send(v).close();
    }
}

/// A role that only **receives** one `i32` and closes. The relay's *sink* (role C).
pub struct JustRecv {
    inb: Chan<Rx<End>>,
}

impl JustRecv {
    /// Try to receive from the predecessor (fallible, as #23 — the value comes from a peer who may never send).
    /// On `Ok` the value has arrived and this role is complete; on `Err(self)` the role is handed back, still
    /// waiting.
    #[inline]
    pub fn recv(self) -> Result<i32, Self> {
        match self.inb.recv() {
            Ok((v, end)) => {
                end.close();
                Ok(v)
            }
            Err(inb) => Err(JustRecv { inb }),
        }
    }
}

/// A role that must **receive before it may send** — the relay's *forwarder* (role B) and every member of the
/// [`ring`]. The recv-before-send order is **type-enforced**: this struct exposes only [`RecvThenSend::recv`],
/// and `send` lives on the [`JustSend`] that a successful `recv` yields. There is no way to send first.
pub struct RecvThenSend {
    inb: Chan<Rx<End>>,
    out: Chan<Tx<End>>,
}

impl RecvThenSend {
    /// Try to receive from the predecessor. On `Ok` you get the value and a [`JustSend`] (now you may forward it
    /// to the successor); on `Err(self)` the role is handed back, **still blocked on the receive** — in the
    /// [`ring`] this `Err` never clears, because no role can reach its send.
    #[inline]
    pub fn recv(self) -> Result<(i32, JustSend), Self> {
        // Disjoint partial moves: `self.inb` is consumed by `recv`; `self.out` is threaded into either branch.
        match self.inb.recv() {
            Ok((v, end)) => {
                end.close();
                Ok((v, JustSend { out: self.out }))
            }
            Err(inb) => Err(RecvThenSend { inb, out: self.out }),
        }
    }
}

/// Mint a **coherent** three-party relay `A→B→C` as one bundle from a single (acyclic) global protocol — the
/// *generative* seam done right. It has a **source** role ([`JustSend`], role A), so a terminating schedule
/// exists: A sends, B forwards, C receives. Progress holds **by construction of this bundle**, not by any
/// per-edge type.
#[inline]
pub fn relay() -> (JustSend, RecvThenSend, JustRecv) {
    let (a_ab, b_ab) = session::<Tx<End>>(); // edge A→B
    let (b_bc, c_bc) = session::<Tx<End>>(); // edge B→C
    (
        JustSend { out: a_ab },
        RecvThenSend { inb: b_ab, out: b_bc },
        JustRecv { inb: c_bc },
    )
}

/// Mint an **incoherent** three-party ring `A→B→C→A` where every role is [`RecvThenSend`] — receive from the
/// predecessor, then send to the successor. Every binary edge is perfectly dual and every role-local type is a
/// well-formed affine chain, **yet there is no source**: no role can send before it receives, and no role can
/// receive before someone sends, so the composition has **no terminating schedule**. The per-edge floor
/// type-checks this globally-deadlocked system; a trusted projection of a *coherent* global type would never
/// emit it, but the raw floor cannot forbid it. This is the floor-mismatch the seam is about.
#[inline]
pub fn ring() -> (RecvThenSend, RecvThenSend, RecvThenSend) {
    let (a_ab, b_ab) = session::<Tx<End>>(); // edge A→B
    let (b_bc, c_bc) = session::<Tx<End>>(); // edge B→C
    let (c_ca, a_ca) = session::<Tx<End>>(); // edge C→A
    (
        RecvThenSend { inb: a_ca, out: a_ab }, // A: recv from C, send to B
        RecvThenSend { inb: b_ab, out: b_bc }, // B: recv from A, send to C
        RecvThenSend { inb: c_bc, out: c_ca }, // C: recv from B, send to A
    )
}
