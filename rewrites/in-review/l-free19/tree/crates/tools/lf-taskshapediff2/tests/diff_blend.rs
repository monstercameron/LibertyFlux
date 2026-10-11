//! Differential cases of the aim blend of the ped task record against its
//! verified 32-bit rewrite.
//!
//! The lifted blend and the rewrite run on the same generated record, target,
//! gate answers and direction vectors. The effects compared are:
//!
//! - the answer word: the rewrite's return value, compared with the lifted
//!   answer rebuilt as a word (the fallback gate's answer, or the position
//!   of the second direction the hook answered);
//! - every byte of the record after the call, which holds the accumulator
//!   and the flag byte the lift writes and the words it must leave alone;
//! - the calls, in order: each gate call with its candidate, and each hook
//!   call with its target. The hook's scratch argument is masked on both
//!   sides, as the registry rows say.
//!
//! The direction vectors are three-float blocks planted in memory. The
//! hook answers a pointer to the next block, as the original does, and the
//! rewrite reads the floats there; the lift gets the floats through its hook.
//!
//! Each routine has deliberately wrong versions. A wrong version is a copy of
//! the blend with one knob turned; the copy with every knob off must agree
//! with the lift, and each knob must make a generated case differ.
//!
//! Runs on the 32-bit target only: the rewrite takes raw addresses and calls
//! numbered stubs and a vtable slot.

#![cfg(target_arch = "x86")]
// The record, the direction blocks and the target's vtable are planted as
// byte and word buffers, and the stubs read them through the addresses the
// rewrite was given. This is test-only code: the lifted module has no unsafe
// code.
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::aim_blend::{
    AimBlend, AimBlendCallees, BlendResult, CELL_ROW_0, CELL_ROW_1, FLAG_OFFSET,
};
use lf_taskshapediff2::rewrites::fn_00a1ef80;
use lf_taskshapediff2::{Xorshift, rt};

/// File address of the scale global the transform reads.
const SCALE_GLOBAL: u32 = 0x0117_35CC;
/// Bytes of the planted auxiliary window that holds the scale global.
const AUX_LEN: u32 = 0x40;

/// Bytes planted for the record: the flag byte is the largest offset read.
const RECORD_BYTES: usize = 0x400;
/// Bytes planted for the two direction blocks, three floats each.
const DIRECTION_BYTES: usize = 24;
/// Word index of the vtable slot the hook is read from (offset 0xec).
const HOOK_SLOT_WORD: usize = 0xec / 4;
/// Bytes planted for the target's vtable.
const VTABLE_BYTES: usize = 0x100;
/// Generated cases.
const CASES: u32 = 3000;
/// Seed of the case generator.
const SEED: u32 = 0x5EED_0020;
/// Callee number of the gate.
const GATE_ID: u32 = 1;
/// Pseudo callee number the hook calls are logged under.
const HOOK_ID: u32 = 9;

/// Serialises the tests of this program: they share the stubs.
static TEST_LOCK: Mutex<()> = Mutex::new(());

/// One call the rewrite made, with the object and argument it passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Call {
    id: u32,
    object: u32,
    arg: u32,
}

/// The stubs' state for one run.
struct Stub {
    gate_answers: [u32; 2],
    gate_calls: usize,
    hook_calls: usize,
    /// Address of the first direction block; the second follows it.
    direction: u32,
    log: Vec<Call>,
}

static STUB: Mutex<Stub> = Mutex::new(Stub {
    gate_answers: [0; 2],
    gate_calls: 0,
    hook_calls: 0,
    direction: 0,
    log: Vec::new(),
});

/// One generated case.
struct Case {
    record: Vec<u8>,
    directions: Vec<u8>,
    gate_answers: [u32; 2],
    scale: u32,
    fallback: u32,
}

/// What a run leaves behind.
#[derive(Debug, PartialEq)]
struct Outcome {
    answer: u32,
    record: Vec<u8>,
    calls: Vec<Call>,
    /// The direction the blend read, as bits, when it ran.
    direction_bits: Option<[u32; 3]>,
}

/// The address of a planted buffer, as the 32-bit rewrite sees it.
fn address_of<T>(buffer: &[T]) -> u32 {
    buffer.as_ptr() as usize as u32
}

/// Writes a little-endian word at a byte offset of a planted buffer.
fn set_word(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// A little-endian word at a byte offset of a planted buffer.
fn word_at(buffer: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        buffer[offset],
        buffer[offset + 1],
        buffer[offset + 2],
        buffer[offset + 3],
    ])
}

/// Stub for the gate: thiscall on the record with the candidate.
extern "thiscall" fn stub_gate(this: u32, candidate: u32) -> u32 {
    let mut stub = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    stub.log.push(Call {
        id: GATE_ID,
        object: this,
        arg: candidate,
    });
    let answer = stub.gate_answers[stub.gate_calls.min(1)];
    stub.gate_calls += 1;
    answer
}

/// Stub for the hook in the target's vtable: thiscall on the target with the
/// scratch slot. It answers the address of the next direction block.
extern "thiscall" fn stub_hook(this: u32, scratch: u32) -> u32 {
    let mut stub = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    stub.log.push(Call {
        id: HOOK_ID,
        object: this,
        arg: scratch,
    });
    let block = stub.hook_calls.min(1);
    stub.hook_calls += 1;
    stub.direction + 12 * block as u32
}

/// Plants the gate stub at callee 1.
fn plant_stubs() {
    rt::set_callee(GATE_ID, stub_gate as *const () as usize as u32);
}

/// Sets the stubs' answers and direction, and clears their log.
fn install(gate_answers: [u32; 2], direction: u32) {
    let mut stub = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    stub.gate_answers = gate_answers;
    stub.gate_calls = 0;
    stub.hook_calls = 0;
    stub.direction = direction;
    stub.log.clear();
}

/// Takes the stubs' log, with the hook's scratch argument masked to zero.
fn take_calls() -> Vec<Call> {
    let mut stub = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    std::mem::take(&mut stub.log)
        .into_iter()
        .map(|call| {
            if call.id == HOOK_ID {
                Call { arg: 0, ..call }
            } else {
                call
            }
        })
        .collect()
}

/// A float's bits from edge values, a moderate value or any bits.
fn float_bits(rng: &mut Xorshift) -> u32 {
    const EDGES: [u32; 8] = [
        0x0000_0000,
        0x8000_0000,
        0x3F80_0000,
        0xBF80_0000,
        0x7FC0_0000,
        0xFFC0_0001,
        0x7F80_0000,
        0xFF80_0000,
    ];
    let pick = rng.next_u32();
    match pick % 4 {
        0 => EDGES[(pick >> 8) as usize % EDGES.len()],
        1 => ((rng.next_u32() % 2000) as f32 / 64.0 - 15.0).to_bits(),
        _ => rng.next_u32(),
    }
}

/// Generates one case. One case in six has a zero first direction, so the
/// zero-length branch comes up; the gate answers have a zero low byte half
/// the time.
fn generate(rng: &mut Xorshift) -> Case {
    let mut record: Vec<u8> = (0..RECORD_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    for lane in 0..3 {
        set_word(&mut record, CELL_ROW_0 + 4 * lane, float_bits(rng));
        set_word(&mut record, CELL_ROW_1 + 4 * lane, float_bits(rng));
    }
    record[FLAG_OFFSET] = (rng.next_u32() & 0xFF) as u8;

    let zero_first = rng.next_u32() % 6 == 0;
    let mut directions: Vec<u8> = vec![0; DIRECTION_BYTES];
    for word in 0..6 {
        let bits = if word < 3 && zero_first {
            if rng.next_u32() % 2 == 0 { 0 } else { 0x8000_0000 }
        } else {
            float_bits(rng)
        };
        set_word(&mut directions, 4 * word, bits);
    }

    let mut gate_answers = [0u32; 2];
    for answer in gate_answers.iter_mut() {
        let word = rng.next_u32();
        *answer = if rng.next_u32() % 2 == 0 {
            word & !0xFF
        } else {
            word | 1
        };
    }
    Case {
        record,
        directions,
        gate_answers,
        scale: float_bits(rng),
        fallback: rng.next_u32() | 1,
    }
}

/// The target's vtable: the hook's slot holds the hook stub.
fn target_vtable() -> Vec<u32> {
    let mut vtable = vec![0u32; VTABLE_BYTES / 4];
    vtable[HOOK_SLOT_WORD] = stub_hook as *const () as usize as u32;
    vtable
}

/// The addresses one run used: the record, the target and the direction
/// block the stubs answer from.
#[derive(Debug, Clone, Copy)]
struct Addresses {
    record: u32,
    target: u32,
    directions: u32,
}

/// Runs the rewrite on a case. The target object and its vtable are planted
/// here, so the rewrite's indirect call reaches the hook stub.
fn run_rewrite(case: &Case) -> (Outcome, Addresses) {
    let mut record = case.record.clone();
    let directions = case.directions.clone();
    let vtable = target_vtable();
    let mut target = vec![0u32; 4];
    target[0] = address_of(&vtable);
    let addresses = Addresses {
        record: address_of(&record),
        target: address_of(&target),
        directions: address_of(&directions),
    };
    install(case.gate_answers, addresses.directions);
    // The scale global lives in the planted auxiliary window for this run.
    let aux = vec![0u8; AUX_LEN as usize];
    rt::set_aux(address_of(&aux));
    rt::set_aux_word(SCALE_GLOBAL, case.scale);
    let answer = fn_00a1ef80::rw_00a1ef80(addresses.record, addresses.target, case.fallback);
    drop(aux);
    let calls = take_calls();
    let direction_bits = if calls.iter().any(|call| call.id == HOOK_ID) {
        Some(direction_of(&directions, 1))
    } else {
        None
    };
    let outcome = Outcome {
        answer,
        record: std::mem::take(&mut record),
        calls,
        direction_bits,
    };
    // Keep the planted buffers alive until the call has returned.
    drop((vtable, target, directions));
    (outcome, addresses)
}

/// The bits of one planted direction block (block 0 or 1).
fn direction_of(directions: &[u8], block: usize) -> [u32; 3] {
    [
        word_at(directions, 12 * block),
        word_at(directions, 12 * block + 4),
        word_at(directions, 12 * block + 8),
    ]
}

/// The floats of one planted direction block.
fn direction_floats(directions: &[u8], block: usize) -> [f32; 3] {
    direction_of(directions, block).map(f32::from_bits)
}

/// The lift's hooks: the gate answers, the two direction blocks and the
/// scale, with each call logged in order.
struct Plan {
    gate_answers: [u32; 2],
    gate_calls: usize,
    directions: [[f32; 3]; 2],
    hook_calls: usize,
    scale: f32,
    log: Vec<Logged>,
}

/// One call as the lift logs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Logged {
    Gate { candidate: u32 },
    Hook,
}

impl AimBlendCallees for Plan {
    type Target = u32;

    fn gate(&mut self, _owner: &mut AimBlend, candidate: u32) -> u32 {
        self.log.push(Logged::Gate { candidate });
        let answer = self.gate_answers[self.gate_calls.min(1)];
        self.gate_calls += 1;
        answer
    }

    fn hook(&mut self, _owner: &mut AimBlend, _target: u32) -> [f32; 3] {
        self.log.push(Logged::Hook);
        let block = self.directions[self.hook_calls.min(1)];
        self.hook_calls += 1;
        block
    }

    fn scale(&mut self, _owner: &mut AimBlend) -> f32 {
        self.scale
    }
}

/// Runs the lifted blend on a case, with the addresses the rewrite run used
/// so the answer and the call objects are rebuilt from the same positions.
fn run_lift(case: &Case, addresses: Addresses) -> Outcome {
    let mut record = case.record.clone();
    let mut owner = decode(&record);
    let mut plan = Plan {
        gate_answers: case.gate_answers,
        gate_calls: 0,
        directions: [
            direction_floats(&case.directions, 0),
            direction_floats(&case.directions, 1),
        ],
        hook_calls: 0,
        scale: f32::from_bits(case.scale),
        log: Vec::new(),
    };
    let result = owner.blend(&mut plan, addresses.target, case.fallback);
    encode_into(&owner, &mut record);
    let (answer, direction_bits) = match result {
        BlendResult::Gated(answer) => (answer, None),
        BlendResult::Blended { vector } => (
            // The original answers the second direction block's position.
            addresses.directions.wrapping_add(12),
            Some(vector.map(f32::to_bits)),
        ),
    };
    let calls = plan
        .log
        .iter()
        .map(|entry| match *entry {
            Logged::Gate { candidate } => Call {
                id: GATE_ID,
                object: addresses.record,
                arg: candidate,
            },
            Logged::Hook => Call {
                id: HOOK_ID,
                object: addresses.target,
                arg: 0,
            },
        })
        .collect();
    Outcome {
        answer,
        record,
        calls,
        direction_bits,
    }
}

/// Decodes the owned fields from a record.
fn decode(bytes: &[u8]) -> AimBlend {
    let float = |offset: usize| f32::from_bits(word_at(bytes, offset));
    AimBlend {
        rows: [
            [float(CELL_ROW_0), float(CELL_ROW_0 + 4), float(CELL_ROW_0 + 8)],
            [float(CELL_ROW_1), float(CELL_ROW_1 + 4), float(CELL_ROW_1 + 8)],
        ],
        flags: bytes[FLAG_OFFSET],
    }
}

/// Writes the owned fields back into a record.
fn encode_into(owner: &AimBlend, bytes: &mut [u8]) {
    for lane in 0..3 {
        set_word(bytes, CELL_ROW_0 + 4 * lane, owner.rows[0][lane].to_bits());
        set_word(bytes, CELL_ROW_1 + 4 * lane, owner.rows[1][lane].to_bits());
    }
    bytes[FLAG_OFFSET] = owner.flags;
}

/// Describes the first difference between the lift and the rewrite.
fn describe(lift: &Outcome, rewrite: &Outcome) -> String {
    if lift.answer != rewrite.answer {
        return format!("answer: lift {:#x}, rewrite {:#x}", lift.answer, rewrite.answer);
    }
    if lift.direction_bits != rewrite.direction_bits {
        return format!(
            "direction: lift {:x?}, rewrite {:x?}",
            lift.direction_bits, rewrite.direction_bits
        );
    }
    if lift.record != rewrite.record {
        let at = lift
            .record
            .iter()
            .zip(rewrite.record.iter())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        return format!("record byte {at:#x} differs");
    }
    format!("calls: lift {:?}, rewrite {:?}", lift.calls, rewrite.calls)
}

/// Which knobs a wrong version turns. Every field off is the lift itself.
#[derive(Debug, Clone, Copy, Default)]
struct Knobs {
    /// A zero first gate returns the first answer instead of trying the
    /// fallback.
    no_fallback_gate: bool,
    /// The factor is the length itself, not length over length.
    factor_is_length: bool,
    /// The factor is length over length even for a zero length.
    no_zero_length_branch: bool,
    /// The second direction is read from the first block.
    second_reads_first: bool,
    /// The third column of the second row adds the second component.
    row1_column2_uses_y: bool,
    /// The gate path leaves the flag byte alone.
    gate_path_no_flag: bool,
    /// A gate failure answers the first gate's answer.
    gate_answers_first: bool,
    /// The hook is called on the fallback target.
    hook_on_fallback: bool,
}

/// The blend with knobs: the same arithmetic and order as the lift, with each
/// knob changing one choice.
fn run_knobs(knobs: Knobs, case: &Case, addresses: Addresses) -> Outcome {
    let mut record = case.record.clone();
    let mut owner = decode(&record);
    let directions = [
        direction_floats(&case.directions, 0),
        direction_floats(&case.directions, 1),
    ];
    let mut log: Vec<Call> = Vec::new();
    let mut gate_calls = 0usize;
    let mut hook_calls = 0usize;
    let mut gate = |candidate: u32, log: &mut Vec<Call>| -> u32 {
        log.push(Call {
            id: GATE_ID,
            object: addresses.record,
            arg: candidate,
        });
        let answer = case.gate_answers[gate_calls.min(1)];
        gate_calls += 1;
        answer
    };
    let mut hook = |target: u32, log: &mut Vec<Call>| -> usize {
        log.push(Call {
            id: HOOK_ID,
            object: target,
            arg: 0,
        });
        let block = hook_calls.min(1);
        hook_calls += 1;
        block
    };
    let primary = addresses.target;
    let fallback = case.fallback;
    let first = gate(primary, &mut log);
    let mut answer_path: Option<u32> = None;
    if (first as u8) == 0 {
        if knobs.no_fallback_gate {
            answer_path = Some(first);
        } else {
            let second = gate(fallback, &mut log);
            if (second as u8) == 0 {
                answer_path = Some(if knobs.gate_answers_first { first } else { second });
            }
        }
    }
    if let Some(answer) = answer_path {
        if !knobs.gate_path_no_flag {
            owner.flags |= 0x02;
        }
        encode_into(&owner, &mut record);
        return Outcome {
            answer,
            record,
            calls: log,
            direction_bits: None,
        };
    }

    let block0 = hook(primary, &mut log);
    let first_block = directions[block0];
    let [x, y, z] = first_block;
    let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
    let len = len2.sqrt();
    let factor = if knobs.no_zero_length_branch {
        div(len, len)
    } else if knobs.factor_is_length {
        len
    } else if len == 0.0 {
        0.0
    } else {
        div(len, len)
    };
    let scale = f32::from_bits(case.scale);
    let block1 = hook(if knobs.hook_on_fallback { fallback } else { primary }, &mut log);
    let vector = if knobs.second_reads_first {
        directions[0]
    } else {
        directions[block1]
    };
    let [vx, vy, vz] = vector;
    let bx = mul(vx, factor);
    let by = mul(vy, factor);
    let bz = mul(vz, factor);
    let cx = mul(scale, bx);
    let cy = mul(by, scale);
    let cz = mul(bz, scale);
    owner.rows[0][0] = add(cx, owner.rows[0][0]);
    owner.rows[0][1] = add(owner.rows[0][1], cy);
    owner.rows[0][2] = add(owner.rows[0][2], cz);
    owner.rows[1][0] = add(cx, owner.rows[1][0]);
    owner.rows[1][1] = add(owner.rows[1][1], cy);
    if knobs.row1_column2_uses_y {
        owner.rows[1][2] = add(owner.rows[1][2], cy);
    } else {
        owner.rows[1][2] = add(owner.rows[1][2], cz);
    }
    owner.flags |= 0x02;
    encode_into(&owner, &mut record);
    Outcome {
        answer: addresses.directions.wrapping_add(12),
        record,
        calls: log,
        direction_bits: Some(vector.map(f32::to_bits)),
    }
}

/// Multiplication with both operands pinned, as the lift pins them.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Addition with both operands pinned, in the order given.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Division with both operands pinned, in the order given.
fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// Checks the lift against the rewrite on every generated case, and returns
/// how many cases reached each path: gated, blended, and blended with a zero
/// first direction.
fn check_cases(seed: u32) -> [u32; 3] {
    let mut rng = Xorshift::new(seed);
    let mut paths = [0u32; 3];
    for number in 0..CASES {
        let case = generate(&mut rng);
        let (rewrite, addresses) = run_rewrite(&case);
        let lift = run_lift(&case, addresses);
        assert!(lift == rewrite, "case {number}: {}", describe(&lift, &rewrite));
        match rewrite.direction_bits {
            None => paths[0] += 1,
            Some(_) => paths[1] += 1,
        }
        if rewrite.direction_bits.is_some() {
            let first_zero = word_at(&case.directions, 0) & 0x7FFF_FFFF == 0
                && word_at(&case.directions, 4) & 0x7FFF_FFFF == 0
                && word_at(&case.directions, 8) & 0x7FFF_FFFF == 0;
            if first_zero {
                paths[2] += 1;
            }
        }
        if rewrite.record[FLAG_OFFSET] & 0x02 == 0 {
            panic!("case {number}: flag bit not set on a path that returns");
        }
    }
    paths
}

/// The knob copy with every knob off must agree with the lift.
fn check_copy(seed: u32) {
    let mut rng = Xorshift::new(seed);
    for number in 0..CASES {
        let case = generate(&mut rng);
        let (_rewrite, addresses) = run_rewrite(&case);
        let lift = run_lift(&case, addresses);
        let copy = run_knobs(Knobs::default(), &case, addresses);
        assert!(copy == lift, "copy, case {number}: {}", describe(&copy, &lift));
    }
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn caught_by(seed: u32, knobs: Knobs) -> u32 {
    let mut rng = Xorshift::new(seed);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = generate(&mut rng);
        let (rewrite, addresses) = run_rewrite(&case);
        if run_knobs(knobs, &case, addresses) != rewrite {
            differing += 1;
        }
    }
    differing
}

#[test]
fn blend_lift_matches_rewrite_on_generated_cases() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let paths = check_cases(SEED);
    eprintln!("blend coverage [gated, blended, zero-length first direction]: {paths:?}");
    assert!(paths[0] > 0 && paths[1] > CASES / 3, "paths: {paths:?}");
    assert!(paths[2] > CASES / 12, "zero-length cases: {paths:?}");
}

#[test]
fn knob_copy_with_no_knob_matches_the_lift() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    check_copy(SEED);
}

#[test]
fn wrong_blend_versions_are_caught() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let cases: [(&str, Knobs); 8] = [
        ("fallback gate not tried", Knobs { no_fallback_gate: true, ..Knobs::default() }),
        ("factor is the length", Knobs { factor_is_length: true, ..Knobs::default() }),
        ("no zero-length branch", Knobs { no_zero_length_branch: true, ..Knobs::default() }),
        ("second direction read from the first block", Knobs { second_reads_first: true, ..Knobs::default() }),
        ("second row third column uses the second component", Knobs { row1_column2_uses_y: true, ..Knobs::default() }),
        ("gate path leaves the flag", Knobs { gate_path_no_flag: true, ..Knobs::default() }),
        ("gate failure answers the first gate", Knobs { gate_answers_first: true, ..Knobs::default() }),
        ("hook called on the fallback", Knobs { hook_on_fallback: true, ..Knobs::default() }),
    ];
    for (name, knobs) in cases {
        let differing = caught_by(SEED, knobs);
        eprintln!("blend wrong version {name}: {differing} of {CASES} cases differ");
        assert!(differing > 0, "wrong blend not caught: {name}");
    }
}
