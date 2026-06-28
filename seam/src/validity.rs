//! # Twelfth seam — a checked bit-pattern (the validity-invariant probe / closed-basis test).
//!
//! *(Project ordinal: the twelfth distinct primitive. Not a fresh-axis hunt so much as a **test of the
//! closed-basis hypothesis**: after nine clean axes + one inseparable danger, does a *new danger class*
//! add a coordinate, or does it project onto the existing basis?)*
//!
//! Same floor, same rules. This seam re-exports a danger no prior primitive covers: a **validity
//! invariant** — a bit pattern that must be one of a restricted set, or it is **instant UB even without
//! a dereference**. A `#[repr(u8)]` enum `Tag { A=0, B=1, C=2 }` is one byte, but only the bytes
//! `0,1,2` are valid; `transmute::<u8, Tag>(7)` is UB the moment it exists (the compiler may assume a
//! `Tag` is always a valid discriminant — exhaustive `match`, niche optimizations). This is a different
//! danger from every axis so far: not aliasing, bounds, ownership, cost, destructor-timing, or address
//! — it is the **bit-level validity of a value**.
//!
//! ## The witness — set-membership, not a range (the cold panel's mandatory correction)
//!
//! The seam reads a `Tag` out of a byte buffer via [`TaggedBuf::tag_at`]: it tests the byte against the
//! **set of valid discriminants** and only then `transmute`s. The discriminants here are deliberately
//! **non-contiguous** (`A=0, B=5, C=200`), because the first cold gauntlet (`REPORT_SEAM_SPEC_validity`)
//! **rightly FAILED** an earlier version whose witness was a *range* check (`b <= MAX_TAG`): a range is
//! *not* exhaustive over the invalid set for non-contiguous discriminants (bytes `3,4,6..=199` are
//! invalid yet `<= 200`), so copied as a "reusable recipe" it would launder UB. The correct witness is
//! **exhaustive discriminant-set membership** (a `match` pinned to the actual variants). For a
//! *contiguous-from-zero* enum a range coincides with the set, but the set form is the general one.
//!
//! ## The closed-basis ruling (the §6 deliverable)
//!
//! This primitive was built not to hunt a tenth axis but to **test whether the cost-basis is closed**.
//! The **meta-claim is unanimously upheld**: a genuinely new danger class (bit-level validity) added
//! **no tenth coordinate** — every witness spends a pre-existing axis, and *"what does the safe witness
//! cost?" has finitely many answers*. The cost-basis is **closed**.
//!
//! On the *classification of validity's own cost* the two cold panels split, and the more careful
//! reading is **(3) INSEPARABLE** — validity is a **second** inseparable danger beside move-stability.
//! The first panel ruled (2) projects-onto-zero-cost (a per-read runtime check = the slab's #6, via a
//! different predicate). But the second panel showed the cost is **witness-non-uniform**: the zero-copy
//! `&[u8] → &[Tag]` view (the real reason systems code transmutes byte buffers) admits *two* sound
//! witnesses — eager runtime validation (spends **#6**, but O(n), destroys mmap laziness) or a **sealed
//! constructor** (spends **API-locality**). Same danger, two axes depending on witness — the
//! move-stability signature. So "inseparable" is a *recurring category*, not a one-off. Two recorded
//! caveats: soundness holds only in the **fieldless single-byte corner** (a data-carrying / multi-byte /
//! `repr(C)` enum needs a *recursive* `is_bit_valid` witness — `zerocopy`'s real shape — so this is a
//! "toy" like the self-ref cell's non-composable single frame); and the whole construction is
//! **non-novel** — it is the shipped design of `num_enum` (`TryFromPrimitive`/`UnsafeFromPrimitive`),
//! `bytemuck::CheckedBitPattern`, `zerocopy::TryFromBytes`, plus the refinement-type literature.
//!
//! ### CF1 — safe code cannot mint an invalid `Tag` (the transmute is unsafe; only the seam may do it):
//! ```compile_fail
//! # use seam_spsc::validity::Tag;
//! let _t: Tag = core::mem::transmute(7u8); // ERROR: call to unsafe function requires an `unsafe` block
//! ```

/// A one-byte value with a **validity invariant**: only the discriminants `0,5,200` are valid bit
/// patterns. Producing any other is instant UB. The discriminants are **deliberately non-contiguous**
/// so the witness must be set-membership, not a range (see the module docs / the first gauntlet's
/// FAILED verdict). `#[repr(u8)]` fixes the layout so the seam can `transmute` a *checked* byte into one.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    A = 0,
    B = 5,
    C = 200,
}

/// The exhaustive set of valid discriminants — the witness is membership in *this set*, kept beside the
/// enum so a new variant forces a matching update. A *range* over these (`b <= 200`) would be unsound.
const VALID_DISCRIMINANTS: [u8; 3] = [0, 5, 200];

/// A buffer of raw bytes some of which are meant to be read back as [`Tag`]s — e.g. a deserialization
/// buffer, a wire format, a memory-mapped table.
pub struct TaggedBuf<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> TaggedBuf<N> {
    /// Wrap raw bytes (e.g. just read off a socket). No validity is assumed yet.
    pub fn new(bytes: [u8; N]) -> Self {
        TaggedBuf { bytes }
    }

    /// Read the byte at `i` *as a `Tag`*, the dangerous primitive (`transmute`) re-exported safe.
    /// Returns `None` if the byte is not a valid discriminant (the runtime **set-membership validity
    /// check**) or `i` is out of range. Sound because membership in [`VALID_DISCRIMINANTS`] precedes
    /// the `transmute`.
    pub fn tag_at(&self, i: usize) -> Option<Tag> {
        let b = *self.bytes.get(i)?;
        if VALID_DISCRIMINANTS.contains(&b) {
            // SAFETY: `Tag` is `#[repr(u8)]` with valid discriminants exactly `VALID_DISCRIMINANTS`,
            // and we have just checked `b` is one of them, so `b` is a valid `Tag` bit pattern. The
            // transmute is sound. (A *range* test would be unsound here — the discriminants are
            // non-contiguous — which is the correction the first gauntlet forced.)
            Some(unsafe { core::mem::transmute::<u8, Tag>(b) })
        } else {
            None
        }
    }

    /// The raw byte at `i` (no validity interpretation) — for contrast: always sound, because a `u8`
    /// has no validity invariant (every bit pattern is a valid `u8`). The seam's danger is entirely in
    /// [`tag_at`](TaggedBuf::tag_at)'s *re-interpretation* of the bits.
    pub fn byte_at(&self, i: usize) -> Option<u8> {
        self.bytes.get(i).copied()
    }
}
