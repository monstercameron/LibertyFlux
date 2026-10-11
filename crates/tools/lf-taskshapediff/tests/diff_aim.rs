//! Differential cases of the aim-record update against its verified 32-bit
//! rewrite.
//!
//! The lifted update and the rewrite run on the same generated owner, task,
//! target point, aim point, driver and collaborator answers. The effects
//! compared are:
//!
//! - the two collaborator calls, in order, with their arguments as the
//!   rewrite's stubs read them at the time of the call (the record words and
//!   the target words the callee would see; the driver's frame word, which
//!   the original fixes at zero; the driver's handle and the owner's handle);
//! - the return value;
//! - the record (four words) and the target point (three words), bit for
//!   bit;
//! - the owner, the task and the aim point, which the rewrite must leave as
//!   it found them.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses and calls
//! numbered stubs.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{AimCallees, AimOwner, AimTask, RecordWords, Vec3};
use lf_taskshapediff::rewrites::fn_00cfdee0;
use lf_taskshapediff::{Xorshift, rt};

/// Bytes planted for the owner: the driver pointer is the largest offset.
const OWNER_BYTES: usize = 0xD6C;
/// Offset of the pointer to the owner's position block.
const POSITION_POINTER: usize = 0x20;
/// Offset of the driver pointer.
const DRIVER_OFFSET: usize = 0xD68;
/// Offsets of the owner's x and y in its position block.
const POSITION_X: usize = 0x30;
const POSITION_Y: usize = 0x34;
/// Bytes planted for the position block.
const POSITION_BYTES: usize = 0x40;
/// Bytes planted for the task: the accumulated words end at 0x68.
const TASK_BYTES: usize = 0x80;
/// Offsets of the accumulated components in the task.
const ACCUMULATED: [usize; 3] = [0x60, 0x64, 0x68];
/// Bytes planted for the driver block (its contents are not read).
const DRIVER_BYTES: usize = 0x10;
/// Bytes of the aim point (two words) and of the target point (three words).
const AIM_BYTES: usize = 8;
const TARGET_BYTES: usize = 12;
/// Bytes of the record the update writes.
const RECORD_BYTES: usize = 16;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00CF;

/// Call tags in the log.
const TAG_ADVANCE: u32 = 1;
const TAG_DRIVER: u32 = 2;

/// One logged call: its tag, then the words it would see.
type Call = Vec<u32>;

/// The call log of the rewrite's stubs, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// The first collaborator's answer.
static GO: Mutex<u32> = Mutex::new(0);
/// The address of the sixteen-byte answer the driver call returns.
static RESULT_PTR: Mutex<u32> = Mutex::new(0);
/// Serialises the cases: the stubs and the callee table are process-wide.
static LOCK: Mutex<()> = Mutex::new(());

/// Reads `count` words at a planted address, as a call would see them.
fn words_at(address: u32, count: usize) -> Vec<u32> {
    (0..count)
        .map(|i| {
            // SAFETY: the rewrite passes addresses of buffers the test planted
            // and sized for every word read here.
            unsafe { ((address as usize + 4 * i) as *const u32).read_unaligned() }
        })
        .collect()
}

/// The first collaborator's stub: logs the record and the target it is
/// given, and the constant fifth argument, then answers the planted flag.
extern "cdecl" fn stub_advance(
    driver: u32,
    owner: u32,
    record: u32,
    target: u32,
    zero: u32,
) -> u32 {
    let mut entry = vec![TAG_ADVANCE, driver, owner];
    entry.extend(words_at(record, 4));
    entry.extend(words_at(target, 3));
    entry.push(zero);
    LOG.lock().unwrap().push(entry);
    *GO.lock().unwrap()
}

/// The driver call's stub: logs the driver, the frame word it reads and the
/// record, then answers the address of the planted result.
extern "thiscall" fn stub_driver(driver: u32, frame: u32, record: u32) -> u32 {
    let mut entry = vec![TAG_DRIVER, driver];
    entry.extend(words_at(frame, 1));
    entry.extend(words_at(record, 4));
    LOG.lock().unwrap().push(entry);
    *RESULT_PTR.lock().unwrap()
}

/// The collaborator the lifted update calls. It logs the same entries as the
/// rewrite's stubs, with the constant arguments the original fixes.
struct Recorder {
    go: u32,
    result: RecordWords,
    log: Vec<Call>,
}

impl AimCallees for Recorder {
    fn advance(&mut self, driver: u32, owner: u32, record: RecordWords, target: Vec3) -> u32 {
        let mut entry = vec![TAG_ADVANCE, driver, owner];
        entry.extend(record);
        entry.extend([target.x.to_bits(), target.y.to_bits(), target.z.to_bits()]);
        entry.push(0);
        self.log.push(entry);
        self.go
    }
    fn driver_call(&mut self, driver: u32, record: RecordWords) -> RecordWords {
        let mut entry = vec![TAG_DRIVER, driver, 0];
        entry.extend(record);
        self.log.push(entry);
        self.result
    }
}

/// One generated case and what the rewrite did with it.
struct Case {
    owner: Vec<u8>,
    position: Vec<u8>,
    task: Vec<u8>,
    aim: Vec<u8>,
    target_before: Vec<u8>,
    target_after: Vec<u8>,
    record_after: Vec<u8>,
    owner_before: Vec<u8>,
    task_before: Vec<u8>,
    aim_before: Vec<u8>,
    go: u32,
    result: RecordWords,
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

/// Random bytes of the given length.
fn random_bytes(rng: &mut Xorshift, len: usize) -> Vec<u8> {
    (0..len).map(|_| (rng.next_u32() & 0xFF) as u8).collect()
}

/// Generates a case, plants it and runs the rewrite on it.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let mut position = random_bytes(rng, POSITION_BYTES);
    let owner_x = rng.word();
    let owner_y = rng.word();
    set_word(&mut position, POSITION_X, owner_x);
    set_word(&mut position, POSITION_Y, owner_y);

    let driver = random_bytes(rng, DRIVER_BYTES);
    let has_driver = rng.next_u32() % 4 != 0;

    let mut owner = random_bytes(rng, OWNER_BYTES);
    set_word(&mut owner, POSITION_POINTER, address_of(&position));
    set_word(
        &mut owner,
        DRIVER_OFFSET,
        if has_driver { address_of(&driver) } else { 0 },
    );

    let mut task = random_bytes(rng, TASK_BYTES);
    for offset in ACCUMULATED {
        set_word(&mut task, offset, rng.word());
    }

    // A quarter of the aim points sit on the owner, so the length is zero
    // and the direction takes the zero branch.
    let mut aim = random_bytes(rng, AIM_BYTES);
    if rng.next_u32() % 4 == 0 {
        set_word(&mut aim, 0, owner_x);
        set_word(&mut aim, 4, owner_y);
    } else {
        set_word(&mut aim, 0, rng.word());
        set_word(&mut aim, 4, rng.word());
    }

    let mut target = random_bytes(rng, TARGET_BYTES);
    for k in 0..3 {
        set_word(&mut target, 4 * k, rng.word());
    }

    let mut record = random_bytes(rng, RECORD_BYTES);

    let result_words: RecordWords = [rng.word(), rng.word(), rng.word(), rng.word()];
    let mut result_block: Vec<u32> = result_words.to_vec();
    let go = match rng.next_u32() % 4 {
        0 => 0x0000_0100,
        _ => rng.word(),
    };

    let owner_before = owner.clone();
    let task_before = task.clone();
    let aim_before = aim.clone();
    let target_before = target.clone();

    *GO.lock().unwrap() = go;
    *RESULT_PTR.lock().unwrap() = result_block.as_mut_ptr() as usize as u32;
    LOG.lock().unwrap().clear();
    let returned = fn_00cfdee0::rw_00cfdee0(
        address_of(&task),
        address_of(&owner),
        address_of(&aim),
        address_of(&record),
        address_of(&target),
    );
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    Case {
        owner,
        position,
        task,
        aim,
        target_before,
        target_after: target,
        record_after: record,
        owner_before,
        task_before,
        aim_before,
        go,
        result: result_words,
        returned,
        calls,
    }
}

/// Decodes the owner the lift takes: its position and its driver handle.
fn owner_of(case: &Case) -> AimOwner {
    let x = f32::from_bits(word_at(&case.position, POSITION_X));
    let y = f32::from_bits(word_at(&case.position, POSITION_Y));
    let driver = word_at(&case.owner_before, DRIVER_OFFSET);
    AimOwner {
        position: [x, y],
        driver: if driver == 0 { None } else { Some(driver) },
    }
}

/// Decodes the task the lift takes: its accumulated offset.
fn task_of(case: &Case) -> AimTask {
    AimTask {
        accumulated: Vec3 {
            x: f32::from_bits(word_at(&case.task_before, ACCUMULATED[0])),
            y: f32::from_bits(word_at(&case.task_before, ACCUMULATED[1])),
            z: f32::from_bits(word_at(&case.task_before, ACCUMULATED[2])),
        },
    }
}

/// The aim point the lift takes.
fn aim_of(case: &Case) -> [f32; 2] {
    [
        f32::from_bits(word_at(&case.aim_before, 0)),
        f32::from_bits(word_at(&case.aim_before, 4)),
    ]
}

/// The target point the lift takes, before the update.
fn target_of(case: &Case) -> Vec3 {
    Vec3 {
        x: f32::from_bits(word_at(&case.target_before, 0)),
        y: f32::from_bits(word_at(&case.target_before, 4)),
        z: f32::from_bits(word_at(&case.target_before, 8)),
    }
}

/// The record's four words, as the rewrite left them.
fn record_after(case: &Case) -> RecordWords {
    [
        word_at(&case.record_after, 0),
        word_at(&case.record_after, 4),
        word_at(&case.record_after, 8),
        word_at(&case.record_after, 12),
    ]
}

/// The target's three words, as the rewrite left them.
fn target_after(case: &Case) -> [u32; 3] {
    [
        word_at(&case.target_after, 0),
        word_at(&case.target_after, 4),
        word_at(&case.target_after, 8),
    ]
}

/// Checks the rewrite's own effects: the owner, the task and the aim point
/// are unchanged.
fn check_rewrite_effects(case: &Case) {
    assert_eq!(case.owner, case.owner_before, "the owner is unchanged");
    assert_eq!(case.task, case.task_before, "the task is unchanged");
    assert_eq!(case.aim, case.aim_before, "the aim point is unchanged");
}

/// Runs the lift on a case and returns its answer, record, target and calls.
fn run_lift(case: &Case) -> (u32, RecordWords, Vec3, Vec<Call>) {
    let owner = owner_of(case);
    let task = task_of(case);
    let mut target = target_of(case);
    let mut record: RecordWords = [0; 4];
    let mut recorder = Recorder {
        go: case.go,
        result: case.result,
        log: Vec::new(),
    };
    let owner_handle = address_of(&case.owner);
    let answer = task.update(
        owner_handle,
        &owner,
        &mut target,
        aim_of(case),
        &mut record,
        &mut recorder,
    );
    (answer, record, target, recorder.log)
}

/// Asserts that the lift matches the rewrite on one case.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    let (answer, record, target, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the collaborator calls match in order");
    assert_eq!(answer, case.returned, "the answer matches");
    assert_eq!(record, record_after(case), "the record matches bit for bit");
    assert_eq!(
        [target.x.to_bits(), target.y.to_bits(), target.z.to_bits()],
        target_after(case),
        "the target matches bit for bit"
    );
}

/// The deviations of a wrong version: each is the update with one change.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The zero-length guard is dropped.
    NoGuard,
    /// The z component is zero, not one over the length times zero.
    ZeroZ,
    /// The accumulated offset is added before the first collaborator.
    AddBeforeAdvance,
    /// The second collaborator is never called.
    NoDriverCall,
    /// The answer is returned without its low byte set.
    RawAnswer,
    /// The accumulated x and y are swapped.
    AccumulatedSwapped,
    /// The first collaborator's answer is ignored.
    IgnoreContinue,
}

/// The wrong version of the update: the same inputs, one deviation. Returns
/// the answer, the record, the target and the calls.
fn run_wrong(kind: Wrong, case: &Case) -> (u32, RecordWords, Vec3, Vec<Call>) {
    let owner = owner_of(case);
    let task = task_of(case);
    let mut target = target_of(case);
    let aim = aim_of(case);
    let mut recorder = Recorder {
        go: case.go,
        result: case.result,
        log: Vec::new(),
    };
    let owner_handle = address_of(&case.owner);
    let dx = sub(aim[0], owner.position[0]);
    let dy = sub(aim[1], owner.position[1]);
    let len2 = add(mul(dy, dy), mul(dx, dx));
    let root = core::hint::black_box(len2).sqrt();
    let inv = if matches!(kind, Wrong::NoGuard) {
        core::hint::black_box(1.0_f32) / core::hint::black_box(root)
    } else if len2 <= 0.0 {
        0.0
    } else {
        core::hint::black_box(1.0_f32) / core::hint::black_box(root)
    };
    let z = if matches!(kind, Wrong::ZeroZ) {
        0.0
    } else {
        mul(inv, 0.0)
    };
    let mut record: RecordWords = [
        mul(inv, dx).to_bits(),
        mul(dy, inv).to_bits(),
        z.to_bits(),
        0,
    ];
    let Some(driver) = owner.driver else {
        return (0, record, target, recorder.log);
    };
    if matches!(kind, Wrong::AddBeforeAdvance) {
        let acc = task.accumulated;
        target.x = add(target.x, acc.x);
        target.y = add(target.y, acc.y);
        target.z = add(target.z, acc.z);
    }
    let proceed = recorder.advance(driver, owner_handle, record, target);
    if !matches!(kind, Wrong::IgnoreContinue) && proceed & 0xFF == 0 {
        return (0, record, target, recorder.log);
    }
    if !matches!(kind, Wrong::AddBeforeAdvance) {
        let acc = task.accumulated;
        if matches!(kind, Wrong::AccumulatedSwapped) {
            target.x = add(target.x, acc.y);
            target.y = add(target.y, acc.x);
        } else {
            target.x = add(target.x, acc.x);
            target.y = add(target.y, acc.y);
        }
        target.z = add(target.z, acc.z);
    }
    if matches!(kind, Wrong::NoDriverCall) {
        return ((record[3] & 0xFFFF_FF00) | 1, record, target, recorder.log);
    }
    let answer_words = recorder.driver_call(driver, record);
    record = answer_words;
    let answer = if matches!(kind, Wrong::RawAnswer) {
        record[3]
    } else {
        (record[3] & 0xFFFF_FF00) | 1
    };
    (answer, record, target, recorder.log)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (answer, record, target, calls) = run_wrong(kind, &case);
        let same = calls == case.calls
            && answer == case.returned
            && record == record_after(&case)
            && [target.x.to_bits(), target.y.to_bits(), target.z.to_bits()] == target_after(&case);
        if !same {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

#[test]
fn aim_update_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_advance as *const () as usize as u32);
    rt::set_callee(2, stub_driver as *const () as usize as u32);
    let mut rng = Xorshift::new(SEED);
    let mut answered_nonzero = 0;
    let mut zero_length = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        if case.returned != 0 {
            answered_nonzero += 1;
        }
        let (dx, dy) = (
            f32::from_bits(word_at(&case.aim_before, 0))
                - f32::from_bits(word_at(&case.position, POSITION_X)),
            f32::from_bits(word_at(&case.aim_before, 4))
                - f32::from_bits(word_at(&case.position, POSITION_Y)),
        );
        if dx == 0.0 && dy == 0.0 {
            zero_length += 1;
        }
    }
    eprintln!("cases {CASES}: non-zero answers {answered_nonzero}, zero-length aims {zero_length}");
}

#[test]
fn aim_update_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_advance as *const () as usize as u32);
    rt::set_callee(2, stub_driver as *const () as usize as u32);
    for kind in [
        Wrong::NoGuard,
        Wrong::ZeroZ,
        Wrong::AddBeforeAdvance,
        Wrong::NoDriverCall,
        Wrong::RawAnswer,
        Wrong::AccumulatedSwapped,
        Wrong::IgnoreContinue,
    ] {
        assert!(
            wrong_mismatches(kind) > 0,
            "wrong version {kind:?} is caught"
        );
    }
}

/// Single-precision product with the operands pinned in the original's order.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Single-precision sum with the operands pinned in the original's order.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Single-precision difference with the operands pinned in the original's order.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
