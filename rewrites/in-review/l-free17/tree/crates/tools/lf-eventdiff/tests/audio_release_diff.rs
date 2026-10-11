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

use lf_eventdiff::rewrites::{fn_0097eea0, fn_0097eef0, fn_0097ef80};
use lf_eventdiff::rt;
use lf_peds_tasks::ped_task::{
    AudioObjects, AudioSlot, AudioTask, CLEANUP_ENABLED, CLEANUP_MODE_SENTINEL, ReleaseGlobals,
    SKIP_MODE,
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
/// Generated cases per release.
const CASES: u32 = 3000;

/// Helper-call tags in the call log.
const TAG_CLEANUP: u32 = 1;
const TAG_RELEASE: u32 = 2;

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
    with_cleanup: u32,
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
        let slots = AudioTask {
            primary: word(&init, RECORD_VA + PRIMARY_OFFSET),
            radio: word(&init, RECORD_VA + RADIO_OFFSET),
            secondary: word(&init, RECORD_VA + SECONDARY_OFFSET),
        };

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
            tally.with_cleanup += 1;
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
    let aux: &'static mut [u32] = Box::leak(vec![0_u32; rt::AUX_LEN as usize / 4].into_boxed_slice());
    rt::set_aux(aux.as_ptr() as u32);
    plant_globals();
    guard
}

fn assert_clean(name: &str, tally: &Tally) {
    eprintln!(
        "{name}: {} cases, {} with cleanup calls, {} mismatches",
        tally.cases, tally.with_cleanup, tally.mismatches
    );
    assert_eq!(tally.cases, CASES, "{name}: case count");
    assert_eq!(
        tally.mismatches, 0,
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
