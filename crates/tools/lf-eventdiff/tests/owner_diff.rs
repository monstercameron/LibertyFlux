//! Differential cases of the lifted owner replacement against its verified
//! 32-bit rewrite.
//!
//! The receiver sits in the planted window with its owner word at the fixed
//! offset. Each lifted run and the rewrite see the same words. The effects
//! compared are every byte of the window and the helper calls in order with
//! their arguments; the routine has no return value. Deliberately wrong
//! versions must differ from the rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only, under one lock per test program.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_eventdiff::rewrites::fn_009c12b0;
use lf_eventdiff::rt;
use lf_peds_tasks::ped_task::{OwnerCallees, TaskOwner};

/// File address of the receiver and the offset of its owner word.
const RECEIVER_VA: u32 = 0x0129_5600;
const OWNER_OFFSET: u32 = 0x25C;
/// Two candidate owner objects; the stubs never read them.
const OBJECT_A_VA: u32 = 0x0129_6000;
const OBJECT_B_VA: u32 = 0x0129_6100;
/// Generated cases.
const CASES: u32 = 3000;

/// Helper-call tags in the call log.
const TAG_RELEASE: u32 = 1;
const TAG_DEALLOCATE: u32 = 2;

/// One logged call: tag, first argument, second argument (0 when absent).
type Call = (u32, u32, u32);

/// Serialises the cases: the window and the stubs are process-wide.
static LOCK: Mutex<()> = Mutex::new(());
/// Calls the planted stubs have received, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());

fn log_call(call: Call) {
    LOG.lock().unwrap().push(call);
}

// The helpers: slot 1 is the thiscall release of the previous object, slot 2
// the cdecl deallocator.

extern "thiscall" fn stub_release(previous: u32) -> u32 {
    log_call((TAG_RELEASE, previous, 0));
    0
}

extern "cdecl" fn stub_deallocate(previous: u32) -> u32 {
    log_call((TAG_DEALLOCATE, previous, 0));
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

/// An owner pointer: zero, one of the two objects, or a random value.
fn gen_pointer(rng: &mut Rng) -> u32 {
    match rng.below(5) {
        0 => 0,
        1 => rt::relocated(OBJECT_A_VA),
        2 => rt::relocated(OBJECT_B_VA),
        _ => rng.next(),
    }
}

/// The window bytes of one case: random bytes and a generated owner word.
fn gen_window(rng: &mut Rng) -> (Vec<u8>, u32) {
    let mut bytes: Vec<u8> = (0..rt::WINDOW_LEN).map(|_| rng.next() as u8).collect();
    let owner = gen_pointer(rng);
    put_word(&mut bytes, RECEIVER_VA + OWNER_OFFSET, owner);
    (bytes, owner)
}

/// The observable effects of one run.
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

/// The lifted side's recorder: it logs each helper call.
struct Recorder {
    log: Vec<Call>,
}

impl OwnerCallees for Recorder {
    fn release(&mut self, previous: u32) {
        self.log.push((TAG_RELEASE, previous, 0));
    }

    fn deallocate(&mut self, previous: u32) {
        self.log.push((TAG_DEALLOCATE, previous, 0));
    }
}

type OwnerLift = fn(&mut TaskOwner, u32, &mut Recorder);

/// Runs one owner replacement over generated cases.
fn owner_tally(lift: OwnerLift) -> Tally {
    let mut rng = Rng(0x0FF1_CE00);
    let mut tally = Tally::default();
    rt::set_callee(1, stub_release as *const () as usize as u32);
    rt::set_callee(2, stub_deallocate as *const () as usize as u32);
    for case in 0..CASES {
        let (init, owner) = gen_window(&mut rng);
        // The new owner: the current one (no change), zero, an object, or random.
        let new_object = match rng.below(5) {
            0 => owner,
            1 => 0,
            2 => rt::relocated(OBJECT_A_VA),
            3 => rt::relocated(OBJECT_B_VA),
            _ => rng.next(),
        };

        let mut task = TaskOwner {
            object: word(&init, RECEIVER_VA + OWNER_OFFSET),
        };
        let mut recorder = Recorder { log: Vec::new() };
        lift(&mut task, new_object, &mut recorder);
        let mut expected = init.clone();
        put_word(&mut expected, RECEIVER_VA + OWNER_OFFSET, task.object);
        let lifted = Outcome {
            bytes: expected,
            log: recorder.log,
        };

        rt::write_window(&init);
        LOG.lock().unwrap().clear();
        fn_009c12b0::rw_009c12b0(rt::relocated(RECEIVER_VA), new_object);
        let rewritten = Outcome {
            bytes: rt::read_window(),
            log: std::mem::take(&mut *LOG.lock().unwrap()),
        };

        tally.cases += 1;
        if !rewritten.log.is_empty() {
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
fn owner_replacement_matches_rewrite() {
    let _guard = begin();
    let tally = owner_tally(|task, new_object, calls| task.replace_owner(new_object, calls));
    assert_clean("replace_owner_object", &tally);
}

#[test]
fn owner_wrong_versions_are_caught() {
    let _guard = begin();
    // The deallocator is not called.
    assert_caught(
        "owner, no deallocation",
        &owner_tally(|task, new_object, calls| {
            let previous = task.object;
            if previous != 0 && previous != new_object {
                calls.release(previous);
            }
            task.object = new_object;
        }),
    );
    // The release also runs when the owner is unchanged.
    assert_caught(
        "owner, release on equal",
        &owner_tally(|task, new_object, calls| {
            let previous = task.object;
            if previous != 0 {
                calls.release(previous);
                if previous != new_object {
                    calls.deallocate(previous);
                }
            }
            task.object = new_object;
        }),
    );
}
