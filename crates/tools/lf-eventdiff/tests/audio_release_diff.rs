//! Differential cases of the lifted audio slot release against its three
//! verified 32-bit rewrites (the primary, radio and secondary releases).
//!
//! The task record sits in the planted window, its audio objects in the
//! same window, and the game globals in planted extra words. Each lifted
//! run and its rewrite see the same words. The effects compared are every
//! byte of the window, the helper calls in order with their arguments, and
//! nothing else: the releases have no return value.
//!
//! Each release also has deliberately wrong versions, which must be caught.
//! Runs on the 32-bit target only, under one lock per test program.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_eventdiff::rewrites::{
    fn_0097eac0, fn_0097eb70, fn_0097ee90, fn_0097eea0, fn_0097eef0, fn_0097ef80, fn_0097fc80,
    fn_0097fce0,
};
use lf_eventdiff::rt;
use lf_peds_tasks::ped_task::{
    AudioObjects, AudioSlot, AudioTask, CANDIDATE_KIND_MASK, CANDIDATE_KIND_SPECIAL,
    CANDIDATE_STATE_READY, CLEANUP_ENABLED, CLEANUP_MODE_SENTINEL, Deadline, PROGRESS_LIMIT,
    ReleaseGlobals, SKIP_MODE, SubtaskCallees,
};

/// File address of the task record, which holds the three slots.
const RECORD_VA: u32 = 0x0129_4D00;
/// Record offsets of the primary, radio and secondary slots.
const PRIMARY_OFFSET: u32 = 0x1C;
const RADIO_OFFSET: u32 = 0x130;
const SECONDARY_OFFSET: u32 = 0x134;
/// Offset of the resource word inside an audio object.
const RESOURCE_OFFSET: u32 = 0xA4;
/// File address of the first audio object and the stride between objects.
const OBJECT_BASE_VA: u32 = 0x0129_5000;
const OBJECT_STRIDE: u32 = 0x100;
const OBJECT_COUNT: u32 = 3;
/// File addresses of the three game globals the release reads.
const CLEANUP_SWITCH_VA: u32 = 0x011F_7060;
const CURRENT_MODE_VA: u32 = 0x0120_88B4;
const SKIP_MODE_VA: u32 = 0x0103_7720;
/// File address of the game clock word the window and stamp routines read.
const CURRENT_TIME_VA: u32 = 0x0117_35B4;
/// Record offsets of the progress pair, the deadline words and the window word.
const FIRST_PROGRESS_OFFSET: u32 = 0x40;
const SECOND_PROGRESS_OFFSET: u32 = 0x144;
const FIRST_DEADLINE_OFFSET: u32 = 0x13C;
const SECOND_DEADLINE_OFFSET: u32 = 0x150;
const WINDOW_DEADLINE_OFFSET: u32 = 0x14C;
/// Generated cases per release.
const CASES: u32 = 3000;

/// Helper-call tags in the call log.
const TAG_CLEANUP: u32 = 1;
const TAG_RELEASE: u32 = 2;
const TAG_DESCRIBE: u32 = 3;
const TAG_CONSTRUCT: u32 = 4;
/// Record offsets of the subtask slot, value, state and status words.
const SUBTASK_SLOT_OFFSET: u32 = 0xB4;
const SUBTASK_VALUE_OFFSET: u32 = 0xB8;
const SUBTASK_STATE_OFFSET: u32 = 0xAC;
const SUBTASK_STATUS_OFFSET: u32 = 0xB0;
/// Candidate object: its flags word and state word offsets, and its place.
const CANDIDATE_VA: u32 = 0x0129_5400;
const CANDIDATE_FLAGS_OFFSET: u32 = 0x28;
const CANDIDATE_STATE_OFFSET: u32 = 0x1304;
/// The default object word, and the two globals that reach values for the
/// subtask routine.
const DEFAULT_OBJECT_VA: u32 = 0x0129_5800;
const DEFAULT_GLOBAL_VA: u32 = 0x0123_1290;
const SPECIAL_GLOBAL_VA: u32 = 0x0123_1308;

/// One logged call: tag, first argument, second argument.
type Call = (u32, u32, u32);

/// Serialises the cases: the windows, globals and stubs are process-wide.
static LOCK: Mutex<()> = Mutex::new(());
/// Calls the planted stubs have received, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());

fn log_call(call: Call) {
    LOG.lock().unwrap().push(call);
}

// The helpers the rewrites call through numbered slots: slot 1 is the cdecl
// cleanup helper, slot 2 the thiscall release helper.

extern "cdecl" fn stub_cleanup(resource: u32) -> u32 {
    log_call((TAG_CLEANUP, resource, 0));
    0
}

extern "thiscall" fn stub_release(object: u32, zero: u32) -> u32 {
    log_call((TAG_RELEASE, object, zero));
    0
}

fn word(bytes: &[u8], va: u32) -> u32 {
    let o = (va - rt::WINDOW_VA) as usize;
    u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
}

fn put_word(bytes: &mut [u8], va: u32, value: u32) {
    let o = (va - rt::WINDOW_VA) as usize;
    bytes[o..o + 4].copy_from_slice(&value.to_le_bytes());
}

fn object_va(index: u32) -> u32 {
    OBJECT_BASE_VA + index * OBJECT_STRIDE
}

/// A small deterministic generator (xorshift32).
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

/// The window bytes of one case: random bytes, the audio objects with random
/// resource words, and a record whose slots are null or audio objects.
fn gen_window(rng: &mut Rng) -> Vec<u8> {
    let mut bytes: Vec<u8> = (0..rt::WINDOW_LEN).map(|_| rng.next() as u8).collect();
    for index in 0..OBJECT_COUNT {
        let resource = rng.next();
        put_word(&mut bytes, object_va(index) + RESOURCE_OFFSET, resource);
    }
    for offset in [PRIMARY_OFFSET, RADIO_OFFSET, SECONDARY_OFFSET] {
        let slot = match rng.below(4) {
            0 => 0,
            _ => rt::relocated(object_va(rng.below(OBJECT_COUNT))),
        };
        put_word(&mut bytes, RECORD_VA + offset, slot);
    }
    bytes
}

/// The game globals of one case, drawn from the values that matter: the
/// switch values, the sentinel and the skip value, and others.
fn gen_globals(rng: &mut Rng) -> ReleaseGlobals {
    let cleanup_enabled = match rng.below(4) {
        0 => CLEANUP_ENABLED,
        1 => 0,
        2 => 2,
        _ => rng.next(),
    };
    let current_mode = match rng.below(4) {
        0 | 1 => CLEANUP_MODE_SENTINEL,
        2 => 0,
        _ => rng.next(),
    };
    let skip_mode = match rng.below(4) {
        0 => SKIP_MODE,
        1 => 0,
        2 => 3,
        _ => rng.next(),
    };
    ReleaseGlobals {
        cleanup_enabled,
        current_mode,
        skip_mode,
    }
}

/// Plants the three globals' cells once per test.
fn plant_globals() {
    for va in [CLEANUP_SWITCH_VA, CURRENT_MODE_VA, SKIP_MODE_VA] {
        let cell: &'static mut [u32] = Box::leak(vec![0_u32; 1].into_boxed_slice());
        rt::set_extra(va, cell.as_ptr() as u32);
    }
}

fn set_globals(globals: ReleaseGlobals) {
    rt::set_extra_word(CLEANUP_SWITCH_VA, globals.cleanup_enabled);
    rt::set_extra_word(CURRENT_MODE_VA, globals.current_mode);
    rt::set_extra_word(SKIP_MODE_VA, globals.skip_mode);
}

/// The lifted side's recorder: it reads resources from the case's window
/// bytes and logs each helper call with the arguments the rewrite passes.
struct Recorder {
    bytes: Vec<u8>,
    log: Vec<Call>,
}

impl AudioObjects for Recorder {
    fn resource(&self, object: u32) -> u32 {
        let offset = object - rt::relocated(rt::WINDOW_VA);
        let va = rt::WINDOW_VA + offset;
        word(&self.bytes, va + RESOURCE_OFFSET)
    }

    fn cleanup(&mut self, resource: u32) {
        self.log.push((TAG_CLEANUP, resource, 0));
    }

    fn release(&mut self, object: u32) {
        self.log.push((TAG_RELEASE, object, 0));
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    bytes: Vec<u8>,
    log: Vec<Call>,
}

#[derive(Default)]
struct Tally {
    cases: u32,
    with_calls: u32,
    mismatches: u32,
    first: Option<String>,
}

type ReleaseLift = fn(&AudioTask, ReleaseGlobals, &mut Recorder);

/// Runs one release over generated cases. `rewrite` is the rewrite's export
/// for the slot the lift claims to release.
fn release_tally(lift: ReleaseLift, rewrite: fn(u32)) -> Tally {
    let mut rng = Rng(0x00C0_FFEE);
    let mut tally = Tally::default();
    rt::set_callee(1, stub_cleanup as *const () as usize as u32);
    rt::set_callee(2, stub_release as *const () as usize as u32);
    for case in 0..CASES {
        let init = gen_window(&mut rng);
        let globals = gen_globals(&mut rng);
        let slots = decode_task(&init);

        let mut recorder = Recorder {
            bytes: init.clone(),
            log: Vec::new(),
        };
        lift(&slots, globals, &mut recorder);
        let lifted = Outcome {
            bytes: init.clone(),
            log: recorder.log,
        };

        rt::write_window(&init);
        set_globals(globals);
        LOG.lock().unwrap().clear();
        rewrite(rt::relocated(RECORD_VA));
        let rewritten = Outcome {
            bytes: rt::read_window(),
            log: std::mem::take(&mut *LOG.lock().unwrap()),
        };

        tally.cases += 1;
        if !rewritten.log.is_empty() && rewritten.log.iter().any(|c| c.0 == TAG_CLEANUP) {
            tally.with_calls += 1;
        }
        if lifted != rewritten {
            tally.mismatches += 1;
            if tally.first.is_none() {
                tally.first = Some(format!(
                    "case {case}: calls lift {:?} rewrite {:?}",
                    lifted.log, rewritten.log
                ));
            }
        }
    }
    tally
}

fn begin() -> std::sync::MutexGuard<'static, ()> {
    let guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    let window: &'static mut [u32] =
        Box::leak(vec![0_u32; rt::WINDOW_LEN as usize / 4].into_boxed_slice());
    rt::set_window(window.as_ptr() as u32);
    let aux: &'static mut [u32] =
        Box::leak(vec![0_u32; rt::AUX_LEN as usize / 4].into_boxed_slice());
    rt::set_aux(aux.as_ptr() as u32);
    plant_globals();
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
fn primary_release_matches_rewrite() {
    let _guard = begin();
    let tally = release_tally(
        |slots, globals, calls| slots.release_slot(AudioSlot::Primary, globals, calls),
        |this| fn_0097eea0::rw_0097EEA0(this),
    );
    assert_clean("release primary", &tally);
}

#[test]
fn radio_release_matches_rewrite() {
    let _guard = begin();
    let tally = release_tally(
        |slots, globals, calls| slots.release_slot(AudioSlot::Radio, globals, calls),
        |this| fn_0097ef80::rw_0097EF80(this),
    );
    assert_clean("release radio", &tally);
}

#[test]
fn secondary_release_matches_rewrite() {
    let _guard = begin();
    let tally = release_tally(
        |slots, globals, calls| slots.release_slot(AudioSlot::Secondary, globals, calls),
        |this| fn_0097eef0::rw_0097EEF0(this),
    );
    assert_clean("release secondary", &tally);
}

#[test]
fn release_wrong_versions_are_caught() {
    let _guard = begin();
    // Cleanup on either condition instead of both.
    assert_caught(
        "release primary, cleanup on either condition",
        &release_tally(
            |slots, globals, calls| {
                let object = slots.primary;
                if object == 0 {
                    return;
                }
                if globals.cleanup_enabled != CLEANUP_ENABLED
                    || globals.current_mode == CLEANUP_MODE_SENTINEL
                        && globals.skip_mode != SKIP_MODE
                {
                    let resource = calls.resource(object);
                    calls.cleanup(resource);
                }
                calls.release(object);
            },
            |this| fn_0097eea0::rw_0097EEA0(this),
        ),
    );
    // The object is released before its resource is cleaned up.
    assert_caught(
        "release radio, order",
        &release_tally(
            |slots, globals, calls| {
                let object = slots.radio;
                if object == 0 {
                    return;
                }
                calls.release(object);
                if globals.cleanup_enabled != CLEANUP_ENABLED
                    && globals.current_mode == CLEANUP_MODE_SENTINEL
                    && globals.skip_mode != SKIP_MODE
                {
                    let resource = calls.resource(object);
                    calls.cleanup(resource);
                }
            },
            |this| fn_0097ef80::rw_0097EF80(this),
        ),
    );
    // A null slot is released too.
    assert_caught(
        "release secondary, null object",
        &release_tally(
            |slots, globals, calls| {
                let _ = globals;
                calls.release(slots.secondary);
            },
            |this| fn_0097eef0::rw_0097EEF0(this),
        ),
    );
    // The radio release reads the secondary slot.
    assert_caught(
        "release radio, secondary slot",
        &release_tally(
            |slots, globals, calls| slots.release_slot(AudioSlot::Secondary, globals, calls),
            |this| fn_0097ef80::rw_0097EF80(this),
        ),
    );
}

// ---------------------------------------------------------------------------
// The other routines of the audio task record: the unexpired-window test, the
// two deadline stamps and the progress advance.
// ---------------------------------------------------------------------------

/// Reads the record's owned fields out of the window bytes.
fn decode_task(bytes: &[u8]) -> AudioTask {
    AudioTask {
        primary: word(bytes, RECORD_VA + PRIMARY_OFFSET),
        radio: word(bytes, RECORD_VA + RADIO_OFFSET),
        secondary: word(bytes, RECORD_VA + SECONDARY_OFFSET),
        first_progress: f32::from_bits(word(bytes, RECORD_VA + FIRST_PROGRESS_OFFSET)),
        second_progress: f32::from_bits(word(bytes, RECORD_VA + SECOND_PROGRESS_OFFSET)),
        first_deadline: word(bytes, RECORD_VA + FIRST_DEADLINE_OFFSET),
        second_deadline: word(bytes, RECORD_VA + SECOND_DEADLINE_OFFSET),
        window_deadline: word(bytes, RECORD_VA + WINDOW_DEADLINE_OFFSET),
        subtask: word(bytes, RECORD_VA + SUBTASK_SLOT_OFFSET),
        subtask_value: word(bytes, RECORD_VA + SUBTASK_VALUE_OFFSET),
        subtask_state: word(bytes, RECORD_VA + SUBTASK_STATE_OFFSET),
        subtask_status: word(bytes, RECORD_VA + SUBTASK_STATUS_OFFSET),
    }
}

/// Writes the record's owned fields back into the window bytes.
fn encode_task(bytes: &mut [u8], task: &AudioTask) {
    put_word(bytes, RECORD_VA + PRIMARY_OFFSET, task.primary);
    put_word(bytes, RECORD_VA + RADIO_OFFSET, task.radio);
    put_word(bytes, RECORD_VA + SECONDARY_OFFSET, task.secondary);
    put_word(
        bytes,
        RECORD_VA + FIRST_PROGRESS_OFFSET,
        task.first_progress.to_bits(),
    );
    put_word(
        bytes,
        RECORD_VA + SECOND_PROGRESS_OFFSET,
        task.second_progress.to_bits(),
    );
    put_word(
        bytes,
        RECORD_VA + FIRST_DEADLINE_OFFSET,
        task.first_deadline,
    );
    put_word(
        bytes,
        RECORD_VA + SECOND_DEADLINE_OFFSET,
        task.second_deadline,
    );
    put_word(
        bytes,
        RECORD_VA + WINDOW_DEADLINE_OFFSET,
        task.window_deadline,
    );
    put_word(bytes, RECORD_VA + SUBTASK_SLOT_OFFSET, task.subtask);
    put_word(bytes, RECORD_VA + SUBTASK_VALUE_OFFSET, task.subtask_value);
    put_word(bytes, RECORD_VA + SUBTASK_STATE_OFFSET, task.subtask_state);
    put_word(
        bytes,
        RECORD_VA + SUBTASK_STATUS_OFFSET,
        task.subtask_status,
    );
}

/// Single-precision progress inputs and values: the exact steps and limits,
/// values past the limit, a negative value, a quiet NaN and random bits.
const PROGRESS_POOL: [f32; 8] = [0.0, 0.5, 1.0, 2.5, -0.25, 0.1, 0.9, f32::NAN];

fn gen_progress(rng: &mut Rng) -> f32 {
    if rng.below(5) == 0 {
        f32::from_bits(rng.next())
    } else {
        PROGRESS_POOL[rng.below(PROGRESS_POOL.len() as u32) as usize]
    }
}

/// Clock-relative deadline words: the stamp, the window edges and wrap-around.
fn gen_deadline(rng: &mut Rng, now: u32) -> u32 {
    match rng.below(9) {
        0 => now,
        1 => now.wrapping_sub(1000),
        2 => now.wrapping_sub(999),
        3 => now.wrapping_sub(1001),
        4 => now.wrapping_add(1),
        5 => 0,
        _ => rng.next(),
    }
}

/// The record bytes of one progress or window case: random bytes, the
/// slots, the progress pair and the deadline words set from the case's `arg`
/// (the clock for the deadline routines).
fn gen_task_window(rng: &mut Rng, arg: u32) -> Vec<u8> {
    let mut bytes = gen_window(rng);
    let task = AudioTask {
        primary: word(&bytes, RECORD_VA + PRIMARY_OFFSET),
        radio: word(&bytes, RECORD_VA + RADIO_OFFSET),
        secondary: word(&bytes, RECORD_VA + SECONDARY_OFFSET),
        first_progress: gen_progress(rng),
        second_progress: gen_progress(rng),
        first_deadline: gen_deadline(rng, arg),
        second_deadline: gen_deadline(rng, arg),
        window_deadline: gen_deadline(rng, arg),
        subtask: 0,
        subtask_value: 0,
        subtask_state: 0,
        subtask_status: 0,
    };
    encode_task(&mut bytes, &task);
    bytes
}

/// A lift of one routine: it changes the task and returns the answer (zero
/// for routines without one).
type TaskLift = fn(&mut AudioTask, u32) -> u32;
/// The rewrite of one routine, given the record and the case's argument.
type TaskRewrite = fn(u32, u32) -> u32;

/// Runs one routine over generated cases. `gen_arg` draws the case's
/// argument; `ret_checked` says whether the answer is compared.
fn task_tally(
    lift: TaskLift,
    rewrite: TaskRewrite,
    gen_arg: fn(&mut Rng) -> u32,
    ret_checked: bool,
) -> Tally {
    let mut rng = Rng(0x0A0B_0C0D);
    let mut tally = Tally::default();
    for case in 0..CASES {
        let arg = gen_arg(&mut rng);
        let init = gen_task_window(&mut rng, arg);

        let mut task = decode_task(&init);
        let ret = lift(&mut task, arg);
        let mut expected = init.clone();
        encode_task(&mut expected, &task);
        let lifted = Outcome {
            bytes: expected,
            log: Vec::new(),
        };

        rt::write_window(&init);
        LOG.lock().unwrap().clear();
        let rewrite_ret = rewrite(rt::relocated(RECORD_VA), arg);
        let rewritten = Outcome {
            bytes: rt::read_window(),
            log: Vec::new(),
        };

        tally.cases += 1;
        let same_ret = !ret_checked || ret == rewrite_ret;
        if lifted != rewritten || !same_ret {
            tally.mismatches += 1;
            if tally.first.is_none() {
                tally.first = Some(format!(
                    "case {case}: arg {arg:#x}; ret lift {ret:#x} rewrite {rewrite_ret:#x}"
                ));
            }
        }
    }
    tally
}

fn gen_clock(rng: &mut Rng) -> u32 {
    match rng.below(5) {
        0 => 0,
        1 => u32::MAX,
        2 => 0x8000_0000,
        _ => rng.next(),
    }
}

fn gen_progress_bits(rng: &mut Rng) -> u32 {
    gen_progress(rng).to_bits()
}

/// The rewrites that read the clock take it from the game-clock global, so
/// each case plants its clock before the rewrite runs.
fn with_clock(now: u32) {
    rt::set_aux_word(CURRENT_TIME_VA, now);
}

#[test]
fn window_matches_rewrite() {
    let _guard = begin();
    let tally = task_tally(
        |task, now| u32::from(task.has_unexpired_window(now)),
        |this, now| {
            with_clock(now);
            u32::from(fn_0097eb70::rw_0097EB70(this))
        },
        gen_clock,
        true,
    );
    assert_clean("has_unexpired_window", &tally);
}

#[test]
fn window_wrong_versions_are_caught() {
    let _guard = begin();
    // The window is closed on the first word only.
    assert_caught(
        "window, first word only",
        &task_tally(
            |task, now| u32::from(now < task.second_deadline.wrapping_add(1000)),
            |this, now| {
                with_clock(now);
                u32::from(fn_0097eb70::rw_0097EB70(this))
            },
            gen_clock,
            true,
        ),
    );
    // The window is one tick too wide.
    assert_caught(
        "window, inclusive edge",
        &task_tally(
            |task, now| {
                u32::from(
                    now <= task.second_deadline.wrapping_add(1000)
                        || now <= task.window_deadline.wrapping_add(1000),
                )
            },
            |this, now| {
                with_clock(now);
                u32::from(fn_0097eb70::rw_0097EB70(this))
            },
            gen_clock,
            true,
        ),
    );
}

#[test]
fn first_stamp_matches_rewrite() {
    let _guard = begin();
    let tally = task_tally(
        |task, now| {
            task.stamp_deadline(Deadline::First, now);
            0
        },
        |this, now| {
            with_clock(now);
            fn_0097ee90::rw_0097EE90(this);
            0
        },
        gen_clock,
        false,
    );
    assert_clean("stamp first deadline", &tally);
}

#[test]
fn second_stamp_matches_rewrite() {
    let _guard = begin();
    let tally = task_tally(
        |task, now| {
            task.stamp_deadline(Deadline::Second, now);
            0
        },
        |this, now| {
            with_clock(now);
            fn_0097fce0::rw_0097FCE0(this);
            0
        },
        gen_clock,
        false,
    );
    assert_clean("stamp second deadline", &tally);
}

#[test]
fn stamp_wrong_version_is_caught() {
    let _guard = begin();
    // The second stamp writes the first word.
    assert_caught(
        "second stamp, first word",
        &task_tally(
            |task, now| {
                task.stamp_deadline(Deadline::First, now);
                0
            },
            |this, now| {
                with_clock(now);
                fn_0097fce0::rw_0097FCE0(this);
                0
            },
            gen_clock,
            false,
        ),
    );
}

#[test]
fn progress_matches_rewrite() {
    let _guard = begin();
    let tally = task_tally(
        |task, bits| {
            task.add_float_progress(f32::from_bits(bits));
            0
        },
        |this, bits| {
            fn_0097fc80::rw_0097FC80(this, bits);
            0
        },
        gen_progress_bits,
        false,
    );
    assert_clean("add float progress", &tally);
}

#[test]
fn progress_wrong_versions_are_caught() {
    let _guard = begin();
    // The first value may move down.
    assert_caught(
        "progress, first value moves down",
        &task_tally(
            |task, bits| {
                let scaled = (f32::from_bits(bits) * 0.1).min(PROGRESS_LIMIT);
                task.first_progress = scaled;
                let advanced = task.second_progress + 0.4;
                task.second_progress = advanced.min(PROGRESS_LIMIT);
                0
            },
            |this, bits| {
                fn_0097fc80::rw_0097FC80(this, bits);
                0
            },
            gen_progress_bits,
            false,
        ),
    );
    // The second value is not capped.
    assert_caught(
        "progress, second value uncapped",
        &task_tally(
            |task, bits| {
                task.add_float_progress(f32::from_bits(bits));
                task.second_progress += 0.4;
                0
            },
            |this, bits| {
                fn_0097fc80::rw_0097FC80(this, bits);
                0
            },
            gen_progress_bits,
            false,
        ),
    );
}

// ---------------------------------------------------------------------------
// The subtask replacement: the record's subtask words, one candidate object,
// the two helpers and the two globals.
// ---------------------------------------------------------------------------

/// Stubs for the describe and construct helpers (thiscall; slot 1 and 2).
extern "thiscall" fn stub_describe(previous: u32, slot: u32) -> u32 {
    log_call((TAG_DESCRIBE, previous, slot));
    0
}

extern "thiscall" fn stub_construct(candidate: u32, slot: u32) -> u32 {
    log_call((TAG_CONSTRUCT, candidate, slot));
    0
}

/// The lifted side's subtask host: reads the candidate and the globals from
/// the case's window bytes and logs the two helper calls.
struct SubtaskHost {
    bytes: Vec<u8>,
    special: u32,
    log: Vec<Call>,
}

impl SubtaskHost {
    fn slot_address() -> u32 {
        rt::relocated(RECORD_VA + SUBTASK_SLOT_OFFSET)
    }

    fn window_word(&self, address: u32, offset: u32) -> u32 {
        let base = rt::relocated(rt::WINDOW_VA);
        word(&self.bytes, rt::WINDOW_VA + (address - base) + offset)
    }
}

impl SubtaskCallees for SubtaskHost {
    fn describe(&mut self, previous: u32) {
        self.log
            .push((TAG_DESCRIBE, previous, Self::slot_address()));
    }

    fn construct(&mut self, candidate: u32) {
        self.log
            .push((TAG_CONSTRUCT, candidate, Self::slot_address()));
    }

    fn candidate_flags(&self, candidate: u32) -> u32 {
        self.window_word(candidate, CANDIDATE_FLAGS_OFFSET)
    }

    fn candidate_state(&self, candidate: u32) -> u32 {
        self.window_word(candidate, CANDIDATE_STATE_OFFSET)
    }

    fn default_object(&self) -> u32 {
        word(&self.bytes, DEFAULT_OBJECT_VA)
    }

    fn special_value(&self) -> u32 {
        self.special
    }
}

/// The candidate's flags and state, drawn to hit the special-candidate test
/// on both sides of each comparison.
fn gen_candidate(rng: &mut Rng) -> (u32, u32) {
    let flags = match rng.below(6) {
        0 | 1 => CANDIDATE_KIND_SPECIAL,
        2 => CANDIDATE_KIND_SPECIAL | 0x01,
        3 => 0x0000_0040,
        4 => CANDIDATE_KIND_MASK | CANDIDATE_KIND_SPECIAL,
        _ => rng.next(),
    };
    let state = match rng.below(4) {
        0 | 1 => CANDIDATE_STATE_READY,
        2 => 0,
        _ => rng.next(),
    };
    (flags, state)
}

/// One subtask case: the window with the record's subtask words, the
/// candidate object and the default object word all generated.
fn gen_subtask_window(rng: &mut Rng) -> (Vec<u8>, u32, u32, u32, u32) {
    let mut bytes = gen_window(rng);
    let (flags, state) = gen_candidate(rng);
    put_word(&mut bytes, CANDIDATE_VA + CANDIDATE_FLAGS_OFFSET, flags);
    put_word(&mut bytes, CANDIDATE_VA + CANDIDATE_STATE_OFFSET, state);
    let subtask = match rng.below(3) {
        0 => 0,
        1 => rt::relocated(CANDIDATE_VA),
        _ => 0xABCD_0007,
    };
    let initial = match rng.below(4) {
        0 => 0,
        _ => rng.next(),
    };
    let candidate = match rng.below(3) {
        0 => 0,
        _ => rt::relocated(CANDIDATE_VA),
    };
    let status = rng.next();
    let special = rng.next();
    put_word(&mut bytes, RECORD_VA + SUBTASK_SLOT_OFFSET, subtask);
    put_word(&mut bytes, RECORD_VA + SUBTASK_VALUE_OFFSET, rng.next());
    put_word(&mut bytes, RECORD_VA + SUBTASK_STATE_OFFSET, rng.next());
    put_word(&mut bytes, RECORD_VA + SUBTASK_STATUS_OFFSET, rng.next());
    // The records' default object word is the value a null initial stores.
    put_word(&mut bytes, DEFAULT_OBJECT_VA, rng.next());
    (bytes, initial, candidate, status, special)
}

/// A subtask routine: the lift and the wrong versions share this shape.
type SubtaskLift = fn(&mut AudioTask, u32, u32, u32, &mut SubtaskHost);

fn subtask_tally(lift: SubtaskLift) -> Tally {
    let mut rng = Rng(0x5B5B_1234);
    let mut tally = Tally::default();
    rt::set_callee(1, stub_describe as *const () as usize as u32);
    rt::set_callee(2, stub_construct as *const () as usize as u32);
    // The two globals: the default global holds the address of the default
    // object word, and the special global holds the special value.
    let default_cell: &'static mut [u32] = Box::leak(vec![0_u32; 1].into_boxed_slice());
    rt::set_extra(DEFAULT_GLOBAL_VA, default_cell.as_ptr() as u32);
    let special_cell: &'static mut [u32] = Box::leak(vec![0_u32; 1].into_boxed_slice());
    rt::set_extra(SPECIAL_GLOBAL_VA, special_cell.as_ptr() as u32);
    rt::set_extra_word(DEFAULT_GLOBAL_VA, rt::relocated(DEFAULT_OBJECT_VA));
    for case in 0..CASES {
        let (init, initial, candidate, status, special) = gen_subtask_window(&mut rng);

        let mut task = decode_task(&init);
        let mut host = SubtaskHost {
            bytes: init.clone(),
            special,
            log: Vec::new(),
        };
        lift(&mut task, initial, candidate, status, &mut host);
        let mut expected = init.clone();
        encode_task(&mut expected, &task);
        let lifted = Outcome {
            bytes: expected,
            log: host.log,
        };

        rt::write_window(&init);
        rt::set_extra_word(SPECIAL_GLOBAL_VA, special);
        LOG.lock().unwrap().clear();
        fn_0097eac0::rw_0097EAC0(rt::relocated(RECORD_VA), initial, candidate, status);
        let rewritten = Outcome {
            bytes: rt::read_window(),
            log: std::mem::take(&mut *LOG.lock().unwrap()),
        };

        tally.cases += 1;
        if rewritten.log.iter().any(|c| c.0 == TAG_CONSTRUCT) {
            tally.with_calls += 1;
        }
        if lifted != rewritten {
            tally.mismatches += 1;
            if tally.first.is_none() {
                tally.first = Some(format!(
                    "case {case}: calls lift {:?} rewrite {:?}",
                    lifted.log, rewritten.log
                ));
            }
        }
    }
    tally
}

#[test]
fn subtask_matches_rewrite() {
    let _guard = begin();
    let tally = subtask_tally(|task, initial, candidate, status, host| {
        task.replace_subtask(initial, candidate, status, host)
    });
    assert_clean("replace_subtask", &tally);
}

#[test]
fn subtask_wrong_versions_are_caught() {
    let _guard = begin();
    // The describe helper is skipped.
    assert_caught(
        "subtask, no describe",
        &subtask_tally(|task, initial, candidate, status, host| {
            let previous = task.subtask;
            if previous != 0 {
                task.subtask = 0;
            }
            if initial == 0 {
                task.subtask_value = host.default_object();
                task.subtask_state = 0;
                task.subtask_status = 0;
                return;
            }
            task.subtask_value = initial;
            if candidate != 0 {
                let flags = host.candidate_flags(candidate);
                if flags & CANDIDATE_KIND_MASK == CANDIDATE_KIND_SPECIAL
                    && host.candidate_state(candidate) == CANDIDATE_STATE_READY
                {
                    task.subtask_value = host.special_value();
                }
            }
            task.subtask_state = 0;
            task.subtask = candidate;
            host.construct(candidate);
            task.subtask_status = status;
        }),
    );
    // The special value ignores the candidate's state.
    assert_caught(
        "subtask, special without state",
        &subtask_tally(|task, initial, candidate, status, host| {
            let previous = task.subtask;
            if previous != 0 {
                host.describe(previous);
                task.subtask = 0;
            }
            if initial == 0 {
                task.subtask_value = host.default_object();
                task.subtask_state = 0;
                task.subtask_status = 0;
                return;
            }
            task.subtask_value = initial;
            if candidate != 0
                && host.candidate_flags(candidate) & CANDIDATE_KIND_MASK == CANDIDATE_KIND_SPECIAL
            {
                task.subtask_value = host.special_value();
            }
            task.subtask_state = 0;
            task.subtask = candidate;
            host.construct(candidate);
            task.subtask_status = status;
        }),
    );
    // The candidate is constructed only when it is nonnull.
    assert_caught(
        "subtask, construct only when nonnull",
        &subtask_tally(|task, initial, candidate, status, host| {
            let previous = task.subtask;
            if previous != 0 {
                host.describe(previous);
                task.subtask = 0;
            }
            if initial == 0 {
                task.subtask_value = host.default_object();
                task.subtask_state = 0;
                task.subtask_status = 0;
                return;
            }
            task.subtask_value = initial;
            if candidate != 0 {
                let flags = host.candidate_flags(candidate);
                if flags & CANDIDATE_KIND_MASK == CANDIDATE_KIND_SPECIAL
                    && host.candidate_state(candidate) == CANDIDATE_STATE_READY
                {
                    task.subtask_value = host.special_value();
                }
            }
            task.subtask_state = 0;
            task.subtask = candidate;
            if candidate != 0 {
                host.construct(candidate);
            }
            task.subtask_status = status;
        }),
    );
}
