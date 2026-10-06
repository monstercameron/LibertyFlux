//! Host tests for the streaming slots: edge cases a reader would ask about.
//!
//! Runs on the 64-bit host. The differential proof crate runs the same
//! methods against their verified 32-bit rewrites.

use lf_streaming::slots::AdoptOutcome;
use lf_streaming::slots::AdoptSlot;
use lf_streaming::slots::AllocEntry;
use lf_streaming::slots::AllocTable;
use lf_streaming::slots::ControlBlock;
use lf_streaming::slots::DataSlots;
use lf_streaming::slots::FlagBank;
use lf_streaming::slots::GeoSlot;
use lf_streaming::slots::GeoSlots;
use lf_streaming::slots::IdArray;
use lf_streaming::slots::KeyEntries;
use lf_streaming::slots::KeyEntry;
use lf_streaming::slots::KindBytes;
use lf_streaming::slots::KindSlots;
use lf_streaming::slots::LANE_TABLE_LEN;
use lf_streaming::slots::LANES;
use lf_streaming::slots::LaneTable;
use lf_streaming::slots::ROWS;
use lf_streaming::slots::RecordSet;
use lf_streaming::slots::ResetSlot;
use lf_streaming::slots::SLOT_HEADER_LEN;
use lf_streaming::slots::SLOT_LEN;
use lf_streaming::slots::SlotFlags;
use lf_streaming::slots::SlotHeader;
use lf_streaming::slots::SlotLiveness;
use lf_streaming::slots::SlotTable;
use lf_streaming::slots::State;
use lf_streaming::slots::StateSlots;
use lf_streaming::slots::StreamEntry;
use lf_streaming::slots::WordTable;

fn entry(words: [u32; 6]) -> StreamEntry {
    StreamEntry::from_words(words)
}

#[test]
fn registry_counts_all_proven() {
    assert_eq!(ROWS.len(), 34);
    assert_eq!(
        ROWS.iter().filter(|row| row.state == State::Proven).count(),
        31
    );
    assert_eq!(
        ROWS.iter()
            .filter(|row| row.state == State::Missing)
            .count(),
        3
    );
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
    let e = entry([
        0x0102_0304,
        0xA5A5_A5A5,
        0xffff_ffff,
        0x8000_1234,
        7,
        0xDEAD_BEEF,
    ]);
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
    assert_eq!(
        entry([0, 0, 0xffff_fffc, 0, 0, 0]).block_count(),
        0x0008_0000
    );
    assert_eq!(
        entry([0, 0, 0xffff_ffff, 0, 0, 0]).block_count(),
        0x0008_0000
    );
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
    let table = SlotTable::from_parts(vec![StreamEntry::empty(), entry([0, 0, 4, 0, 0, 0])], 2);
    assert!(!table.entry_is_active(0));
    assert!(table.entry_is_active(1));
}

#[test]
// Two 40KB tables: small for the test stack, exact for the ends.
#[allow(clippy::large_stack_arrays)]
fn kind_flag_composition() {
    let mut bytes = [0u8; KindBytes::LEN];
    bytes[3 * 160] = 0xAB;
    let kinds = KindBytes::from_bytes(bytes);
    let table = SlotTable::from_parts(vec![entry([0, 3, 0, 0, 0, 0])], 1);
    assert_eq!(
        table.kind_flag(0, &kinds),
        (3u32 * 160) & 0xffff_ff00 | 0xAB
    );
    // Top value-kind reaches the last table byte exactly.
    let mut bytes = [0u8; KindBytes::LEN];
    bytes[255 * 160] = 0x7E;
    let kinds = KindBytes::from_bytes(bytes);
    let table = SlotTable::from_parts(vec![entry([0, 0xff, 0, 0, 0, 0])], 1);
    assert_eq!(
        table.kind_flag(0, &kinds),
        (255u32 * 160) & 0xffff_ff00 | 0x7E
    );
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
    let _ = SlotTable::from_parts(vec![StreamEntry::empty()], 8).test_mask(1, 0);
}

#[test]
#[should_panic(expected = "past 0 owned entries")]
// One 40KB zeroed table: the panic happens before any read.
#[allow(clippy::large_stack_arrays)]
fn kind_flag_empty_panics() {
    let _ = SlotTable::from_parts(vec![], 8)
        .kind_flag(0, &KindBytes::from_bytes([0u8; KindBytes::LEN]));
}

#[test]
#[should_panic(expected = "past 2 owned entries")]
fn set_flag_past_end_panics() {
    SlotTable::from_parts(vec![StreamEntry::empty(), StreamEntry::empty()], 8).set_flag_bit15(9);
}

#[test]
#[should_panic(expected = "past 1 owned entries")]
fn relative_slot_past_end_panics() {
    let _ = SlotTable::from_parts(vec![StreamEntry::empty()], 8)
        .relative_slot(4, &KindSlots::from_slots([0u32; 256]));
}

#[test]
fn clearer_each_exit() {
    // Bit 3 cleared, bit 4 set: the first exit, other bits untouched.
    let mut flags = SlotFlags::new(0x1f);
    assert_eq!(flags.clear_and_review(3, 4), 0x1);
    assert_eq!(flags.word(), 0x17);
    // Other bit clear, bit 5 set: the second exit, live bit kept.
    let mut flags = SlotFlags::new(0x33);
    assert_eq!(flags.clear_and_review(4, 3), 0x1);
    assert_eq!(flags.word(), 0x23);
    // Neither state bit: the live bit drops too.
    let mut flags = SlotFlags::new(0x09);
    assert_eq!(flags.clear_and_review(3, 4), 0x0);
    assert_eq!(flags.word(), 0x0);
    // The shifted answer keeps its upper bits.
    let mut flags = SlotFlags::new(0xffff_fff0);
    assert_eq!(flags.clear_and_review(3, 4), 0x0fff_ffff);
}

#[test]
fn marker_sets_live_and_bit3() {
    let mut flags = SlotFlags::new(0);
    flags.mark_live();
    assert_eq!(flags.word(), 0x9);
    let mut flags = SlotFlags::new(0xffff_fff6);
    flags.mark_live();
    assert_eq!(flags.word(), 0xffff_ffff);
}

#[test]
#[should_panic(expected = "flag bit out of range")]
fn clearer_bad_bit_panics() {
    let mut flags = SlotFlags::new(0);
    let _ = flags.clear_and_review(32, 4);
}

#[test]
fn key_scans_first_match_wins() {
    let table = KeyEntries {
        entries: vec![
            KeyEntry {
                key_a: 7,
                key_b: 9,
                target: None,
            },
            KeyEntry {
                key_a: 7,
                key_b: 9,
                target: Some(1),
            },
            KeyEntry {
                key_a: 8,
                key_b: 7,
                target: Some(0),
            },
        ],
    };
    assert_eq!(table.find_by_key(9), Some(0));
    assert_eq!(table.find_by_key(7), Some(2));
    assert_eq!(table.find_by_key(1), None);
    // Live scan: null link and zero state both skip.
    assert_eq!(table.find_live(7), Some(1));
    assert_eq!(table.find_live(8), None);
    assert_eq!(table.find_live(9), None);
}

#[test]
fn state_scan_is_exact_equality() {
    let slots = StateSlots {
        states: vec![2, 4, 3, 3],
    };
    assert_eq!(slots.find_first_holding(3), Some(2));
    assert_eq!(slots.find_first_holding(2), Some(0));
    assert!(
        StateSlots { states: vec![] }
            .find_first_holding(3)
            .is_none()
    );
}

#[test]
fn alloc_direct_and_growth() {
    let free = AllocEntry {
        id: 0,
        flag: 0,
        pad: [1, 2, 3],
        zero: 9,
        link: 8,
    };
    let mut table = AllocTable {
        counter: 0x10,
        entries: vec![free, free],
    };
    // Direct index into a free entry: no match.
    assert_eq!(table.lookup_or_alloc(0), None);
    // Allocate: first free entry wins, pad preserved.
    assert_eq!(table.lookup_or_alloc(0x80), Some(0));
    assert_eq!(table.counter, 0x11);
    let won = &table.entries[0];
    assert_eq!(
        (won.flag, won.id, won.zero, won.link),
        (1, 0x1000_0010, 0, 0xffff_ffff)
    );
    assert_eq!(won.pad, [1, 2, 3]);
    // Direct index into a taken entry: match.
    assert_eq!(table.lookup_or_alloc(0), Some(0));
    // Full table: allocation fails, counter untouched.
    assert_eq!(table.lookup_or_alloc(0xffff_ffff), Some(1));
    assert_eq!(table.lookup_or_alloc(0x81), None);
    assert_eq!(table.counter, 0x12);
    // Counter wraps past the top.
    table.counter = 0xffff_ffff;
    table.entries[1].flag = 0;
    assert_eq!(table.lookup_or_alloc(0x90), Some(1));
    assert_eq!(table.entries[1].id, 0x0fff_ffff);
    assert_eq!(table.counter, 0);
}

#[test]
fn alloc_bytes_round_trip() {
    let entry = AllocEntry {
        id: 0x1234_5678,
        flag: 0xAB,
        pad: [9, 8, 7],
        zero: 1,
        link: 2,
    };
    assert_eq!(AllocEntry::from_bytes(entry.to_bytes()), entry);
}

#[test]
fn radius_edges() {
    let slots = GeoSlots {
        slots: vec![
            GeoSlot {
                flag: 1,
                id: 5,
                center: [3.0, 4.0, 0.0],
            },
            GeoSlot {
                flag: 0,
                id: 5,
                center: [0.0, 0.0, 0.0],
            },
            GeoSlot {
                flag: 2,
                id: 6,
                center: [0.0, 0.0, 0.0],
            },
        ],
    };
    // Exactly on the boundary counts (ordered >=).
    assert!(slots.any_in_radius(5, [0.0, 0.0, 0.0], 5.0));
    assert!(!slots.any_in_radius(5, [0.0, 0.0, 0.0], 4.999));
    // Dead flags and wrong ids never match.
    assert!(!slots.any_in_radius(5, [100.0, 0.0, 0.0], 1.0));
    assert!(slots.any_in_radius(6, [0.0, 0.0, 0.0], 0.0));
    // NaN radius matches nothing.
    assert!(!slots.any_in_radius(5, [0.0, 0.0, 0.0], f32::NAN));
}

#[test]
fn adopt_paths() {
    let mut hook_log = vec![];
    let mut mark_log = vec![];
    // Already adopted: no calls, no write.
    let mut slot = AdoptSlot::new(0x42);
    let out = slot.adopt(
        0x99,
        &mut |generation| hook_log.push(generation),
        &mut |seen: &AdoptSlot| {
            mark_log.push(*seen);
            0x1234
        },
    );
    assert_eq!(out, AdoptOutcome::AlreadyAdopted);
    assert_eq!(slot.owner, 0x42);
    assert!(hook_log.is_empty() && mark_log.is_empty());
    // Orphan: owner stored, hook then marker, marker's answer kept.
    let mut slot = AdoptSlot::new(0xffff_ffff);
    let out = slot.adopt(
        0x99,
        &mut |generation| hook_log.push(generation),
        &mut |seen: &AdoptSlot| {
            mark_log.push(*seen);
            0x1234
        },
    );
    assert_eq!(out, AdoptOutcome::Marked(0x1234));
    assert_eq!(slot.owner, 0x99);
    assert_eq!(hook_log, vec![0x99]);
    assert_eq!(mark_log, vec![AdoptSlot::new(0x99)]);
}

#[test]
fn slot_reset_keeps_top_bits_and_gap() {
    let mut slot = ResetSlot {
        bytes: [0xFF; SLOT_LEN],
    };
    slot.reset();
    assert_eq!(slot.bytes[0], 0xE0);
    assert_eq!(slot.bytes[1], 0);
    // Bytes +2..+8 survive; everything from +8 on is zero.
    assert_eq!(&slot.bytes[2..8], &[0xFF; 6]);
    assert_eq!(&slot.bytes[8..SLOT_LEN], &[0; SLOT_LEN - 8]);
    let mut slot = ResetSlot {
        bytes: [0x1F; SLOT_LEN],
    };
    slot.reset();
    assert_eq!(slot.bytes[0], 0x00);
}

#[test]
fn lane_table_reset_and_codec() {
    let table = LaneTable {
        row0: [1; LANES],
        row1: [2; LANES],
        tail: 3,
        tag: 4,
    };
    assert_eq!(LaneTable::from_bytes(table.to_bytes()), table);
    let mut table = LaneTable {
        row0: [0x1234_5678; LANES],
        row1: [0x9ABC_DEF0; LANES],
        tail: 0x55,
        tag: 0xAA,
    };
    table.reset();
    assert_eq!(table.row0, [0; LANES]);
    assert_eq!(table.row1, [0xFFFF_FFFF; LANES]);
    assert_eq!((table.tail, table.tag), (0, 0));
    let bytes = table.to_bytes();
    assert_eq!(bytes.len(), LANE_TABLE_LEN);
    assert_eq!(&bytes[..0x2c], &[0; 0x2c]);
    assert_eq!(&bytes[0x58..], &[0; 5]);
}

#[test]
fn find_index_edges() {
    let array = IdArray {
        ids: vec![10, 20, 30, 20],
        len: 4,
    };
    assert_eq!(array.find_from(20, 0), Some(1));
    assert_eq!(array.find_from(20, 2), Some(3));
    assert_eq!(array.find_from(10, 0), Some(0));
    assert_eq!(array.find_from(99, 0), None);
    // Start at or past the end: no match.
    assert_eq!(array.find_from(10, 4), None);
    assert_eq!(array.find_from(10, 40), None);
    // A short length hides the tail.
    let array = IdArray {
        ids: vec![10, 20, 30, 20],
        len: 2,
    };
    assert_eq!(array.find_from(30, 0), None);
    // A negative length matches nothing from a valid start.
    let array = IdArray {
        ids: vec![10],
        len: -1,
    };
    assert_eq!(array.find_from(10, 0), None);
}

#[test]
#[should_panic(expected = "past 2 owned ids")]
fn find_index_past_owned_panics() {
    let _ = IdArray {
        ids: vec![1, 2],
        len: 9,
    }
    .find_from(9, 0);
}

#[test]
#[should_panic(expected = "is negative")]
fn find_index_negative_start_panics() {
    let _ = IdArray {
        ids: vec![1],
        len: 1,
    }
    .find_from(1, 0xFFFF_FFFF);
}

#[test]
fn control_match_banks_and_calls() {
    let block = ControlBlock {
        flags_a: vec![0, 5],
        flags_b: vec![7, 0],
        entries: vec![[1; 8], [2; 8]],
    };
    let mut calls = 0;
    // Bank A enables slot 1 only; bank B slot 0 only.
    assert!(!block.match_slot(9, 0, FlagBank::A, &mut |_, _| {
        calls += 1;
        0
    }));
    assert_eq!(calls, 0);
    assert!(block.match_slot(9, 1, FlagBank::A, &mut |entry, key| {
        assert_eq!((entry, key), ([2; 8], 9));
        calls += 1;
        0
    }));
    assert!(block.match_slot(9, 0, FlagBank::B, &mut |_, _| {
        calls += 1;
        0
    }));
    assert!(!block.match_slot(9, 1, FlagBank::B, &mut |_, _| {
        calls += 1;
        0
    }));
    assert_eq!(calls, 2);
    // A nonzero difference word is a mismatch.
    assert!(!block.match_slot(9, 1, FlagBank::A, &mut |_, _| 1));
}

#[test]
fn control_reset_order_and_effects() {
    let mut block = ControlBlock {
        flags_a: vec![9, 8],
        flags_b: vec![7, 6],
        entries: vec![[1; 8], [2; 8]],
    };
    let mut log: Vec<(u32, u32, u32)> = vec![];
    let mut releases: Vec<u32> = vec![];
    block.reset_slot(
        1,
        &mut |idx, code| log.push((0, idx, code)),
        &mut |offset| releases.push(offset),
    );
    assert_eq!(log, vec![(0, 1, 1)]);
    assert_eq!(releases, vec![(1 + 2) * 8, (1 + 0x10) * 8]);
    assert_eq!(block.flags_a, vec![9, 0]);
    assert_eq!(block.flags_b, vec![7, 6]);
    assert_eq!(block.entries, vec![[1; 8], [0; 8]]);
}

#[test]
#[should_panic(expected = "past 1 owned slots")]
fn control_match_past_end_panics() {
    let block = ControlBlock {
        flags_a: vec![1],
        flags_b: vec![1],
        entries: vec![[0; 8]],
    };
    let _ = block.match_slot(0, 4, FlagBank::A, &mut |_, _| 0);
}

#[test]
fn key_search_words() {
    let set = RecordSet {
        keys: vec![b"beta".to_vec(), b"delta".to_vec()],
        count: 2,
    };
    assert_eq!(set.contains_key(b"beta"), 1);
    assert_eq!(set.contains_key(b"delta"), 1);
    // No match: the last key sorts below "gamma".
    assert_eq!(set.contains_key(b"gamma"), 0xFFFF_FF00);
    // No match: the last key sorts above "aardvark".
    assert_eq!(set.contains_key(b"aardvark"), 0);
    // Prefix keys: the terminator sorts below any longer key.
    let set = RecordSet {
        keys: vec![b"app".to_vec()],
        count: 1,
    };
    assert_eq!(set.contains_key(b"apple"), 0xFFFF_FF00);
    assert_eq!(set.contains_key(b"app"), 1);
    // An empty count matches nothing.
    assert_eq!(
        RecordSet {
            keys: vec![],
            count: 0
        }
        .contains_key(b"x"),
        0
    );
    // High counts scan like any other: an early match answers 1.
    // (The rewrite's doc says 0x8000+ answers the count residue, but
    // its signed check sits behind a zero-extending load, so only
    // the zero count takes it; the original's bytes agree.)
    let set = RecordSet {
        keys: vec![b"hit".to_vec()],
        count: 0x8000,
    };
    assert_eq!(set.contains_key(b"hit"), 1);
    assert_eq!(set.position(b"hit"), Some(0));
    // Lookups share the compare but answer indexes.
    let set = RecordSet {
        keys: vec![b"a".to_vec(), b"b".to_vec(), b"a".to_vec()],
        count: 3,
    };
    assert_eq!(set.position(b"a"), Some(0));
    assert_eq!(set.position(b"b"), Some(1));
    assert_eq!(set.position(b"c"), None);
    assert_eq!(
        RecordSet {
            keys: vec![b"a".to_vec()],
            count: 0
        }
        .position(b"a"),
        None
    );
    assert_eq!(
        RecordSet {
            keys: vec![b"a".to_vec()],
            count: 0x8000
        }
        .position(b"a"),
        Some(0)
    );
}

#[test]
#[should_panic(expected = "past 1 owned keys")]
fn key_search_high_count_past_owned_panics() {
    let _ = RecordSet {
        keys: vec![b"a".to_vec()],
        count: 0xFFFF,
    }
    .contains_key(b"z");
}

#[test]
#[should_panic(expected = "past 1 owned keys")]
fn key_search_past_owned_panics() {
    let _ = RecordSet {
        keys: vec![b"a".to_vec()],
        count: 5,
    }
    .contains_key(b"z");
}

#[test]
fn checked_fetch_edges() {
    let table = WordTable {
        words: vec![11, 0, 33],
        count: 3,
    };
    assert_eq!(table.get(0), Some(11));
    assert_eq!(table.get(1), Some(0));
    assert_eq!(table.get(2), Some(33));
    assert_eq!(table.get(3), None);
    assert_eq!(table.get(0x7FFF_FFFF), None);
    assert_eq!(
        WordTable {
            words: vec![11],
            count: 0
        }
        .get(0),
        None
    );
}

#[test]
#[should_panic(expected = "is negative")]
fn checked_fetch_negative_panics() {
    let _ = WordTable {
        words: vec![11],
        count: 1,
    }
    .get(0xFFFF_FFFF);
}

#[test]
fn liveness_limits_and_masking() {
    let live = SlotLiveness { used: vec![0; 75] };
    // Small set (flag low byte nonzero): 0..15 free, 15 not.
    assert!(live.is_free(0, 1));
    assert!(live.is_free(14, 0xFF));
    assert!(!live.is_free(15, 1));
    // Big set (flag low byte zero): 0..75 free.
    assert!(live.is_free(15, 0));
    assert!(live.is_free(74, 0x100));
    assert!(!live.is_free(75, 0));
    // Only the low flag byte matters: 0x100 behaves like 0.
    assert!(live.is_free(20, 0x100));
    assert!(!live.is_free(20, 0x101));
    // Negative indexes are never free.
    assert!(!live.is_free(0x8000_0000, 0));
    // A used byte is not free under either limit.
    let mut used = vec![0; 75];
    used[3] = 7;
    let live = SlotLiveness { used };
    assert!(!live.is_free(3, 0));
    assert!(!live.is_free(3, 1));
}

#[test]
fn header_init_writes_three_fields() {
    let mut header = SlotHeader::from_bytes([0xAA; SLOT_HEADER_LEN]);
    header.init(0x1234_5678);
    assert_eq!(
        (header.zero, header.flag, header.value),
        (0, 1, 0x1234_5678)
    );
    assert_eq!(header.pad, [0xAA; 3]);
    assert_eq!(header.to_bytes()[..4], [0; 4]);
    assert_eq!(SlotHeader::from_bytes(header.to_bytes()), header);
}
