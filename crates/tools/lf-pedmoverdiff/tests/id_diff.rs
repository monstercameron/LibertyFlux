//! Differential tests: the lifted task id lookup against its verified 32-bit
//! rewrite.
//!
//! The rewrite reads the task key from its owner, asks the primary lookup
//! (a cdecl stub), asks the fallback when the primary answers zero, and
//! reads the id word of the record it ends with. The stubs answer from the
//! case's script and record their arguments; the owner image and the record
//! images must stay as they were. Both sides must agree on the calls in
//! order, on the id, and on the owner image. A wrong version runs through
//! the same harness and must be caught.
#![cfg(target_arch = "x86")]

use std::sync::{Mutex, MutexGuard};

use lf_pedmoverdiff::{rewrites, set_callee};
use lf_peds_tasks::ped_task::{DEFAULT_ID, NO_ID, TaskIdTable, TaskKey};

/// The owner's key word sits at this offset.
const KEY_OFF: usize = 0xB94;
const OWNER_LEN: usize = 0xC00;
const RECORD_LEN: usize = 8;

/// Call kinds recorded by the stubs and by the lifted side.
const PRIMARY: u8 = 1;
const FALLBACK: u8 = 2;

static TEST_LOCK: Mutex<()> = Mutex::new(());
static RECORD: Mutex<Vec<(u8, u32)>> = Mutex::new(Vec::new());
/// The answers of the case in progress: the primary's record (zero for no
/// record) and the record the fallback gives.
static ANSWERS: Mutex<(u32, u32)> = Mutex::new((0, 0));

fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn answers() -> (u32, u32) {
    *ANSWERS.lock().unwrap()
}

extern "cdecl" fn stub_primary(key: u32) -> u32 {
    RECORD.lock().unwrap().push((PRIMARY, key));
    answers().0
}

extern "cdecl" fn stub_fallback(key: u32) -> u32 {
    RECORD.lock().unwrap().push((FALLBACK, key));
    answers().1
}

/// The lifted side's stand-in table: the two lookups' answers, and the id
/// word each planted record holds (keyed by the record's address).
struct Table {
    calls: Vec<(u8, u32)>,
    answers: (u32, u32),
    record_ids: [(u32, u32); 2],
}

impl TaskIdTable for Table {
    fn primary(&mut self, key: u32) -> u32 {
        self.calls.push((PRIMARY, key));
        self.answers.0
    }

    fn fallback(&mut self, key: u32) -> u32 {
        self.calls.push((FALLBACK, key));
        self.answers.1
    }

    fn record_id(&mut self, record: u32) -> u32 {
        // Not a call: the record's id word is a plain read of the planted image.
        self.record_ids
            .iter()
            .find(|(addr, _)| *addr == record)
            .map_or(0, |(_, id)| *id)
    }
}

struct Rng(u32);

impl Rng {
    fn u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

/// One generated case.
struct Case {
    key: u32,
    primary_hit: bool,
    primary_record: usize,
    fallback_record: usize,
    record_ids: [u32; 2],
}

fn gen_case(rng: &mut Rng) -> Case {
    let ids = [
        if rng.u32() % 3 == 0 { NO_ID } else { rng.u32() },
        if rng.u32() % 3 == 0 { NO_ID } else { rng.u32() },
    ];
    Case {
        key: rng.u32(),
        primary_hit: rng.u32() % 2 == 0,
        primary_record: 0,
        fallback_record: 1,
        record_ids: ids,
    }
}

/// Runs the rewrite on fresh images for one case. Returns the id, the call
/// log and the owner image afterwards.
fn run_rewrite(
    owner: &mut [u8],
    records: &[Vec<u8>; 2],
    case: &Case,
) -> (u32, Vec<(u8, u32)>, Vec<u8>) {
    let record_addr = |i: usize| records[i].as_ptr() as usize as u32;
    let primary = if case.primary_hit {
        record_addr(case.primary_record)
    } else {
        0
    };
    *ANSWERS.lock().unwrap() = (primary, record_addr(case.fallback_record));
    RECORD.lock().unwrap().clear();
    let id = rewrites::fn_009E74A0::rw_009e74a0(owner.as_ptr() as usize as u32);
    let calls = std::mem::take(&mut *RECORD.lock().unwrap());
    (id, calls, owner.to_vec())
}

/// Runs the lifted side on the same images. `lifted` is the routine under
/// test: the real one, or a wrong version.
fn run_lifted(
    owner: &[u8],
    records: &[Vec<u8>; 2],
    case: &Case,
    lifted: fn(&TaskKey, &mut Table) -> u32,
) -> (u32, Vec<(u8, u32)>, Vec<u8>) {
    let record_addr = |i: usize| records[i].as_ptr() as usize as u32;
    let primary = if case.primary_hit {
        record_addr(case.primary_record)
    } else {
        0
    };
    let key = u32::from_le_bytes([
        owner[KEY_OFF],
        owner[KEY_OFF + 1],
        owner[KEY_OFF + 2],
        owner[KEY_OFF + 3],
    ]);
    let mut table = Table {
        calls: Vec::new(),
        answers: (primary, record_addr(case.fallback_record)),
        record_ids: [
            (record_addr(0), case.record_ids[0]),
            (record_addr(1), case.record_ids[1]),
        ],
    };
    let id = lifted(&TaskKey { key }, &mut table);
    (id, table.calls, owner.to_vec())
}

/// Runs both sides and counts the disagreements.
fn diff_run(
    seed: u32,
    trials: u32,
    lifted: fn(&TaskKey, &mut Table) -> u32,
) -> (u32, Option<String>) {
    let mut rng = Rng(seed);
    let mut owner = vec![0u8; OWNER_LEN];
    let mut records = [vec![0u8; RECORD_LEN], vec![0u8; RECORD_LEN]];
    let mut mismatches = 0;
    let mut first = None;
    for trial in 0..trials {
        let case = gen_case(&mut rng);
        for byte in owner.iter_mut() {
            *byte = rng.u32() as u8;
        }
        owner[KEY_OFF..KEY_OFF + 4].copy_from_slice(&case.key.to_le_bytes());
        for (i, record) in records.iter_mut().enumerate() {
            record[4..8].copy_from_slice(&case.record_ids[i].to_le_bytes());
        }

        let (rw_id, rw_calls, rw_owner) = run_rewrite(&mut owner, &records, &case);
        let (lf_id, lf_calls, lf_owner) = run_lifted(&owner, &records, &case, lifted);

        if rw_id != lf_id || rw_calls != lf_calls || rw_owner != lf_owner {
            mismatches += 1;
            if first.is_none() {
                first = Some(format!(
                    "trial {trial}: rewrite id {rw_id:#x} calls {rw_calls:?}; lifted id {lf_id:#x} calls {lf_calls:?}"
                ));
            }
        }
    }
    (mismatches, first)
}

/// The routine under test, adapted to the table.
fn lifted_real(key: &TaskKey, table: &mut Table) -> u32 {
    key.id_or_default(table)
}

/// Wrong: the fallback is asked even when the primary has a record.
fn lifted_wrong_fallback(key: &TaskKey, table: &mut Table) -> u32 {
    let primary = table.primary(key.key);
    let fallback = table.fallback(key.key);
    let record = if primary == 0 { fallback } else { primary };
    let id = table.record_id(record);
    if id == NO_ID { DEFAULT_ID } else { id }
}

/// Wrong: the default id is a different value.
fn lifted_wrong_default(key: &TaskKey, table: &mut Table) -> u32 {
    let mut record = table.primary(key.key);
    if record == 0 {
        record = table.fallback(key.key);
    }
    let id = table.record_id(record);
    if id == NO_ID { DEFAULT_ID + 1 } else { id }
}

const TRIALS: u32 = 4000;

#[test]
fn id_or_default_matches_rewrite() {
    let _guard = lock();
    set_callee(1, stub_primary as *const () as usize as u32);
    set_callee(2, stub_fallback as *const () as usize as u32);
    let (mismatches, first) = diff_run(0x1D00_0001, TRIALS, lifted_real);
    assert_eq!(mismatches, 0, "{first:?}");
}

#[test]
fn id_or_default_wrong_fallback_is_caught() {
    let _guard = lock();
    set_callee(1, stub_primary as *const () as usize as u32);
    set_callee(2, stub_fallback as *const () as usize as u32);
    let (mismatches, _) = diff_run(0x1D00_0001, TRIALS, lifted_wrong_fallback);
    assert!(mismatches > 0, "the wrong version passed every case");
}

#[test]
fn id_or_default_wrong_default_is_caught() {
    let _guard = lock();
    set_callee(1, stub_primary as *const () as usize as u32);
    set_callee(2, stub_fallback as *const () as usize as u32);
    let (mismatches, _) = diff_run(0x1D00_0001, TRIALS, lifted_wrong_default);
    assert!(mismatches > 0, "the wrong version passed every case");
}
