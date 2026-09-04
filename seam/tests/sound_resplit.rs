//! REGRESSION: `split` may be called more than once, and the second call must not hand out
//! handles whose caches disagree with the live indices.
//!
//! `split` takes `&mut self` but does not consume the buffer, so a second call is ordinary safe
//! code. Before the fix it hardcoded `cached_head: 0` / `cached_tail: 0` while `head`/`tail`
//! persisted, so after any earlier split that popped, `pop`'s "possibly empty" test
//! (`head == self.cached_tail`) was false on an EMPTY ring — the Acquire refresh was skipped and
//! `assume_init_read` ran on a never-written slot. Four values came out of an empty
//! `RingBuffer<u64, 4>`; `RingBuffer<String, 4>` segfaulted.
//!
//! This is the crate's flagship seam and the direct counterexample to lib.rs:27, "safe code
//! cannot launder UB". No `unsafe` appears below.

#![forbid(unsafe_code)]

use seam_spsc::RingBuffer;

#[test]
fn resplit_of_an_emptied_ring_reports_empty() {
    let mut rb: RingBuffer<u64, 4> = RingBuffer::new();
    {
        let (mut p, mut c) = rb.split();
        p.push(7).unwrap();
        assert_eq!(c.pop(), Some(7));
        assert_eq!(c.pop(), None, "empty within the first split");
    }
    // head == tail == 1 here. The ring is empty and must still say so.
    let (_p, mut c) = rb.split();
    assert_eq!(c.pop(), None, "REGRESSION: re-split ring materialised a value out of uninit memory");
}

#[test]
fn resplit_preserves_unconsumed_elements() {
    let mut rb: RingBuffer<u64, 4> = RingBuffer::new();
    {
        let (mut p, mut c) = rb.split();
        p.push(10).unwrap();
        p.push(20).unwrap();
        assert_eq!(c.pop(), Some(10));
    }
    // head == 1, tail == 2: exactly one element still live.
    let (mut p, mut c) = rb.split();
    assert_eq!(c.pop(), Some(20), "the un-popped element must survive the re-split");
    assert_eq!(c.pop(), None);
    p.push(30).unwrap();
    assert_eq!(c.pop(), Some(30), "the ring is usable after a re-split");
}

// NOTE: no overfill regression test here. The subagent that found this defect reported that
// `push` also returns Ok(()) into a full ring after a re-split; I could not reproduce that in
// two configurations (ring emptied at head==tail==1, and ring part-full at head==1/tail==3).
// In both, `push`'s "possible-full" branch (`next == self.cached_head`) fires at the wrap
// boundary precisely when the stale zero would matter, refreshes from the real `head`, and
// correctly returns Err. The producer-side claim is UNCONFIRMED; the consumer-side reads above
// are reproduced and fixed. A test asserting the overfill would have passed against the
// unfixed code, which is worse than no test.
