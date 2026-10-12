//! Host tests of the lifted counted-run search, on any host.
//!
//! These cover the edges a reader of the method would ask about: an empty run,
//! a zero or negative count, a zero key under the guard, a match at the first
//! entry, a truncated object, the masked answers near the top of the address
//! range, and the first-entry and end-of-run offsets. They do not use the
//! verified rewrites; the differential test does that on the 32-bit target.

use lf_core::counted_run::{Answer, Count, CountedRun, NO_INDEX, Truncated};

/// A run of `count` entries of four bytes, starting at offset 4.
fn run_of(count: Count, answer: Answer, zero_key_misses: bool) -> CountedRun {
    CountedRun {
        count,
        first: 4,
        stride: 4,
        zero_key_misses,
        answer,
    }
}

/// Little-endian bytes of the given words, laid out from offset zero.
fn bytes_of(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

#[test]
fn empty_fixed_run_is_a_miss_at_once() {
    let run = run_of(Count::Fixed(0), Answer::Flag, false);
    assert_eq!(run.evaluate(&[0; 8], 0x1000, 7), Ok(0));
    let run = run_of(Count::Fixed(0), Answer::Index, false);
    assert_eq!(run.evaluate(&[0; 8], 0x1000, 7), Ok(NO_INDEX));
}

#[test]
fn match_at_the_first_entry_gives_index_zero() {
    // The count word is at offset 0 (a fixed count reads nothing there); the
    // first entry starts at offset 4 and equals the key.
    let object = bytes_of(&[0, 0x55, 0, 0]);
    let run = run_of(Count::Fixed(3), Answer::Index, false);
    assert_eq!(run.evaluate(&object, 0, 0x55), Ok(0));
}

#[test]
fn signed_count_of_zero_or_less_holds_no_entries() {
    // The count word is at offset 0; the entry at offset 4 equals the key.
    for raw in [0i32, -1, i32::MIN] {
        let object = bytes_of(&[raw as u32, 7]);
        let run = run_of(Count::Signed { offset: 0 }, Answer::Flag, false);
        assert_eq!(run.evaluate(&object, 0, 7), Ok(0), "count {raw}");
    }
}

#[test]
fn signed_count_of_one_sees_its_entry() {
    let object = bytes_of(&[1, 7]);
    let run = run_of(Count::Signed { offset: 0 }, Answer::Flag, false);
    assert_eq!(run.evaluate(&object, 0, 7), Ok(1));
}

#[test]
fn unsigned_count_reads_the_word_as_unsigned() {
    // A count with the top bit set is a large unsigned count, not a negative
    // one: the scan reaches the entry that matches only if it can read it.
    let object = bytes_of(&[0x8000_0000, 1, 2, 3]);
    let run = run_of(Count::Unsigned { offset: 0 }, Answer::Index, false);
    assert_eq!(run.evaluate(&object, 0, 3), Ok(2));
}

#[test]
fn zero_key_under_the_guard_is_a_miss_without_reading() {
    // The object is empty: a read would be truncated, so a guarded zero key
    // must not read at all.
    let run = run_of(Count::Fixed(8), Answer::Flag, true);
    assert_eq!(run.evaluate(&[], 0, 0), Ok(0));
    let run = run_of(Count::Fixed(8), Answer::Index, true);
    assert_eq!(run.evaluate(&[], 0, 0), Ok(NO_INDEX));
}

#[test]
fn zero_key_without_the_guard_matches_a_zero_entry() {
    let object = bytes_of(&[1, 0, 2]);
    let run = run_of(Count::Fixed(2), Answer::Index, false);
    assert_eq!(run.evaluate(&object, 0, 0), Ok(0));
}

#[test]
fn truncated_object_is_reported_at_the_first_read_outside_it() {
    // Entry 0 and entry 1 are inside a 12-byte object; entry 2 is not.
    let object = bytes_of(&[0, 1, 2]);
    let run = run_of(Count::Fixed(3), Answer::Flag, false);
    assert_eq!(run.evaluate(&object, 0, 9), Err(Truncated));
    // The key at entry 0 is found before any read outside the object.
    assert_eq!(run.evaluate(&object, 0, 1), Ok(1));
}

#[test]
fn truncated_count_word_is_reported() {
    // The count word would be at offset 6, and the object ends at 8.
    let object = bytes_of(&[1, 2]);
    let run = run_of(Count::Unsigned { offset: 6 }, Answer::Flag, false);
    assert_eq!(run.evaluate(&object, 0, 0), Err(Truncated));
}

#[test]
fn masked_entry_address_clears_the_low_byte_and_sets_bit_zero_on_a_match() {
    // Object at 0x1200_0040: the match is entry 1 at offset 8, so its address
    // is 0x1200_0048, which becomes 0x1200_0001 after the mask.
    let object = bytes_of(&[0, 5, 6]);
    let run = run_of(Count::Fixed(2), Answer::MaskedEntryAddress, false);
    assert_eq!(run.evaluate(&object, 0x1200_0040, 6), Ok(0x1200_0001));
}

#[test]
fn masked_entry_address_on_a_miss_is_the_run_end_with_low_byte_cleared() {
    // The run ends at offset 4 + 2 * 4 = 12; 0x3000_0000 + 12 has low byte
    // 0x0C, which is cleared, and bit 0 is not set on a miss.
    let object = bytes_of(&[0, 5, 6]);
    let run = run_of(Count::Fixed(2), Answer::MaskedEntryAddress, false);
    assert_eq!(run.evaluate(&object, 0x3000_0000, 9), Ok(0x3000_0000));
}

#[test]
fn masked_entry_address_wraps_near_the_top_of_the_address_range() {
    let object = bytes_of(&[0, 5]);
    let run = run_of(Count::Fixed(1), Answer::MaskedEntryAddress, false);
    // The match is entry 0 at offset 4, so the entry's address is
    // 0xFFFF_FFFC + 4, which wraps to 0; masked and with bit 0 set, it is 1.
    assert_eq!(run.evaluate(&object, 0xFFFF_FFFC, 5), Ok(0x0000_0001));
    // Without the wrap the same match near the top is 0xFFFF_FF01.
    assert_eq!(run.evaluate(&object, 0xFFFF_FF00, 5), Ok(0xFFFF_FF01));
}

#[test]
fn masked_key_is_the_key_with_low_byte_cleared_and_bit_zero_on_a_match() {
    // The single entry, at offset 4, holds 0x1234_5678.
    let object = bytes_of(&[0, 0x1234_5678]);
    let run = run_of(Count::Fixed(1), Answer::MaskedKey, false);
    assert_eq!(run.evaluate(&object, 0, 0x1234_5678), Ok(0x1234_5601));
    assert_eq!(run.evaluate(&object, 0, 0x1234_56FF), Ok(0x1234_5600));
}

#[test]
fn run_offsets_wrap_as_32_bit_words() {
    // A stride of 0x8000_0000 makes the second entry wrap back to offset 4.
    let object = bytes_of(&[0, 0, 0, 0]);
    let run = CountedRun {
        count: Count::Fixed(2),
        first: 4,
        stride: 0x8000_0000,
        zero_key_misses: false,
        answer: Answer::Index,
    };
    // Entry 1 starts at 4 + 0x8000_0000 (wrapping), which is outside the
    // object; the first entry matches a zero key, so no read goes there.
    assert_eq!(run.evaluate(&object, 0, 0), Ok(0));
    assert_eq!(run.evaluate(&object, 0, 5), Err(Truncated));
}
