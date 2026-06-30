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
//! ## HYPOTHESIS (pre-gauntlet — the §6 deliverable is what the cold panel adjudicates)
//!
//! Multiparty does **not** open a new domain (still liveness) or a new coordinate (progress, named by #24) or a
//! new mode (N stays 9). Its content is a sharpening of the trilemma's **concentration leg (horn 3)**: a
//! *global* property like deadlock-freedom **cannot be concentrated in a per-call re-import rule** — it can only
//! be concentrated **whole-protocol**, in a trusted projection of a coherent *global type* to per-role *local
//! types* (Honda–Yoshida–Carbone, POPL 2008; Coppo–Dezani–Yoshida–Padovani, MSCS 2016; the `mpstthree` Rust
//! crate, Lagaillardie–Neykova–Yoshida, COORDINATION 2020 — deadlock-freedom holds *by construction of the
//! generated endpoints*, not by checking arbitrary compositions). So the seam's **location is property-scoped**:
//! local properties (order, linearity, the per-role core) concentrate per-call and **CLOSE** as in #23; global
//! progress concentrates only whole-protocol — a *generative* seam, not a re-import rule. The per-edge floor
//! demonstrably accepts a deadlocking ring, so global progress is a **floor-mismatch for the re-import SHAPE** —
//! but memory-safe throughout (a deadlocked session is not UB; leakpocalypse), so this is **not** a soundness
//! break (the FAILED tally should stay 7). The expected verdict: **SEAM-COSTS [horn 3]** for the per-role core;
//! global progress is the whole-protocol concentration the floor cannot supply.
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
