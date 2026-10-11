//! Differential cases of the lifted task manager slot arrays against their
//! verified 32-bit rewrites.
//!
//! The manager object sits in the planted window. Each lifted routine runs
//! on the same generated slot words as its rewrite, and the effects compared
//! are every byte of the window, the task interface calls in order with
//! their arguments, and the return value where the routine has one. Each
//! routine also has deliberately wrong versions, which must be caught.
//!
//! Runs on the 32-bit target only. The test program holds one lock, so its
//! cases never interleave: the planted windows and callee stubs are shared.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_eventdiff::rewrites::{fn_00b80af0, fn_00b80b20, fn_00b80d50, fn_00b80dd0, fn_00b81340};
use lf_eventdiff::rt;
use lf_peds_tasks::ped_task::{
    FIRST_BOMB_KIND, FIRST_BOMB_SLOT, ManagerCallees, PRIMARY_COUNT, SECOND_BOMB_KIND,
    SECOND_BOMB_SLOT, SECONDARY_COUNT, TaskSlots,
};

/// File address of the manager object inside the planted window.
const MANAGER_VA: u32 = 0x0129_4A00;
/// Generated cases per routine.
const CASES: u32 = 3000;

/// Interface-call tags in the call log. Both sides append the same tags.
const TAG_REQUEST: u32 = 1;
const TAG_REQUEST_ARGS: u32 = 2;
const TAG_PREPARE: u32 = 3;
const TAG_REFRESH: u32 = 4;
const TAG_FINAL: u32 = 5;

/// One logged call: tag, first argument, second argument.
type Call = (u32, u32, u32);

/// The values the planted stubs return.
#[derive(Clone, Copy)]
struct Plant {
    /// Answer of the final update method.
    value: u32,
}

/// Serialises the cases: the window and stubs are process-wide.
static LOCK: Mutex<()> = Mutex::new(());
/// Calls the planted stubs have received, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// The answers the planted stubs return.
static PLANT: Mutex<Plant> = Mutex::new(Plant { value: 0 });

fn planted() -> Plant {
    *PLANT.lock().unwrap()
}

fn log_call(call: Call) {
    LOG.lock().unwrap().push(call);
}

// Stubs the rewrites call through their numbered callee slots, with the
// thiscall convention and the argument shape of each slot.

extern "thiscall" fn stub_request(this: u32, first: u32, kind: u32, last: u32) -> u32 {
    log_call((TAG_REQUEST, this, kind));
    log_call((TAG_REQUEST_ARGS, first, last));
    0
}

extern "thiscall" fn stub_prepare(this: u32) -> u32 {
    log_call((TAG_PREPARE, this, 0));
    0
}

extern "thiscall" fn stub_refresh(this: u32) -> u32 {
    log_call((TAG_REFRESH, this, 0));
    0
}

extern "thiscall" fn stub_final(this: u32) -> u32 {
    log_call((TAG_FINAL, this, 0));
    planted().value
}

fn plant_stubs(slots: &[(u32, u32)]) {
    for &(id, addr) in slots {
        rt::set_callee(id, addr);
    }
}

fn word(bytes: &[u8], va: u32) -> u32 {
    let o = (va - rt::WINDOW_VA) as usize;
    u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
}

fn put_word(bytes: &mut [u8], va: u32, value: u32) {
    let o = (va - rt::WINDOW_VA) as usize;
    bytes[o..o + 4].copy_from_slice(&value.to_le_bytes());
}

/// File address of primary slot `index`.
fn primary_va(index: usize) -> u32 {
    MANAGER_VA + 4 * index as u32
}

/// File address of secondary slot `index`, which follows the primary array.
fn secondary_va(index: usize) -> u32 {
    MANAGER_VA + 4 * (PRIMARY_COUNT + index) as u32
}

/// Reads the slot arrays out of the window bytes.
fn decode(bytes: &[u8]) -> TaskSlots {
    let mut slots = TaskSlots::default();
    for (index, slot) in slots.primary.iter_mut().enumerate() {
        *slot = word(bytes, primary_va(index));
    }
    for (index, slot) in slots.secondary.iter_mut().enumerate() {
        *slot = word(bytes, secondary_va(index));
    }
    slots
}

/// Writes the slot arrays into the window bytes. The rest of the window
/// keeps its bytes.
fn encode(bytes: &mut [u8], slots: &TaskSlots) {
    for (index, slot) in slots.primary.iter().enumerate() {
        put_word(bytes, primary_va(index), *slot);
    }
    for (index, slot) in slots.secondary.iter().enumerate() {
        put_word(bytes, secondary_va(index), *slot);
    }
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

/// One slot word: zero most often, a few small handles (so needles collide),
/// and random values.
fn gen_slot(rng: &mut Rng) -> u32 {
    match rng.below(8) {
        0 | 1 | 2 => 0,
        3 => 0xABCD_0001,
        4 => 0xABCD_0002,
        5 => 0xABCD_0003,
        _ => rng.next(),
    }
}

fn gen_slots(rng: &mut Rng) -> TaskSlots {
    let mut slots = TaskSlots::default();
    for slot in &mut slots.primary {
        *slot = gen_slot(rng);
    }
    for slot in &mut slots.secondary {
        *slot = gen_slot(rng);
    }
    slots
}

/// The window bytes of one case: random bytes, then the generated slots.
fn gen_window(rng: &mut Rng) -> Vec<u8> {
    let mut bytes: Vec<u8> = (0..rt::WINDOW_LEN).map(|_| rng.next() as u8).collect();
    encode(&mut bytes, &gen_slots(rng));
    bytes
}

/// The observable effects of one run.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    bytes: Vec<u8>,
    log: Vec<Call>,
    ret: u32,
}

/// Mismatch counts over one run of generated cases.
#[derive(Default)]
struct Tally {
    cases: u32,
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
        let same_ret = !ret_checked || lift.ret == rewrite.ret;
        if lift.bytes == rewrite.bytes && lift.log == rewrite.log && same_ret {
            return;
        }
        self.mismatches += 1;
        if self.first.is_none() {
            self.first = Some(format!(
                "case {case}: calls lift {:?} rewrite {:?}; ret lift {:#x} rewrite {:#x}",
                lift.log, rewrite.log, lift.ret, rewrite.ret
            ));
        }
    }
}

/// Runs one rewrite on the planted window and returns what it did.
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

fn gen_plant(rng: &mut Rng) -> Plant {
    Plant {
        value: match rng.below(4) {
            0 => 0,
            1 => 1,
            _ => rng.next(),
        },
    }
}

/// Plants a fresh window and auxiliary window for one test.
fn fresh_windows() {
    let window: &'static mut [u32] =
        Box::leak(vec![0_u32; rt::WINDOW_LEN as usize / 4].into_boxed_slice());
    rt::set_window(window.as_ptr() as u32);
    let aux: &'static mut [u32] = Box::leak(vec![0_u32; rt::AUX_LEN as usize / 4].into_boxed_slice());
    rt::set_aux(aux.as_ptr() as u32);
}

/// The lifted side's recorder. It answers from the plant and logs each
/// interface call with the manager handle the rewrite would pass.
struct Recorder {
    plant: Plant,
    log: Vec<Call>,
}

impl ManagerCallees for Recorder {
    fn request_task(&mut self, kind: u32) {
        let this = rt::relocated(MANAGER_VA);
        self.log.push((TAG_REQUEST, this, kind));
        self.log.push((TAG_REQUEST_ARGS, 0, 0));
    }

    fn prepare_tasks(&mut self) {
        self.log.push((TAG_PREPARE, rt::relocated(MANAGER_VA), 0));
    }

    fn refresh_tasks(&mut self) {
        self.log.push((TAG_REFRESH, rt::relocated(MANAGER_VA), 0));
    }

    fn final_update(&mut self) -> u32 {
        self.log.push((TAG_FINAL, rt::relocated(MANAGER_VA), 0));
        self.plant.value
    }
}

type RequestLift = fn(&TaskSlots, &mut Recorder);
type ReturnLift = fn(&TaskSlots, &mut Recorder) -> u32;

/// Runs one request routine (no return value) over generated slots.
fn request_tally(lift: RequestLift, rewrite_call: fn()) -> Tally {
    let mut rng = Rng(0x0BAD_B0B0);
    let mut tally = Tally::default();
    plant_stubs(&[(1, stub_request as *const () as usize as u32)]);
    for case in 0..CASES {
        let init = gen_window(&mut rng);
        let plant = gen_plant(&mut rng);
        let slots = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        lift(&slots, &mut recorder);
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

/// Runs the sequence routine, which returns the final update's answer.
fn sequence_tally(lift: ReturnLift) -> Tally {
    let mut rng = Rng(0x5EED_0042);
    let mut tally = Tally::default();
    plant_stubs(&[
        (1, stub_prepare as *const () as usize as u32),
        (2, stub_refresh as *const () as usize as u32),
        (3, stub_final as *const () as usize as u32),
    ]);
    for case in 0..CASES {
        let init = gen_window(&mut rng);
        let plant = gen_plant(&mut rng);
        let slots = decode(&init);
        let mut recorder = Recorder {
            plant,
            log: Vec::new(),
        };
        let ret = lift(&slots, &mut recorder);
        let lifted = Outcome {
            bytes: init.clone(),
            log: recorder.log,
            ret,
        };
        let rewrite = run_rewrite(&init, plant, || fn_00b81340::rw_b81340(rt::relocated(MANAGER_VA)));
        tally.judge(case, &lifted, &rewrite, true);
    }
    tally
}

/// Runs a pure reading routine: the answer is compared, and the window must
/// come back unchanged.
fn reader_tally<F: Fn(&TaskSlots, u32) -> u32>(lift: F, rewrite: fn(u32) -> u32, needle_of: bool) -> Tally {
    let mut rng = Rng(0x7A7A_9191);
    let mut tally = Tally::default();
    for case in 0..CASES {
        let init = gen_window(&mut rng);
        let plant = gen_plant(&mut rng);
        let slots = decode(&init);
        // The needle: a stored secondary word, zero, or a random value.
        let needle = if needle_of {
            match rng.below(4) {
                0 => 0,
                1 | 2 => slots.secondary[rng.below(SECONDARY_COUNT as u32) as usize],
                _ => rng.next(),
            }
        } else {
            0
        };
        let lifted = Outcome {
            bytes: init.clone(),
            log: Vec::new(),
            ret: lift(&slots, needle),
        };
        let rewrite_out = run_rewrite(&init, plant, || rewrite(needle));
        tally.judge(case, &lifted, &rewrite_out, true);
    }
    tally
}

fn first_occupied_tally(lift: fn(&TaskSlots) -> usize) -> Tally {
    let lifted = |slots: &TaskSlots, _needle: u32| lift(slots) as u32;
    reader_tally(lifted, |_| fn_00b80d50::rw_b80d50(rt::relocated(MANAGER_VA)), false)
}

fn contains_tally(lift: fn(&TaskSlots, u32) -> bool) -> Tally {
    let lifted = |slots: &TaskSlots, needle: u32| u32::from(lift(slots, needle));
    reader_tally(
        lifted,
        |needle| u32::from(fn_00b80dd0::rw_b80dd0(rt::relocated(MANAGER_VA), needle)),
        true,
    )
}

/// Holds the lock and plants fresh windows for one test. The rewrites read
/// the manager at the window address, so the window is planted first.
fn begin() -> std::sync::MutexGuard<'static, ()> {
    let guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    fresh_windows();
    guard
}

fn assert_clean(name: &str, tally: &Tally) {
    eprintln!(
        "{name}: {} cases, {} with interface calls, {} mismatches",
        tally.cases, tally.with_calls, tally.mismatches
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
fn first_occupied_matches_rewrite() {
    let _guard = begin();
    assert_clean(
        "first_occupied",
        &first_occupied_tally(TaskSlots::first_occupied),
    );
}

#[test]
fn first_occupied_wrong_version_is_caught() {
    let _guard = begin();
    // The scan stops at the last slot instead of returning the count.
    assert_caught(
        "first_occupied, last slot",
        &first_occupied_tally(|slots| {
            slots
                .primary
                .iter()
                .rposition(|&task| task != 0)
                .unwrap_or(PRIMARY_COUNT)
        }),
    );
}

#[test]
fn contains_matches_rewrite() {
    let _guard = begin();
    assert_clean("contains", &contains_tally(TaskSlots::contains));
}

#[test]
fn contains_wrong_versions_are_caught() {
    let _guard = begin();
    // Only the first five secondary words are compared.
    assert_caught(
        "contains, five entries",
        &contains_tally(|slots, needle| slots.secondary[..5].contains(&needle)),
    );
    // A zero needle matches an empty slot in the wrong version only.
    assert_caught(
        "contains, zero matches empty",
        &contains_tally(|slots, needle| needle == 0 || slots.secondary.contains(&needle)),
    );
}

#[test]
fn request_bomb_matches_rewrite() {
    let _guard = begin();
    let tally = request_tally(
        |slots, calls| slots.request_bomb_task(calls),
        || {
            fn_00b80af0::rw_b80af0(rt::relocated(MANAGER_VA));
        },
    );
    assert_clean("request_bomb_task", &tally);
}

#[test]
fn request_bomb_wrong_versions_are_caught() {
    let _guard = begin();
    // The second bomb slot is tested first.
    assert_caught(
        "request_bomb, second slot first",
        &request_tally(
            |slots, calls| {
                if slots.primary[SECOND_BOMB_SLOT] != 0 {
                    calls.request_task(SECOND_BOMB_KIND);
                } else if slots.primary[FIRST_BOMB_SLOT] != 0 {
                    calls.request_task(FIRST_BOMB_KIND);
                }
            },
            || {
                fn_00b80af0::rw_b80af0(rt::relocated(MANAGER_VA));
            },
        ),
    );
    // Both kinds are requested when both slots are set.
    assert_caught(
        "request_bomb, both kinds",
        &request_tally(
            |slots, calls| {
                if slots.primary[FIRST_BOMB_SLOT] != 0 {
                    calls.request_task(FIRST_BOMB_KIND);
                }
                if slots.primary[SECOND_BOMB_SLOT] != 0 {
                    calls.request_task(SECOND_BOMB_KIND);
                }
            },
            || {
                fn_00b80af0::rw_b80af0(rt::relocated(MANAGER_VA));
            },
        ),
    );
}

#[test]
fn request_first_bomb_matches_rewrite() {
    let _guard = begin();
    let tally = request_tally(
        |slots, calls| slots.request_first_bomb_task(calls),
        || {
            fn_00b80b20::rw_b80b20(rt::relocated(MANAGER_VA));
        },
    );
    assert_clean("request_first_bomb_task", &tally);
}

#[test]
fn request_first_bomb_wrong_version_is_caught() {
    let _guard = begin();
    // The wrong version requests the second kind for the first slot.
    assert_caught(
        "request_first_bomb, wrong kind",
        &request_tally(
            |slots, calls| {
                if slots.primary[FIRST_BOMB_SLOT] != 0 {
                    calls.request_task(SECOND_BOMB_KIND);
                }
            },
            || {
                fn_00b80b20::rw_b80b20(rt::relocated(MANAGER_VA));
            },
        ),
    );
}

#[test]
fn update_sequence_matches_rewrite() {
    let _guard = begin();
    assert_clean(
        "update_sequence",
        &sequence_tally(|slots, calls| slots.update_sequence(calls)),
    );
}

#[test]
fn update_sequence_wrong_versions_are_caught() {
    let _guard = begin();
    // The refresh is called before the preparation.
    assert_caught(
        "update_sequence, order",
        &sequence_tally(|_slots, calls| {
            calls.refresh_tasks();
            calls.prepare_tasks();
            calls.final_update()
        }),
    );
    // The final answer is dropped.
    assert_caught(
        "update_sequence, answer dropped",
        &sequence_tally(|_slots, calls| {
            calls.prepare_tasks();
            calls.refresh_tasks();
            calls.final_update();
            0
        }),
    );
}
