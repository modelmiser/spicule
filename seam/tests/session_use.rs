//! Eighteenth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves both components of the composite precondition in SAFE code: the *safety* half (operations run in
//! protocol order, enforced by typestate — out-of-order is a compile error, see the module doctests) and the
//! *completion* half's break (a safe `mem::forget` abandons the session mid-protocol — the floor cannot
//! compel the protocol to reach `End`). The transport's byte-moving is the trusted core; we attack neither.
#![forbid(unsafe_code)]

use seam_spsc::session::{open, Rx, Transport, Tx, End};

#[test]
fn a_well_typed_session_runs_in_order_and_completes() {
    let mut t = Transport::new();
    let c = open::<Tx<Rx<End>>>(&mut t); // protocol: send i32, recv i32, end
    let c = c.send(7);
    let (v, c) = c.recv();
    assert_eq!(v, 7);
    c.close();
    assert!(t.completed(), "a session run to End and closed marks the transport complete");
}

#[test]
fn the_empty_protocol_completes_trivially() {
    // Opening directly at End is the empty protocol — closing it is the whole (trivial) session.
    let mut t = Transport::new();
    let c = open::<End>(&mut t);
    c.close();
    assert!(t.completed());
}

#[test]
fn the_break_a_session_can_be_abandoned_mid_protocol_by_forget() {
    // `mem::forget` is SAFE (this file forbids `unsafe` and still compiles). It abandons the handle
    // mid-protocol, so `close` never runs and the session never completes — the affine floor cannot compel
    // a *linear* protocol to run to its end (the #17 gap, here costing protocol progress).
    let mut t = Transport::new();
    {
        let c = open::<Tx<Rx<End>>>(&mut t);
        let c = c.send(7); // now at Rx<End>, mid-protocol
        core::mem::forget(c); // abandon: never recv, never close
    }
    assert!(
        !t.completed(),
        "the protocol never reached End — a dual peer would block on the missing recv forever"
    );
}

#[test]
fn the_break_dropping_a_handle_also_abandons_the_protocol() {
    // A plain `drop` (the implicit scope-end drop) abandons just as silently as `forget` — `Chan` has no
    // Drop that could detect or repair an unfinished protocol (that would require linearity).
    let mut t = Transport::new();
    {
        let c = open::<Tx<Rx<End>>>(&mut t);
        let _c = c.send(7); // bound to `_c`, dropped at scope end mid-protocol
    }
    assert!(!t.completed(), "scope-end drop mid-protocol leaves the session incomplete, silently");
}
