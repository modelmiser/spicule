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
//! ## The witness, and the closed-basis question
//!
//! The seam reads a `Tag` out of a byte buffer via [`TaggedBuf::tag_at`]: it checks `b <= 2` and only
//! then `transmute`s. Sound because the check precedes the unsafe. The cost of that soundness is a
//! **runtime validity check** (`b <= 2`) per read — i.e. exactly the **zero-cost (#6)** axis the slab
//! pays, via a *different* mechanism (a discriminant range, not a generation tag).
//!
//! So the §6 deliverable is not "a tenth axis" but an adjudication of the **closed-basis hypothesis**:
//! - **(1) a new axis** — validity is its own coordinate, distinct from #6; or
//! - **(2) it projects onto an existing axis** — validity is a *new danger* that spends an *old cost*
//!   (zero-cost #6, a runtime tag check), the dual of the `get_unchecked` family (one danger / many
//!   witnesses) read backwards (many dangers / one axis); or
//! - **(3) inseparable** — like move-stability, its cost is non-uniform across witnesses with no
//!   isolable coordinate.
//!
//! The builder's prediction is **(2)**: validity deflates to zero-cost, evidence that the lattice's
//! cost-basis is *closed* — new dangers project onto it rather than extending it.
//!
//! ### CF1 — safe code cannot mint an invalid `Tag` (the transmute is unsafe; only the seam may do it):
//! ```compile_fail
//! # use seam_spsc::validity::Tag;
//! let _t: Tag = core::mem::transmute(7u8); // ERROR: call to unsafe function requires an `unsafe` block
//! ```

/// A one-byte value with a **validity invariant**: only the discriminants `0,1,2` are valid bit
/// patterns. Producing any other is instant UB. `#[repr(u8)]` fixes the layout so the seam can
/// `transmute` a *checked* byte into one.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tag {
    A = 0,
    B = 1,
    C = 2,
}

/// The highest valid discriminant. The seam's runtime validity check is `b <= MAX_TAG`.
const MAX_TAG: u8 = 2;

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
    /// Returns `None` if the byte is not a valid discriminant (the runtime **validity check**) or `i`
    /// is out of range. Sound because the `b <= MAX_TAG` check precedes the `transmute`.
    pub fn tag_at(&self, i: usize) -> Option<Tag> {
        let b = *self.bytes.get(i)?;
        if b <= MAX_TAG {
            // SAFETY: `Tag` is `#[repr(u8)]` with valid discriminants 0..=MAX_TAG, and we have just
            // checked `b <= MAX_TAG`, so `b` is a valid `Tag` bit pattern. The transmute is sound.
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
