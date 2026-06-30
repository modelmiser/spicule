//! Twenty-fifth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The MULTIPARTY (n-party progress) probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • a COHERENT three-party relay A→B→C runs to completion — it has a source role (A), so a terminating
//!     schedule exists; the forwarded value reaches the sink;
//!   • role-local recv-before-send order is type-enforced (the compile_fail lives in the module doc);
//!   • the global-progress FLOOR-MISMATCH: `ring()` — three perfectly-dual binary edges wired into a cycle with
//!     no source — TYPE-CHECKS, yet has no terminating schedule. Every role must receive first; nobody can send
//!     first; so every role's `recv` stays `Err` forever. The per-edge floor accepted a globally-deadlocked
//!     system. This is #23's asserted interleaved mutual-wait deadlock, now a compiled three-party artifact.

#![forbid(unsafe_code)]

use seam_spsc::multiparty::{relay, ring};

// A coherent global type (A→B→C) projects to a source, a forwarder, and a sink. Because a source exists, the
// system makes progress and completes; the value A sends reaches C through B.
#[test]
fn coherent_relay_completes() {
    let (src, fwd, sink) = relay();

    // Role A (source) sends 7 to B and completes.
    src.send(7);

    // Role B (forwarder) receives from A — its predecessor has sent, so this succeeds — then forwards to C.
    let (v, b_send) = match fwd.recv() {
        Ok(x) => x,
        Err(_) => panic!("coherent relay: B's recv from the source should be ready"),
    };
    assert_eq!(v, 7);
    b_send.send(v);

    // Role C (sink) receives the forwarded value and completes.
    let got = match sink.recv() {
        Ok(x) => x,
        Err(_) => panic!("coherent relay: C's recv from the forwarder should be ready"),
    };
    assert_eq!(got, 7, "the value A sent reached C through B");
}

// The floor-mismatch: a ring of three perfectly-dual binary edges, with every role obliged to receive before it
// sends, has NO source — so it can never start. Each role's first action is `recv`, and no role can perform the
// `send` that would unblock its successor, because reaching `send` requires having received. The result: every
// role's `recv` returns `Err`, and re-trying in any order never changes that. The type system accepted it.
#[test]
fn ring_typechecks_but_has_no_terminating_schedule() {
    let (a, b, c) = ring();

    // No role can start: every initial receive fails (nothing has been sent on any edge).
    let a = match a.recv() {
        Ok(_) => panic!("the ring has no source — A's recv cannot succeed"),
        Err(a) => a,
    };
    let b = match b.recv() {
        Ok(_) => panic!("the ring has no source — B's recv cannot succeed"),
        Err(b) => b,
    };
    let c = match c.recv() {
        Ok(_) => panic!("the ring has no source — C's recv cannot succeed"),
        Err(c) => c,
    };

    // Re-trying in any order still makes no progress — there is no schedule that breaks the cycle, because
    // unblocking any one role's recv requires its predecessor to have reached a send it can never reach.
    let _a = a.recv().err().expect("A still blocked: predecessor C never sent");
    let _c = c.recv().err().expect("C still blocked: predecessor B never sent");
    let _b = b.recv().err().expect("B still blocked: predecessor A never sent");
}

// Coherence = the existence of a source. The very same per-edge floor that builds the deadlocking ring also
// builds the progressing relay; only the global topology differs, and nothing in the per-role types distinguishes
// them. This test documents the contrast: identical building blocks, opposite global outcomes, floor blind to it.
#[test]
fn same_floor_builds_both_outcomes() {
    // The ring deadlocks (no source):
    let (a, _b, _c) = ring();
    assert!(a.recv().is_err(), "ring: no role can start");

    // The relay completes (source = A), from the identical RecvThenSend / Tx-edge building blocks:
    let (src, fwd, sink) = relay();
    src.send(1);
    let (v, b_send) = fwd.recv().ok().expect("relay has a source, so B's recv is ready");
    b_send.send(v);
    assert_eq!(sink.recv().ok(), Some(1));
}
