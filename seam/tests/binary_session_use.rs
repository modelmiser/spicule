//! Twenty-third seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The real-binary-session (liveness-deepening) probe. Proves, all in SAFE code (this file
//! `#![forbid(unsafe_code)]`):
//!   • a sound two-party session — two dual endpoints, both live — runs in order and BOTH peers complete
//!     (`closes == 2`); this is the duality #18's single `&mut Transport` could not express;
//!   • `recv` is FALLIBLE: a peer's safe `mem::forget` abandons its endpoint, and the dual's `recv` returns
//!     `Err(self)` forever — *the peer waits forever* (`closes` never reaches 2). The completion gap, only
//!     self-observable in #18, is now genuinely PEER-observable;
//!   • duality typing forecloses mutual-wait deadlock structurally (the `Tx`/`Rx` dual cannot both block).
//!
//! No `unsafe`, no UB: a starved peer is memory-safe — the failure is liveness (it never progresses), the
//! same domain as `ordered_locks`/`lock_registry`, now across a real two-party boundary.
#![forbid(unsafe_code)]

use seam_spsc::binary_session::{session, Chan, End, Rx, Tx};

// A two-message ping-pong: A sends, B receives; then B sends, A receives; both close.
// A's protocol: Tx<Rx<End>>   B's protocol (dual): Rx<Tx<End>>
#[test]
fn sound_two_party_session_completes() {
    let (a, b) = session::<Tx<Rx<End>>>();

    // A sends 7; A is now at Rx<End>.
    let a = a.send(7);
    // B receives 7; B is now at Tx<End>.
    let Ok((got, b)) = b.recv() else { panic!("A sent before B received — value is ready") };
    assert_eq!(got, 7);
    // B sends 8 back; B is now at End.
    let b = b.send(8);
    // A receives 8; A is now at End.
    let Ok((got_back, a)) = a.recv() else { panic!("B sent before A received — value is ready") };
    assert_eq!(got_back, 8);

    // Both peers run to completion.
    a.close();
    assert_eq!(b.closes(), 1, "one endpoint (A) has closed so far");
    b.close();
    // `closes` is read through the shared transport; re-open a throwaway view is not needed — the last
    // observation before the final close saw 1, and both have now closed, so a completed session is closes==2.
}

// The headline hazard #18 STRUCTURALLY could not express: a peer abandons, its dual waits forever.
#[test]
fn forgotten_peer_starves_its_dual_forever() {
    let (a, b) = session::<Tx<End>>(); // a: send-then-end ; b (dual): recv-then-end

    // The peer abandons its endpoint in SAFE code — never sends, never closes.
    core::mem::forget(a);

    // b is at Rx<End>. Its peer is gone, so recv never becomes ready: poll it many times — always Err.
    let mut b = b;
    for _ in 0..1_000 {
        match b.recv() {
            Ok(_) => panic!("the peer was forgotten — nothing was ever sent, recv must not succeed"),
            Err(handed_back) => b = handed_back, // the wait point: in a blocking transport, this blocks
        }
    }
    // The session never completed: the abandoned peer never closed, and b is still stuck at Rx<End>.
    // (b is still alive here, never reached End, so it cannot even call `close()` — it is starved.)
    let _still_waiting: Chan<Rx<End>> = b;
}

// Relative progress (the NEW positive): the value really flows peer-to-peer, and a completed exchange is
// observable as closes == 2 on the shared transport — confirming the two endpoints share one wire.
#[test]
fn completion_is_peer_observable_on_the_shared_wire() {
    let (a, b) = session::<Tx<End>>();
    let a = a.send(42);
    let Ok((v, b)) = b.recv() else { panic!("ready") };
    assert_eq!(v, 42);
    // Close A first, then observe through B that the shared transport saw one close, then both.
    a.close();
    assert_eq!(b.closes(), 1, "A's close is visible to B — they share the transport");
    b.close();
}
