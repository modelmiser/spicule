//! Twelfth seam — consumer side, with `unsafe` forbidden at compile time.
//!
//! Proves: ordinary safe code reads validity-checked `Tag`s out of a raw byte buffer with no `unsafe`
//! and no proof — valid discriminants (`0,5,200`) come back as `Some`, everything else as `None` (never
//! an invalid-bit-pattern `Tag`). The danger (an invalid discriminant) is unconstructible in safe code.

#![forbid(unsafe_code)]

use seam_spsc::validity::{Tag, TaggedBuf};

#[test]
fn valid_discriminants_read_back_as_tags() {
    let buf = TaggedBuf::new([0u8, 5, 200, 0]);
    assert_eq!(buf.tag_at(0), Some(Tag::A));
    assert_eq!(buf.tag_at(1), Some(Tag::B));
    assert_eq!(buf.tag_at(2), Some(Tag::C));
    assert_eq!(buf.tag_at(3), Some(Tag::A));
}

#[test]
fn the_set_membership_regression_bytes_inside_the_range_but_invalid() {
    // THE bug the first gauntlet found: a *range* check `b <= 200` would launder these into invalid
    // enums (UB). Set-membership rejects them. `1,2,3,7,199` are all `<= 200` yet NOT valid Tags.
    let buf = TaggedBuf::new([1u8, 2, 3, 7, 199, 201, 255]);
    for i in 0..7 {
        assert_eq!(buf.tag_at(i), None, "byte at {} is in-range-but-invalid or out-of-range", i);
    }
}

#[test]
fn out_of_range_index_is_none() {
    let buf = TaggedBuf::new([5u8, 200]);
    assert_eq!(buf.tag_at(2), None);
    assert_eq!(buf.byte_at(2), None);
    // Raw bytes are always readable (a `u8` has no validity invariant).
    assert_eq!(buf.byte_at(0), Some(5));
}

#[test]
fn a_returned_tag_is_a_genuine_valid_value() {
    // What we got back is a real `Tag` we can `match` exhaustively — proof it is a valid bit pattern,
    // not a laundered invalid one (an invalid discriminant would make this `match` unsound).
    let buf = TaggedBuf::new([200u8]);
    let n = match buf.tag_at(0).expect("valid") {
        Tag::A => 0,
        Tag::B => 5,
        Tag::C => 200,
    };
    assert_eq!(n, 200);
}
