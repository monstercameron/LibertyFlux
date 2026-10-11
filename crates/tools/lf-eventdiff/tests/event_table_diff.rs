//! Differential cases of the lifted task event table against its verified
//! 32-bit rewrites.
//!
//! Each lifted routine runs on the same generated inputs as its rewrite.
//! The effects compared are every byte of the planted image window (the
//! table and the words around it), the helper calls in order with their
//! arguments, and the return value where the routine has a meaningful one.
//! Record arguments are indices on the lifted side; the helper calls
//! compare the address the rewrite passes, rebuilt from the index.
//!
//! Each routine also has deliberately wrong versions. Each must differ
//! from its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only. The test program holds one lock, so its
//! cases never interleave: the planted windows and callee stubs are shared.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_eventdiff::rewrites::{
    fn_009bf5b0, fn_009bf7f0, fn_009bf550, fn_009c0d80, fn_009c0e40, fn_009c0e70, fn_009c07f0,
    fn_009c0990, fn_009c1270,
};
use lf_eventdiff::rt;
use lf_peds_tasks::ped_task::{
    DELAY_GAP, EventCallees, EventObjects, EventRecord, EventTable, NEARBY_DISTANCE_SQUARED,
    QUEUED_KIND, RECORD_COUNT, REDUCED_THRESHOLD, STANDARD_THRESHOLD,
};

/// File address of record 0 of the table.
const TABLE_VA: u32 = 0x0129_2450;
/// Bytes from one record to the next.
const STRIDE: u32 = 0xD0;
/// File address of the game clock word the refresh routine reads.
const CURRENT_TIME_VA: u32 = 0x0117_35B4;
/// Generated cases per routine.
const CASES: u32 = 3000;

/// Helper-call tags in the call log. Both sides append the same tags.
const TAG_DELAY: u32 = 1;
const TAG_MODE: u32 = 2;
const TAG_RELEASE: u32 = 3;
const TAG_OBJECT_RELEASE: u32 = 4;
const TAG_OBJECT_RETAIN: u32 = 5;
const TAG_PROXIMITY: u32 = 6;
const TAG_PROXIMITY_ARGS: u32 = 7;

/// File address of the proximity helper's descriptor (the rewrite passes it).
const PROXIMITY_DESCRIPTOR_VA: u32 = 0x0110_DB70;
/// File address of the three words of the target vector.
const TARGET_VA: u32 = 0x0129_3E80;
/// File address of the first event object, the stride between objects, and
/// how many objects the generated cases use.
const EVENT_BASE_VA: u32 = 0x0129_3F00;
const EVENT_STRIDE: u32 = 0x200;
const EVENT_COUNT: u32 = 5;

/// One helper call: tag, first argument, second argument (0 when absent).
type Call = (u32, u32, u32);

/// The values the planted helpers return.
#[derive(Clone, Copy)]
struct Plant {
    /// Answer of the delay helper.
    delay: u32,
    /// Answer of the selection-mode helper.
    mode: u32,
}

/// Serialises the test cases: the windows and stubs are process-wide.
static LOCK: Mutex<()> = Mutex::new(());
/// Calls the planted stubs have received, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// The answers the planted stubs return.
static PLANT: Mutex<Plant> = Mutex::new(Plant { delay: 0, mode: 0 });

fn planted() -> Plant {
    *PLANT.lock().unwrap()
}

fn log_call(call: Call) {
    LOG.lock().unwrap().push(call);
}

// Stubs the rewrites call through their numbered callee slots. Each has the
// convention and argument shape of the slot it is planted in.

extern "cdecl" fn stub_delay(task_id: u32) -> u32 {
    log_call((TAG_DELAY, task_id, 0));
    planted().delay
}

extern "cdecl" fn stub_mode() -> u32 {
    log_call((TAG_MODE, 0, 0));
    planted().mode
}

extern "cdecl" fn stub_release(record: u32) -> u32 {
    log_call((TAG_RELEASE, record, 0));
    0
}

extern "cdecl" fn stub_proximity(record: u32, option: u32, descriptor: u32, zero: u32) -> u32 {
    log_call((TAG_PROXIMITY, record, descriptor));
    log_call((TAG_PROXIMITY_ARGS, option, zero));
    0
}

extern "thiscall" fn stub_object_release(object: u32, slot: u32) -> u32 {
    log_call((TAG_OBJECT_RELEASE, object, slot));
    0
}

extern "thiscall" fn stub_object_retain(object: u32, slot: u32) -> u32 {
    log_call((TAG_OBJECT_RETAIN, object, slot));
    0
}

/// Plants the stub addresses in the callee slots of one rewrite.
fn plant_stubs(slots: &[(u32, u32)]) {
    for &(id, addr) in slots {
        rt::set_callee(id, addr);
    }
}

/// File address of record `index`.
fn record_va(index: usize) -> u32 {
    TABLE_VA + index as u32 * STRIDE
}

/// File address of record `index`'s object word (offset 0x1c).
fn slot_va(index: usize) -> u32 {
    record_va(index) + 0x1C
}

/// Byte offset of a file address inside the planted window.
fn at(va: u32) -> usize {
    (va - rt::WINDOW_VA) as usize
}

fn word(bytes: &[u8], va: u32) -> u32 {
    let o = at(va);
    u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
}

fn put_word(bytes: &mut [u8], va: u32, value: u32) {
    let o = at(va);
    bytes[o..o + 4].copy_from_slice(&value.to_le_bytes());
}

/// Reads the modelled fields of every record out of the window bytes.
fn decode(bytes: &[u8]) -> EventTable {
    let mut table = EventTable::default();
    for (index, record) in table.records.iter_mut().enumerate() {
        let va = record_va(index);
        *record = EventRecord {
            event: word(bytes, va + 0x10),
            kind: word(bytes, va + 0x18),
            object: word(bytes, va + 0x1C),
            deadline: word(bytes, va + 0x24),
            flag: bytes[at(va + 0xCB)],
        };
    }
    table
}

/// Writes the modelled fields of every record into the window bytes. Bytes
/// the model does not carry are left as they were.
fn encode(bytes: &mut [u8], table: &EventTable) {
    for (index, record) in table.records.iter().enumerate() {
        let va = record_va(index);
        put_word(bytes, va + 0x10, record.event);
        put_word(bytes, va + 0x18, record.kind);
        put_word(bytes, va + 0x1C, record.object);
        put_word(bytes, va + 0x24, record.deadline);
        bytes[at(va + 0xCB)] = record.flag;
    }
}

/// A small deterministic generator (xorshift32), so every run sees the
/// same cases.
struct Rng(u32);

impl Rng {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

/// Generates one record. The pools are small on purpose, so keys, ids and
/// kinds collide often, and the deadline pool includes the exact edges of
/// the delay gap, the unsigned order and the wrap-around.
fn gen_record(rng: &mut Rng, now: u32) -> EventRecord {
    let event = match rng.below(8) {
        0 | 1 => 0,
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 0x1234_5678,
        _ => rng.next(),
    };
    let kind = match rng.below(8) {
        0..=2 => QUEUED_KIND,
        3 => 0,
        4 => 1,
        5 => 5,
        6 => u32::MAX,
        _ => rng.next(),
    };
    let object = match rng.below(8) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 0xABCD_0001,
        _ => rng.next(),
    };
    let deadline = match rng.below(12) {
        0 => now,
        1 => now.wrapping_add(DELAY_GAP),
        2 => now.wrapping_add(DELAY_GAP + 1),
        3 => now.wrapping_add(DELAY_GAP - 1),
        4 => now.wrapping_sub(1),
        5 => 0,
        6 => u32::MAX,
        7 => 0x8000_0000,
        8 => now.wrapping_add(rng.below(5000)),
        _ => rng.next(),
    };
    EventRecord {
        event,
        kind,
        object,
        deadline,
        flag: rng.next() as u8,
    }
}

/// The window bytes of one case: random bytes everywhere, then a generated
/// table written over them, so every byte the model does not carry is
/// random too.
fn gen_window(rng: &mut Rng, now: u32) -> Vec<u8> {
    let mut bytes: Vec<u8> = (0..rt::WINDOW_LEN).map(|_| rng.next() as u8).collect();
    let mut table = EventTable::default();
    for record in &mut table.records {
        *record = gen_record(rng, now);
    }
    encode(&mut bytes, &table);
    bytes
}

fn gen_plant(rng: &mut Rng) -> Plant {
    let delay = match rng.below(6) {
        0 => 0,
        1 => 1,
        2 => DELAY_GAP,
        3 => u32::MAX,
        4 => 999,
        _ => rng.next(),
    };
    let mode = match rng.below(6) {
        0 => 0,
        1 => 1,
        2 => 0x100,
        3 => 0xFF,
        4 => 2,
        _ => rng.next(),
    };
    Plant { delay, mode }
}

/// The observable effects of one run.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    /// The window after the run.
    bytes: Vec<u8>,
    /// Helper calls in order.
    log: Vec<Call>,
    /// The return value, when the routine has one.
    ret: u32,
}

/// Mismatch counts over one run of generated cases.
#[derive(Default)]
struct Tally {
    cases: u32,
    /// Cases in which the rewrite made at least one helper call.
    with_calls: u32,
    mismatches: u32,
    first: Option<String>,
}

impl Tally {
    fn judge(&mut self, case: u32, lift: &Outcome, rewrite: &Outcome, ret_checked: bool) {
        self.cases += 1;
        if !rewrite.log.is_empty() {
            self.with_calls += 1;
        }
        let same_bytes = lift.bytes == rewrite.bytes;
        let same_log = lift.log == rewrite.log;
        let same_ret = !ret_checked || lift.ret == rewrite.ret;
        if same_bytes && same_log && same_ret {
            return;
        }
        self.mismatches += 1;
        if self.first.is_none() {
            let differing = lift
                .bytes
                .iter()
                .zip(&rewrite.bytes)
                .position(|(a, b)| a != b);
            self.first = Some(format!(
                "case {case}: first differing window byte {differing:?}; \
                 calls lift {:?} rewrite {:?}; ret lift {:#x} rewrite {:#x}",
                lift.log, rewrite.log, lift.ret, rewrite.ret
            ));
        }
    }
}

/// Runs one rewrite on the planted window and returns what it did. The
/// caller plants the stubs first.
fn run_rewrite(init: &[u8], plant: Plant, call: impl FnOnce() -> u32) -> Outcome {
    rt::write_window(init);
    *PLANT.lock().unwrap() = plant;
    LOG.lock().unwrap().clear();
    let ret = call();
    let log = std::mem::take(&mut *LOG.lock().unwrap());
    Outcome {
        bytes: rt::read_window(),
        log,
        ret,
    }
}

/// Plants a fresh window and auxiliary window for one test.
fn fresh_windows() {
    let window: &'static mut [u32] =
        Box::leak(vec![0_u32; rt::WINDOW_LEN as usize / 4].into_boxed_slice());
    rt::set_window(window.as_ptr() as u32);
    let aux: &'static mut [u32] =
        Box::leak(vec![0_u32; rt::AUX_LEN as usize / 4].into_boxed_slice());
    rt::set_aux(aux.as_ptr() as u32);
}

/// The lifted side's recorder: it answers from the plant and logs every
/// helper call with the address the rewrite would pass.
struct Recorder {
    plant: Plant,
    log: Vec<Call>,
}

impl EventCallees for Recorder {
    fn proximity_notify(&mut self, record: usize) {
        self.log.push((
            TAG_PROXIMITY,
            rt::relocated(record_va(record)),
            rt::relocated(PROXIMITY_DESCRIPTOR_VA),
        ));
        self.log.push((TAG_PROXIMITY_ARGS, 1, 0));
    }

    fn delay_ticks(&mut self, task_id: u32) -> u32 {
        self.log.push((TAG_DELAY, task_id, 0));
        self.plant.delay
    }

    fn selection_mode(&mut self) -> u32 {
        self.log.push((TAG_MODE, 0, 0));
        self.plant.mode
    }

    fn release_record(&mut self, record: usize) {
        self.log
            .push((TAG_RELEASE, rt::relocated(record_va(record)), 0));
    }

    fn release_object(&mut self, object: u32, record: usize) {
        self.log
            .push((TAG_OBJECT_RELEASE, object, rt::relocated(slot_va(record))));
    }

    fn retain_object(&mut self, object: u32, record: usize) {
        self.log
            .push((TAG_OBJECT_RETAIN, object, rt::relocated(slot_va(record))));
    }
}

// ---------------------------------------------------------------------------
// ped_task_refresh_matching_deadlines
// ---------------------------------------------------------------------------

type RefreshLift = fn(&mut EventTable, u32, u32, &mut Recorder);

fn refresh_tally(lift: RefreshLift) -> Tally {
    let mut rng = Rng(0x1357_9BDF);
    let mut tally = Tally::default();
    plant_stubs(&[(1, stub_delay as *const () as usize as u32)]);
    for case in 0..CASES {
        let task = rng.pick_task();
        let now = match rng.below(5) {
            0 => 0,
            1 => u32::MAX,
            2 => 0x8000_0000,
            _ => rng.next(),
        };
        let init = gen_window(&mut rng, now);
        let plant = gen_plant(&mut rng);

        let mut table = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&mut table, task, now, &mut recorder);
        let mut expected = init.clone();
        encode(&mut expected, &table);
        let lifted = Outcome {
            bytes: expected,
            log: recorder.log,
            ret: 0,
        };

        rt::set_aux_word(CURRENT_TIME_VA, now);
        let rewrite = run_rewrite(&init, plant, || {
            fn_009bf550::rw_009bf550(task);
            0
        });
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

trait PickTask {
    fn pick_task(&mut self) -> u32;
}

impl PickTask for Rng {
    fn pick_task(&mut self) -> u32 {
        match self.below(6) {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            _ => 0xABCD_0001,
        }
    }
}

// ---------------------------------------------------------------------------
// ped_task_select_lowest_deadline_record
// ---------------------------------------------------------------------------

type SelectLift = fn(&mut EventTable, u32, &mut Recorder);

fn select_tally(lift: SelectLift) -> Tally {
    let mut rng = Rng(0x2468_ACE1);
    let mut tally = Tally::default();
    plant_stubs(&[
        (1, stub_mode as *const () as usize as u32),
        (2, stub_release as *const () as usize as u32),
    ]);
    for case in 0..CASES {
        let task = rng.pick_task();
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);

        let mut table = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&mut table, task, &mut recorder);
        let mut expected = init.clone();
        encode(&mut expected, &table);
        let lifted = Outcome {
            bytes: expected,
            log: recorder.log,
            ret: 0,
        };

        let rewrite = run_rewrite(&init, plant, || {
            fn_009bf5b0::rw_009bf5b0(task);
            0
        });
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

// ---------------------------------------------------------------------------
// ped_task_record_lookup
// ---------------------------------------------------------------------------

type LookupLift = fn(&EventTable, u32) -> Option<usize>;

/// Lookup keys: the event pool, zero, and values no record holds.
fn gen_key(rng: &mut Rng) -> u32 {
    match rng.below(7) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 0x1234_5678,
        _ => rng.next(),
    }
}

fn lookup_tally(lift: LookupLift) -> Tally {
    let mut rng = Rng(0x0F0F_1234);
    let mut tally = Tally::default();
    for case in 0..CASES {
        let key = gen_key(&mut rng);
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);

        let table = decode(&init);
        let found = lift(&table, key);
        let lifted = Outcome {
            bytes: init.clone(),
            log: Vec::new(),
            ret: found.map_or(0, |index| rt::relocated(record_va(index))),
        };

        let rewrite = run_rewrite(&init, plant, || fn_009bf7f0::rw_009bf7f0(key));
        tally.judge(case, &lifted, &rewrite, true);
    }
    tally
}

// ---------------------------------------------------------------------------
// ped_task_record_matches
// ---------------------------------------------------------------------------

type MatchesLift = fn(&EventTable, u32, u32) -> bool;

fn matches_tally(lift: MatchesLift) -> Tally {
    let mut rng = Rng(0x7777_1111);
    let mut tally = Tally::default();
    for case in 0..CASES {
        let index = match rng.below(8) {
            0 | 1 | 2 | 3 => rng.below(RECORD_COUNT as u32),
            4 => RECORD_COUNT as u32,
            5 => RECORD_COUNT as u32 + 1,
            6 => 0xFFFF_FFFF,
            _ => rng.next(),
        };
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);
        // The pointer compared with the object word: a stored object, zero,
        // a random value, or the word just past the table.
        let stored = |i: usize| word(&init, slot_va(i));
        let pointer = match rng.below(6) {
            0 => 0,
            1 | 2 => stored(rng.below(RECORD_COUNT as u32) as usize),
            3 => word(&init, slot_va(RECORD_COUNT)),
            _ => rng.next(),
        };

        let table = decode(&init);
        let lifted = Outcome {
            bytes: init.clone(),
            log: Vec::new(),
            ret: u32::from(lift(&table, index, pointer)),
        };

        let rewrite = run_rewrite(&init, plant, || {
            u32::from(fn_009c07f0::rw_009c07f0(index, pointer))
        });
        tally.judge(case, &lifted, &rewrite, true);
    }
    tally
}

// ---------------------------------------------------------------------------
// ped_task_event_flag_update
// ---------------------------------------------------------------------------

type FlagLift = fn(&mut EventTable, u32, u32);

fn flag_tally(lift: FlagLift) -> Tally {
    let mut rng = Rng(0x5A5A_0033);
    let mut tally = Tally::default();
    for case in 0..CASES {
        let event_key = match rng.below(5) {
            0 | 1 => QUEUED_KIND,
            2 => 0,
            3 => 1,
            _ => rng.next(),
        };
        let flag = match rng.below(6) {
            0 => 0,
            1 => 1,
            2 => 0x100,
            3 => 0x1FF,
            _ => rng.next(),
        };
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);

        let mut table = decode(&init);
        lift(&mut table, event_key, flag);
        let mut expected = init.clone();
        encode(&mut expected, &table);
        let lifted = Outcome {
            bytes: expected,
            log: Vec::new(),
            ret: 0,
        };

        let rewrite = run_rewrite(&init, plant, || {
            fn_009c0990::rw_009c0990(event_key, flag);
            0
        });
        // The rewrite returns the address one past its scanned table; the
        // lift does not return it, and no caller of the routine reads it.
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

// ---------------------------------------------------------------------------
// ped_task_release_active_records and ped_task_release_nonqueued_records
// ---------------------------------------------------------------------------

type ReleaseLift = fn(&EventTable, &mut Recorder);

fn release_tally(lift: ReleaseLift, rewrite_call: fn()) -> Tally {
    let mut rng = Rng(0x9999_0A0A);
    let mut tally = Tally::default();
    plant_stubs(&[(1, stub_release as *const () as usize as u32)]);
    for case in 0..CASES {
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);

        let table = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&table, &mut recorder);
        let lifted = Outcome {
            bytes: init.clone(),
            log: recorder.log,
            ret: 0,
        };

        let rewrite = run_rewrite(&init, plant, || {
            rewrite_call();
            0
        });
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

// ---------------------------------------------------------------------------
// ped_task_record_replace_object
// ---------------------------------------------------------------------------

type ReplaceLift = fn(&mut EventTable, u32, u32, &mut Recorder);

fn replace_tally(lift: ReplaceLift) -> Tally {
    let mut rng = Rng(0x3C3C_F00D);
    let mut tally = Tally::default();
    plant_stubs(&[
        (1, stub_object_release as *const () as usize as u32),
        (2, stub_object_retain as *const () as usize as u32),
    ]);
    for case in 0..CASES {
        let index = match rng.below(8) {
            0..=5 => rng.below(RECORD_COUNT as u32),
            6 => RECORD_COUNT as u32,
            _ => 0xFFFF_FFFF,
        };
        let init = gen_window(&mut rng, 0);
        let plant = gen_plant(&mut rng);
        // The replacement: the current object (no change), zero, a stored
        // object of another record, or a random value.
        let current = if (index as usize) < RECORD_COUNT {
            word(&init, slot_va(index as usize))
        } else {
            0
        };
        let replacement = match rng.below(6) {
            0 => current,
            1 => 0,
            2 => word(&init, slot_va(rng.below(RECORD_COUNT as u32) as usize)),
            3 => 0xABCD_0001,
            _ => rng.next(),
        };

        let mut table = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&mut table, index, replacement, &mut recorder);
        let mut expected = init.clone();
        encode(&mut expected, &table);
        let lifted = Outcome {
            bytes: expected,
            log: recorder.log,
            ret: 0,
        };

        let rewrite = run_rewrite(&init, plant, || {
            fn_009c1270::rw_009c1270(index, replacement);
            0
        });
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

// ---------------------------------------------------------------------------
// The proofs: each lifted routine matches its rewrite, and each wrong version
// is caught.
// ---------------------------------------------------------------------------

/// Holds the lock and plants fresh windows for one test.
fn begin() -> std::sync::MutexGuard<'static, ()> {
    let guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    fresh_windows();
    guard
}

fn assert_clean(name: &str, tally: &Tally) {
    eprintln!(
        "{name}: {} cases, {} with helper calls, {} mismatches",
        tally.cases, tally.with_calls, tally.mismatches
    );
    assert_eq!(tally.cases, CASES, "{name}: case count");
    assert_eq!(
        tally.mismatches,
        0,
        "{name}: {}",
        tally.first.as_deref().unwrap_or("")
    );
}

fn assert_caught(name: &str, tally: &Tally) {
    assert!(
        tally.mismatches > 0,
        "{name}: the wrong version was not caught in {} cases",
        tally.cases
    );
}

#[test]
fn refresh_matches_rewrite() {
    let _guard = begin();
    let tally =
        refresh_tally(|table, task, now, calls| table.refresh_matching_deadlines(task, now, calls));
    assert_clean("refresh_matching_deadlines", &tally);
}

#[test]
fn refresh_wrong_versions_are_caught() {
    let _guard = begin();
    // The gap test is inclusive: a deadline exactly DELAY_GAP beyond now is
    // refreshed by the wrong version and must not be by the rewrite.
    assert_caught(
        "refresh, inclusive gap",
        &refresh_tally(|table, task, now, calls| {
            for record in &mut table.records {
                if !record.is_active() || record.kind != QUEUED_KIND || record.object != task {
                    continue;
                }
                if record.deadline.wrapping_sub(now) >= DELAY_GAP {
                    let delay = calls.delay_ticks(task);
                    record.deadline = now.wrapping_add(delay);
                }
            }
        }),
    );
    // No kind check: records of other kinds are refreshed too.
    assert_caught(
        "refresh, any kind",
        &refresh_tally(|table, task, now, calls| {
            for record in &mut table.records {
                if !record.is_active() || record.object != task {
                    continue;
                }
                if record.deadline.wrapping_sub(now) > DELAY_GAP {
                    let delay = calls.delay_ticks(task);
                    record.deadline = now.wrapping_add(delay);
                }
            }
        }),
    );
}

#[test]
fn select_matches_rewrite() {
    let _guard = begin();
    let tally =
        select_tally(|table, task, calls| table.release_lowest_deadline_record(task, calls));
    assert_clean("release_lowest_deadline_record", &tally);
}

#[test]
fn select_wrong_versions_are_caught() {
    let _guard = begin();
    // Signed order on the deadline: a deadline with the high bit set is the
    // lowest under the wrong order.
    assert_caught(
        "select, signed deadline order",
        &select_tally(|table, task, calls| {
            let mut matches = 0_u32;
            let mut lowest: Option<(usize, u32)> = None;
            for (index, record) in table.records.iter().enumerate() {
                if !record.is_active() || record.kind != QUEUED_KIND || record.object != task {
                    continue;
                }
                matches += 1;
                if lowest.is_none_or(|(_, best)| (record.deadline as i32) < (best as i32)) {
                    lowest = Some((index, record.deadline));
                }
            }
            let mode = calls.selection_mode();
            let threshold = if (mode & 0xFF) != 0 {
                REDUCED_THRESHOLD
            } else {
                STANDARD_THRESHOLD
            };
            if let Some((index, _)) = lowest {
                if matches >= threshold {
                    calls.release_record(index);
                }
            }
        }),
    );
    // Ties go to the last record.
    assert_caught(
        "select, last record on a tie",
        &select_tally(|table, task, calls| {
            let mut matches = 0_u32;
            let mut lowest: Option<(usize, u32)> = None;
            for (index, record) in table.records.iter().enumerate() {
                if !record.is_active() || record.kind != QUEUED_KIND || record.object != task {
                    continue;
                }
                matches += 1;
                if lowest.is_none_or(|(_, best)| record.deadline <= best) {
                    lowest = Some((index, record.deadline));
                }
            }
            let mode = calls.selection_mode();
            let threshold = if (mode & 0xFF) != 0 {
                REDUCED_THRESHOLD
            } else {
                STANDARD_THRESHOLD
            };
            if let Some((index, _)) = lowest {
                if matches >= threshold {
                    calls.release_record(index);
                }
            }
        }),
    );
    // The threshold choice inverted.
    assert_caught(
        "select, inverted threshold",
        &select_tally(|table, task, calls| {
            let mut matches = 0_u32;
            let mut lowest: Option<(usize, u32)> = None;
            for (index, record) in table.records.iter().enumerate() {
                if !record.is_active() || record.kind != QUEUED_KIND || record.object != task {
                    continue;
                }
                matches += 1;
                if lowest.is_none_or(|(_, best)| record.deadline < best) {
                    lowest = Some((index, record.deadline));
                }
            }
            let mode = calls.selection_mode();
            let threshold = if (mode & 0xFF) == 0 {
                REDUCED_THRESHOLD
            } else {
                STANDARD_THRESHOLD
            };
            if let Some((index, _)) = lowest {
                if matches >= threshold {
                    calls.release_record(index);
                }
            }
        }),
    );
}

#[test]
fn lookup_matches_rewrite() {
    let _guard = begin();
    let tally = lookup_tally(|table, key| table.find_by_event(key));
    assert_clean("find_by_event", &tally);
}

#[test]
fn lookup_wrong_versions_are_caught() {
    let _guard = begin();
    // The last match instead of the first.
    assert_caught(
        "lookup, last match",
        &lookup_tally(|table, key| {
            if key == 0 {
                return None;
            }
            table.records.iter().rposition(|record| record.event == key)
        }),
    );
    // No zero-key check: an inactive record (event zero) matches key zero.
    assert_caught(
        "lookup, zero key scans",
        &lookup_tally(|table, key| table.records.iter().position(|record| record.event == key)),
    );
}

#[test]
fn matches_matches_rewrite() {
    let _guard = begin();
    let tally = matches_tally(|table, index, pointer| table.holds_object(index, pointer));
    assert_clean("holds_object", &tally);
}

#[test]
fn matches_wrong_version_is_caught() {
    let _guard = begin();
    // Index 32 is treated as a record whose object word is zero, so a zero
    // pointer matches it; the rewrite finds the word beyond the table.
    assert_caught(
        "matches, phantom record 32",
        &matches_tally(|table, index, pointer| {
            if index == RECORD_COUNT as u32 {
                return pointer == 0;
            }
            usize::try_from(index)
                .ok()
                .and_then(|i| table.records.get(i))
                .is_some_and(|record| record.object == pointer)
        }),
    );
}

#[test]
fn flag_matches_rewrite() {
    let _guard = begin();
    let tally = flag_tally(|table, event_key, flag| table.set_event_flag(event_key, flag));
    assert_clean("set_event_flag", &tally);
}

#[test]
fn flag_wrong_versions_are_caught() {
    let _guard = begin();
    // Inactive records are flagged too.
    assert_caught(
        "flag, inactive records",
        &flag_tally(|table, event_key, flag| {
            for record in &mut table.records {
                if record.kind == event_key {
                    record.flag = (flag & 0xFF) as u8;
                }
            }
        }),
    );
    // A boolean is stored instead of the low byte.
    assert_caught(
        "flag, boolean store",
        &flag_tally(|table, event_key, flag| {
            for record in &mut table.records {
                if record.is_active() && record.kind == event_key {
                    record.flag = u8::from(flag != 0);
                }
            }
        }),
    );
}

#[test]
fn release_active_matches_rewrite() {
    let _guard = begin();
    let tally = release_tally(
        |table, calls| table.release_active(calls),
        || fn_009c0e40::rw_009c0e40(),
    );
    assert_clean("release_active", &tally);
}

#[test]
fn release_active_wrong_version_is_caught() {
    let _guard = begin();
    assert_caught(
        "release_active, inactive records",
        &release_tally(
            |table, calls| {
                for (index, record) in table.records.iter().enumerate() {
                    if !record.is_active() {
                        calls.release_record(index);
                    }
                }
            },
            || fn_009c0e40::rw_009c0e40(),
        ),
    );
}

#[test]
fn release_nonqueued_matches_rewrite() {
    let _guard = begin();
    let tally = release_tally(
        |table, calls| table.release_nonqueued(calls),
        || fn_009c0e70::rw_009c0e70(),
    );
    assert_clean("release_nonqueued", &tally);
}

#[test]
fn release_nonqueued_wrong_version_is_caught() {
    let _guard = begin();
    assert_caught(
        "release_nonqueued, queued records",
        &release_tally(
            |table, calls| {
                for (index, record) in table.records.iter().enumerate() {
                    if record.is_active() && record.kind == QUEUED_KIND {
                        calls.release_record(index);
                    }
                }
            },
            || fn_009c0e70::rw_009c0e70(),
        ),
    );
}

#[test]
fn replace_matches_rewrite() {
    let _guard = begin();
    let tally = replace_tally(|table, index, replacement, calls| {
        table.replace_object(index, replacement, calls)
    });
    assert_clean("replace_object", &tally);
}

#[test]
fn replace_wrong_versions_are_caught() {
    let _guard = begin();
    // The release of the old object is skipped.
    assert_caught(
        "replace, no release",
        &replace_tally(|table, index, replacement, calls| {
            let Some(index) = usize::try_from(index).ok().filter(|&i| i < RECORD_COUNT) else {
                return;
            };
            let current = table.records[index].object;
            if current == replacement {
                return;
            }
            table.records[index].object = replacement;
            if replacement != 0 {
                calls.retain_object(replacement, index);
            }
        }),
    );
    // The retain is called for a null replacement too.
    assert_caught(
        "replace, retain null",
        &replace_tally(|table, index, replacement, calls| {
            let Some(index) = usize::try_from(index).ok().filter(|&i| i < RECORD_COUNT) else {
                return;
            };
            let current = table.records[index].object;
            if current == replacement {
                return;
            }
            if current != 0 {
                calls.release_object(current, index);
            }
            table.records[index].object = replacement;
            calls.retain_object(replacement, index);
        }),
    );
}

// ---------------------------------------------------------------------------
// ped_task_check_nearby_event
// ---------------------------------------------------------------------------

/// Reads an event object straight out of the planted window. Handles are
/// process addresses inside the window, as the table holds them.
struct WindowObjects<'a> {
    base: u32,
    bytes: &'a [u8],
}

fn word_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

impl EventObjects for WindowObjects<'_> {
    fn type_byte(&self, event: u32) -> u8 {
        self.bytes[(event - self.base) as usize + 0x1E2]
    }

    fn vector_id(&self, event: u32) -> u32 {
        word_at(self.bytes, (event - self.base) as usize + 0x1BC)
    }

    fn position(&self, event: u32) -> [f32; 3] {
        let object = (event - self.base) as usize;
        let pointer = word_at(self.bytes, object + 0x20);
        let start = if pointer == 0 {
            object + 0x10
        } else {
            (pointer - self.base) as usize + 0x30
        };
        [
            f32::from_bits(word_at(self.bytes, start)),
            f32::from_bits(word_at(self.bytes, start + 4)),
            f32::from_bits(word_at(self.bytes, start + 8)),
        ]
    }
}

/// Single-precision values the generated positions and targets draw from:
/// exact halves and quarters (so squared distances land on the threshold),
/// a value that is not exact, and a quiet NaN.
const POSITION_POOL: [f32; 9] = [0.0, 0.5, -0.5, 0.25, 0.1, 1.0, -0.25, 0.3, f32::NAN];

fn gen_position(rng: &mut Rng) -> f32 {
    if rng.below(10) == 0 {
        f32::from_bits(rng.next())
    } else {
        POSITION_POOL[rng.below(POSITION_POOL.len() as u32) as usize]
    }
}

/// The window bytes of one nearby case: random bytes, event objects with
/// generated types, vector ids, pointers and positions, a target vector, and
/// a table whose object words are zero or event objects.
fn gen_nearby_window(rng: &mut Rng) -> (Vec<u8>, u32) {
    let mut bytes: Vec<u8> = (0..rt::WINDOW_LEN).map(|_| rng.next() as u8).collect();
    let target_id = rt::relocated(TARGET_VA);
    for k in 0..EVENT_COUNT {
        let object = EVENT_BASE_VA + k * EVENT_STRIDE;
        bytes[at(object + 0x1E2)] = match rng.below(6) {
            0 => 0x00,
            1 => 0x01,
            2 => 0x02,
            3 => 0x12,
            4 => 0x0F,
            _ => rng.next() as u8,
        };
        let vector = match rng.below(4) {
            0 | 1 => target_id,
            2 => 0,
            _ => rng.next(),
        };
        put_word(&mut bytes, object + 0x1BC, vector);
        let pointer = if rng.below(2) == 0 {
            0
        } else {
            rt::relocated(EVENT_BASE_VA + rng.below(EVENT_COUNT) * EVENT_STRIDE)
        };
        put_word(&mut bytes, object + 0x20, pointer);
        for axis in 0..3 {
            let inline = gen_position(rng).to_bits();
            put_word(&mut bytes, object + 0x10 + 4 * axis, inline);
            let external = gen_position(rng).to_bits();
            put_word(&mut bytes, object + 0x30 + 4 * axis, external);
        }
    }
    for axis in 0..3 {
        let value = gen_position(rng).to_bits();
        put_word(&mut bytes, TARGET_VA + 4 * axis, value);
    }
    let mut table = EventTable::default();
    for record in &mut table.records {
        *record = gen_record(rng, 0);
        record.event = match rng.below(3) {
            0 => 0,
            _ => rt::relocated(EVENT_BASE_VA + rng.below(EVENT_COUNT) * EVENT_STRIDE),
        };
    }
    encode(&mut bytes, &table);
    (bytes, target_id)
}

type NearbyLift = fn(&EventTable, u32, [f32; 3], &WindowObjects<'_>, &mut Recorder);

fn nearby_tally(lift: NearbyLift) -> Tally {
    let mut rng = Rng(0x4455_6677);
    let mut tally = Tally::default();
    plant_stubs(&[(1, stub_proximity as *const () as usize as u32)]);
    // The descriptor is an address the rewrite passes and never reads.
    let descriptor: &'static mut [u32] = Box::leak(vec![0_u32; 4].into_boxed_slice());
    rt::set_extra(PROXIMITY_DESCRIPTOR_VA, descriptor.as_ptr() as u32);
    for case in 0..CASES {
        let (init, target_id) = gen_nearby_window(&mut rng);
        let plant = gen_plant(&mut rng);
        let target = [
            f32::from_bits(word_at(&init, at(TARGET_VA))),
            f32::from_bits(word_at(&init, at(TARGET_VA) + 4)),
            f32::from_bits(word_at(&init, at(TARGET_VA) + 8)),
        ];

        let table = decode(&init);
        let objects = WindowObjects {
            base: rt::window_base(),
            bytes: &init,
        };
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&table, target_id, target, &objects, &mut recorder);
        let lifted = Outcome {
            bytes: init.clone(),
            log: recorder.log,
            ret: 0,
        };

        let rewrite = run_rewrite(&init, plant, || {
            fn_009c0d80::rw_009c0d80(target_id);
            0
        });
        tally.judge(case, &lifted, &rewrite, false);
    }
    tally
}

#[test]
fn nearby_matches_rewrite() {
    let _guard = begin();
    let tally = nearby_tally(|table, target_id, target, objects, calls| {
        table.check_nearby_event(target_id, target, objects, calls)
    });
    assert_clean("check_nearby_event", &tally);
}

#[test]
fn nearby_wrong_versions_are_caught() {
    let _guard = begin();
    // The threshold is inclusive: an event exactly at the distance is called.
    assert_caught(
        "nearby, inclusive distance",
        &nearby_tally(|table, target_id, target, objects, calls| {
            for (index, record) in table.records.iter().enumerate() {
                let event = record.event;
                if event == 0 {
                    continue;
                }
                let event_type = objects.type_byte(event) & 0x0F;
                let vector_id = if event_type >= 2 {
                    objects.vector_id(event)
                } else {
                    0
                };
                if vector_id != target_id {
                    continue;
                }
                let [ex, ey, ez] = objects.position(event);
                let [tx, ty, tz] = target;
                let (dy, dx, dz) = (ey - ty, ex - tx, ez - tz);
                let d = ((dy * dy) + (dx * dx)) + (dz * dz);
                if d <= NEARBY_DISTANCE_SQUARED {
                    calls.proximity_notify(index);
                }
            }
        }),
    );
    // The z difference is left out of the distance.
    assert_caught(
        "nearby, no z term",
        &nearby_tally(|table, target_id, target, objects, calls| {
            for (index, record) in table.records.iter().enumerate() {
                let event = record.event;
                if event == 0 {
                    continue;
                }
                let event_type = objects.type_byte(event) & 0x0F;
                let vector_id = if event_type >= 2 {
                    objects.vector_id(event)
                } else {
                    0
                };
                if vector_id != target_id {
                    continue;
                }
                let [ex, ey, _] = objects.position(event);
                let [tx, ty, _] = target;
                let (dy, dx) = (ey - ty, ex - tx);
                let d = (dy * dy) + (dx * dx);
                if d < NEARBY_DISTANCE_SQUARED {
                    calls.proximity_notify(index);
                }
            }
        }),
    );
    // Types below two are matched on the stored vector word, not on zero.
    assert_caught(
        "nearby, stored vector for low types",
        &nearby_tally(|table, target_id, target, objects, calls| {
            for (index, record) in table.records.iter().enumerate() {
                let event = record.event;
                if event == 0 {
                    continue;
                }
                let vector_id = objects.vector_id(event);
                if vector_id != target_id {
                    continue;
                }
                let [ex, ey, ez] = objects.position(event);
                let [tx, ty, tz] = target;
                let (dy, dx, dz) = (ey - ty, ex - tx, ez - tz);
                let d = ((dy * dy) + (dx * dx)) + (dz * dz);
                if d < NEARBY_DISTANCE_SQUARED {
                    calls.proximity_notify(index);
                }
            }
        }),
    );
}
