//! Twenty-seventh seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! The PROJECTED (deadlock-freedom by construction) probe — the #26 salvage. Proves, all in SAFE code (this file
//! `#![forbid(unsafe_code)]`):
//!   • the trusted 3-party `relay()` builder mints a deadlock-free assembly that runs to completion;
//!   • the trusted 4-party `relay4()` pipeline (monotone priorities N0<N1<N2) does the same — the builder
//!     generalizes over acyclic shapes;
//!   • the #26 exploit is now uncompilable: there is no public per-role constructor to wire a cyclic assembly
//!     (the `compile_fail` for the sealed `worker()` lives in the module doc). Safe code can only obtain roles
//!     from a trusted, edge-consistent, monotone-priority builder — so deadlock-freedom holds by construction.

#![forbid(unsafe_code)]

use seam_spsc::projected::{relay, relay4};

// The trusted 3-party builder mints A→B→C with a source and a monotone priority order; it completes.
#[test]
fn projected_relay_completes() {
    let (src, fwd, sink) = relay();
    src.send(11);
    let (v, b_send) = fwd.recv().ok().expect("B's recv from the source is ready (a source exists)");
    assert_eq!(v, 11);
    b_send.send(v);
    assert_eq!(sink.recv().ok(), Some(11), "value reached C through B, deadlock-free by construction");
}

// The trusted builder generalizes over acyclic shapes: a 4-party monotone pipeline A→B→C→D also completes.
#[test]
fn projected_pipeline_of_four_completes() {
    let (a, b, c, d) = relay4();
    a.send(7);
    let (v1, b_send) = b.recv().ok().expect("B ready");
    b_send.send(v1);
    let (v2, c_send) = c.recv().ok().expect("C ready");
    assert_eq!(v2, 7);
    c_send.send(v2);
    assert_eq!(d.recv().ok(), Some(7), "value traversed the whole monotone pipeline to D");
}
