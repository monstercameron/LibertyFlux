//! Differential test of the lifted record field access against the four
//! verified routines of this family.
//!
//! Each instance is one verified rewrite. For each instance, CASES generated
//! states are built in this test program: a window of the global tables (the
//! record pointer table and the validity byte table), four record blocks the
//! table entries point to, an index, an operand and a value. The runtime maps
//! the window, so the rewrite reads the tables there and writes the real record
//! blocks. The lifted routine runs through the collaborator trait on copies of
//! the blocks and window. The returned value (for the flag read) and every byte
//! of the blocks and the window are compared, so every store must be made at
//! the same place with the same value, and nothing else may change.
//!
//! Record pointers that a validity-guarded store would write through are never
//! zero in the generated states: the original does not check them, so a zero
//! pointer there would write to address zero. Zero pointers are generated only
//! for the routines that check them.
//!
//! Each instance is also run against three deliberately wrong versions of its
//! own parameters (the field moved by four bytes, the null index changed, and
//! for a store the guard flipped or for a flag read the byte moved by one);
//! every one must be caught on every instance.

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{Instance, INSTANCES, Routine};
use lf_27_recfielddiff::rt::set_window;
use lf_core::record_field_access::{Guard, RecordFlag, RecordStore, RecordTable};

/// Generated states per instance.
const CASES: u32 = 1000;

/// File address of the first byte of the window: the tables sit inside it.
const WINDOW_VA: u32 = 0x0117_3540;

/// Bytes of the window: covers the record table (eight entries) and the validity table.
const WINDOW_LEN: usize = 0x40;

/// Byte offset in the window of the record pointer table's first entry.
const TABLE_AT: usize = 0x0117_3560 - 0x0117_3540;

/// Byte offset in the window of the validity table's first byte.
const VALID_AT: usize = 0x0117_354C - 0x0117_3540;

/// Table entries the generator fills (indices zero to seven are inside the window).
const SLOTS: u32 = 8;

/// Record blocks the table entries point to.
const RECORDS: usize = 4;

/// Bytes of one record block: past the largest offset the members write or read.
const RECORD_LEN: usize = 0x3800;

/// The null index the members test for.
const NULL: u32 = 0xFFFF_FFA6;

/// One lock across the whole test program: the window is process-wide state.
static LOCK: Mutex<()> = Mutex::new(());

/// The window of global-table bytes, aligned for the rewrites' word accesses.
#[repr(C, align(8))]
struct Window([u8; WINDOW_LEN]);

/// One record block, aligned for the rewrites' word accesses.
#[repr(C, align(8))]
struct Record([u8; RECORD_LEN]);

/// The state of one case: the window and the record blocks.
#[derive(Clone)]
struct Blocks {
    window: Vec<u8>,
    records: Vec<Vec<u8>>,
}

/// The collaborators of the lifted routines, over a copy of the blocks. The
/// record pointers are the addresses of the original blocks, and the copy maps
/// them to its own blocks by position. No pointer is dereferenced here.
struct Table {
    blocks: Blocks,
    addresses: [u32; RECORDS],
    /// Set when a routine stores through a pointer that is no record block: the
    /// original would write to memory it does not own, so the case is a mismatch.
    faulted: bool,
}

impl Table {
    fn block(&self, record: u32) -> Option<usize> {
        self.addresses.iter().position(|&a| a == record)
    }
}

impl RecordTable for Table {
    /// An index past the generated table has no entry: the collaborator answers
    /// as an empty slot. The correct routines never ask for such an index (the
    /// null index is answered first); a wrong routine that does is caught by the
    /// comparison, not by a panic here.
    fn record_of(&self, index: u32) -> u32 {
        if index >= SLOTS {
            return 0;
        }
        let at = TABLE_AT + 4 * index as usize;
        let w = &self.blocks.window[at..at + 4];
        u32::from_le_bytes([w[0], w[1], w[2], w[3]])
    }

    fn valid(&self, index: u32) -> u8 {
        if index >= SLOTS {
            return 0;
        }
        self.blocks.window[VALID_AT + index as usize]
    }

    fn byte_at(&self, record: u32, offset: u32) -> u8 {
        match self.block(record) {
            Some(block) => self.blocks.records[block][offset as usize],
            None => 0,
        }
    }

    fn put_byte(&mut self, record: u32, offset: u32, value: u8) {
        match self.block(record) {
            Some(block) => self.blocks.records[block][offset as usize] = value,
            None => self.faulted = true,
        }
    }

    fn put_word(&mut self, record: u32, offset: u32, value: u32) {
        match self.block(record) {
            Some(block) => {
                let at = offset as usize;
                self.blocks.records[block][at..at + 4].copy_from_slice(&value.to_le_bytes());
            }
            None => self.faulted = true,
        }
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

/// The lifted routine under test.
#[derive(Clone, Copy)]
enum Lifted {
    /// A store with an operand and a value.
    Store3(RecordStore),
    /// A store with a value and no operand.
    Store2(RecordStore),
    /// A flag read.
    Flag(RecordFlag),
}

/// Whether a routine's guard lets the table hold a zero record pointer.
fn zero_records_allowed(lifted: Lifted) -> bool {
    match lifted {
        Lifted::Store3(s) | Lifted::Store2(s) => s.guard == Guard::Record,
        Lifted::Flag(_) => true,
    }
}

/// Fills `state` with one generated case and returns the index, the operand and the value.
fn make_case(rng: &mut Rng, state: &mut Blocks, addresses: &[u32; RECORDS], zero_ok: bool) -> (u32, u32, u32) {
    for byte in state.window.iter_mut() {
        *byte = rng.next() as u8;
    }
    // Half the blocks are mostly zero, so that a flag byte read at a neighbouring
    // offset differs from the one the original reads.
    for block in state.records.iter_mut() {
        let sparse = rng.below(2) == 0;
        for byte in block.iter_mut() {
            *byte = if sparse && rng.below(4) != 0 { 0 } else { rng.next() as u8 };
        }
    }
    for slot in 0..SLOTS as usize {
        let pointer = if zero_ok && rng.below(4) == 0 {
            0
        } else {
            addresses[rng.below(RECORDS as u64) as usize]
        };
        state.window[TABLE_AT + 4 * slot..TABLE_AT + 4 * slot + 4].copy_from_slice(&pointer.to_le_bytes());
        state.window[VALID_AT + slot] = match rng.below(3) {
            0 => 0,
            _ => 1 + rng.below(255) as u8,
        };
    }
    let index = if rng.below(8) == 0 { NULL } else { rng.below(SLOTS as u64) as u32 };
    let operand = rng.below(24) as u32;
    let value = rng.next() as u32;
    (index, operand, value)
}

/// The outcome of one instance's cases against one lifted routine.
struct Outcome {
    /// Cases run.
    cases: u32,
    /// Cases whose returned value or memory differed.
    mismatches: u32,
    /// The first failing case, described for a reader.
    first: Option<String>,
}

/// Runs `CASES` generated states through the rewrite and through the lifted
/// routine, and compares them.
fn diff(instance: &Instance, lifted: Lifted, seed: u64) -> Outcome {
    let mut rng = Rng(seed);
    let mut outcome = Outcome { cases: 0, mismatches: 0, first: None };
    // The generated states follow the instance's own routine, not the wrong
    // version under test: the original decides which record pointers may be zero.
    let zero_ok = zero_records_allowed(lifted_of(instance));
    let mut window = Box::new(Window([0; WINDOW_LEN]));
    let mut real: Vec<Box<Record>> = (0..RECORDS).map(|_| Box::new(Record([0; RECORD_LEN]))).collect();
    for case in 0..CASES {
        let addresses: [u32; RECORDS] = std::array::from_fn(|b| &*real[b] as *const Record as usize as u32);
        let mut state = Blocks {
            window: vec![0; WINDOW_LEN],
            records: vec![vec![0; RECORD_LEN]; RECORDS],
        };
        let (index, operand, value) = make_case(&mut rng, &mut state, &addresses, zero_ok);
        window.0.copy_from_slice(&state.window);
        for (b, record) in real.iter_mut().enumerate() {
            record.0.copy_from_slice(&state.records[b]);
        }
        let window_address = &*window as *const Window as usize as u32;
        set_window(WINDOW_VA, window_address, WINDOW_LEN as u32);

        let mut table = Table { blocks: state.clone(), addresses, faulted: false };
        let (expected, got) = match (instance.routine, lifted) {
            (Routine::Store3(f, _), Lifted::Store3(store)) => {
                f(index, operand, value);
                store.store(&mut table, index, operand, value);
                (None, None)
            }
            (Routine::Store2(f, _), Lifted::Store2(store)) => {
                f(index, value);
                store.store(&mut table, index, 0, value);
                (None, None)
            }
            (Routine::Flag(f, _), Lifted::Flag(flag)) => {
                let answer = u32::from(f(index, operand));
                (Some(answer), Some(flag.read(&table, index, operand)))
            }
            _ => panic!("test bug: a routine paired with the wrong lifted form"),
        };
        outcome.cases += 1;

        let mut problem = None;
        if table.faulted {
            problem = Some("the lifted routine stored through a pointer that is no record".to_string());
        }
        if problem.is_none() && window.0[..] != state.window[..] {
            problem = Some("the window differs".to_string());
        }
        for (b, record) in real.iter().enumerate() {
            if problem.is_none() && record.0[..] != table.blocks.records[b][..] {
                let at = record.0.iter().zip(table.blocks.records[b].iter()).position(|(a, c)| a != c).unwrap_or(0);
                problem = Some(format!("record block {b} differs first at byte {at:#x}"));
            }
        }
        if problem.is_none() {
            if let (Some(answer), Some(lifted_answer)) = (expected, got) {
                if answer != lifted_answer {
                    problem = Some(format!("rewrite answered {answer}, lifted {lifted_answer}"));
                }
            }
        }
        if let Some(text) = problem {
            outcome.mismatches += 1;
            if outcome.first.is_none() {
                outcome.first = Some(format!("case {case}: index {index:#010x}, operand {operand}: {text}"));
            }
        }
    }
    outcome
}

/// The three wrong versions of an instance's lifted routine, each a single change.
fn wrong_versions(lifted: Lifted) -> [(&'static str, Lifted); 3] {
    match lifted {
        Lifted::Store3(s) => [
            ("field plus four", Lifted::Store3(RecordStore { field: s.field + 4, ..s })),
            ("null index changed", Lifted::Store3(RecordStore { null_index: 0, ..s })),
            ("guard flipped", Lifted::Store3(RecordStore {
                guard: if s.guard == Guard::Record { Guard::Validity } else { Guard::Record },
                ..s
            })),
        ],
        Lifted::Store2(s) => [
            ("field plus four", Lifted::Store2(RecordStore { field: s.field + 4, ..s })),
            ("null index changed", Lifted::Store2(RecordStore { null_index: 0, ..s })),
            ("guard flipped", Lifted::Store2(RecordStore {
                guard: if s.guard == Guard::Record { Guard::Validity } else { Guard::Record },
                ..s
            })),
        ],
        Lifted::Flag(f) => [
            ("field plus four", Lifted::Flag(RecordFlag { field: f.field + 4, ..f })),
            ("null index changed", Lifted::Flag(RecordFlag { null_index: 0, ..f })),
            ("field plus one", Lifted::Flag(RecordFlag { field: f.field + 1, ..f })),
        ],
    }
}

/// The lifted form of an instance.
fn lifted_of(instance: &Instance) -> Lifted {
    match instance.routine {
        Routine::Store3(_, s) => Lifted::Store3(s),
        Routine::Store2(_, s) => Lifted::Store2(s),
        Routine::Flag(_, f) => Lifted::Flag(f),
    }
}

/// A seed per instance, from its position in the list.
fn seed_of(index: usize) -> u64 {
    0x2F0B_3A62_9D5C_1E77 ^ ((index as u64 + 1) * 0x100_0000_01B3)
}

#[test]
fn lifted_record_field_access_matches_every_verified_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut total = 0;
    for (index, instance) in INSTANCES.iter().enumerate() {
        let outcome = diff(instance, lifted_of(instance), seed_of(index));
        eprintln!(
            "{} {} cases {} mismatches {} first {:?}",
            instance.address, instance.name, outcome.cases, outcome.mismatches, outcome.first
        );
        total += outcome.mismatches;
    }
    assert_eq!(total, 0, "the lifted record field access differs from a verified rewrite");
}

#[test]
fn wrong_versions_are_caught_on_every_instance() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    for (index, instance) in INSTANCES.iter().enumerate() {
        for (label, wrong) in wrong_versions(lifted_of(instance)) {
            let outcome = diff(instance, wrong, seed_of(index));
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
