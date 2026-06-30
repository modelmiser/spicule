//! Twenty-fourth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The DELEGATION (channel-over-channel) probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • a sound THREE-party delegation runs and completes — A delegates its worker session to B, B drives it
//!     with C; the receive capability is transferred at the type level (the higher-order session step #23
//!     lacked);
//!   • don't-use-after-delegate is carried by the affine MOVE (the compile_fail lives in the module doc);
//!   • the NEW hazard — LEAK AMPLIFICATION: abandoning a carrier session with a delegated channel in flight
//!     strands the whole payload session (a leak, not UB — the leak-freedom axis #9/#10 resurfacing through
//!     delegation's `Rc` payload), shown by contrast against a properly-dropped carrier that releases it.
#![forbid(unsafe_code)]

use seam_spsc::delegation::{session, End, Rx, Tx, TxChan};

// Sound three-party delegation: A holds the receive end of a worker session with C, but delegates that end to
// B over the A–B carrier. B then receives the worker's value from C. The capability moved from A to B.
#[test]
fn sound_three_party_delegation_completes() {
    // Worker session: c_end will SEND an i32; a_end (dual) will RECEIVE it.
    let (c_end, a_end) = session::<Tx<End>>(); // c_end: Chan<Tx<End>>, a_end: Chan<Rx<End>>

    // Carrier session A–B: A delegates a `Chan<Rx<End>>`, then End. B receives it, then End.
    let (a_main, b_main) = session::<TxChan<Rx<End>, End>>();

    // A delegates the worker's receive end to B.
    let a_main = a_main.send_chan(a_end); // a_main: Chan<End>; a_end is moved away
    let Ok((a_end_at_b, b_main)) = b_main.recv_chan() else { panic!("A delegated before B received") };

    // Now B owns the worker's receive end. C sends; B receives.
    let c_end = c_end.send(99);
    let Ok((got, a_end_at_b)) = a_end_at_b.recv() else { panic!("C sent before B received") };
    assert_eq!(got, 99, "the delegated session delivered C's value to its NEW owner B");

    // Everyone completes.
    c_end.close();
    a_end_at_b.close();
    a_main.close();
    b_main.close();
}

// THE NEW HAZARD: abandoning a carrier with a delegated channel in flight strands the payload session.
// We keep the worker's other endpoint `w_b` as an OBSERVER (it shares the worker wire) and read
// `wire_strong_count()`: after delegation the worker wire has 2 strands (w_b + the carrier's parked slot).
// Forgetting the carrier never releases the slot → the count stays 2 (one strand has no usable owner = a leak).
#[test]
fn abandoned_carrier_leaks_the_in_flight_delegated_session() {
    let (w_a, w_b) = session::<Tx<End>>();
    assert_eq!(w_b.wire_strong_count(), 2, "two endpoints reference the worker wire");

    let (carrier_tx, carrier_rx) = session::<TxChan<Tx<End>, End>>();

    // Delegate w_a into the carrier's slot — w_a's Rc is parked there; w_a the handle is consumed.
    let carrier_tx_end = carrier_tx.send_chan(w_a);
    assert_eq!(
        w_b.wire_strong_count(),
        2,
        "still two strands: the observer w_b + the carrier slot now holding the delegated wire"
    );

    // Drop the carrier RECEIVER (it never recv_chan'd the delegation) and ABANDON the carrier sender by
    // forgetting it — its Drop never runs, so the slot (holding the worker's Rc) is never released.
    drop(carrier_rx);
    core::mem::forget(carrier_tx_end);

    // LEAK: the worker wire still has two strands, but one of them (the carrier's stranded slot) is reachable
    // by no usable handle. If delegation were leak-free, abandoning the carrier would have dropped that strand.
    assert_eq!(
        w_b.wire_strong_count(),
        2,
        "the in-flight delegated worker session leaked: the forgotten carrier stranded its Rc in the slot"
    );
}

// CONTROL: the SAME delegation, but the carrier is properly DROPPED (not forgotten). Its Drop releases the
// slot, so the parked worker strand is freed and the count falls to 1 — isolating the leak to the abandonment.
#[test]
fn dropped_carrier_releases_the_in_flight_delegated_session() {
    let (w_a, w_b) = session::<Tx<End>>();
    let (carrier_tx, carrier_rx) = session::<TxChan<Tx<End>, End>>();

    let carrier_tx_end = carrier_tx.send_chan(w_a);
    assert_eq!(w_b.wire_strong_count(), 2);

    // Drop BOTH carrier endpoints normally → the carrier transport drops → its slot drops → worker Rc released.
    drop(carrier_rx);
    drop(carrier_tx_end);

    assert_eq!(
        w_b.wire_strong_count(),
        1,
        "a properly-dropped carrier releases the parked delegation — only the observer w_b remains"
    );
}
