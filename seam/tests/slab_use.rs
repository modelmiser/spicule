//! SOUND-USE side of the slab seam: ordinary consumer code inserts, looks up, and removes via
//! `Copy` keys with **zero `unsafe`** and **zero proofs**. The generation check turns every
//! stale/ABA key into `None` rather than UB — the runtime witness doing the work the static
//! floor cannot.
//!
//! Seam-width note: `insert`/`get`/`remove` are all ordinary safe calls (the whole interface is
//! safe, like any std collection) — seam width is effectively 0 crossings. The slab's price is
//! NOT seam width (horn 3 is aced) but runtime: the generation word per slot + the compare per
//! access. That is the zero-cost (#6) axis, separate from concentration.

#![forbid(unsafe_code)] // consumer code contains no unsafe whatsoever

use seam_spsc::slab::Slab;

#[test]
fn insert_get_remove_roundtrip() {
    let mut slab = Slab::<&'static str, 4>::new();
    let a = slab.insert("a").unwrap();
    let b = slab.insert("b").unwrap();
    assert_eq!(slab.len(), 2);
    assert_eq!(slab.get(a), Some(&"a"));
    assert_eq!(slab.get(b), Some(&"b"));
    assert_eq!(slab.remove(a), Some("a"));
    assert_eq!(slab.get(a), None, "removed key now reads None, not freed storage");
    assert_eq!(slab.len(), 1);
}

#[test]
fn full_slab_hands_value_back() {
    let mut slab = Slab::<u32, 2>::new();
    let _k0 = slab.insert(0).unwrap();
    let _k1 = slab.insert(1).unwrap();
    assert_eq!(slab.insert(2), Err(2), "full slab returns the value, no overrun into uninit");
}

#[test]
fn stale_key_after_reuse_is_rejected_not_uaf() {
    // The ABA scenario: a Copy key survives a remove + reuse of its slot. The generation bump
    // makes the old key mismatch — it reads None, never the new occupant's storage.
    let mut slab = Slab::<u64, 2>::new();
    let old = slab.insert(100).unwrap();
    assert_eq!(slab.remove(old), Some(100));

    // Reinsert — very likely reuses `old`'s slot index, but with a bumped generation.
    let new = slab.insert(999).unwrap();
    assert_eq!(slab.get(new), Some(&999), "fresh key reaches the new occupant");

    // The OLD key is Copy and still around. It must NOT alias the new value (S3 / ABA).
    assert_eq!(slab.get(old), None, "stale key rejected by generation check — no ABA, no UB");
    assert_eq!(slab.remove(old), None, "stale key cannot double-free");
}

#[test]
fn copy_keys_survive_and_are_reusable() {
    // A Key is Copy: hold it, copy it, look up repeatedly — the thing affinity would forbid.
    let mut slab = Slab::<i32, 8>::new();
    let k = slab.insert(7).unwrap();
    let k_copy = k; // Copy — no move
    assert_eq!(slab.get(k), Some(&7));
    assert_eq!(slab.get(k_copy), Some(&7), "both copies of the key resolve");
    *slab.get_mut(k).unwrap() += 1;
    assert_eq!(slab.get(k_copy), Some(&8), "mutation visible through the other copy");
}

#[test]
fn drops_occupied_payloads_exactly_once() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct Noisy;
    impl Drop for Noisy {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
        }
    }
    {
        let mut slab = Slab::<Noisy, 8>::new();
        let k = slab.insert(Noisy).ok().unwrap();
        slab.insert(Noisy).ok().unwrap();
        let _ = slab.remove(k); // one dropped here (well, returned then dropped)
        // one Noisy still occupied → dropped by Slab::drop
    }
    assert_eq!(DROPS.load(Ordering::SeqCst), 2, "each payload dropped exactly once");
}

#[test]
fn harness_exploit_fresh_slab_gen0_is_rejected_by_impl() {
    // The cold harness's exact UB path against the SPEC (generation-only check). Verify the
    // IMPLEMENTATION is not vulnerable: my `get` matches Entry::Occupied BEFORE comparing
    // generations, so a fresh slab's vacant slot 0 yields None, never assume_init on uninit.
    let mut a = Slab::<u64, 4>::new();
    let key = a.insert(123).ok().unwrap(); // first-ever insert → {index:0, generation:0}
    let mut b = Slab::<u64, 4>::new();     // fresh slab: slot 0 VACANT at generation 0
    assert_eq!(b.get(key), None, "impl rejects fresh-slab gen-0 key (occupancy guards assume_init)");
    assert_eq!(b.remove(key), None, "remove likewise rejects it");
}
