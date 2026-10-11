//! Differential cases of the turn tracker against its verified 32-bit
//! rewrite.
//!
//! The lifted advance and the rewrite run on the same generated task
//! object, target point, parameter block, inner direction and gate. The
//! effects compared are:
//!
//! - the trigonometric calls, in order, with their argument bits (the
//!   rewrite's stubs and the lifted collaborator log the same entries);
//! - the rate word and the flag byte of the task, and the three target
//!   words, compared bit for bit with the values the lift returns;
//! - every other byte of the task object, which the rewrite must leave as it
//!   found it.
//!
//! The rewrite's return value is compared with the target's handle.
//!
//! Each routine also has deliberately wrong versions. Each must differ from
//! its rewrite on at least one generated case.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses and calls
//! numbered stubs.

#![cfg(target_arch = "x86")]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::{
    DIR_SCALE, RATE_GATE, RATE_STEP, TurnParams, TurnTask, TurnTrig, Vec3,
};
use lf_taskshapediff::rewrites::fn_00a218a0;
use lf_taskshapediff::{Xorshift, rt};

/// Bytes planted for the task object: the largest offset read is the flag.
const OBJECT_BYTES: usize = 0x400;
/// Offset of the rate word in the task object.
const RATE_OFFSET: usize = 0x2C4;
/// Offset of the flag byte in the task object.
const FLAG_OFFSET: usize = 0x38C;
/// Bytes planted for the parameter block.
const PARAM_BYTES: usize = 0x40;
/// Offset of the angle word in the parameter block.
const ANGLE_OFFSET: usize = 0x1C;
/// Offset of the inner-direction pointer in the parameter block.
const INNER_OFFSET: usize = 0x20;
/// Bytes planted for the inner direction block.
const INNER_BYTES: usize = 0x20;
/// Offsets of the direction components in the inner block.
const INNER_X: usize = 0x10;
const INNER_Y: usize = 0x14;
const INNER_Z: usize = 0x18;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Seed of the case generator for every test in this program.
const SEED: u32 = 0x5EED_00A2;

/// Call tags in the log.
const TAG_SINE: u32 = 1;
const TAG_COSINE: u32 = 2;

/// One logged call: tag, argument bits, zero.
type Call = (u32, u32, u32);

/// The call log of the rewrite's stubs, in order.
static LOG: Mutex<Vec<Call>> = Mutex::new(Vec::new());
/// Serialises the cases: the stubs and the callee table are process-wide.
static LOCK: Mutex<()> = Mutex::new(());

/// The sine stand-in: a fixed bit mixing of the argument, the same on both
/// sides of the comparison.
fn sine_bits(bits: u32) -> u32 {
    bits.rotate_left(5) ^ 0x51ED_2701
}

/// The cosine stand-in: a fixed bit mixing of the argument.
fn cosine_bits(bits: u32) -> u32 {
    bits.wrapping_mul(0x9E37_79B9) ^ 0x0C0F_FEE5
}

extern "cdecl" fn stub_sine(bits: u32) -> u32 {
    LOG.lock().unwrap().push((TAG_SINE, bits, 0));
    sine_bits(bits)
}

extern "cdecl" fn stub_cosine(bits: u32) -> u32 {
    LOG.lock().unwrap().push((TAG_COSINE, bits, 0));
    cosine_bits(bits)
}

/// The collaborator the lifted advance calls: it logs the same entries as
/// the rewrite's stubs and answers with the same mixing.
#[derive(Default)]
struct Recorder {
    log: Vec<Call>,
}

impl TurnTrig for Recorder {
    fn sine(&mut self, angle: f32) -> f32 {
        self.log.push((TAG_SINE, angle.to_bits(), 0));
        f32::from_bits(sine_bits(angle.to_bits()))
    }
    fn cosine(&mut self, angle: f32) -> f32 {
        self.log.push((TAG_COSINE, angle.to_bits(), 0));
        f32::from_bits(cosine_bits(angle.to_bits()))
    }
}

/// One generated case and what the rewrite did with it.
struct Case {
    object_before: Vec<u8>,
    object_after: Vec<u8>,
    param: Vec<u8>,
    inner: Vec<u8>,
    target_before: Vec<u8>,
    target_after: Vec<u8>,
    gate_bits: u32,
    returned: u32,
    target_handle: u32,
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

/// A flag byte drawn from boundary values or at random; half the time the
/// reset bit is set.
fn flag_pick(rng: &mut Xorshift) -> u8 {
    let pick = rng.next_u32();
    let base = if pick % 2 == 0 {
        [0x00, 0x01, 0x7F, 0x80, 0xFF][(pick >> 8) as usize % 5]
    } else {
        (rng.next_u32() & 0xFF) as u8
    };
    if (pick >> 16) % 2 == 0 {
        base | 0x80
    } else {
        base
    }
}

/// Generates a case and runs the rewrite on it.
fn run_rewrite(rng: &mut Xorshift) -> Case {
    let mut object: Vec<u8> = (0..OBJECT_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    set_word(&mut object, RATE_OFFSET, rng.word());
    object[FLAG_OFFSET] = flag_pick(rng);
    let object_before = object.clone();

    let mut inner: Vec<u8> = (0..INNER_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    set_word(&mut inner, INNER_X, rng.word());
    set_word(&mut inner, INNER_Y, rng.word());
    set_word(&mut inner, INNER_Z, rng.word());

    let mut param: Vec<u8> = (0..PARAM_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    set_word(&mut param, ANGLE_OFFSET, rng.word());
    let has_inner = rng.next_u32() % 2 == 0;
    set_word(
        &mut param,
        INNER_OFFSET,
        if has_inner { address_of(&inner) } else { 0 },
    );

    let mut target: Vec<u8> = (0..12).map(|_| (rng.next_u32() & 0xFF) as u8).collect();
    set_word(&mut target, 0, rng.word());
    set_word(&mut target, 4, rng.word());
    set_word(&mut target, 8, rng.word());
    let target_before = target.clone();

    // A quarter of the gates sit exactly on the threshold or one step either side of it,
    // so the strict comparison is exercised at its boundary.
    let gate_bits = match rng.next_u32() % 4 {
        0 => RATE_GATE.to_bits(),
        1 => RATE_GATE.to_bits() + 1,
        2 => RATE_GATE.to_bits() - 1,
        _ => rng.word(),
    };
    let target_handle = address_of(&target);
    LOG.lock().unwrap().clear();
    let returned = fn_00a218a0::rw_00A218A0(
        address_of(&object),
        target_handle,
        address_of(&param),
        gate_bits,
    );
    let calls = std::mem::take(&mut *LOG.lock().unwrap());
    let object_after = object;
    Case {
        object_before,
        object_after,
        param,
        inner,
        target_before,
        target_after: target,
        gate_bits,
        returned,
        target_handle,
        calls,
    }
}

/// Decodes the task words the lift takes from a case's object.
fn task_of(case: &Case) -> TurnTask {
    TurnTask {
        rate: f32::from_bits(word_at(&case.object_before, RATE_OFFSET)),
        flag: case.object_before[FLAG_OFFSET],
    }
}

/// Decodes the parameter block the lift takes: the angle, and the inner
/// direction when the inner pointer is set.
fn params_of(case: &Case) -> TurnParams {
    let pointer = word_at(&case.param, INNER_OFFSET);
    let inner = if pointer == 0 {
        None
    } else {
        Some(Vec3 {
            x: f32::from_bits(word_at(&case.inner, INNER_X)),
            y: f32::from_bits(word_at(&case.inner, INNER_Y)),
            z: f32::from_bits(word_at(&case.inner, INNER_Z)),
        })
    };
    TurnParams {
        angle: f32::from_bits(word_at(&case.param, ANGLE_OFFSET)),
        inner,
    }
}

/// The target point the lift takes, from the case's planted words.
fn target_of(words: &[u8]) -> Vec3 {
    Vec3 {
        x: f32::from_bits(word_at(words, 0)),
        y: f32::from_bits(word_at(words, 4)),
        z: f32::from_bits(word_at(words, 8)),
    }
}

/// The bits of a point's three components.
fn bits_of(point: Vec3) -> [u32; 3] {
    [point.x.to_bits(), point.y.to_bits(), point.z.to_bits()]
}

/// Checks the rewrite's own effects: its return value, and that it changed
/// only the rate word of the task object.
fn check_rewrite_effects(case: &Case) {
    assert_eq!(
        case.returned, case.target_handle,
        "the rewrite returns the target"
    );
    for offset in 0..OBJECT_BYTES {
        if offset < RATE_OFFSET || offset >= RATE_OFFSET + 4 {
            assert_eq!(
                case.object_before[offset], case.object_after[offset],
                "object byte {offset:#x} is unchanged"
            );
        }
    }
}

/// Runs the lift on a case and returns its task, its target and its calls.
fn run_lift(case: &Case) -> (TurnTask, Vec3, Vec<Call>) {
    let mut task = task_of(case);
    let mut target = target_of(&case.target_before);
    let mut recorder = Recorder::default();
    task.advance(
        &mut target,
        &params_of(case),
        f32::from_bits(case.gate_bits),
        &mut recorder,
    );
    (task, target, recorder.log)
}

/// Asserts that the lift matches the rewrite on one case.
fn assert_case(case: &Case) {
    check_rewrite_effects(case);
    let (task, target, calls) = run_lift(case);
    assert_eq!(calls, case.calls, "the trigonometric calls match in order");
    assert_eq!(
        task.rate.to_bits(),
        word_at(&case.object_after, RATE_OFFSET),
        "the rate matches bit for bit"
    );
    assert_eq!(
        task.flag, case.object_after[FLAG_OFFSET],
        "the flag is unchanged"
    );
    assert_eq!(
        bits_of(target),
        bits_of(target_of(&case.target_after)),
        "the target matches bit for bit"
    );
}

/// The deviations of a wrong version: each is the lift with one change.
#[derive(Clone, Copy, Debug)]
enum Wrong {
    /// The reset flag is ignored.
    NoReset,
    /// The gate admits a value equal to it.
    GateInclusive,
    /// The rate rises by twice the step.
    DoubleStep,
    /// The rate is not capped at one.
    NoCap,
    /// The scaling uses the rate from before the update.
    PreUpdateRate,
    /// The direction is not scaled by the tenth.
    NoScale,
    /// The sine is not negated.
    SineNotNegated,
    /// The inner direction is ignored.
    InnerIgnored,
    /// The cosine is taken before the sine.
    CosineFirst,
    /// The third component is the sine rather than zero.
    SineInZ,
}

/// The wrong version of the tracker, on the same inputs as the lift.
fn run_wrong(kind: Wrong, case: &Case) -> (TurnTask, Vec3, Vec<Call>) {
    let task = task_of(case);
    let params = params_of(case);
    let gate = f32::from_bits(case.gate_bits);
    let mut target = target_of(&case.target_before);
    let mut recorder = Recorder::default();
    let mut rate = task.rate;
    if task.flag & 0x80 != 0 && !matches!(kind, Wrong::NoReset) {
        rate = 1.0;
    }
    let open = if matches!(kind, Wrong::GateInclusive) {
        gate >= RATE_GATE
    } else {
        gate > RATE_GATE
    };
    let before = rate;
    if open {
        let step = if matches!(kind, Wrong::DoubleStep) {
            RATE_STEP * 2.0
        } else {
            RATE_STEP
        };
        let mut raised = add(rate, step);
        if !matches!(kind, Wrong::NoCap) && !(1.0 > raised) {
            raised = 1.0;
        }
        rate = raised;
    }
    let used = if matches!(kind, Wrong::PreUpdateRate) {
        before
    } else {
        rate
    };
    let inner = if matches!(kind, Wrong::InnerIgnored) {
        None
    } else {
        params.inner
    };
    let (dx, dy, dz) = match inner {
        Some(direction) => (direction.x, direction.y, direction.z),
        None => {
            let (sine, cosine) = if matches!(kind, Wrong::CosineFirst) {
                let c = recorder.cosine(params.angle);
                let s = recorder.sine(params.angle);
                (s, c)
            } else {
                let s = recorder.sine(params.angle);
                let c = recorder.cosine(params.angle);
                (s, c)
            };
            let x = if matches!(kind, Wrong::SineNotNegated) {
                sine
            } else {
                f32::from_bits(sine.to_bits() ^ 0x8000_0000)
            };
            let z = if matches!(kind, Wrong::SineInZ) {
                sine
            } else {
                0.0
            };
            (x, cosine, z)
        }
    };
    let scale = if matches!(kind, Wrong::NoScale) {
        1.0
    } else {
        DIR_SCALE
    };
    target.x = sub(target.x, mul(mul(dx, scale), used));
    target.y = sub(target.y, mul(mul(dy, scale), used));
    target.z = sub(target.z, mul(mul(dz, scale), used));
    let out = TurnTask {
        rate,
        flag: task.flag,
    };
    (out, target, recorder.log)
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn wrong_mismatches(kind: Wrong) -> u32 {
    let mut rng = Xorshift::new(SEED);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        let (task, target, calls) = run_wrong(kind, &case);
        let same = calls == case.calls
            && task.rate.to_bits() == word_at(&case.object_after, RATE_OFFSET)
            && bits_of(target) == bits_of(target_of(&case.target_after));
        if !same {
            differing += 1;
        }
    }
    eprintln!("wrong {kind:?}: differing {differing} of {CASES}");
    differing
}

#[test]
fn turn_advance_matches_its_rewrite() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_sine as *const () as usize as u32);
    rt::set_callee(2, stub_cosine as *const () as usize as u32);
    let mut rng = Xorshift::new(SEED);
    let mut with_inner = 0;
    let mut gate_open = 0;
    for _ in 0..CASES {
        let case = run_rewrite(&mut rng);
        assert_case(&case);
        if word_at(&case.param, INNER_OFFSET) != 0 {
            with_inner += 1;
        }
        if f32::from_bits(case.gate_bits) > RATE_GATE {
            gate_open += 1;
        }
    }
    eprintln!("cases {CASES}: inner direction {with_inner}, gate open {gate_open}");
}

#[test]
fn turn_advance_wrong_versions_are_caught() {
    let _guard = LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    rt::set_callee(1, stub_sine as *const () as usize as u32);
    rt::set_callee(2, stub_cosine as *const () as usize as u32);
    for kind in [
        Wrong::NoReset,
        Wrong::GateInclusive,
        Wrong::DoubleStep,
        Wrong::NoCap,
        Wrong::PreUpdateRate,
        Wrong::NoScale,
        Wrong::SineNotNegated,
        Wrong::InnerIgnored,
        Wrong::CosineFirst,
        Wrong::SineInZ,
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
