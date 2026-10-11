//! Differential cases of the task block initialiser against its verified
//! 32-bit rewrite.
//!
//! The lifted initialiser and the rewrite run on the same generated blocks,
//! draws, tick values and scale factors. The effects compared are:
//!
//! - the callee calls, in order, with their arguments (the rewrite's stubs
//!   log them; the lifted side's collaborator logs the same entries);
//! - the words the initialiser owns, decoded from the block after the
//!   rewrite and compared with the owned value the lift returns;
//! - every byte of the block the initialiser does not own, which the
//!   rewrite must leave as it found it (the sub-object bytes belong to the
//!   callees, and the stubs write nothing).
//!
//! The rewrite's return value is compared with the block's handle. The lift
//! returns the owned words, not that handle; the caller already holds it.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses and calls
//! numbered stubs.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{
    BLOCK_BYTES, BlockCallees, COUNTDOWN_BASE, SELECTOR_NONE, ScaleFactors, TaskBlock,
    truncate_to_i32,
};
use lf_taskshapediff::rewrites::fn_00ca1d70;
use lf_taskshapediff::{Xorshift, rt};

/// File address of the shared tick counter the initialiser copies.
const TICK_VA: u32 = 0x0117_35B4;
/// File address of the first scale factor (the reciprocal of 2^15).
const INVERSE_VA: u32 = 0x00FE_8680;
/// File address of the second scale factor (the negative of 500).
const FACTOR_VA: u32 = 0x00EA_EBE4;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00CA;
/// Words in the block.
const BLOCK_WORDS: usize = BLOCK_BYTES as usize / 4;

/// Callee tags in the call log. The rewrite's stubs and the lifted
/// collaborator append the same tags with the same arguments.
const TAG_CONSTRUCT_FIRST: u32 = 0;
const TAG_CONSTRUCT_MIDDLE: u32 = 1;
const TAG_CONSTRUCT_TAIL: u32 = 2;
const TAG_SETUP_FIRST: u32 = 3;
const TAG_DRAW: u32 = 4;
const TAG_SETUP_SECOND: u32 = 5;
const TAG_SETUP_THIRD: u32 = 6;

/// One logged call: tag, first argument, second argument.
type Call = (u32, u32, u32);

/// The call log of the rewrite's stubs, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// The value the draw stub returns.
static DRAW: Mutex<u32> = Mutex::new(0);
/// Serialises the cases: the stubs, the callee table and the globals are
/// process-wide.
static LOCK: Mutex<()> = Mutex::new(());

fn log_call(call: Call) {
    LOG.lock().unwrap().push(call);
}

// The stubs the rewrite's numbered callee slots reach. Each has the
// convention and argument shape of the slot it is planted in.

extern "thiscall" fn stub_construct_first(block: u32) -> u32 {
    log_call((TAG_CONSTRUCT_FIRST, block, 0));
    0
}

extern "thiscall" fn stub_construct_middle(sub: u32) -> u32 {
    log_call((TAG_CONSTRUCT_MIDDLE, sub, 0));
    0
}

extern "thiscall" fn stub_construct_tail(sub: u32) -> u32 {
    log_call((TAG_CONSTRUCT_TAIL, sub, 0));
    0
}

extern "thiscall" fn stub_setup_first(block: u32) -> u32 {
    log_call((TAG_SETUP_FIRST, block, 0));
    0
}

extern "cdecl" fn stub_draw() -> u32 {
    log_call((TAG_DRAW, 0, 0));
    *DRAW.lock().unwrap()
}

extern "thiscall" fn stub_setup_second(block: u32, arg: u32) -> u32 {
    log_call((TAG_SETUP_SECOND, block, arg));
    0
}

extern "thiscall" fn stub_setup_third(block: u32, arg: u32) -> u32 {
    log_call((TAG_SETUP_THIRD, block, arg));
    0
}

/// Plants the seven stub addresses in the rewrite's callee slots.
fn plant_stubs() {
    rt::set_callee(0, stub_construct_first as *const () as usize as u32);
    rt::set_callee(1, stub_construct_middle as *const () as usize as u32);
    rt::set_callee(2, stub_construct_tail as *const () as usize as u32);
    rt::set_callee(3, stub_setup_first as *const () as usize as u32);
    rt::set_callee(4, stub_draw as *const () as usize as u32);
    rt::set_callee(5, stub_setup_second as *const () as usize as u32);
    rt::set_callee(6, stub_setup_third as *const () as usize as u32);
}

/// The collaborator the lifted initialiser calls. It logs the same entries
/// as the rewrite's stubs and answers the same draw and tick values.
struct Recorder {
    draw: u32,
    tick: u32,
    log: Vec<Call>,
}

impl BlockCallees for Recorder {
    fn construct_first(&mut self, block: u32) {
        self.log.push((TAG_CONSTRUCT_FIRST, block, 0));
    }
    fn construct_middle(&mut self, sub: u32) {
        self.log.push((TAG_CONSTRUCT_MIDDLE, sub, 0));
    }
    fn construct_tail(&mut self, sub: u32) {
        self.log.push((TAG_CONSTRUCT_TAIL, sub, 0));
    }
    fn setup_first(&mut self, block: u32) {
        self.log.push((TAG_SETUP_FIRST, block, 0));
    }
    fn draw(&mut self) -> u32 {
        self.log.push((TAG_DRAW, 0, 0));
        self.draw
    }
    fn setup_second(&mut self, block: u32, arg: u32) {
        self.log.push((TAG_SETUP_SECOND, block, arg));
    }
    fn setup_third(&mut self, block: u32, arg: u32) {
        self.log.push((TAG_SETUP_THIRD, block, arg));
    }
    fn tick(&self) -> u32 {
        self.tick
    }
}

/// One generated case, and what the rewrite did with it.
struct Case {
    /// The block's words before the call.
    before: Vec<u32>,
    /// The block's words after the rewrite ran on it.
    after: Vec<u32>,
    /// The rewrite's return value.
    returned: u32,
    /// The handle the rewrite was given (the block's address).
    handle: u32,
    /// The draw the stub answered.
    draw: u32,
    /// The tick word the global held.
    tick: u32,
    /// The two scale factors the global words held.
    scale: ScaleFactors,
    /// The calls the rewrite made, in order.
    calls: Vec<Call>,
}

/// Bytes of a word array, little-endian, in memory order.
fn bytes_of(words: &[u32]) -> Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

/// Reads one word at a byte offset from a word array.
fn word_at(words: &[u32], offset: u32) -> u32 {
    words[offset as usize / 4]
}

/// Decodes the owned words of the block from its memory words, by the
/// offsets the lifted type names.
fn decode(words: &[u32]) -> TaskBlock {
    let bytes = bytes_of(words);
    let mut params = [0_u32; 6];
    for (i, slot) in params.iter_mut().enumerate() {
        *slot = word_at(words, 0x12C + 4 * i as u32);
    }
    TaskBlock {
        params,
        unit_scale: f32::from_bits(word_at(words, 0x144)),
        unit_scale_tail: word_at(words, 0x148),
        tick_copy: word_at(words, 0x14C),
        countdown: word_at(words, 0x150),
        ready: u16::from_le_bytes([bytes[0x154], bytes[0x155]]),
        result: word_at(words, 0x158),
        ctrl_a: word_at(words, 0x170),
        selector: word_at(words, 0x174),
        ctrl_b: word_at(words, 0x178),
        flag_b: bytes[0x17C],
        tail: word_at(words, 0x180),
    }
}

/// Whether a byte offset of the block is one of the words the lift owns.
fn owned(offset: usize) -> bool {
    const RANGES: [(usize, usize); 11] = [
        (0x12C, 0x144),
        (0x144, 0x14C),
        (0x14C, 0x150),
        (0x150, 0x154),
        (0x154, 0x156),
        (0x158, 0x15C),
        (0x170, 0x174),
        (0x174, 0x178),
        (0x178, 0x17C),
        (0x17C, 0x17D),
        (0x180, 0x184),
    ];
    RANGES.iter().any(|&(lo, hi)| offset >= lo && offset < hi)
}

/// Generates a case and runs the rewrite on it. The block is random words,
/// a third of them boundary values; the draw, the tick and the two scale
/// factors are random words, with boundary values in the mix.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let mut block: Vec<u32> = (0..BLOCK_WORDS).map(|_| rng.word()).collect();
    let before = block.clone();
    let draw = rng.word();
    let tick = rng.word();
    let inverse = rng.word();
    let factor = rng.word();
    // The globals live in one planted array; each file address maps to one
    // word of it.
    let mut globals = vec![tick, inverse, factor];
    let base = globals.as_mut_ptr() as usize as u32;
    rt::set_extra(TICK_VA, base);
    rt::set_extra(INVERSE_VA, base + 4);
    rt::set_extra(FACTOR_VA, base + 8);
    *DRAW.lock().unwrap() = draw;
    LOG.lock().unwrap().clear();
    let handle = block.as_mut_ptr() as usize as u32;
    let returned = fn_00ca1d70::rw_00ca1d70(handle);
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    Case {
        before,
        after: block,
        returned,
        handle,
        draw,
        tick,
        scale: ScaleFactors {
            inverse_range: f32::from_bits(inverse),
            factor: f32::from_bits(factor),
        },
        calls,
    }
}

/// Checks that the rewrite changed no byte the lift does not own, and that
/// its return value is the handle.
fn check_rewrite_effects(case: &Case) {
    assert_eq!(
        case.returned, case.handle,
        "the rewrite returns the block handle"
    );
    let before = bytes_of(&case.before);
    let after = bytes_of(&case.after);
    for offset in 0..BLOCK_BYTES as usize {
        if !owned(offset) {
            assert_eq!(
                before[offset], after[offset],
                "unowned byte {offset:#x} unchanged"
            );
        }
    }
}

/// Runs the lifted initialiser on the case's inputs.
fn run_lift(case: &Case) -> (TaskBlock, Vec<Call>) {
    let mut recorder = Recorder {
        draw: case.draw,
        tick: case.tick,
        log: Vec::new(),
    };
    let block = TaskBlock::initialise(case.handle, &mut recorder, case.scale);
    (block, recorder.log)
}

/// Asserts that the lifted initialiser matches the rewrite on one case:
/// the calls in order, the owned words, and the rewrite's own effects.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    let (lifted, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the callee calls match in order");
    assert_eq!(lifted, decode(&case.after), "the owned words match");
}

/// Ordering and arithmetic deviations of a wrong version. Each deviates from
/// the lift in one place and is otherwise the same routine.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The countdown base is one less.
    CountdownBase,
    /// The selector is cleared rather than set to none.
    SelectorZero,
    /// The tail sub-object is built before the middle one.
    TailFirst,
    /// The draw keeps only fifteen bits.
    DrawFifteenBits,
    /// The truncation gives the processor's indefinite value (the most negative
    /// integer) for a not-a-number and outside the range, where the verified
    /// rewrite gives zero or saturates.
    IndefiniteTruncation,
    /// The ready flag is left clear.
    ReadyClear,
}

/// The wrong version of the initialiser: the same calls and the same words,
/// with one deviation.
fn run_wrong(kind: Wrong, case: &Case) -> (TaskBlock, Vec<Call>) {
    let mut recorder = Recorder {
        draw: case.draw,
        tick: case.tick,
        log: Vec::new(),
    };
    let handle = case.handle;
    recorder.construct_first(handle);
    if matches!(kind, Wrong::TailFirst) {
        recorder.construct_tail(handle.wrapping_add(0x110));
        recorder.construct_middle(handle.wrapping_add(0x80));
    } else {
        recorder.construct_middle(handle.wrapping_add(0x80));
        recorder.construct_tail(handle.wrapping_add(0x110));
    }
    recorder.setup_first(handle);
    let draw = recorder.draw();
    let tick_copy = recorder.tick();
    let mask = if matches!(kind, Wrong::DrawFifteenBits) {
        0x7FFF
    } else {
        0xFFFF
    };
    let scaled = ((draw & mask) as f32 * case.scale.inverse_range) * case.scale.factor;
    let in_range = scaled >= -2_147_483_648.0 && scaled < 2_147_483_648.0;
    let take = if matches!(kind, Wrong::IndefiniteTruncation) && !in_range {
        i32::MIN
    } else {
        truncate_to_i32(scaled)
    };
    let base = if matches!(kind, Wrong::CountdownBase) {
        COUNTDOWN_BASE - 1
    } else {
        COUNTDOWN_BASE
    };
    recorder.setup_second(handle, 0);
    recorder.setup_third(handle, 0);
    let block = TaskBlock {
        params: [0; 6],
        unit_scale: 1.0,
        unit_scale_tail: 0,
        tick_copy,
        countdown: base.wrapping_sub(take as u32),
        ready: if matches!(kind, Wrong::ReadyClear) {
            0
        } else {
            1
        },
        result: 0,
        ctrl_a: 0,
        selector: if matches!(kind, Wrong::SelectorZero) {
            0
        } else {
            SELECTOR_NONE
        },
        ctrl_b: 0,
        flag_b: 0,
        tail: 0,
    };
    (block, recorder.log)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (block, calls) = run_wrong(kind, &case);
        if calls != case.calls || block != decode(&case.after) {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

#[test]
fn block_init_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let mut rng = Xorshift::new(SEED);
    let mut draws_inside_range = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        let countdown = decode(&case.after).countdown;
        if (500..=1499).contains(&countdown) {
            draws_inside_range += 1;
        }
    }
    eprintln!("countdowns in the nominal range: {draws_inside_range} of {CASES}");
}

#[test]
fn block_init_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    for kind in [
        Wrong::CountdownBase,
        Wrong::SelectorZero,
        Wrong::TailFirst,
        Wrong::DrawFifteenBits,
        Wrong::IndefiniteTruncation,
        Wrong::ReadyClear,
    ] {
        assert!(
            wrong_mismatches(kind) > 0,
            "wrong version {kind:?} is caught"
        );
    }
}
