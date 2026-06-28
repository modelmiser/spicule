//! Consumer of the seventh seam (reference-counted shared ownership). `#![forbid(unsafe_code)]`
//! proves the refcount manager is driven entirely from safe code — `new`/`clone`/deref/drop carry
//! no consumer `unsafe`, while the dangerous heap+refcount lifecycle stays in the foundation. The
//! load-bearing test is `value_freed_exactly_once`: dynamic shared ownership, freed on the *last*
//! drop — the lifecycle no static analysis can schedule, which is why this seam needs a manager.

#![forbid(unsafe_code)]

use core::cell::Cell;
use seam_spsc::shared::Shared;

/// A payload that records its own destruction into a caller-owned counter (no global state, so
/// tests stay race-free under parallel execution).
struct Noisy<'a> {
    drops: &'a Cell<u32>,
}

impl Drop for Noisy<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn single_owner_reads_and_counts() {
    let s = Shared::new(42u32);
    assert_eq!(*s, 42);
    assert_eq!(s.strong_count(), 1);
}

#[test]
fn clone_shares_one_allocation_and_tracks_count() {
    let a = Shared::new([1u8, 2, 3]);
    let b = a.clone();
    assert_eq!(a.strong_count(), 2);
    assert_eq!(b.strong_count(), 2);
    assert_eq!(*a, *b); // both owners read the same shared value
    drop(b);
    assert_eq!(a.strong_count(), 1); // a clone dropped, but the value lives — count, not free
}

/// THE keystone: the value is freed exactly once, on the drop of the *last* owner — never on an
/// earlier drop (would be a use-after-free for the survivors), never zero times (a leak).
#[test]
fn value_freed_exactly_once_when_last_owner_drops() {
    let drops = Cell::new(0u32);
    {
        let first = Shared::new(Noisy { drops: &drops });
        let second = first.clone();
        let third = second.clone();
        assert_eq!(first.strong_count(), 3);
        drop(third);
        assert_eq!(drops.get(), 0); // still two owners: NOT freed
        drop(second);
        assert_eq!(drops.get(), 0); // still one owner: NOT freed
        drop(first);
        assert_eq!(drops.get(), 1); // last owner gone: freed exactly once
    }
    assert_eq!(drops.get(), 1);
}

/// Dynamic shared ownership: the live-owner set is determined at run time (a `Vec` of clones), and
/// the value survives until that set empties — the data-dependent lifetime affinity cannot express.
#[test]
fn dynamic_owner_set_decided_at_runtime() {
    let drops = Cell::new(0u32);
    let root = Shared::new(Noisy { drops: &drops });
    let owners: Vec<Shared<Noisy>> = (0..16).map(|_| root.clone()).collect();
    assert_eq!(root.strong_count(), 17);
    drop(owners); // drop all 16 clones at once
    assert_eq!(root.strong_count(), 1);
    assert_eq!(drops.get(), 0); // root still holds it
    drop(root);
    assert_eq!(drops.get(), 1);
}
