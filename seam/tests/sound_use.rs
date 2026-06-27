//! The SOUND-USE side of the seam: realistic consumer code that uses the re-exported
//! interface across two threads with **zero `unsafe`** and **zero proofs**. If this
//! compiles and runs clean (and under the compile_fail doctests in the lib it cannot be
//! abused), the re-import rule held for this primitive.
//!
//! Seam-width note (horn 3): the consumer crosses the seam exactly **once**, at `split`.
//! Thereafter every `push`/`pop` is an ordinary safe call — 0 crossings per element.

#![forbid(unsafe_code)] // the WHOLE point: consumer code contains no unsafe whatsoever

use seam_spsc::RingBuffer;

#[test]
fn spsc_cross_thread_roundtrip() {
    const N: usize = 8;
    const M: u64 = 100_000;

    let mut rb = RingBuffer::<u64, N>::new();
    let (mut prod, mut cons) = rb.split(); // <-- the ONE seam crossing

    // Producer thread pushes 0..M; consumer thread pops and sums. Scoped threads let the
    // handles borrow the stack-local buffer — NLL ties their lifetime to `rb`.
    let sum = std::thread::scope(|s| {
        s.spawn(move || {
            let mut i = 0u64;
            while i < M {
                // spin until there's room — push is a plain safe call
                if prod.push(i).is_ok() {
                    i += 1;
                }
            }
        });
        let consumer = s.spawn(move || {
            let mut got = 0u64;
            let mut acc = 0u64;
            while got < M {
                if let Some(v) = cons.pop() {
                    acc = acc.wrapping_add(v);
                    got += 1;
                }
            }
            acc
        });
        consumer.join().unwrap()
    });

    let expected = (0..M).fold(0u64, |a, b| a.wrapping_add(b));
    assert_eq!(sum, expected, "every produced value must arrive exactly once, in order");
}

#[test]
fn full_and_empty_are_reported_not_ub() {
    // Single-threaded exercise of the Result/Option edges (full → Err, empty → None).
    let mut rb = RingBuffer::<u32, 4>::new(); // capacity 4 ⇒ 3 usable slots
    let (mut prod, mut cons) = rb.split();

    assert!(prod.push(1).is_ok());
    assert!(prod.push(2).is_ok());
    assert!(prod.push(3).is_ok());
    assert_eq!(prod.push(4), Err(4), "full buffer hands the value back, no overrun");

    assert_eq!(cons.pop(), Some(1));
    assert_eq!(cons.pop(), Some(2));
    assert_eq!(cons.pop(), Some(3));
    assert_eq!(cons.pop(), None, "empty buffer yields None, no read of uninit");
}

#[test]
fn drops_unconsumed_payloads_exactly_once() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct Noisy;
    impl Drop for Noisy {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }

    {
        let mut rb = RingBuffer::<Noisy, 8>::new();
        let (mut prod, _cons) = rb.split();
        prod.push(Noisy).ok();
        prod.push(Noisy).ok();
        // drop rb (and handles) with 2 payloads still queued
    }
    assert_eq!(DROPS.load(Ordering::SeqCst), 2, "queued payloads dropped exactly once");
}
