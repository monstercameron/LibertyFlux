//! Differential cases of the owner's subject-flag update against its verified
//! 32-bit rewrite.
//!
//! The lifted update and the rewrite run on the same generated owner, subject,
//! draw and collaborator answers. The effects compared are:
//!
//! - the collaborator calls, in order, with their arguments as the rewrite's
//!   stubs read them (the component and owner handles, the enable word, the
//!   scratch tokens, which the stubs see only as non-null pointers, and the
//!   fixed zero arguments);
//! - the subject's flag word, bit for bit;
//! - every other byte of the owner and the subject, which the rewrite must
//!   leave as it found it.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses and calls
//! numbered stubs.

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{FlagCallees, FlagOwner, SubjectFlag};
use lf_taskshapediff::rewrites::fn_00ccb540;
use lf_taskshapediff::{Xorshift, rt};

/// Bytes planted for the owner: the largest offset read is the subject
/// pointer, and the task flags and mode lie below it.
const OWNER_BYTES: usize = 0x2C8;
/// Offset of the task-flag byte.
const TASK_FLAGS_OFFSET: usize = 0x268;
/// Offset of the mode byte.
const MODE_OFFSET: usize = 0x219;
/// Offset of the subject pointer.
const SUBJECT_OFFSET: usize = 0x2C4;
/// Bytes planted for the subject.
const SUBJECT_BYTES: usize = 0x220;
/// Offset of the subject's flag word.
const SUBJECT_FLAG_OFFSET: usize = 0x210;
/// Bytes planted for the task the component finds (its address is the answer).
const TASK_BYTES: usize = 0x10;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00CC;

/// Call tags in the log.
const TAG_DRAW: u32 = 1;
const TAG_ACCEPTS: u32 = 2;
const TAG_FIND: u32 = 3;
const TAG_FILL: u32 = 4;
const TAG_APPLY: u32 = 5;
const TAG_FALLBACK: u32 = 6;

/// The call log of the rewrite's stubs, in order.
static LOG: Mutex<Vec<Vec<u32>>> = Mutex::new(Vec::new());
/// The draw the first stub answers.
static DRAW: Mutex<u32> = Mutex::new(0);
/// The answer of the acceptance stub.
static ACCEPT: Mutex<u32> = Mutex::new(0);
/// The task the lookup stub answers, as an address.
static TASK: Mutex<u32> = Mutex::new(0);
/// Serialises the cases: the stubs and the callee table are process-wide.
static LOCK: Mutex<()> = Mutex::new(());

fn log(entry: Vec<u32>) {
    LOG.lock().unwrap().push(entry);
}

/// Whether a scratch pointer is non-null, the most the stub can say of it.
fn token(pointer: u32) -> u32 {
    u32::from(pointer != 0)
}

extern "cdecl" fn stub_draw() -> u32 {
    log(vec![TAG_DRAW]);
    *DRAW.lock().unwrap()
}

extern "thiscall" fn stub_accepts(component: u32, subject: u32) -> u32 {
    log(vec![TAG_ACCEPTS, component, subject]);
    *ACCEPT.lock().unwrap()
}

extern "thiscall" fn stub_find(component: u32, owner: u32, enable: u32) -> u32 {
    log(vec![TAG_FIND, component, owner, enable]);
    *TASK.lock().unwrap()
}

extern "thiscall" fn stub_fill(owner: u32, first: u32, second: u32, third: u32) -> u32 {
    log(vec![
        TAG_FILL,
        owner,
        token(first),
        token(second),
        token(third),
    ]);
    0
}

extern "thiscall" fn stub_apply(task: u32, scratch: u32, zero_a: u32, zero_b: u32) -> u32 {
    log(vec![TAG_APPLY, task, token(scratch), zero_a, zero_b]);
    0
}

extern "cdecl" fn stub_fallback(owner: u32) -> u32 {
    log(vec![TAG_FALLBACK, owner]);
    0
}

/// The collaborator the lifted update calls. It logs the same entries as the
/// rewrite's stubs, with the scratch tokens and the fixed zeros.
struct Recorder {
    draw: u32,
    accept: u32,
    task: u32,
    log: Vec<Vec<u32>>,
}

impl FlagCallees for Recorder {
    fn draw(&mut self) -> u32 {
        self.log.push(vec![TAG_DRAW]);
        self.draw
    }
    fn accepts(&mut self, component: u32, subject: u32) -> u32 {
        self.log.push(vec![TAG_ACCEPTS, component, subject]);
        self.accept
    }
    fn find_task(&mut self, component: u32, owner: u32, enable: u32) -> u32 {
        self.log.push(vec![TAG_FIND, component, owner, enable]);
        self.task
    }
    fn fill_scratch(&mut self, owner: u32) -> u32 {
        self.log.push(vec![TAG_FILL, owner, 1, 1, 1]);
        0
    }
    fn apply_task(&mut self, task: u32) -> u32 {
        self.log.push(vec![TAG_APPLY, task, 1, 0, 0]);
        0
    }
    fn fallback(&mut self, owner: u32) -> u32 {
        self.log.push(vec![TAG_FALLBACK, owner]);
        0
    }
}

/// One generated case and what the rewrite did with it.
struct Case {
    owner_address: u32,
    owner_before: Vec<u8>,
    owner_after: Vec<u8>,
    subject_before: Vec<u8>,
    subject_after: Vec<u8>,
    has_subject: bool,
    draw: u32,
    accept: u32,
    task_address: u32,
    calls: Vec<Vec<u32>>,
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

/// A task-flag byte, with the skip bit set about half the time.
fn flags_pick(rng: &mut Xorshift) -> u8 {
    let base = (rng.next_u32() & 0xFF) as u8;
    if rng.next_u32() % 2 == 0 {
        base | 0x08
    } else {
        base & !0x08
    }
}

/// Generates a case, plants it and runs the rewrite on it.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let mut owner = random_bytes(rng, OWNER_BYTES);
    owner[TASK_FLAGS_OFFSET] = flags_pick(rng);
    owner[MODE_OFFSET] = if rng.next_u32() % 2 == 0 {
        0
    } else {
        (rng.next_u32() & 0xFF) as u8
    };
    let mut subject = random_bytes(rng, SUBJECT_BYTES);
    set_word(&mut subject, SUBJECT_FLAG_OFFSET, rng.word());
    let has_subject = rng.next_u32() % 4 != 0;
    set_word(
        &mut owner,
        SUBJECT_OFFSET,
        if has_subject { address_of(&subject) } else { 0 },
    );
    let task = random_bytes(rng, TASK_BYTES);
    let task_address = address_of(&task);
    // A quarter of the draws sit on the threshold or one either side of it.
    let draw = match rng.next_u32() % 4 {
        0 => 0x3FFE,
        1 => 0x3FFF,
        2 => 0x4000,
        _ => rng.word(),
    };
    let accept = match rng.next_u32() % 4 {
        0 => 0,
        _ => rng.word(),
    };
    *DRAW.lock().unwrap() = draw;
    *ACCEPT.lock().unwrap() = accept;
    *TASK.lock().unwrap() = task_address;
    let owner_address = address_of(&owner);
    let owner_before = owner.clone();
    let subject_before = subject.clone();
    LOG.lock().unwrap().clear();
    fn_00ccb540::rw_00ccb540(owner_address);
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    Case {
        owner_address,
        owner_before,
        owner_after: owner,
        subject_before,
        subject_after: subject,
        has_subject,
        draw,
        accept,
        task_address,
        calls,
    }
}

/// Decodes the owner the lift takes from the case's planted owner.
fn owner_of(case: &Case) -> FlagOwner {
    let subject = if case.has_subject {
        Some(SubjectFlag {
            handle: word_at(&case.owner_before, SUBJECT_OFFSET),
            flag: word_at(&case.subject_before, SUBJECT_FLAG_OFFSET),
        })
    } else {
        None
    };
    FlagOwner {
        handle: case.owner_address,
        task_flags: case.owner_before[TASK_FLAGS_OFFSET],
        subject,
        mode: case.owner_before[MODE_OFFSET],
    }
}

/// Checks the rewrite's own effects: only the subject flag word changes, and
/// only when a subject is present.
fn check_rewrite_effects(case: &Case) {
    for offset in 0..OWNER_BYTES {
        assert_eq!(
            case.owner_before[offset], case.owner_after[offset],
            "owner byte {offset:#x} is unchanged"
        );
    }
    for offset in 0..SUBJECT_BYTES {
        let flag_word = (SUBJECT_FLAG_OFFSET..SUBJECT_FLAG_OFFSET + 4).contains(&offset);
        if !flag_word {
            assert_eq!(
                case.subject_before[offset], case.subject_after[offset],
                "subject byte {offset:#x} is unchanged"
            );
        }
    }
}

/// Runs the lift on a case and returns its subject flag word and its calls.
fn run_lift(case: &Case) -> (u32, Vec<Vec<u32>>) {
    let mut owner = owner_of(case);
    let mut recorder = Recorder {
        draw: case.draw,
        accept: case.accept,
        task: case.task_address,
        log: Vec::new(),
    };
    owner.update(&mut recorder);
    let flag = owner.subject.map_or(0, |subject| subject.flag);
    (flag, recorder.log)
}

/// Asserts that the lift matches the rewrite on one case.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    let (flag, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the collaborator calls match in order");
    if case.has_subject {
        assert_eq!(
            flag,
            word_at(&case.subject_after, SUBJECT_FLAG_OFFSET),
            "the subject flag matches bit for bit"
        );
    }
}

/// The deviations of a wrong version.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The skip bit is ignored.
    IgnoreSkip,
    /// The draw threshold is one less.
    ThresholdOff,
    /// The set and clear are swapped.
    SwappedBit,
    /// The enable word is the mode itself, not whether it is zero.
    RawMode,
    /// The fallback runs when the component accepts, and the task when not.
    SwappedBranches,
    /// The scratch fill is left out.
    NoFill,
}

/// The wrong version of the update on the same inputs. Returns the subject
/// flag word and the calls.
fn run_wrong(kind: Wrong, case: &Case) -> (u32, Vec<Vec<u32>>) {
    let owner = owner_of(case);
    let mut flag = owner.subject.map_or(0, |subject| subject.flag);
    let mut calls: Vec<Vec<u32>> = Vec::new();
    let skip = if matches!(kind, Wrong::IgnoreSkip) {
        0
    } else {
        owner.task_flags & 0x08
    };
    if skip == 0 && owner.subject.is_some() {
        calls.push(vec![TAG_DRAW]);
        let threshold = if matches!(kind, Wrong::ThresholdOff) {
            0x3FFE
        } else {
            0x3FFF
        };
        let desired = if (case.draw as i32) < threshold {
            if matches!(kind, Wrong::SwappedBit) {
                0
            } else {
                0x0200_0000
            }
        } else if matches!(kind, Wrong::SwappedBit) {
            0x0200_0000
        } else {
            0
        };
        flag ^= (flag ^ desired) & 0x0200_0000;
    }
    let component = owner.handle.wrapping_add(0x2B0);
    let subject_handle = owner.subject.map_or(0, |subject| subject.handle);
    calls.push(vec![TAG_ACCEPTS, component, subject_handle]);
    let accepted = (case.accept & 0xFF) != 0;
    let take_task = if matches!(kind, Wrong::SwappedBranches) {
        !accepted
    } else {
        accepted
    };
    if take_task {
        let enable = if matches!(kind, Wrong::RawMode) {
            u32::from(owner.mode)
        } else {
            u32::from(owner.mode == 0)
        };
        calls.push(vec![TAG_FIND, component, owner.handle, enable]);
        if !matches!(kind, Wrong::NoFill) {
            calls.push(vec![TAG_FILL, owner.handle, 1, 1, 1]);
        }
        calls.push(vec![TAG_APPLY, case.task_address, 1, 0, 0]);
    } else {
        calls.push(vec![TAG_FALLBACK, owner.handle]);
    }
    (flag, calls)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (flag, calls) = run_wrong(kind, &case);
        let expected = if case.has_subject {
            word_at(&case.subject_after, SUBJECT_FLAG_OFFSET)
        } else {
            0
        };
        let flag_same = !case.has_subject || flag == expected;
        if calls != case.calls || !flag_same {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

/// Plants the six stubs in the rewrite's callee slots.
fn plant_stubs() {
    rt::set_callee(1, stub_draw as *const () as usize as u32);
    rt::set_callee(2, stub_accepts as *const () as usize as u32);
    rt::set_callee(3, stub_find as *const () as usize as u32);
    rt::set_callee(4, stub_fill as *const () as usize as u32);
    rt::set_callee(5, stub_apply as *const () as usize as u32);
    rt::set_callee(6, stub_fallback as *const () as usize as u32);
}

#[test]
fn flag_update_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let mut rng = Xorshift::new(SEED);
    let mut task_taken = 0;
    let mut subject_changed = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        if case.calls.iter().any(|call| call[0] == TAG_APPLY) {
            task_taken += 1;
        }
        if case.has_subject
            && word_at(&case.subject_after, SUBJECT_FLAG_OFFSET)
                != word_at(&case.subject_before, SUBJECT_FLAG_OFFSET)
        {
            subject_changed += 1;
        }
    }
    eprintln!("cases {CASES}: task applied {task_taken}, subject flag changed {subject_changed}");
}

#[test]
fn flag_update_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    for kind in [
        Wrong::IgnoreSkip,
        Wrong::ThresholdOff,
        Wrong::SwappedBit,
        Wrong::RawMode,
        Wrong::SwappedBranches,
        Wrong::NoFill,
    ] {
        assert!(
            wrong_mismatches(kind) > 0,
            "wrong version {kind:?} is caught"
        );
    }
}
