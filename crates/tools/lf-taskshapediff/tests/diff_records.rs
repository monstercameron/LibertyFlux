//! Differential cases of the typed-record scan against its verified 32-bit
//! rewrite.
//!
//! The lifted scan and the rewrite run on the same generated record table,
//! anchor, radius, limit and fetched points. The effects compared are:
//!
//! - the fetch calls, in order, with the record index and the scratch slot
//!   (the rewrite's stub maps the record address back to its index and
//!   reads the tag the table holds; the lifted collaborator logs the same
//!   entries);
//! - the returned count, and every entry the scan wrote to the output, bit
//!   for bit (four fetched values and four constants per entry);
//! - every output byte past the entries, which the rewrite must leave as it
//!   found it.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses, reads
//! the table through the planted globals and calls a numbered stub.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{
    Anchor, ENTRY_CONSTANTS, FetchCallee, RecordTable, ScanEntry, ScratchSlot, TABLE_SLOTS,
    TableRecord, slot_for_tag,
};
use lf_taskshapediff::rewrites::fn_00b2fd30;
use lf_taskshapediff::{Xorshift, rt};

/// File address of record 0 of the table.
const TABLE_VA: u32 = 0x0166_1A60;
/// Bytes from one record to the next.
const RECORD_STRIDE: u32 = 0x50;
/// Bytes of the table: thirty-two records.
const TABLE_BYTES: usize = 0xA00;
/// Bytes of one output entry.
const ENTRY_BYTES: usize = 0x20;
/// Output bytes planted: room for every entry plus a tail the scan must not
/// touch.
const OUT_BYTES: usize = ENTRY_BYTES * 34;
/// Offset of the anchor pointer in the object.
const ANCHOR_SLOT: usize = 0x20;
/// Offset of the anchor's three components.
const ANCHOR_X: usize = 0x30;
/// Bytes planted for the object and for the anchor block.
const OBJECT_BYTES: usize = 0x40;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00B2;

/// Callee tag in the log.
const TAG_FETCH: u32 = 1;

/// One logged call: tag, record index, slot code.
type Call = (u32, u32, u32);

/// The call log of the rewrite's stub, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// The planted table's process address, as the stub sees it.
static TABLE_PTR: Mutex<u32> = Mutex::new(0);
/// The tags of the planted records, in table order.
static TAGS: Mutex<[u32; TABLE_SLOTS]> = Mutex::new([0; TABLE_SLOTS]);
/// The four values each record's fetch answers, in table order.
static POINTS: Mutex<[[f32; 4]; TABLE_SLOTS]> = Mutex::new([[0.0; 4]; TABLE_SLOTS]);
/// Serialises the cases: the stub, the callee table and the extras are
/// process-wide.
static LOCK: Mutex<()> = Mutex::new(());

fn slot_code(slot: ScratchSlot) -> u32 {
    match slot {
        ScratchSlot::First => 0,
        ScratchSlot::Second => 1,
        ScratchSlot::Third => 2,
    }
}

/// The fetch stub the rewrite reaches through its callee slot. It maps the
/// record address back to the record's index, writes the record's four
/// values into the buffer it is given and answers the buffer.
extern "thiscall" fn stub_fetch(record_address: u32, buffer: u32) -> u32 {
    let index = (record_address.wrapping_sub(*TABLE_PTR.lock().unwrap()) / RECORD_STRIDE) as usize;
    let tag = TAGS.lock().unwrap()[index];
    let slot = slot_for_tag(tag).unwrap_or(ScratchSlot::First);
    LOG.lock()
        .unwrap()
        .push((TAG_FETCH, index as u32, slot_code(slot)));
    let points = POINTS.lock().unwrap()[index];
    for (i, value) in points.iter().enumerate() {
        let at = buffer.wrapping_add(4 * i as u32);
        // SAFETY: the rewrite passes a four-word scratch buffer on its own stack, which is live for the call.
        unsafe { (at as *mut u32).write_unaligned(value.to_bits()) };
    }
    buffer
}

/// The collaborator the lifted scan calls: it logs the same entries as the
/// rewrite's stub and answers the same values.
struct Recorder {
    anchor: Anchor,
    log: Vec<Call>,
}

impl FetchCallee for Recorder {
    fn anchor(&self) -> Anchor {
        self.anchor
    }
    fn fetch(&mut self, record: usize, slot: ScratchSlot) -> [f32; 4] {
        self.log.push((TAG_FETCH, record as u32, slot_code(slot)));
        POINTS.lock().unwrap()[record]
    }
}

/// One generated case and what the rewrite did with it.
struct Case {
    table: Vec<u8>,
    anchor_block: Vec<u8>,
    out_before: Vec<u8>,
    out_after: Vec<u8>,
    radius_bits: u32,
    max: i32,
    returned: u32,
    calls: Vec<Call>,
}

/// The address of a planted buffer, as the 32-bit rewrite sees it.
fn address_of(buffer: &[u8]) -> u32 {
    buffer.as_ptr() as usize as u32
}

/// Reads a little-endian word at a byte offset of a planted buffer.
fn word_at(buffer: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],
    ])
}

/// Writes a little-endian word at a byte offset of a planted buffer.
fn set_word(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// Decodes the planted table into the owned records: each record's tag.
fn decode_table(table: &[u8]) -> RecordTable {
    let mut out = RecordTable::default();
    for (k, record) in out.records.iter_mut().enumerate() {
        *record = TableRecord {
            tag: word_at(table, k * RECORD_STRIDE as usize),
        };
    }
    out
}

/// A tag drawn from the three scanned tags, or at random.
fn tag_pick(rng: &mut Xorshift) -> u32 {
    match rng.next_u32() % 5 {
        0 => 0x11,
        1 => 0x12,
        2 => 0x1B,
        _ => rng.word(),
    }
}

/// Generates a case, plants it and runs the rewrite on it.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let mut table: Vec<u8> = (0..TABLE_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    let table_address = address_of(&table);
    let mut tags = [0_u32; TABLE_SLOTS];
    let mut points = [[0.0_f32; 4]; TABLE_SLOTS];
    let mut anchor_block: Vec<u8> = (0..OBJECT_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    let anchor = [rng.word(), rng.word(), rng.word()];
    set_word(&mut anchor_block, ANCHOR_X, anchor[0]);
    set_word(&mut anchor_block, ANCHOR_X + 4, anchor[1]);
    set_word(&mut anchor_block, ANCHOR_X + 8, anchor[2]);
    for k in 0..TABLE_SLOTS {
        let tag = tag_pick(rng);
        tags[k] = tag;
        set_word(&mut table, k * RECORD_STRIDE as usize, tag);
        // A quarter of the records fetch the anchor itself, so the distance
        // is exactly zero and the strict comparison is exercised at zero.
        let on_anchor = rng.next_u32() % 4 == 0;
        let first = if on_anchor {
            f32::from_bits(anchor[0])
        } else {
            f32::from_bits(rng.word())
        };
        let second = if on_anchor {
            f32::from_bits(anchor[1])
        } else {
            f32::from_bits(rng.word())
        };
        let third = if on_anchor {
            f32::from_bits(anchor[2])
        } else {
            f32::from_bits(rng.word())
        };
        points[k] = [first, second, third, f32::from_bits(rng.word())];
        rt::set_extra(
            TABLE_VA + k as u32 * RECORD_STRIDE,
            table_address + k as u32 * RECORD_STRIDE,
        );
    }
    *TABLE_PTR.lock().unwrap() = table_address;
    *TAGS.lock().unwrap() = tags;
    *POINTS.lock().unwrap() = points;

    let mut object: Vec<u8> = (0..OBJECT_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    set_word(&mut object, ANCHOR_SLOT, address_of(&anchor_block));

    let mut out: Vec<u8> = (0..OUT_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    let out_before = out.clone();

    let radius_bits = match rng.next_u32() % 4 {
        0 => 0,
        1 => 0x447A_0000,
        _ => rng.word(),
    };
    let max = match rng.next_u32() % 6 {
        0 => -2,
        1 => 0,
        2 => 1,
        3 => 2,
        4 => 5,
        _ => 40,
    };
    LOG.lock().unwrap().clear();
    let returned = fn_00b2fd30::rw_00b2fd30(
        address_of(&object),
        radius_bits,
        address_of(&out),
        max as u32,
    );
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    let out_after = std::mem::take(&mut out);
    Case {
        table,
        anchor_block,
        out_before,
        out_after,
        radius_bits,
        max,
        returned,
        calls,
    }
}

/// Decodes the anchor the lift takes from the case's anchor block.
fn anchor_of(case: &Case) -> Anchor {
    Anchor {
        x: f32::from_bits(word_at(&case.anchor_block, ANCHOR_X)),
        y: f32::from_bits(word_at(&case.anchor_block, ANCHOR_X + 4)),
        z: f32::from_bits(word_at(&case.anchor_block, ANCHOR_X + 8)),
    }
}

/// Decodes the entries the rewrite wrote: `count` entries from the start of
/// the output, bit for bit.
fn entries_written(out: &[u8], count: usize) -> Vec<[u32; 8]> {
    (0..count)
        .map(|k| {
            let base = k * ENTRY_BYTES;
            let mut words = [0_u32; 8];
            for (i, word) in words.iter_mut().enumerate() {
                *word = word_at(out, base + 4 * i);
            }
            words
        })
        .collect()
}

/// The bit pattern of an entry, in output order.
fn entry_bits(entry: &ScanEntry) -> [u32; 8] {
    let mut words = [0_u32; 8];
    for (i, value) in entry
        .points
        .iter()
        .chain(entry.constants.iter())
        .enumerate()
    {
        words[i] = value.to_bits();
    }
    words
}

/// Checks the rewrite's own effects: that it wrote only the entries, and
/// that its count is the number of entries it wrote.
fn check_rewrite_effects(case: &Case) {
    let count = case.returned as usize;
    assert!(count <= TABLE_SLOTS, "at most one entry per record");
    let written_end = count * ENTRY_BYTES;
    for offset in written_end..OUT_BYTES {
        assert_eq!(
            case.out_before[offset], case.out_after[offset],
            "output byte {offset:#x} past the entries is unchanged"
        );
    }
}

/// Runs the lift on a case and returns its entries and its calls.
fn run_lift(case: &Case) -> (Vec<ScanEntry>, Vec<Call>) {
    let table = decode_table(&case.table);
    let mut recorder = Recorder {
        anchor: anchor_of(case),
        log: Vec::new(),
    };
    let kept = table.collect_near(f32::from_bits(case.radius_bits), case.max, &mut recorder);
    (kept, recorder.log)
}

/// Asserts that the lift matches the rewrite on one case.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    let (kept, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the fetch calls match in order");
    assert_eq!(kept.len(), case.returned as usize, "the count matches");
    let written = entries_written(&case.out_after, kept.len());
    for (k, entry) in kept.iter().enumerate() {
        assert_eq!(
            entry_bits(entry),
            written[k],
            "entry {k} matches bit for bit"
        );
    }
}

/// The deviations of a wrong version. Each is the scan with one change.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// Records with an unscanned tag are fetched into the first slot too.
    NoTagFilter,
    /// The limit is compared with a greater-or-equal test.
    LessOrEqual,
    /// The limit is checked one entry late.
    LateBreak,
    /// The radius is not squared.
    RadiusNotSquared,
    /// The third entry constant is 499.
    OtherConstant,
    /// The fetched values are stored in reverse order.
    ReversedPoints,
    /// The third component is left out of the distance.
    NoThirdComponent,
}

/// The wrong version of the scan: the same calls and entries, with one
/// deviation. Returns the entries and the calls.
fn run_wrong(kind: Wrong, case: &Case) -> (Vec<[u32; 8]>, Vec<Call>) {
    let table = decode_table(&case.table);
    let anchor = anchor_of(case);
    let radius = f32::from_bits(case.radius_bits);
    let limit = if matches!(kind, Wrong::RadiusNotSquared) {
        radius
    } else {
        radius * radius
    };
    let points_table = *POINTS.lock().unwrap();
    let mut calls = Vec::new();
    let mut entries = Vec::new();
    let mut late_taken = false;
    for (index, record) in table.records.iter().enumerate() {
        let slot = match slot_for_tag(record.tag) {
            Some(slot) => slot,
            None if matches!(kind, Wrong::NoTagFilter) => ScratchSlot::First,
            None => continue,
        };
        calls.push((TAG_FETCH, index as u32, slot_code(slot)));
        let mut points = points_table[index];
        if matches!(kind, Wrong::ReversedPoints) {
            points.reverse();
        }
        let dx = points[0] - anchor.x;
        let dy = points[1] - anchor.y;
        let dz = points[2] - anchor.z;
        let dist = if matches!(kind, Wrong::NoThirdComponent) {
            (dy * dy) + (dx * dx)
        } else {
            ((dy * dy) + (dx * dx)) + (dz * dz)
        };
        let keep = if matches!(kind, Wrong::LessOrEqual) {
            limit >= dist
        } else {
            limit > dist
        };
        if keep {
            let mut constants = ENTRY_CONSTANTS;
            if matches!(kind, Wrong::OtherConstant) {
                constants[2] = 499.0;
            }
            let entry = ScanEntry { points, constants };
            entries.push(entry_bits(&entry));
            if entries.len() as i32 >= case.max {
                if !matches!(kind, Wrong::LateBreak) || late_taken {
                    break;
                }
                late_taken = true;
            }
        }
    }
    (entries, calls)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (entries, calls) = run_wrong(kind, &case);
        let written = entries_written(&case.out_after, case.returned as usize);
        if calls != case.calls || entries != written {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

#[test]
fn record_scan_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_fetch as *const () as usize as u32);
    let mut rng = Xorshift::new(SEED);
    let mut total = 0_u32;
    let mut zero_radius = 0_u32;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        total += case.returned;
        if case.radius_bits == 0 {
            zero_radius += 1;
        }
    }
    eprintln!("cases {CASES}: entries kept {total}, zero radius {zero_radius}");
}

#[test]
fn record_scan_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_fetch as *const () as usize as u32);
    for kind in [
        Wrong::NoTagFilter,
        Wrong::LessOrEqual,
        Wrong::LateBreak,
        Wrong::RadiusNotSquared,
        Wrong::OtherConstant,
        Wrong::ReversedPoints,
        Wrong::NoThirdComponent,
    ] {
        assert!(
            wrong_mismatches(kind) > 0,
            "wrong version {kind:?} is caught"
        );
    }
}
