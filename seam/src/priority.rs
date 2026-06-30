//! # Twenty-sixth seam — PRIORITY-TYPED sessions: can the floor carry rung (b)?
//!
//! *(Project ordinal: the twenty-sixth primitive. NOT a fourth-domain bid — it stays in the **third confirmed
//! domain (liveness)** and probes the exact rung the twenty-fifth seam's rejection handed back. #25
//! (`multiparty`) staked a false dichotomy — that global progress lives *only* whole-protocol in a *generative*
//! global-type projection — and the cold panel named the omitted middle rung: **per-channel priority/obligation
//! typing** (Kobayashi linear-π; Padovani; Dardha–Gay, FoSSaCS 2018; Kokke–Dardha), which rejects the
//! source-less ring *compositionally, with no global type*, via `prio(recv) < prio(send)` chaining to a
//! contradictory `p < p` on the cycle. This seam **builds that rung** in Rust and asks: can the affine floor
//! carry it — does the deadlocking ring become *un-typable* — and is that concentration **per-call** or does it
//! still rest on a **whole-program** obligation, exactly as #25's surviving weaker thesis predicts?)*
//!
//! ## The primitive — a per-channel priority discipline over binary edges
//!
//! Built on #23's binary edges (`crate::binary_session`) and #8's sealed type-level naturals
//! (`crate::typed_nat`, whose [`Lt`] is Peano `<`: `Z: Lt<S<M>>`, `S<I>: Lt<S<M>>` iff `I: Lt<M>`, **no
//! reflexive `Lt<N>`**). A *role* receives on its inbound edge at priority `Pi`, then sends on its outbound edge
//! at priority `Po` — and the seam's discipline is the bound **`Pi: Lt<Po>`**: you cannot construct a role whose
//! inbound priority is not strictly below its outbound. That is the Kobayashi/Dardha–Gay rule (an action may not
//! block on one of equal-or-higher priority), encoded as a const trait bound the compiler discharges. Three
//! role shapes: [`Source`] (pure send, no constraint), [`Sink`] (pure recv), [`Relay`]`<Pi, Po>` (recv-then-send,
//! `Pi: Lt<Po>`).
//!
//! ## The test — the ring that #25's bare floor accepted, now UN-typable
//!
//! In #25 `ring()` *type-checked* a globally-deadlocked system: the per-edge `HasDual` floor was blind to the
//! source-less cycle. Under priorities the cycle is rejected. The ring `A→B→C→A` (every role recv-then-send,
//! sharing edge priorities `p_AB`, `p_BC`, `p_CA`) forces:
//! - A: recv `p_CA`, send `p_AB` ⇒ `p_CA: Lt<p_AB>`
//! - B: recv `p_AB`, send `p_BC` ⇒ `p_AB: Lt<p_BC>`
//! - C: recv `p_BC`, send `p_CA` ⇒ `p_BC: Lt<p_CA>`
//!
//! Chaining: `p_CA < p_AB < p_BC < p_CA`, i.e. `p_CA: Lt<p_CA>` — which `typed_nat` has **no instance for**, for
//! any concrete assignment. No priorities the programmer can pick satisfy all three role bounds, so the ring is
//! **untypable** (a `compile_fail`, below). The acyclic relay `A→B→C` (a source, no cycle) types with `p_AB=N0 <
//! p_BC=N1` and runs to completion.
//!
//! ### The cyclic role is untypable — the priority discipline rejects the deadlock the bare floor accepted:
//! ```compile_fail
//! # use seam_spsc::priority::role;
//! # use seam_spsc::typed_nat::{N0, N2};
//! # use seam_spsc::binary_session::{session, Tx, End};
//! let (_tx, rx) = session::<Tx<End>>();    // some inbound edge, here at priority N2
//! let (tx2, _rx2) = session::<Tx<End>>();   // some outbound edge, here at priority N0
//! // A ring role receiving at priority N2 and sending at N0 — the cyclic step `p_BC < p_CA` (N2 < N0):
//! let _c = role::<N2, N0>(rx, tx2);          // ERROR: `S<S<Z>>: Lt<Z>` is not satisfied — cycle untypable
//! ```
//!
//! ### The acyclic relay types and runs — see `tests/priority_use.rs`.
//!
//! ## HYPOTHESIS (pre-gauntlet — the §6 deliverable is what the cold panel adjudicates)
//!
//! This seam is built to **confirm**, not extend, #25's surviving weaker thesis — a deliberate departure after
//! three deliverable rejections (#8, #24, #25) caused by building "the recognized feature" to crown a *new*
//! claim. The expected finding: priority typing (rung b) **does** carry the deadlock-freedom *check* into Rust's
//! types — the ring is genuinely untypable, where #25's bare floor accepted it — at the **cost** that (1) the
//! programmer must **supply a globally-consistent, acyclic priority assignment** (the `Lt` bounds are local and
//! per-call, but their joint satisfiability is the *whole-program acyclicity* obligation the type system only
//! *checks*, never *searches*), and (2) edge-priority **consistency across the two endpoints** of each edge is
//! maintained by the *constructor* ([`relay`] threads the same `Nat`), not by any per-endpoint type — so the
//! guarantee still rests on a trusted whole-program assignment, a mini-projection. This **confirms** #25: global
//! progress is not concentrable per-call; it needs strictly more than the per-edge floor, dischargeable here by
//! rung (b)'s local bounds **plus** a whole-program assignment. It **unifies** progress's faces: the same shape
//! as `ordered_locks` #15/#16 (type-level rank carries local order; whole-program lock-acyclicity escapes) and
//! reuses `typed_nat` #8's `Lt`. Expected verdict: **SEAM-COSTS [horn 3]** — the priority discipline closes the
//! local order/acyclicity *check*; the consistent-acyclic *assignment* is the horn-3 whole-program cost. Limits
//! (pre-conceded): only static, fixed-shape, assignable protocols (recursion/dynamic-N need inference, as
//! Scribble automates); a programmer can always drop to the bare floor (#25) and rebuild the deadlock — the
//! guarantee is conditional on opting into the discipline.

use crate::binary_session::{session, Chan, End, Rx, Tx};
use crate::typed_nat::{Lt, N0, N1};
use core::marker::PhantomData;

/// A pure **source** role: it only sends (at priority `Po`) and closes — no inbound, so no ordering constraint.
/// The relay's role A.
pub struct Source<Po> {
    out: Chan<Tx<End>>,
    _p: PhantomData<Po>,
}

impl<Po> Source<Po> {
    /// Send `v` to the successor and complete this role's outbound edge.
    #[inline]
    pub fn send(self, v: i32) {
        self.out.send(v).close();
    }
}

/// A pure **sink** role: it only receives (at priority `Pi`) and closes. The relay's role C.
pub struct Sink<Pi> {
    inb: Chan<Rx<End>>,
    _p: PhantomData<Pi>,
}

impl<Pi> Sink<Pi> {
    /// Try to receive from the predecessor (fallible, as #23). On `Ok` the value has arrived and the role is
    /// complete; on `Err(self)` the role is handed back, still waiting.
    #[inline]
    pub fn recv(self) -> Result<i32, Self> {
        match self.inb.recv() {
            Ok((v, end)) => {
                end.close();
                Ok(v)
            }
            Err(inb) => Err(Sink { inb, _p: PhantomData }),
        }
    }
}

/// A **forwarder/member** role: it receives at priority `Pi`, then sends at priority `Po`. The priority
/// discipline is the bound on its impl — **`Pi: Lt<Po>`** (Kobayashi/Dardha–Gay: receive at a strictly lower
/// priority than you send). A `Relay<Pi, Po>` whose `Pi` is not `Lt<Po>` is an inhabited type with **no usable
/// method** — and [`role`] cannot construct it at all. This is rung (b): a per-channel, decidable
/// (`typed_nat::Lt`, Θ(depth)) discipline that needs **no global type**.
pub struct Relay<Pi, Po> {
    inb: Chan<Rx<End>>,
    out: Chan<Tx<End>>,
    _p: PhantomData<(Pi, Po)>,
}

impl<Pi, Po> Relay<Pi, Po>
where
    Pi: Lt<Po>,
{
    /// Try to receive (at `Pi`), then become a [`Source`] able to send (at `Po`). Available only when the
    /// priority discipline `Pi: Lt<Po>` holds — so a role that would close a deadlock cycle has no `recv`.
    #[inline]
    pub fn recv(self) -> Result<(i32, Source<Po>), Self> {
        match self.inb.recv() {
            Ok((v, end)) => {
                end.close();
                Ok((v, Source { out: self.out, _p: PhantomData }))
            }
            Err(inb) => Err(Relay { inb, out: self.out, _p: PhantomData }),
        }
    }
}

/// Build a priority-typed forwarder that receives at `Pi` then sends at `Po`. The bound `Pi: Lt<Po>` is the
/// per-channel priority discipline: **construction is impossible** unless the inbound priority is strictly below
/// the outbound. A ring role whose priorities would close the cycle (e.g. `role::<N2, N0>`) fails this bound.
#[inline]
pub fn role<Pi, Po>(inb: Chan<Rx<End>>, out: Chan<Tx<End>>) -> Relay<Pi, Po>
where
    Pi: Lt<Po>,
{
    Relay { inb, out, _p: PhantomData }
}

/// Mint a **coherent** priority-typed relay `A→B→C` with an acyclic assignment (`p_AB = N0 < p_BC = N1`). The
/// source role A makes a terminating schedule exist, and the single forwarder's bound `N0: Lt<N1>` is
/// satisfiable — so the bundle both type-checks and completes. (Compare `multiparty::ring`, which type-checked a
/// *deadlock* because the bare floor had no such discipline; here no acyclic assignment exists for the ring, so
/// the ring is untypable — see the module-doc `compile_fail`.)
#[inline]
pub fn relay() -> (Source<N0>, Relay<N0, N1>, Sink<N1>) {
    let (a_ab, b_ab) = session::<Tx<End>>(); // edge A→B at priority N0
    let (b_bc, c_bc) = session::<Tx<End>>(); // edge B→C at priority N1
    (
        Source { out: a_ab, _p: PhantomData },
        role::<N0, N1>(b_ab, b_bc),
        Sink { inb: c_bc, _p: PhantomData },
    )
}
