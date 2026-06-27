//! # Fourth seam — a generational slab allocator (Phase C contrast: the zero-cost axis)
//!
//! Same floor, same rules. This primitive is chosen because its danger has a shape none of
//! the first three had: the handle (`Key`) is **`Copy`** — it must be, since the whole point
//! of a slab is to store and pass keys around — and a `Copy` handle can outlive the slot it
//! names. Affinity (which carried SPSC's cardinality) is useless here, and the NLL region
//! vocabulary is blind to a copied index's validity. So S1–S3 (key-still-valid, no
//! use-after-remove, no ABA) cannot be discharged statically.
//!
//! The resolution is a **runtime witness**: a per-slot generation counter, bumped on `remove`,
//! that `get`/`remove` check before touching storage. This keeps all three *trilemma* horns —
//! sound, decidable + proof-free, and concentrated (the whole interface is safe, seam width is
//! effectively zero) — and pays instead on the **zero-runtime-cost** property the trilemma had
//! folded away. The generation word per slot and the compare-branch per access do **not** erase
//! at compile time. This is the constructive analog of `RefCell`: spend runtime to keep the
//! static system sound and decidable. The slab does not break the trilemma; it pays on a fourth
//! axis (zero-cost), confirming that seam-width (horn 3) and erasure (#6) are different things.
//!
//! `Key` fields are private, so safe code cannot forge one:
//! ```compile_fail
//! # use seam_spsc::slab::Key;
//! let _k = Key { index: 0, generation: 0 }; // ERROR: fields are private
//! ```

use core::array;
use core::mem::MaybeUninit;

/// A `Copy` handle into a slab: a slot index plus the generation that was live when the key
/// was minted. Forgeable only by `Slab::insert` (fields are private).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Key {
    index: usize,
    generation: u32,
}

enum Entry {
    Occupied,
    /// Free-list link: the index of the next vacant slot, or `N` (sentinel) if none.
    Vacant { next_free: usize },
}

struct Slot<T> {
    generation: u32,
    entry: Entry,
    storage: MaybeUninit<T>,
}

/// A fixed-capacity generational slab. Hands out `Copy` keys and a fully-safe
/// `insert`/`get`/`remove` interface; the only `unsafe` (the `MaybeUninit` access) is gated by
/// the generation check.
pub struct Slab<T, const N: usize> {
    slots: [Slot<T>; N],
    /// Index of the first free slot, or `N` if the slab is full.
    free_head: usize,
    len: usize,
}

impl<T, const N: usize> Slab<T, N> {
    /// Create an empty slab. All slots vacant, threaded into a free list.
    pub fn new() -> Self {
        let slots = array::from_fn(|i| Slot {
            generation: 0,
            entry: Entry::Vacant { next_free: i + 1 }, // last points to N (sentinel)
            storage: MaybeUninit::uninit(),
        });
        Slab { slots, free_head: 0, len: 0 }
    }

    /// Number of occupied slots.
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Insert `value`, returning its `Key`, or `Err(value)` if the slab is full.
    pub fn insert(&mut self, value: T) -> Result<Key, T> {
        let idx = self.free_head;
        if idx >= N {
            return Err(value); // full
        }
        let slot = &mut self.slots[idx];
        let next_free = match slot.entry {
            Entry::Vacant { next_free } => next_free,
            // The free list only ever links vacant slots; this arm is unreachable.
            Entry::Occupied => return Err(value),
        };
        slot.storage.write(value);
        slot.entry = Entry::Occupied;
        self.free_head = next_free;
        self.len += 1;
        Ok(Key { index: idx, generation: slot.generation })
    }

    /// Borrow the value named by `key`, or `None` if the key is stale/vacant (S1–S3).
    pub fn get(&self, key: Key) -> Option<&T> {
        let slot = self.slots.get(key.index)?;
        match slot.entry {
            Entry::Occupied if slot.generation == key.generation => {
                // SAFETY: occupied AND the generation matches the key ⇒ this slot currently
                // holds the very `T` this key named; the runtime check is what discharges
                // S1/S2/S3, so `assume_init_ref` cannot touch uninitialized or reused storage.
                Some(unsafe { slot.storage.assume_init_ref() })
            }
            _ => None, // vacant, or generation mismatch (stale / ABA) ⇒ reject, no UB
        }
    }

    /// Mutably borrow the value named by `key`, or `None` if stale/vacant.
    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = self.slots.get_mut(key.index)?;
        match slot.entry {
            Entry::Occupied if slot.generation == key.generation => {
                // SAFETY: as in `get`; `&mut self` gives unique access to the slot.
                Some(unsafe { slot.storage.assume_init_mut() })
            }
            _ => None,
        }
    }

    /// Remove and return the value named by `key`, or `None` if stale/vacant. Bumps the slot's
    /// generation so every outstanding `Copy` of this key is rejected thereafter (S3).
    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = self.slots.get_mut(key.index)?;
        match slot.entry {
            Entry::Occupied if slot.generation == key.generation => {
                // SAFETY: occupied + match ⇒ initialized; we read it out exactly once and then
                // mark the slot vacant, so no later access can take it again.
                let value = unsafe { slot.storage.assume_init_read() };
                // Generation bump is the ABA defense. NOTE (honest bound): `u32` wraps after
                // 2^32 removes of the SAME slot — a theoretical ABA collision, the standard
                // slotmap caveat. Use `u64` to remove it; called out per the spec's threat model.
                slot.generation = slot.generation.wrapping_add(1);
                slot.entry = Entry::Vacant { next_free: self.free_head };
                self.free_head = key.index;
                self.len -= 1;
                Some(value)
            }
            _ => None,
        }
    }
}

impl<T, const N: usize> Default for Slab<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for Slab<T, N> {
    fn drop(&mut self) {
        for slot in &mut self.slots {
            if let Entry::Occupied = slot.entry {
                // SAFETY: occupied ⇒ initialized; we own `self` uniquely, drop each once.
                unsafe { slot.storage.assume_init_drop() }
            }
        }
    }
}
