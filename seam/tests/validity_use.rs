//! Twelfth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code reads validity-checked `Tag`s out of a raw byte buffer with no `unsafe`
//! and no proof — valid discriminants come back as `Some`, invalid bytes as `None` (never an
//! invalid-bit-pattern `Tag`). The danger (an invalid discriminant) is unconstructible in safe code.

#![forbid(unsafe_code)]

use seam_spsc::validity::{Tag, TaggedBuf};

#[test]
fn valid_discriminants_read_back_as_tags() {
    let buf = TaggedBuf::new([0u8, 1, 2, 0]);
    assert_eq!(buf.tag_at(0), Some(Tag::A));
    assert_eq!(buf.tag_at(1), Some(Tag::B));
    assert_eq!(buf.tag_at(2), Some(Tag::C));
    assert_eq!(buf.tag_at(3), Some(Tag::A));
}

#[test]
fn invalid_bytes_are_rejected_not_transmuted() {
    // The runtime validity check: bytes outside 0..=2 never become a `Tag` (which would be UB).
    let buf = TaggedBuf::new([3u8, 7, 255, 42]);
    for i in 0..4 {
        assert_eq!(buf.tag_at(i), None, "byte {} is not a valid discriminant", i);
    }
    // The raw bytes are still readable as `u8` (every bit pattern is a valid `u8` — no invariant).
    assert_eq!(buf.byte_at(0), Some(3));
    assert_eq!(buf.byte_at(2), Some(255));
}

#[test]
fn out_of_range_index_is_none() {
    let buf = TaggedBuf::new([1u8, 2]);
    assert_eq!(buf.tag_at(2), None);
    assert_eq!(buf.byte_at(2), None);
}

#[test]
fn a_returned_tag_is_a_genuine_valid_value() {
    // What we got back is a real `Tag` we can `match` exhaustively — proof it is a valid bit pattern,
    // not a laundered invalid one (an invalid discriminant would make this `match` unsound).
    let buf = TaggedBuf::new([2u8]);
    let n = match buf.tag_at(0).expect("valid") {
        Tag::A => 0,
        Tag::B => 1,
        Tag::C => 2,
    };
    assert_eq!(n, 2);
}
