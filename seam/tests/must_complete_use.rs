//! Twenty-ninth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The MUST-COMPLETE-SESSION probe. Asks whether the affine floor can *compel* a session to run to `End`.
//! Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • a session that IS run to `End` and `finish`-ed defuses its bomb cleanly (the happy path);
//!   • a `live` session dropped before `End` fires the completion bomb (the discipline has teeth — when `Drop`
//!     actually runs);
//!   • BUT `core::mem::forget` skips `Drop` in safe code, so the bomb NEVER fires: the session is abandoned and
//!     the peer starved with no abort. "Must complete" is unenforceable — the session face of `linear` #20's
//!     must-run break, the #17/#18/#23/#27/#28 completion residue made mechanical.

#![forbid(unsafe_code)]

use seam_spsc::must_complete::{start, End, Tx};

// HAPPY PATH: both endpoints walk to End and finish(). No live handle is dropped, so no bomb fires.
#[test]
fn both_endpoints_run_to_end_and_finish_no_bomb() {
    let (a, b) = start::<Tx<End>>(); // a: send-then-end ; b (dual): recv-then-end
    let a = a.send(99); // Session<End>
    let (v, b) = b.recv().ok().expect("a sent before b receives");
    assert_eq!(v, 99, "the sent value arrived");

    a.finish(); // the one clean exit — bomb defused
    assert!(b.is_completed(), "finish() marked the shared session completed");
    b.finish(); // clean exit — bomb defused
}

// THE BOMB HAS TEETH (when Drop runs): a live, mid-protocol session that is genuinely dropped aborts. We forget
// the peer first so only `a`'s bomb is in play (a clean single panic for `should_panic` to catch).
#[test]
#[should_panic(expected = "completion bomb")]
fn dropping_a_live_session_before_end_fires_the_bomb() {
    let (a, b) = start::<Tx<End>>();
    core::mem::forget(b); // isolate: take the peer out of teardown
    drop(a); // `a`: Session<Tx<End>>, still live, never advanced → the bomb fires
}

// THE BREAK: choice carried branch-safety, this seam claims to carry COMPLETION — but `core::mem::forget` is a
// SAFE function that skips `Drop`, so the completion bomb never fires. The session is abandoned and the peer
// starves, with no abort and no `unsafe`. Must-complete is a clean NONE: "runs to End" is LINEAR, Rust's floor
// is AFFINE. This is the remote (peer-starving) twin of `linear` #20's local must-run-cleanup break.
#[test]
fn mem_forget_bypasses_the_completion_bomb_and_starves_the_peer() {
    let (a, b) = start::<Tx<End>>(); // the discipline says `a` MUST send then finish

    core::mem::forget(a); // SAFE — skips Drop; the bomb NEVER fires; `a` never sent

    // The peer is starved: nothing on the wire, no abort, no panic.
    match b.recv() {
        Ok(_) => panic!("a never sent — recv must not succeed"),
        Err(b) => {
            // The bomb was defeated in safe code; "must complete" did not hold.
            assert!(!b.is_completed(), "the session was abandoned, never completed");
            core::mem::forget(b); // tidy the peer the same safe way (the bypass is symmetric)
        }
    }
}
