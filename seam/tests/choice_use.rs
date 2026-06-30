//! Twenty-eighth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The CHOICE (offer/select, `&`/`⊕`) probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • internal choice (`select_left`/`select_right`) and external choice (`offer` → `match Branch`) run end to
//!     end, each branch continuing at its correct, dual protocol;
//!   • the consumer MUST handle both branches (the `Branch` enum + exhaustive `match` — carried by the floor's
//!     sum types) and cannot take one label and continue as the other (the variant types tie tag to state);
//!   • choice carries branch-SAFETY, not branch-COMPLETION: a peer that selects a branch then abandons its
//!     handle mid-branch still starves its dual — the #17/#18/#23/#27 completion residue, unchanged by branching.

#![forbid(unsafe_code)]

use seam_spsc::choice::{session, Branch, End, Offer, Rx};

// The server OFFERS { Left: receive an i32 then end ; Right: end }. The client SELECTS. Both branches run to
// completion, each side at its dual protocol.
#[test]
fn offer_select_left_branch_completes() {
    // server: Offer<Rx<End>, End> ; client (dual): Select<Tx<End>, End>
    let (server, client) = session::<Offer<Rx<End>, End>>();

    // Client picks the left branch and sends 42.
    let c = client.select_left(); // Chan<Tx<End>>
    c.send(42).close();

    // Server handles the choice — exhaustively. Left → receive the value; Right → just end.
    match server.offer() {
        Ok(Branch::Left(rx)) => {
            let (v, end) = rx.recv().ok().expect("client sent on the left branch");
            assert_eq!(v, 42, "the left branch's value arrived");
            end.close();
        }
        Ok(Branch::Right(end)) => {
            end.close();
            panic!("client selected left, not right");
        }
        Err(_) => panic!("client had already selected"),
    }
}

// The right branch: client selects right (no payload), both just end.
#[test]
fn offer_select_right_branch_completes() {
    let (server, client) = session::<Offer<Rx<End>, End>>();

    client.select_right().close(); // Chan<End>

    match server.offer() {
        Ok(Branch::Left(rx)) => {
            let _ = rx.recv();
            panic!("client selected right, not left");
        }
        Ok(Branch::Right(end)) => end.close(),
        Err(_) => panic!("client had already selected"),
    }
}

// Choice carries branch-SAFETY, not COMPLETION. The client selects the left branch (depositing the label) then
// abandons its handle WITHOUT sending. The server's offer resolves to Left correctly — branch-safety holds — but
// its inner `recv` is starved forever: the peer chose a branch and then walked away inside it. The completion
// residue (#17/#18/#23/#27) is untouched by branching.
#[test]
fn abandoning_a_chosen_branch_starves_the_peer() {
    let (server, client) = session::<Offer<Rx<End>, End>>();

    let c = client.select_left(); // Chan<Tx<End>> — label deposited, but...
    core::mem::forget(c); // SAFE — the client abandons the branch without sending (the #17/#18 linearity gap)

    match server.offer() {
        Ok(Branch::Left(rx)) => {
            // Branch-safety held: we are correctly in the left branch. But completion did not:
            match rx.recv() {
                Ok(_) => panic!("nothing was ever sent on the abandoned branch"),
                Err(_starved) => { /* the peer waits forever — choice did not carry completion */ }
            }
        }
        Ok(Branch::Right(_)) => panic!("the client selected left"),
        Err(_) => panic!("the label was deposited before forget"),
    }
}
