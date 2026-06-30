//! # Twenty-seventh seam — PROJECTED sessions: the #26 salvage, deadlock-freedom by construction.
//!
//! *(Project ordinal: the twenty-seventh primitive. The direct salvage of the twenty-sixth (`priority`), which
//! **broke** (SEAM-BREAKS [horn 1]) because its per-role constructor `role::<Pi, Po>` was **public**: a caller
//! wired three locally-valid `role::<N1, N2>` relays into a source-less ring that compiled and deadlocked. The
//! r33 verdict named the real carrier of deadlock-freedom — not the per-channel `Lt` discipline (which carries
//! nothing global) but the **trusted, edge-consistent constructor** (`relay()` was a "mini-projection"). This
//! seam makes that explicit: it **seals the per-role constructor** and vends *only* trusted builders. The
//! question: does that turn the break into a sound seam — and at what cost?)*
//!
//! ## The primitive — a sealed constructor that mints deadlock-free assemblies
//!
//! Same role shapes as #26 ([`Source`], [`Sink`], [`Worker`]`<Pi, Po>` with `Pi: Lt<Po>`, over #23's binary
//! edges and #8's sealed `typed_nat::Lt`), but the per-role constructor `worker()` is **module-private** — the
//! one keystroke #26 skipped. The only way to obtain roles is a **trusted builder** ([`relay`], [`relay4`]) that
//! threads **one strictly-increasing `Nat` per edge** (`N0 < N1 < N2 < …`). Two safety mechanisms compound:
//! 1. **Access** — there is no public `worker()`, so safe code cannot mint an arbitrary role and wire it into a
//!    cyclic assembly (the exact #26 exploit is now uncompilable: see the `compile_fail` below).
//! 2. **Structure** — a monotone-priority chain *cannot close into a ring*: the closing edge would need a
//!    *decreasing* `Lt` step (`p_last: Lt<p_first>` with `p_first < … < p_last`), and `typed_nat::Lt` has no
//!    such instance. So even the foundation author cannot write a deadlocking builder.
//!
//! Together these make **deadlock-freedom hold by construction**: every assembly a safe caller can obtain has a
//! source and a strictly-increasing priority order, hence a terminating schedule.
//!
//! ### The #26 exploit no longer compiles — the per-role constructor is sealed:
//! ```compile_fail
//! use seam_spsc::projected::worker; // ERROR: function `worker` is private — no public per-role constructor
//! ```
//!
//! ### The trusted builders mint schedulable assemblies that run to completion — see `tests/projected_use.rs`.
//!
//! ## The cold panel's verdict (REPORT_SEAM_SPEC_projected, r34): SEAM-COSTS [horn 3] — SOUND, the salvage WORKS,
//! ## but the gauntlet FAILED the §6 deliverable (the REJECTION mode, 4th firing): "concentrated entirely" and
//! ## "deadlock-freedom by construction" overclaimed — a *progress residue* stays on the untrusted consumer.
//!
//! The salvage is **sound** and **resolves the #26 break**: the horn-classifier ruled **SEAM-COSTS [horn 3]**,
//! every attacker conceding Horn 1 (the #26 deadlocking ring is **unconstructible** — no public `worker()`, no
//! decreasing `Lt`) and Horn 2 (Θ(depth)). But the **judge ruled FAILED** on the §6 deliverable — the **rejection
//! mode** (a sound seam whose deliverable is rejected), firing the **fourth time** (after `typed_nat` #8,
//! `delegation` #24, `multiparty` #25; #26 was a *break*, not a rejection). I accept it.
//!
//! **The overclaim (the genuine flaw).** My §4 claimed "**deadlock-freedom by construction**" and that the
//! "consumer carries nothing global." False in kind. The monotone-priority builder proves a deadlock-free
//! schedule **exists** (the `Nat`s are a *static* witness), but **affine Rust never forces the runtime to follow
//! it** — nothing at the type level compels the caller to drive the witnessed schedule. So a **progress residue**
//! stays on the untrusted consumer: *run-all, in-order, no-drop* (actually drive every minted role, in a
//! priority-respecting order, without dropping one). The honest phrasing is "**schedulability by construction,
//! given all roles execute**." Concentration is **partial, not total**: the trusted constructor carries the
//! **topology** obligation (no cycle — the genuine #26 fix), but the **completion/execution** obligation never
//! left the consumer — it is the same #17/#18/#23 horn-3 completion gap. The giveaway, from prior art: generated
//! MPST endpoints (Scribble, `mpstthree`) are deadlock-free **only under the standard assumption that every
//! participant executes its endpoint** — I silently imported that assumption (calling it the "single-threaded
//! witness") while claiming the consumer carries nothing; **the execution assumption *is* the leftover global
//! obligation.**
//!
//! **Secondary deflation (the cost was OVERstated, not understated).** My "only acyclic topologies, each needing
//! a foundation-authored builder" overstates the price: acyclic **generality is recoverable via one generic
//! trusted combinator** (`mpstthree` proves it) — the expressiveness attacks *remove* cost, not add it. The
//! vacuity charge (excluding cyclic topologies is feature-removal, not re-import) is real but is the design's
//! **pre-conceded** expressiveness corner, not a soundness defect.
//!
//! **The refined finding (what the salvage actually teaches).** The **progress coordinate decomposes** into two
//! sub-obligations: **deadlock-freedom / topology** (no cyclic wait — which a trusted whole-program constructor
//! *can* carry, by construction, resolving #26) and **completion / execution** (run-all, in-order, no-drop —
//! which the consumer *always* carries, the #17/#18/#23 residue, because affine Rust witnesses a schedule but
//! never forces it). I conflated the two. So the salvage **confirms** the #24–26 arc with a sharper split: the
//! constructor carries progress's *topology* half; the *completion* half never left the consumer. The seam is
//! sound, so the **FAILED broken-seam tally STAYS 8** (a deliverable rejection, not a soundness break — like
//! `typed_nat`/`delegation`/`multiparty`); **N stays 9** (rejection's 4th firing); 27 primitives; liveness stays
//! the third domain. Lean `projected_schedulability_closes` (a `..._closes` — the sound *schedulability*-by-
//! construction core closes; the first since #25). Prior-art deflation confirmed (Scribble, `mpstthree`,
//! deadlock-free-by-construction-of-generated-code) — and, as ever, its own sources sharpened the correction.

use crate::binary_session::{session, Chan, End, Rx, Tx};
use crate::typed_nat::{Lt, N0, N1, N2};
use core::marker::PhantomData;

/// A pure **source** role: it only sends (at priority `Po`) and closes. The head of every acyclic assembly.
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

/// A pure **sink** role: it only receives (at priority `Pi`) and closes. The tail of every acyclic assembly.
pub struct Sink<Pi> {
    inb: Chan<Rx<End>>,
    _p: PhantomData<Pi>,
}

impl<Pi> Sink<Pi> {
    /// Try to receive from the predecessor (fallible, as #23).
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

/// A **forwarder** role: receives at `Pi`, then sends at `Po`, under `Pi: Lt<Po>` (as #26). The difference from
/// #26 is entirely in *construction*: there is **no public constructor** for a `Worker`, and its fields are
/// private — so safe code outside this module can name `Worker<Pi, Po>` but can never mint one to wire into a
/// cyclic assembly. Roles exist only inside the trusted builders below.
pub struct Worker<Pi, Po> {
    inb: Chan<Rx<End>>,
    out: Chan<Tx<End>>,
    _p: PhantomData<(Pi, Po)>,
}

impl<Pi, Po> Worker<Pi, Po>
where
    Pi: Lt<Po>,
{
    /// Try to receive (at `Pi`), then become a [`Source`] able to send (at `Po`).
    #[inline]
    pub fn recv(self) -> Result<(i32, Source<Po>), Self> {
        match self.inb.recv() {
            Ok((v, end)) => {
                end.close();
                Ok((v, Source { out: self.out, _p: PhantomData }))
            }
            Err(inb) => Err(Worker { inb, out: self.out, _p: PhantomData }),
        }
    }
}

/// The **sealed** per-role constructor — `pub(crate)` was rejected too; this is *module-private* (the #26 fix).
/// Only the trusted builders in this module may mint a `Worker`, and they always thread strictly-increasing,
/// edge-consistent priorities — so no caller, and no other module, can construct a role with priorities that
/// would close a deadlock cycle.
#[inline]
fn worker<Pi, Po>(inb: Chan<Rx<End>>, out: Chan<Tx<End>>) -> Worker<Pi, Po>
where
    Pi: Lt<Po>,
{
    Worker { inb, out, _p: PhantomData }
}

/// Mint a deadlock-free **3-party** relay `A→B→C`, threading one strictly-increasing `Nat` per edge
/// (`p_AB = N0 < p_BC = N1`). A source exists and the priority order is monotone, so a terminating schedule
/// exists *by construction*. Because [`worker`] is sealed, no caller can replace `B` with a role that would
/// close a cycle.
#[inline]
pub fn relay() -> (Source<N0>, Worker<N0, N1>, Sink<N1>) {
    let (a_ab, b_ab) = session::<Tx<End>>(); // edge A→B at priority N0
    let (b_bc, c_bc) = session::<Tx<End>>(); // edge B→C at priority N1
    (
        Source { out: a_ab, _p: PhantomData },
        worker::<N0, N1>(b_ab, b_bc),
        Sink { inb: c_bc, _p: PhantomData },
    )
}

/// Mint a deadlock-free **4-party** pipeline `A→B→C→D`, threading `N0 < N1 < N2`. Demonstrates the trusted
/// builder *generalizes* over acyclic shapes: any-length monotone chain is deadlock-free by the same
/// construction, while a *cycle* is unconstructible (closing it needs a decreasing `Lt` step, which has no
/// instance). Expressiveness is bounded to acyclic topologies — the trilemma's cost, paid in the constructor.
#[inline]
pub fn relay4() -> (Source<N0>, Worker<N0, N1>, Worker<N1, N2>, Sink<N2>) {
    let (a_ab, b_ab) = session::<Tx<End>>(); // A→B at N0
    let (b_bc, c_bc) = session::<Tx<End>>(); // B→C at N1
    let (c_cd, d_cd) = session::<Tx<End>>(); // C→D at N2
    (
        Source { out: a_ab, _p: PhantomData },
        worker::<N0, N1>(b_ab, b_bc),
        worker::<N1, N2>(c_bc, c_cd),
        Sink { inb: d_cd, _p: PhantomData },
    )
}
