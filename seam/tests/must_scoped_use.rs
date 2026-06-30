//! Consumer-side proof for the thirtieth seam (`must_scoped`): the **scoped salvage of #29**.
//!
//! The whole point of this seam is *forcing by return type*: a role closure can only terminate normally
//! by returning a [`Done`], and the only mint for `Done` is [`Session::finish`] at `End`. So every test
//! here that compiles-and-returns is itself evidence of the forcing — `session_scope` returns **only
//! because both role closures completed their sessions**. The negative side (a body that does not finish,
//! or that tries to smuggle the session out of scope) is demonstrated by the `compile_fail` doctests CF1
//! and CF2 in the module — those cannot live in a normally-compiled test file *because they do not
//! compile*, which is exactly the guarantee.
//!
//! No `unsafe` anywhere in consumer code.
#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::must_scoped::{session_scope, End, Tx};

/// Happy path: a one-message session. `a` sends, `b` receives and finishes; `session_scope` returns,
/// which can *only* happen because both closures produced a `Done` — i.e. both ran to `End`.
#[test]
fn both_roles_run_to_end_and_the_scope_returns() {
    let got = Cell::new(0);
    session_scope::<Tx<End>, _, _>(
        |s| s.send(42).finish(),
        |s| {
            // `Err` hands the live handle back; the single-threaded witness ran `a` first, so it's `Ok`.
            let (x, s) = match s.recv() {
                Ok(p) => p,
                Err(_) => unreachable!("a sent before b ran (single-threaded witness)"),
            };
            got.set(x);
            s.finish()
        },
    );
    // The scope returned ⇒ both endpoints minted a Done ⇒ both completed. The value confirms the
    // protocol actually ran rather than being short-circuited.
    assert_eq!(got.get(), 42);
}

/// A multi-step protocol (`Tx<Tx<End>>`): `a` sends two values into the FIFO transport, `b` drains
/// both and finishes. Exercises the typestate walking more than one step while still terminating —
/// the affine handle advances `Tx<Tx<End>> → Tx<End> → End` and only then can `finish` mint the `Done`.
#[test]
fn multi_step_protocol_walks_to_end() {
    let sum = Cell::new(0);
    session_scope::<Tx<Tx<End>>, _, _>(
        |s| s.send(10).send(20).finish(),
        |s| {
            let (x, s) = match s.recv() {
                Ok(p) => p,
                Err(_) => unreachable!("first value queued by a"),
            };
            let (y, s) = match s.recv() {
                Ok(p) => p,
                Err(_) => unreachable!("second value queued by a"),
            };
            sum.set(x + y);
            s.finish()
        },
    );
    assert_eq!(sum.get(), 30);
}

/// The Err/retry path is reachable *without* divergence: `b` polls `recv` in a loop. Because the
/// single-threaded witness runs `a` to completion first, the value is already queued and the first
/// poll succeeds — but the point is structural: a body holding a live `Rx` handle that gets `Err`
/// keeps the handle and **must** either retry or diverge to ever produce a `Done`. It can never
/// silently abandon. Here the retry succeeds and the session completes.
#[test]
fn recv_retry_loop_still_reaches_completion() {
    let got = Cell::new(0);
    session_scope::<Tx<End>, _, _>(
        |s| s.send(7).finish(),
        |s| {
            // Drive the receive to success by retrying on Err (it succeeds immediately here).
            let mut rx = Some(s);
            loop {
                let s = rx.take().expect("handle present");
                match s.recv() {
                    Ok((x, s)) => {
                        got.set(x);
                        break s.finish();
                    }
                    Err(s) => rx = Some(s), // keep the live handle; retry (or one would diverge)
                }
            }
        },
    );
    assert_eq!(got.get(), 7);
}

/// `is_completed` observes the shared transport flag after a clean run inside the scope. (Observed
/// *inside* `b` before its own finish: `a` has already finished, so the shared flag is set.)
#[test]
fn shared_completion_flag_is_observable_in_scope() {
    let saw_completed = Cell::new(false);
    session_scope::<Tx<End>, _, _>(
        |s| s.send(1).finish(),
        |s| {
            let (_x, s) = match s.recv() {
                Ok(p) => p,
                Err(_) => unreachable!("queued by a"),
            };
            // a finished first in the single-threaded witness, so the shared flag is already true.
            saw_completed.set(s.is_completed());
            s.finish()
        },
    );
    assert!(saw_completed.get());
}
