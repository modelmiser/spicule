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
//! ## The closed-basis ruling (the §6 deliverable the panel confirmed)
//!
//! This primitive was built not to hunt a tenth axis but to **test whether the cost-basis is closed**.
//! The cold judge ruled **(2): validity *projects onto* zero-cost (#6)** — its safe witness is a
//! per-read runtime validity check, the *same kind* of cost as the slab's generation tag (different
//! predicate, identical `cmp+branch` lowering). A genuinely new danger class (bit-level validity) added
//! **no new coordinate**. This is the **dual** of the `get_unchecked` family: there one danger had many
//! witnesses spending different axes; here many dangers converge on one axis. Evidence is **consistent
//! with the cost-basis being closed**. (Recorded caveat the panel insisted on: prior-art argued the
//! `FromBytes`-vs-`TryFromBytes` split makes the cost *non-uniform across types* → option (3)
//! inseparable; the judge rejected it — an all-bit-patterns-valid type is *danger-absent*, not a cheap
//! witness, and a sealed/branded constructor only *amortizes* the same #6 cost, it does not replace it
//! with a compile-time API-locality proof. So validity lacks move-stability's dual-witness structure.)
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
