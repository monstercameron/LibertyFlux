//! Host tests for the streaming slots: edge cases a reader would ask about.
//!
//! Runs on the 64-bit host. The differential proof crate runs the same
//! methods against their verified 32-bit rewrites.

use lf_streaming::slots::DataSlots;
use lf_streaming::slots::KindBytes;
use lf_streaming::slots::KindSlots;
use lf_streaming::slots::ROWS;
use lf_streaming::slots::SlotTable;
use lf_streaming::slots::State;
use lf_streaming::slots::StreamEntry;

fn entry(words: [u32; 6]) -> StreamEntry {
    StreamEntry::from_words(words)
}

#[test]
fn registry_counts_all_proven() {
    assert_eq!(ROWS.len(), 11);
    assert!(ROWS.iter().all(|row| row.state == State::Proven));
}

#[test]
fn empty_entry_bytes() {
    let bytes = StreamEntry::empty().to_bytes();
    assert_eq!(&bytes[..16], &[0u8; 16]);
    assert_eq!(&bytes[16..], &[0xffu8; 8]);
    assert!(!StreamEntry::empty().is_active());
    assert_eq!(StreamEntry::empty().block_count(), 0);
}

#[test]
fn reset_restores_empty() {
    let mut e = entry([1, 2, 3, 4, 5, 6]);
    e.reset();
    assert_eq!(e, StreamEntry::empty());
}

#[test]
fn bytes_round_trip() {
    let e = entry([0x0102_0304, 0xA5A5_A5A5, 0xffff_ffff, 0x8000_1234, 7, 0xDEAD_BEEF]);
    assert_eq!(StreamEntry::from_bytes(e.to_bytes()), e);
}

#[test]
fn active_edges() {
    // Size with only low two bits: inactive unless bit 11 is set.
    assert!(!entry([0, 0, 3, 0, 0, 0]).is_active());
    assert!(entry([0, 0, 4, 0, 0, 0]).is_active());
    assert!(entry([0, 0, 0xffff_fffc, 0, 0, 0]).is_active());
    // Flag bit 11 (0x0800 in the upper half of word 3) activates alone.
    assert!(!entry([0, 0, 0, 0x07ff_0000, 0, 0]).is_active());
    assert!(entry([0, 0, 0, 0x0800_0000, 0, 0]).is_active());
    // Neighbouring flag bits do not.
    assert!(!entry([0, 0, 0, 0x0400_0000, 0, 0]).is_active());
    assert!(!entry([0, 0, 0, 0x1000_0000, 0, 0]).is_active());
    // Low half of word 3 is not flags.
    assert!(!entry([0, 0, 0, 0x0000_ffff, 0, 0]).is_active());
}

#[test]
fn block_count_edges() {
    // (size >> 2) units of 2048, rounded up.
    assert_eq!(entry([0, 0, 0, 0, 0, 0]).block_count(), 0);
    assert_eq!(entry([0, 0, 4, 0, 0, 0]).block_count(), 1);
    assert_eq!(entry([0, 0, 0x2000, 0, 0, 0]).block_count(), 1);
    assert_eq!(entry([0, 0, 0x2004, 0, 0, 0]).block_count(), 2);
    assert_eq!(entry([0, 0, 0xffff_fffc, 0, 0, 0]).block_count(), 0x0020_0000);
    assert_eq!(entry([0, 0, 0xffff_ffff, 0, 0, 0]).block_count(), 0x0020_0000);
}

#[test]
fn data_address_edges() {
    let slots = DataSlots::from_slots([0x1000u32; 256]);
    assert_eq!(StreamEntry::empty().data_address(&slots), 0);
    // Active through size: slot selected by the low byte, high bits added.
    let e = entry([0, 0x0012_3405, 4, 0, 0, 0]);
    assert_eq!(e.data_address(&slots), 0x1000 + 0x1234);
    // Active through the present bit alone.
    let e = entry([0, 0x00ff_ff00, 0, 0x0800_0000, 0, 0]);
    assert_eq!(e.data_address(&slots), 0x1000u32.wrapping_add(0xffff));
    // Wrapping add.
    let wrap = DataSlots::from_slots([0xffff_fff0u32; 256]);
    assert_eq!(e.data_address(&wrap), 0xffff_fff0u32.wrapping_add(0xffff));
}

#[test]
fn locate_empty_is_none() {
    let mut calls = 0;
    let out = StreamEntry::empty().locate(&mut |_: &StreamEntry| {
        calls += 1;
        0x1234
    });
    assert_eq!(out, None);
    assert_eq!(calls, 0);
}

#[test]
fn locate_active_calls_once() {
    let e = entry([0, 7, 0x2004, 0, 0, 0]);
    let mut seen = 0;
    let out = e.locate(&mut |got: &StreamEntry| {
        assert_eq!(*got, e);
        seen += 1;
        0xABCD
    });
    assert_eq!(out, Some((0xABCD, 2)));
    assert_eq!(seen, 1);
}

#[test]
fn usable_edges() {
    let one = SlotTable::from_parts(vec![StreamEntry::empty()], 1);
    assert!(one.is_slot_usable(0));
    // Slack of six past the capacity.
    assert!(one.is_slot_usable(6));
    assert!(!one.is_slot_usable(7));
    // The sentinel never names a slot, even inside the slack.
    assert!(!SlotTable::from_parts(vec![StreamEntry::empty()], 0x1_0000).is_slot_usable(0xffff));
    // Negative indexes are unusable.
    assert!(!one.is_slot_usable(0x8000_0000));
    assert!(!one.is_slot_usable(0xffff_ffff));
    // Negative capacity or no storage: nothing is usable.
    assert!(!SlotTable::from_parts(vec![StreamEntry::empty()], -1).is_slot_usable(0));
    assert!(!SlotTable::from_parts(vec![], 8).is_slot_usable(0));
    // Capacity at the top wraps its slack below zero: unusable.
    assert!(!SlotTable::from_parts(vec![StreamEntry::empty()], i32::MAX).is_slot_usable(0));
    // Zero capacity still allows the slack.
    assert!(SlotTable::from_parts(vec![StreamEntry::empty()], 0).is_slot_usable(5));
    assert!(!SlotTable::from_parts(vec![StreamEntry::empty()], 0).is_slot_usable(6));
}

#[test]
fn mask_always_tests_c6() {
    let table = SlotTable::from_parts(vec![entry([0, 0, 0, 0x0040_0000, 0, 0])], 1);
    // Flag bit 6 is in the always-tested set: fails even with a zero mask.
    assert!(!table.test_mask(0, 0));
    let table = SlotTable::from_parts(vec![entry([0, 0, 0, 0x0100_0000, 0, 0])], 1);
    assert!(table.test_mask(0, 0));
    assert!(!table.test_mask(0, 0x100));
}

#[test]
fn table_active_null_and_flag_paths() {
    assert!(!SlotTable::from_parts(vec![], 4).entry_is_active(0));
    let table =
        SlotTable::from_parts(vec![StreamEntry::empty(), entry([0, 0, 4, 0, 0, 0])], 2);
    assert!(!table.entry_is_active(0));
    assert!(table.entry_is_active(1));
}

#[test]
fn kind_flag_composition() {
    let mut bytes = [0u8; KindBytes::LEN];
    bytes[3 * 160] = 0xAB;
    let kinds = KindBytes::from_bytes(bytes);
    let table = SlotTable::from_parts(vec![entry([0, 3, 0, 0, 0, 0])], 1);
    assert_eq!(table.kind_flag(0, &kinds), (3 * 160) & 0xffff_ff00 | 0xAB);
    // Top value-kind reaches the last table byte exactly.
    let mut bytes = [0u8; KindBytes::LEN];
    bytes[255 * 160] = 0x7E;
    let kinds = KindBytes::from_bytes(bytes);
    let table = SlotTable::from_parts(vec![entry([0, 0xff, 0, 0, 0, 0])], 1);
    assert_eq!(table.kind_flag(0, &kinds), (255 * 160) & 0xffff_ff00 | 0x7E);
}

#[test]
fn set_flag_bit15_sets_only_that_bit() {
    let mut table = SlotTable::from_parts(vec![entry([0, 0, 0, 0x1234_5678, 0, 0])], 1);
    table.set_flag_bit15(0);
    assert_eq!(table.entries()[0].words()[3], 0x9234_5678);
    // Setting it twice is idempotent.
    table.set_flag_bit15(0);
    assert_eq!(table.entries()[0].words()[3], 0x9234_5678);
}

#[test]
fn relative_slot_edges() {
    // Ten entries, kind 9 with base slot 3: answers index minus base.
    let mut entries = vec![];
    for _ in 0..10 {
        entries.push(entry([0, 0, 0, 0, 0, 0x0900_0000]));
    }
    let mut slots = [0u32; 256];
    slots[9] = 3;
    let table = SlotTable::from_parts(entries, 10);
    let kinds = KindSlots::from_slots(slots);
    assert_eq!(table.relative_slot(0, &kinds), 0u32.wrapping_sub(3));
    assert_eq!(table.relative_slot(7, &kinds), 4);
}

#[test]
#[should_panic(expected = "past 1 owned entries")]
fn mask_past_end_panics() {
    SlotTable::from_parts(vec![StreamEntry::empty()], 8).test_mask(1, 0);
}

#[test]
#[should_panic(expected = "past 0 owned entries")]
fn kind_flag_empty_panics() {
    SlotTable::from_parts(vec![], 8).kind_flag(0, &KindBytes::from_bytes([0u8; KindBytes::LEN]));
}

#[test]
#[should_panic(expected = "past 2 owned entries")]
fn set_flag_past_end_panics() {
    SlotTable::from_parts(vec![StreamEntry::empty(), StreamEntry::empty()], 8).set_flag_bit15(9);
}

#[test]
#[should_panic(expected = "past 1 owned entries")]
fn relative_slot_past_end_panics() {
    SlotTable::from_parts(vec![StreamEntry::empty()], 8)
        .relative_slot(4, &KindSlots::from_slots([0u32; 256]));
}
