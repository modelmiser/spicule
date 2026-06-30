//! Twenty-sixth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The PRIORITY-TYPED session (rung-(b)) probe. Proves, all in SAFE code (this file `#![forbid(unsafe_code)]`):
//!   • a COHERENT priority-typed relay A→B→C with an acyclic assignment (p_AB=N0 < p_BC=N1) runs to completion —
//!     the forwarder's bound `N0: Lt<N1>` is satisfiable, and a source role exists;
//!   • the per-channel priority discipline `Pi: Lt<Po>` is what rejects the deadlocking ring — the cyclic step
//!     `N2: Lt<N0>` is untypable (the `compile_fail` lives in the module doc), where #25's bare floor accepted
//!     the same ring. Here we exercise the positive half: the acyclic assignment that the priority discipline
//!     admits, end to end.

#![forbid(unsafe_code)]

use seam_spsc::binary_session::{session, End, Tx};
use seam_spsc::priority::{relay, role};
use seam_spsc::typed_nat::{N1, N2};

// A coherent priority-typed relay: an acyclic assignment (p_AB=N0 < p_BC=N1) satisfies the forwarder's
// `Lt` bound, a source role (A) exists, and the value flows A→B→C to completion.
#[test]
fn priority_relay_completes() {
    let (src, fwd, sink) = relay();

    // Role A (source, priority N0 outbound) sends 9 to B and completes.
    src.send(9);

    // Role B (forwarder, recv at N0 then send at N1 — the bound N0: Lt<N1> admitted construction) forwards.
    let (v, b_send) = match fwd.recv() {
        Ok(x) => x,
        Err(_) => panic!("coherent relay: B's recv from the source should be ready"),
    };
    assert_eq!(v, 9);
    b_send.send(v);

    // Role C (sink, priority N1 inbound) receives the forwarded value and completes.
    let got = match sink.recv() {
        Ok(x) => x,
        Err(_) => panic!("coherent relay: C's recv from the forwarder should be ready"),
    };
    assert_eq!(got, 9, "the value A sent reached C through B under an acyclic priority assignment");
}

// THE BREAK (cold gauntlet r33, SEAM-BREAKS [horn 1]): the public `role()` constructor lets every role pick its
// OWN priorities, with NO cross-edge consistency. So three roles each using the SAME bound `N1 < N2` type-check
// — every `N1: Lt<N2>` discharges (1 < 2) — yet wire into the deadlocking ring A→B→C→A. The §3 "untypable ring"
// argument only holds for `relay()`-minted chains (one Nat per edge); the bare discipline carries nothing global.
// This falsifies the spec's §4.3 "deadlock-freedom of any well-typed assembly" — a typed, no-`unsafe`, deadlocked
// assembly. (The fix would be to HIDE `role()` and mint only via an edge-consistent constructor.)
#[test]
fn public_role_builds_a_typed_deadlocking_ring() {
    let (ab_s, ab_r) = session::<Tx<End>>(); // edge A→B
    let (bc_s, bc_r) = session::<Tx<End>>(); // edge B→C
    let (ca_s, ca_r) = session::<Tx<End>>(); // edge C→A

    // Every role: recv at N1, send at N2 — bound `N1: Lt<N2>` discharges for all three. Fully priority-typed.
    let a = role::<N1, N2>(ca_r, ab_s); // A: recv from C, send to B
    let b = role::<N1, N2>(ab_r, bc_s); // B: recv from A, send to C
    let c = role::<N1, N2>(bc_r, ca_s); // C: recv from B, send to A

    // It compiled with no `unsafe` — yet the topology is a source-less ring: no role can start. Deadlock.
    assert!(a.recv().is_err(), "A blocked: predecessor C never sent");
    assert!(b.recv().is_err(), "B blocked: predecessor A never sent");
    assert!(c.recv().is_err(), "C blocked: predecessor B never sent");
}
