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

