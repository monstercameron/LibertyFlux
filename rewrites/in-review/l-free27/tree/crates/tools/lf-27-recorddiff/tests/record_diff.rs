//! Differential test of the lifted record word read against the five verified
//! stream-table reads of this family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! states are built in this test program: a window of the global tables (the
//! record pointer table and the validity byte table), four record blocks the
//! table entries point to, and an index. The runtime maps the window, so the
//! rewrite reads the tables there. The same bytes, read through the collaborator
//! trait, are the lifted routine's inputs. The returned value is compared, and
//! so is every byte of the window and of the record blocks: a read must write
//! nothing.
//!
//! Each instance is also run against three deliberately wrong versions of its
//! own parameters (the field moved by four bytes, the null answer changed, the
//! miss answer changed); every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{Instance, INSTANCES};
use lf_core::record_word_read::{RecordSlots, RecordWordRead};
use lf_27_recorddiff::rt::set_window;

/// Generated states per instance.
const CASES: u32 = 1000;

/// File address of the first byte of the window: the tables sit inside it.
const WINDOW_VA: u32 = 0x0117_3540;

/// Bytes of the window: covers the record table (eight entries) and the
/// validity table (eight bytes).
const WINDOW_LEN: usize = 0x40;

/// Byte offset in the window of the record pointer table's first entry.
const TABLE_AT: usize = 0x0117_3560 - 0x0117_3540;

/// Byte offset in the window of the validity table's first byte.
const VALID_AT: usize = 0x0117_354C - 0x0117_3540;

/// Table entries the generator fills (indices zero to seven are inside the window).
const SLOTS: u32 = 8;

/// Record blocks the table entries point to.
const RECORDS: usize = 4;

/// Bytes of one record block: past the largest field offset the members read.
const RECORD_LEN: usize = 0x3800;

/// One lock across the whole test program: the window is process-wide state.
static LOCK: Mutex<()> = Mutex::new(());

/// The window of global-table bytes, aligned for the rewrites' word reads.
#[repr(C, align(8))]
struct Window([u8; WINDOW_LEN]);

/// One record block, aligned for the rewrites' word reads.
#[repr(C, align(8))]
struct Record([u8; RECORD_LEN]);

/// The state of one case: the window, the record blocks and the index.
struct State {
    window: Box<Window>,
    records: Vec<Box<Record>>,
}

/// The collaborators of the lifted routine, read from a state's bytes. The
/// record words are read from the blocks by their addresses; no pointer is
/// dereferenced by the lifted code, and no unsafe code is used here.
struct Slots<'a> {
    state: &'a State,
}

impl Slots<'_> {
    /// The address of record block `block`, as the rewrite sees it.
    fn address_of(&self, block: usize) -> u32 {
        &*self.state.records[block] as *const Record as usize as u32
    }

    /// The four bytes at `offset` of `bytes`, as a little-endian word.
    fn read_word(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
    }
}

impl RecordSlots for Slots<'_> {
    fn record_of(&self, index: u32) -> u32 {
        assert!(index < SLOTS, "test bug: an index outside the generated table");
        Self::read_word(&self.state.window.0, TABLE_AT + 4 * index as usize)
    }

    fn valid(&self, index: u32) -> u8 {
        assert!(index < SLOTS, "test bug: an index outside the generated table");
        self.state.window.0[VALID_AT + index as usize]
    }

    fn word(&self, record: u32, offset: u32) -> u32 {
        let block = (0..RECORDS)
            .find(|&b| self.address_of(b) == record)
            .expect("test bug: a record pointer that is no generated block");
        Self::read_word(&self.state.records[block].0, offset as usize)
    }
}

/// A deterministic generator (xorshift64), seeded per instance.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Builds one generated state and returns the index to read.
fn make_state(rng: &mut Rng, state: &mut State) -> u32 {
    for byte in state.window.0.iter_mut() {
        *byte = rng.next() as u8;
    }
    for record in state.records.iter_mut() {
        for byte in record.0.iter_mut() {
            *byte = rng.next() as u8;
        }
    }
    let addresses: Vec<u32> = state
        .records
        .iter()
        .map(|r| &**r as *const Record as usize as u32)
        .collect();
    for slot in 0..SLOTS as usize {
        // Entries are an absent record, or one of the record blocks.
        let pointer = match rng.below(4) {
            0 => 0,
            _ => addresses[rng.below(RECORDS as u64) as usize],
        };
        state.window.0[TABLE_AT + 4 * slot..TABLE_AT + 4 * slot + 4].copy_from_slice(&pointer.to_le_bytes());
        // Validity bytes are zero one time in three.
        state.window.0[VALID_AT + slot] = match rng.below(3) {
            0 => 0,
            _ => 1 + rng.below(255) as u8,
        };
    }
    if rng.below(8) == 0 {
        0xFFFF_FFA6
    } else {
        rng.below(SLOTS as u64) as u32
    }
}

/// The outcome of one instance's cases against one set of parameters.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned value differed, or where the rewrite wrote memory.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Runs `CASES` generated states through the rewrite and through the lifted
/// read built from `read`, and compares them.
fn diff(instance: &Instance, read: &RecordWordRead, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome { cases: 0, mismatches: 0, first: None };
    let mut state = State {
        window: Box::new(Window([0; WINDOW_LEN])),
        records: (0..RECORDS).map(|_| Box::new(Record([0; RECORD_LEN]))).collect(),
    };
    for case in 0..CASES {
        let index = make_state(&mut rng, &mut state);
        let window_before = state.window.0.to_vec();
        let records_before: Vec<Vec<u8>> = state.records.iter().map(|r| r.0.to_vec()).collect();

        let window_address = &*state.window as *const Window as usize as u32;
        set_window(WINDOW_VA, window_address, WINDOW_LEN as u32);
        let expected = (instance.call)(index);

        let wrote = state.window.0[..] != window_before[..]
            || state.records.iter().zip(records_before.iter()).any(|(r, b)| r.0[..] != b[..]);
        let got = read.read(&Slots { state: &state }, index);
        outcome.cases += 1;

        let problem = if wrote {
            Some("the rewrite wrote the tables or a record".to_string())
        } else if got != expected {
            Some(format!("index {index:#010x}: rewrite {expected:#010x}, lifted {got:#010x}"))
        } else {
            None
        };
        if let Some(text) = problem {
            outcome.mismatches += 1;
            if outcome.first.is_none() {
                outcome.first = Some(format!("case {case}: {text}"));
            }
        }
    }
    outcome
}

/// The three wrong versions of an instance's parameters, each a single change.
fn wrong_versions(read: &RecordWordRead) -> [(&'static str, RecordWordRead); 3] {
    [
        ("field plus four", RecordWordRead { field: read.field + 4, ..*read }),
        ("null answer changed", RecordWordRead { null_result: read.null_result ^ 1, ..*read }),
        ("miss answer changed", RecordWordRead { miss_result: read.miss_result ^ 1, ..*read }),
    ]
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x6C62_272E_07BB_0142 ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn lifted_record_read_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, &instance.read, seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(total, 0, "the lifted record read differs from a verified rewrite");
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(&instance.read) {
            let outcome = diff(instance, &wrong, seed_of(index));
            eprintln!(
                "teeth {} {} [{}] mismatches {} of {}",
                instance.address, instance.name, label, outcome.mismatches, outcome.cases
            );
            assert!(
                outcome.mismatches > 0,
                "{} {}: wrong version [{}] was not caught",
                instance.address,
                instance.name,
                label
            );
        }
    }
}
