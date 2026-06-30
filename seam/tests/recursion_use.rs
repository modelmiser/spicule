//! Consumer-side proof for the thirty-first seam (`recursion`): single-level recursive sessions.
//!
//! The defining fact these tests exhibit: a recursive protocol built without a choice connective has **no
//! reachable `End`**, so the witness sessions *loop a bounded number of times and are then simply dropped* —
//! there is nothing to `finish`. That is not a leak bug (no `Drop` bomb here; the worst case is a starved
//! peer, a liveness matter, exactly as #29/#30 framed it); it is the seam's point — the floor types an
//! unbounded loop and cannot force it to ever leave. The `Var`-soundness discipline (a top-level `Var` has no
//! `recur`; a `Rec` head must be `enter`-ed before its body is walked) is shown by the `compile_fail` doctests
//! CF1/CF2 in the module — they cannot live here because they do not compile, which is the guarantee.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use seam_spsc::recursion::{start, Rec, Tx, Var};

/// The infinite producer/consumer (`Rec<Tx<Var>>` ↔ its dual `Rec<Rx<Var>>`), run for a BOUNDED number of
/// iterations and then dropped. Each iteration re-unfolds to the *same* type (`Tx<Var> → Var → Tx<Var>`), so
/// an ordinary `for` loop with a rebinding `let` walks it — the unbounded protocol, traversed a finite number
/// of times. Duality threads through the recursion: the producer's `Rec<Tx<Var>>` dual is exactly the
/// consumer's `Rec<Rx<Var>>`.
#[test]
fn bounded_loop_over_an_endless_protocol_then_dropped() {
    let (a, b) = start::<Rec<Tx<Var>>>();

    // Producer: enter the loop, send 5 values (each `send` → Var, each `recur` → back to Tx<Var>), then
    // ABANDON the session — there is no `End` to finish at. Plain drop; no bomb, no UB.
    let mut s = a.enter();
    for i in 0..5 {
        s = s.send(i).recur();
    }
    drop(s); // still at Tx<Var>, the loop never terminates — we just stop walking it.

    // Consumer (the dual `Rec<Rx<Var>>`): drain the 5 values in order, then likewise abandon.
    let mut r = b.enter();
    let mut got = Vec::new();
    for _ in 0..5 {
        let (x, r2) = match r.recv() {
            Ok(pair) => pair,
            Err(_) => unreachable!("producer queued 5 values first (single-threaded witness)"),
        };
        got.push(x);
        r = r2.recur();
    }
    drop(r);

    assert_eq!(got, vec![0, 1, 2, 3, 4]);
}

/// A multi-action (guarded) loop body: `Rec<Tx<Tx<Var>>>` sends two values per unfolding before looping.
/// Shows the body can be a non-trivial sequence and that `Var` is correctly the *innermost* continuation —
/// the environment `Rec<Tx<Tx<Var>>>` is threaded unchanged through both `send`s and re-unfolded by `recur`.
#[test]
fn multi_action_guarded_body_two_sends_per_unfolding() {
    let (a, b) = start::<Rec<Tx<Tx<Var>>>>();

    let mut s = a.enter();
    for i in 0..3 {
        s = s.send(i * 10).send(i * 10 + 1).recur();
    }
    drop(s);

    let mut r = b.enter(); // dual: Rec<Rx<Rx<Var>>>
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

    assert_eq!(got, vec![0, 1, 10, 11, 20, 21]);
}

/// Abandonment mid-loop is just a drop: the consumer can stop unfolding at *any* point (here after 2 of the
/// producer's 4 values) and drop the live handle. No `End` is required, no bomb fires — the unconsumed tail
/// is simply a starved-producer residue (a liveness matter, the #29/#30 completion coordinate), not UB.
#[test]
fn abandoning_a_recursive_session_mid_loop_is_a_plain_drop() {
    let (a, b) = start::<Rec<Tx<Var>>>();

    let mut s = a.enter();
    for i in 0..4 {
        s = s.send(i).recur();
    }
    drop(s);

    let r = b.enter();
    let (first, at_var) = match r.recv() {
        Ok(pair) => pair,
        Err(_) => unreachable!("queued"),
    };
    // Re-unfold the loop variable to receive a second value.
    let (second, r3) = match at_var.recur().recv() {
        Ok(pair) => pair,
        Err(_) => unreachable!("queued"),
    };
    assert_eq!((first, second), (0, 1));
    // Stop here: drop the live `Session<Var, Rec<Rx<Var>>>`, leaving values 2 and 3 unconsumed. Fine.
    drop(r3);
}
