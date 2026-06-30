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

use seam_spsc::priority::relay;

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
