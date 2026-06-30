//! Consumer-side proof for the thirty-second seam (`guarded`): the salvage of #31.
//!
//! The salvage's negative half — that the unguarded `Rec<Var>` = µX.X cannot be unfolded — is the
//! `compile_fail` doctest CF1 in the module (it cannot live here because it does not compile, which is the
//! guarantee). These tests exhibit the positive half: every *productively guarded* loop (an action before the
//! loop variable) still unfolds and runs exactly as in #31 — the `Guarded` marker removes ONLY the degenerate
//! spin loop, nothing useful.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::guarded::{start, End, Rec, Tx, Var};

/// A guarded loop (`Rec<Tx<Var>>`: an action `Tx` guards the variable) unfolds and runs as in #31. Bounded,
/// then dropped — guardedness buys *productivity*, not *termination*, so there is still no reachable `End`.
#[test]
fn a_guarded_loop_unfolds_and_runs() {
    let (a, b) = start::<Rec<Tx<Var>>>();

    let mut s = a.enter(); // enter requires the body `Tx<Var>: Guarded` — it is.
    for i in 0..5 {
        s = s.send(i).recur();
    }
    drop(s);

    let mut r = b.enter();
    let mut got = Vec::new();
    for _ in 0..5 {
        let (x, r2) = match r.recv() {
            Ok(pair) => pair,
            Err(_) => unreachable!("producer queued 5 values first"),
        };
        got.push(x);
        r = r2.recur();
    }
    drop(r);

    assert_eq!(got, vec![0, 1, 2, 3, 4]);
}

/// A guarded loop with a multi-action body (`Rec<Tx<Tx<Var>>>`): the head is `Tx`, so the whole body — including
/// the deeper `Var` — is guarded, and it unfolds fine. Confirms the shallow head-check admits arbitrarily deep
/// guarded bodies.
#[test]
fn a_multi_action_guarded_body_unfolds() {
    let (a, b) = start::<Rec<Tx<Tx<Var>>>>();

    let mut s = a.enter();
    for i in 0..3 {
        s = s.send(i).send(i + 100).recur();
    }
    drop(s);

    let mut r = b.enter();
    let mut got = Vec::new();
    for _ in 0..3 {
        let (x, r2) = match r.recv() {
            Ok(pair) => pair,
            Err(_) => unreachable!("queued"),
        };
        let (y, r3) = match r2.recv() {
            Ok(pair) => pair,
            Err(_) => unreachable!("queued"),
        };
        got.push(x);
        got.push(y);
        r = r3.recur();
    }
    drop(r);

    assert_eq!(got, vec![0, 100, 1, 101, 2, 102]);
}

/// A terminal-only body (`Rec<End>`): no `Var`, so vacuously guarded — `enter` is admitted, and the session
/// finishes immediately. A degenerate but *well-formed* recursion (the marker rejects `Rec<Var>`, not `Rec<End>`).
#[test]
fn a_terminal_body_is_vacuously_guarded() {
    let (a, b) = start::<Rec<End>>();
    a.enter().finish();
    b.enter().finish();
}
