//! Differential cases of the frame remap and the matrix transform of the
//! ped task record against their verified 32-bit rewrites.
//!
//! Each lifted method runs on the same generated record, block and script
//! of callee writes as its rewrite. The effects compared are:
//!
//! - the answer word: the rewrite's return value, compared with the lifted
//!   answer rebuilt as a word from the record's own position;
//! - every byte of the record after the call, which holds the owned fields
//!   the lift writes and the fields it only reads;
//! - the calls, in order, with the object each one names (the record for
//!   the refresh, the sub-object's position for a notify) and its argument.
//!
//! The callee writes come from a script: call number `n` may write one
//! count word or one flag byte of the record, on both sides, so the walk's
//! count reads are exercised after the writes.
//!
//! Each routine also has deliberately wrong versions. A wrong version is a
//! copy of the walk and the arithmetic with one knob turned; the copy with
//! every knob off must agree with the lift, and each knob must make a
//! generated case differ from the rewrite.
//!
//! Runs on the 32-bit target only: the rewrites take raw addresses and call
//! numbered stubs.

#![cfg(target_arch = "x86")]
// The record and the block are planted as byte buffers, and the callee stubs
// write the record's count and flag bytes through the address the rewrite
// was given. This is test-only code: the lifted module has no unsafe code.
#![allow(unsafe_code)]

use std::sync::Mutex;

use lf_peds_tasks::ped_task::subobject_tables::{
    ANCHOR_OFFSET, COUNT_OFFSET, FLAG_BYTES, FLAG_OFFSET, OBJECT_BASE, OBJECT_STRIDE, OUT_OFFSET,
    SPARE_OFFSET, object_offset,
};
use lf_peds_tasks::ped_task::{AffineBlock, CountReads, SubobjectHooks, TaskSubobjects, WalkEnd};
use lf_taskshapediff2::rewrites::{fn_00a1ed80, fn_00a23720};
use lf_taskshapediff2::{Xorshift, rt};

/// Bytes planted for a task record: the largest offset the routines touch is
/// the spare word, with room after it.
const RECORD_BYTES: usize = 0x440;
/// Bytes planted for a frame or matrix block: twelve words are read.
const BLOCK_BYTES: usize = 0x40;
/// Generated cases per routine.
const CASES: u32 = 3000;
/// Callee calls the scripts cover. A routine makes at most 1 + 4 * 8 calls.
const MAX_CALLS: usize = 40;
/// Seed of the case generator for the remap, and for the transform.
const SEED_REMAP: u32 = 0x5EED_0019;
const SEED_APPLY: u32 = 0x5EED_001A;

/// Serialises the tests of this program: they share the callee stubs.
static TEST_LOCK: Mutex<()> = Mutex::new(());

/// One scripted callee write, applied during the call with its number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Poke {
    /// A signed count word of one table.
    Count { table: usize, value: i32 },
    /// One flag byte, by its index in the overlapping run.
    Flag { index: usize, value: u8 },
}

/// One call as the rewrite makes it: the callee number, the object it is
/// made on, and its one argument word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Call {
    id: u32,
    object: u32,
    arg: u32,
}

/// One generated case: the record bytes, the block bytes and the script.
struct Case {
    record: Vec<u8>,
    block: Vec<u8>,
    pokes: [Option<Poke>; MAX_CALLS],
}

/// What a run leaves behind: the answer word, the record bytes and the calls.
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    answer: u32,
    record: Vec<u8>,
    calls: Vec<Call>,
}

/// The stubs' shared state: where the record is, the script and the log.
struct StubState {
    record: u32,
    pokes: [Option<Poke>; MAX_CALLS],
    calls: Vec<Call>,
}

static STUB: Mutex<StubState> = Mutex::new(StubState {
    record: 0,
    pokes: [None; MAX_CALLS],
    calls: Vec::new(),
});

/// The address of a planted buffer, as the 32-bit rewrite sees it.
fn address_of(buffer: &[u8]) -> u32 {
    buffer.as_ptr() as usize as u32
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

/// Writes a little-endian word at a byte offset of a planted buffer.
fn set_word(buffer: &mut [u8], offset: usize, value: u32) {
    buffer[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

/// Writes a scripted poke into the planted record at `record`, which is the
/// address of a live planted record of [`RECORD_BYTES`] bytes. The poke's
/// offset lies inside it.
fn apply_poke(record: u32, poke: Poke) {
    match poke {
        Poke::Count { table, value } => {
            let at = record as usize + COUNT_OFFSET + 4 * table;
            // SAFETY: the count word lies inside the planted record (see the
            // function's contract); the write is unaligned-safe.
            unsafe { (at as *mut i32).write_unaligned(value) };
        }
        Poke::Flag { index, value } => {
            let at = record as usize + FLAG_OFFSET + index;
            // SAFETY: the flag byte lies inside the planted record.
            unsafe { (at as *mut u8).write(value) };
        }
    }
}

/// Records one call and applies the script's write for its number.
fn record_call(id: u32, object: u32, arg: u32) -> u32 {
    let mut state = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    state.calls.push(Call { id, object, arg });
    let number = state.calls.len() - 1;
    if number < MAX_CALLS {
        if let Some(poke) = state.pokes[number] {
            apply_poke(state.record, poke);
        }
    }
    0
}

/// Callee 1: the record's refresh hook, thiscall with the record.
extern "thiscall" fn stub_refresh(this: u32) -> u32 {
    record_call(1, this, 0)
}

/// Callee 2: the table 0 notify, thiscall with the object and one argument.
extern "thiscall" fn stub_table0(object: u32, arg: u32) -> u32 {
    record_call(2, object, arg)
}

/// Callee 3: the table 1 notify.
extern "thiscall" fn stub_table1(object: u32, arg: u32) -> u32 {
    record_call(3, object, arg)
}

/// Callee 4: the table 2 notify.
extern "thiscall" fn stub_table2(object: u32, arg: u32) -> u32 {
    record_call(4, object, arg)
}

/// Callee 5: the table 3 notify.
extern "thiscall" fn stub_table3(object: u32, arg: u32) -> u32 {
    record_call(5, object, arg)
}

/// Plants the stubs for callees 1 to 5. Every routine of this program uses
/// the same numbering.
fn plant_stubs() {
    rt::set_callee(1, stub_refresh as *const () as usize as u32);
    rt::set_callee(2, stub_table0 as *const () as usize as u32);
    rt::set_callee(3, stub_table1 as *const () as usize as u32);
    rt::set_callee(4, stub_table2 as *const () as usize as u32);
    rt::set_callee(5, stub_table3 as *const () as usize as u32);
}

/// Sets the stubs' record and script, and clears their log.
fn install(record: u32, pokes: [Option<Poke>; MAX_CALLS]) {
    let mut state = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    state.record = record;
    state.pokes = pokes;
    state.calls.clear();
}

/// Takes the stubs' log.
fn take_calls() -> Vec<Call> {
    let mut state = STUB.lock().unwrap_or_else(|poison| poison.into_inner());
    std::mem::take(&mut state.calls)
}

/// A float's bits from a set of edge values, a moderate value or any bits,
/// so the not-a-number and sign cases come up often.
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

/// A count in the capacity the proof covers: zero, a negative, or a value
/// up to the table capacity.
fn count_value(rng: &mut Xorshift) -> i32 {
    match rng.next_u32() % 8 {
        0 => 0,
        1 => -1,
        2 => -2,
        _ => (rng.next_u32() % 9) as i32,
    }
}

/// Generates one case: a random record with capacity-bounded counts, a
/// block of float words, and a script that writes about one call in five.
fn generate(rng: &mut Xorshift) -> Case {
    let mut record: Vec<u8> = (0..RECORD_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    for table in 0..4 {
        let count = count_value(rng);
        set_word(&mut record, COUNT_OFFSET + 4 * table, count as u32);
    }
    for index in 0..FLAG_BYTES {
        // Half the flag bytes are zero, so the flagged and unflagged slots
        // both come up in each table.
        record[FLAG_OFFSET + index] = if rng.next_u32() % 2 == 0 {
            0
        } else {
            (rng.next_u32() | 1) as u8
        };
    }
    for lane in 0..3 {
        set_word(&mut record, ANCHOR_OFFSET + 4 * lane, float_bits(rng));
        set_word(&mut record, OUT_OFFSET + 4 * lane, float_bits(rng));
    }
    set_word(&mut record, SPARE_OFFSET, rng.word());

    let mut block: Vec<u8> = (0..BLOCK_BYTES)
        .map(|_| (rng.next_u32() & 0xFF) as u8)
        .collect();
    for word in 0..12 {
        set_word(&mut block, 4 * word, float_bits(rng));
    }

    let mut pokes = [None; MAX_CALLS];
    for slot in pokes.iter_mut() {
        if rng.next_u32() % 5 == 0 {
            *slot = Some(if rng.next_u32() % 3 == 0 {
                Poke::Flag {
                    index: rng.next_u32() as usize % FLAG_BYTES,
                    value: (rng.next_u32() & 0xFF) as u8,
                }
            } else {
                Poke::Count {
                    table: rng.next_u32() as usize % 4,
                    value: count_value(rng),
                }
            });
        }
    }
    Case {
        record,
        block,
        pokes,
    }
}

/// Decodes the record's owned fields from planted bytes.
fn decode(bytes: &[u8]) -> TaskSubobjects {
    let float = |offset: usize| f32::from_bits(word_at(bytes, offset));
    let mut owner = TaskSubobjects {
        anchor: [0.0; 3],
        out: [0.0; 3],
        spare: word_at(bytes, SPARE_OFFSET),
        counts: [0; 4],
        flags: [0; FLAG_BYTES],
    };
    for lane in 0..3 {
        owner.anchor[lane] = float(ANCHOR_OFFSET + 4 * lane);
        owner.out[lane] = float(OUT_OFFSET + 4 * lane);
    }
    for table in 0..4 {
        owner.counts[table] = word_at(bytes, COUNT_OFFSET + 4 * table) as i32;
    }
    owner.flags.copy_from_slice(&bytes[FLAG_OFFSET..FLAG_OFFSET + FLAG_BYTES]);
    owner
}

/// Writes the record's owned fields into planted bytes.
fn encode_into(owner: &TaskSubobjects, bytes: &mut [u8]) {
    for lane in 0..3 {
        set_word(bytes, ANCHOR_OFFSET + 4 * lane, owner.anchor[lane].to_bits());
        set_word(bytes, OUT_OFFSET + 4 * lane, owner.out[lane].to_bits());
    }
    set_word(bytes, SPARE_OFFSET, owner.spare);
    for table in 0..4 {
        set_word(bytes, COUNT_OFFSET + 4 * table, owner.counts[table] as u32);
    }
    bytes[FLAG_OFFSET..FLAG_OFFSET + FLAG_BYTES].copy_from_slice(&owner.flags);
}

/// Decodes the affine block: rows 16 bytes apart, the bias after them.
fn decode_block(bytes: &[u8]) -> AffineBlock {
    let float = |offset: usize| f32::from_bits(word_at(bytes, offset));
    let mut block = AffineBlock {
        rows: [[0.0; 3]; 3],
        bias: [0.0; 3],
    };
    for row in 0..3 {
        for column in 0..3 {
            block.rows[row][column] = float(16 * row + 4 * column);
        }
    }
    for column in 0..3 {
        block.bias[column] = float(0x30 + 4 * column);
    }
    block
}

/// The lifted run's hooks: they log each call by its position in the walk
/// and apply the same script to the owned record.
struct Recorder {
    calls: Vec<Logged>,
    pokes: [Option<Poke>; MAX_CALLS],
}

/// One call as the lift logs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Logged {
    Refresh,
    Notify {
        table: usize,
        slot: usize,
        handle: u32,
    },
}

impl Recorder {
    fn new(pokes: [Option<Poke>; MAX_CALLS]) -> Self {
        Self {
            calls: Vec::new(),
            pokes,
        }
    }

    /// Applies the script's write for the call just logged to the record.
    fn after_call(&mut self, owner: &mut TaskSubobjects) {
        let number = self.calls.len() - 1;
        if number < MAX_CALLS {
            match self.pokes[number] {
                Some(Poke::Count { table, value }) => owner.counts[table] = value,
                Some(Poke::Flag { index, value }) => owner.flags[index] = value,
                None => {}
            }
        }
    }

    fn log_refresh(&mut self, owner: &mut TaskSubobjects) {
        self.calls.push(Logged::Refresh);
        self.after_call(owner);
    }

    fn log_notify(&mut self, owner: &mut TaskSubobjects, table: usize, slot: usize, handle: u32) {
        self.calls.push(Logged::Notify {
            table,
            slot,
            handle,
        });
        self.after_call(owner);
    }
}

impl SubobjectHooks for Recorder {
    type Handle = u32;

    fn refresh(&mut self, owner: &mut TaskSubobjects) {
        self.log_refresh(owner);
    }

    fn notify(
        &mut self,
        owner: &mut TaskSubobjects,
        table: usize,
        slot: usize,
        handle: Self::Handle,
    ) {
        self.log_notify(owner, table, slot, handle);
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

/// Subtraction with both operands pinned, in the order given.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// The answer word of a walk, rebuilt from the record's position: a count
/// is the count itself, an object is the record plus its offset, and no
/// table ran is zero.
fn rebuild(end: WalkEnd, record: u32) -> u32 {
    match end {
        WalkEnd::None => 0,
        WalkEnd::Count(walked) => walked,
        WalkEnd::Object { table, objects } => record.wrapping_add(
            OBJECT_BASE[table].wrapping_add(objects.wrapping_mul(OBJECT_STRIDE[table])),
        ),
    }
}

/// The lift's calls as the rewrite makes them, with the record's position
/// and the block's handle.
fn as_calls(logged: &[Logged], record: u32) -> Vec<Call> {
    logged
        .iter()
        .map(|entry| match *entry {
            Logged::Refresh => Call {
                id: 1,
                object: record,
                arg: 0,
            },
            Logged::Notify {
                table,
                slot,
                handle,
            } => Call {
                id: table as u32 + 2,
                object: record.wrapping_add(object_offset(table, slot)),
                arg: handle,
            },
        })
        .collect()
}

/// Runs the rewrite of the remap on a case.
fn run_rewrite_remap(case: &Case) -> (Outcome, u32, u32) {
    let record = case.record.clone();
    let block = case.block.clone();
    let record_address = address_of(&record);
    install(record_address, case.pokes);
    let block_address = address_of(&block);
    let answer = fn_00a1ed80::rw_00a1ed80(record_address, block_address);
    let calls = take_calls();
    (
        Outcome {
            answer,
            record,
            calls,
        },
        record_address,
        block_address,
    )
}

/// Runs the rewrite of the transform on a case.
fn run_rewrite_apply(case: &Case) -> (Outcome, u32, u32) {
    let record = case.record.clone();
    let block = case.block.clone();
    let record_address = address_of(&record);
    install(record_address, case.pokes);
    let block_address = address_of(&block);
    let answer = fn_00a23720::rw_00a23720(record_address, block_address);
    let calls = take_calls();
    (
        Outcome {
            answer,
            record,
            calls,
        },
        record_address,
        block_address,
    )
}

/// Runs the lifted remap on a case.
fn run_lift_remap(case: &Case, record_address: u32, block_address: u32) -> Outcome {
    let mut record = case.record.clone();
    let mut owner = decode(&record);
    let frame = decode_block(&case.block);
    let mut hooks = Recorder::new(case.pokes);
    let end = owner.remap_vectors(&mut hooks, &frame, block_address);
    encode_into(&owner, &mut record);
    Outcome {
        answer: rebuild(end, record_address),
        record,
        calls: as_calls(&hooks.calls, record_address),
    }
}

/// Runs the lifted transform on a case.
fn run_lift_apply(case: &Case, record_address: u32, block_address: u32) -> Outcome {
    let mut record = case.record.clone();
    let mut owner = decode(&record);
    let matrix = decode_block(&case.block);
    let mut hooks = Recorder::new(case.pokes);
    let end = owner.apply_matrix(&mut hooks, &matrix, block_address);
    encode_into(&owner, &mut record);
    Outcome {
        answer: rebuild(end, record_address),
        record,
        calls: as_calls(&hooks.calls, record_address),
    }
}

/// Describes the first difference between two outcomes, for a failure.
fn describe(lift: &Outcome, rewrite: &Outcome) -> String {
    if lift.answer != rewrite.answer {
        return format!(
            "answer: lift {:#x}, rewrite {:#x}",
            lift.answer, rewrite.answer
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
    format!(
        "calls: lift {:?}, rewrite {:?}",
        lift.calls, rewrite.calls
    )
}

/// Which knobs a wrong version turns. Every field off is the lift itself.
#[derive(Debug, Clone, Copy, Default)]
struct Knobs {
    /// Flag byte taken at the slot alone, not at table plus slot.
    flag_slot_only: bool,
    /// Walk the tables from the last to the first.
    reverse_tables: bool,
    /// The refresh hook is not called.
    skip_refresh: bool,
    /// The walk's answer keeps the first table that ran.
    first_table_answer: bool,
    /// The answer is one object short.
    object_one_short: bool,
    /// Table 3 notifies through callee 4.
    callee_shift: bool,
    /// The remap reads each count once per table.
    remap_reads_once: bool,
    /// The transform re-reads each count on every iteration.
    apply_reads_per_iteration: bool,
    /// The remap's third difference uses the second bias.
    remap_dz_bias1: bool,
    /// The transform's first row uses the first weight in its first product.
    apply_row0_w0: bool,
    /// The transform's second row adds the third bias.
    apply_row1_bias2: bool,
    /// The transform leaves the spare word as it found it.
    apply_keep_spare: bool,
}

/// Which routine a knob copy runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Routine {
    Remap,
    Apply,
}

/// A copy of the remap or the transform with knobs, run against the same
/// case. With every knob off it is the lift's arithmetic and walk written
/// out again, so a difference from the lift would be a fault in the copy.
fn run_knobs(routine: Routine, knobs: Knobs, case: &Case, record_address: u32, block_address: u32) -> Outcome {
    let mut record = case.record.clone();
    let mut owner = decode(&record);
    let block = decode_block(&case.block);
    let mut hooks = Recorder::new(case.pokes);
    let m = block.rows;
    let b = block.bias;
    let reads = match routine {
        Routine::Remap => {
            if knobs.remap_reads_once {
                CountReads::OncePerTable
            } else {
                CountReads::PerIteration
            }
        }
        Routine::Apply => {
            if knobs.apply_reads_per_iteration {
                CountReads::PerIteration
            } else {
                CountReads::OncePerTable
            }
        }
    };
    match routine {
        Routine::Remap => {
            let dx = sub(owner.anchor[0], b[0]);
            let dy = sub(owner.anchor[1], b[1]);
            let dz = if knobs.remap_dz_bias1 {
                sub(owner.anchor[2], b[1])
            } else {
                sub(owner.anchor[2], b[2])
            };
            owner.out[0] = add(add(mul(m[0][1], dy), mul(m[0][0], dx)), mul(m[0][2], dz));
            owner.out[1] = add(add(mul(m[1][1], dy), mul(dx, m[1][0])), mul(m[1][2], dz));
            owner.out[2] = add(add(mul(m[2][1], dy), mul(dx, m[2][0])), mul(m[2][2], dz));
        }
        Routine::Apply => {
            let w = owner.out;
            let row = |r: usize, first_weight: usize, bias: usize| {
                add(
                    add(
                        add(mul(m[1][r], w[first_weight]), mul(m[0][r], w[0])),
                        mul(m[2][r], w[2]),
                    ),
                    b[bias],
                )
            };
            let row0 = row(0, if knobs.apply_row0_w0 { 0 } else { 1 }, 0);
            let row1 = row(1, 1, if knobs.apply_row1_bias2 { 2 } else { 1 });
            let row2 = row(2, 1, 2);
            owner.out = [row0, row1, row2];
            if !knobs.apply_keep_spare {
                owner.spare = 0;
            }
        }
    }
    if !knobs.skip_refresh {
        hooks.log_refresh(&mut owner);
    }
    let end = knob_walk(&mut owner, &mut hooks, block_address, reads, knobs);
    encode_into(&owner, &mut record);
    let mut end = end;
    if knobs.object_one_short {
        if let WalkEnd::Object { table, objects } = end {
            end = WalkEnd::Object {
                table,
                objects: objects - 1,
            };
        }
    }
    let mut calls = as_calls(&hooks.calls, record_address);
    if knobs.callee_shift {
        for call in calls.iter_mut() {
            if call.id == 5 {
                call.id = 4;
            }
        }
    }
    Outcome {
        answer: rebuild(end, record_address),
        record,
        calls,
    }
}

/// The table walk with knobs: the same loop as the lift, with the knobs
/// changing one of its choices each.
fn knob_walk(
    owner: &mut TaskSubobjects,
    hooks: &mut Recorder,
    handle: u32,
    reads: CountReads,
    knobs: Knobs,
) -> WalkEnd {
    let order: [usize; 4] = if knobs.reverse_tables {
        [3, 2, 1, 0]
    } else {
        [0, 1, 2, 3]
    };
    let mut end = WalkEnd::None;
    for table in order {
        let first = owner.counts[table];
        if first <= 0 {
            continue;
        }
        let mut walked: u32 = 0;
        loop {
            let slot = walked as usize;
            let flag_index = if knobs.flag_slot_only {
                slot
            } else {
                table + slot
            };
            if owner.flags[flag_index] != 0 {
                hooks.log_notify(owner, table, slot, handle);
            }
            walked += 1;
            let count = match reads {
                CountReads::PerIteration => owner.counts[table],
                CountReads::OncePerTable => first,
            };
            if i64::from(walked) >= i64::from(count) {
                break;
            }
        }
        let candidate = if table == 0 {
            WalkEnd::Count(walked)
        } else {
            WalkEnd::Object {
                table,
                objects: walked,
            }
        };
        if !(knobs.first_table_answer && end != WalkEnd::None) {
            end = candidate;
        }
    }
    end
}

/// Checks the lift against the rewrite on every generated case of one
/// routine, and returns how many cases reached each kind of answer.
fn check_routine(routine: Routine, seed: u32) -> [u32; 4] {
    let mut rng = Xorshift::new(seed);
    let mut kinds = [0u32; 4];
    for number in 0..CASES {
        let case = generate(&mut rng);
        let (rewrite, record_address, block_address) = match routine {
            Routine::Remap => run_rewrite_remap(&case),
            Routine::Apply => run_rewrite_apply(&case),
        };
        let lift = match routine {
            Routine::Remap => run_lift_remap(&case, record_address, block_address),
            Routine::Apply => run_lift_apply(&case, record_address, block_address),
        };
        assert!(
            lift == rewrite,
            "case {number}: {}",
            describe(&lift, &rewrite)
        );
        match rewrite.answer {
            0 => kinds[0] += 1,
            _ => kinds[1] += 1,
        }
        if rewrite.calls.len() > 1 {
            kinds[2] += 1;
        }
        if rewrite.calls.iter().any(|call| call.id == 5) {
            kinds[3] += 1;
        }
    }
    kinds
}

/// The knob copy with every knob off must agree with the lift: otherwise
/// the copy used for the wrong versions is not the lift's form.
fn check_copy_matches(routine: Routine, seed: u32) {
    let mut rng = Xorshift::new(seed);
    for number in 0..CASES {
        let case = generate(&mut rng);
        let record_address = address_of(&case.record);
        let block_address = address_of(&case.block);
        let lift = match routine {
            Routine::Remap => run_lift_remap(&case, record_address, block_address),
            Routine::Apply => run_lift_apply(&case, record_address, block_address),
        };
        let copy = run_knobs(routine, Knobs::default(), &case, record_address, block_address);
        assert!(copy == lift, "copy, case {number}: {}", describe(&copy, &lift));
    }
}

/// Counts the cases on which a wrong version differs from the rewrite.
fn caught_by(routine: Routine, seed: u32, knobs: Knobs) -> u32 {
    let mut rng = Xorshift::new(seed);
    let mut differing = 0;
    for _ in 0..CASES {
        let case = generate(&mut rng);
        let (rewrite, record_address, block_address) = match routine {
            Routine::Remap => run_rewrite_remap(&case),
            Routine::Apply => run_rewrite_apply(&case),
        };
        let wrong = run_knobs(routine, knobs, &case, record_address, block_address);
        if wrong != rewrite {
            differing += 1;
        }
    }
    differing
}

#[test]
fn remap_lift_matches_rewrite_on_generated_cases() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let kinds = check_routine(Routine::Remap, SEED_REMAP);
    eprintln!("remap coverage [no-answer, answer, multi-call, table-3 notify]: {kinds:?}");
    // Coverage: zero and nonzero answers, multi-call walks and the fifth
    // table-callee path must all come up often.
    assert!(kinds[0] > 0 && kinds[1] > 0, "answers: {kinds:?}");
    assert!(kinds[2] > CASES / 2, "multi-call cases: {kinds:?}");
    assert!(kinds[3] > CASES / 10, "table-3 notify cases: {kinds:?}");
}

#[test]
fn apply_lift_matches_rewrite_on_generated_cases() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let kinds = check_routine(Routine::Apply, SEED_APPLY);
    eprintln!("transform coverage [no-answer, answer, multi-call, table-3 notify]: {kinds:?}");
    assert!(kinds[0] > 0 && kinds[1] > 0, "answers: {kinds:?}");
    assert!(kinds[2] > CASES / 2, "multi-call cases: {kinds:?}");
    assert!(kinds[3] > CASES / 10, "table-3 notify cases: {kinds:?}");
}

#[test]
fn knob_copies_with_no_knob_match_the_lift() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    check_copy_matches(Routine::Remap, SEED_REMAP);
    check_copy_matches(Routine::Apply, SEED_APPLY);
}

#[test]
fn wrong_remap_versions_are_caught() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let cases: [(&str, Knobs); 8] = [
        ("flag index without the table", Knobs { flag_slot_only: true, ..Knobs::default() }),
        ("tables walked backwards", Knobs { reverse_tables: true, ..Knobs::default() }),
        ("refresh hook skipped", Knobs { skip_refresh: true, ..Knobs::default() }),
        ("first table's answer kept", Knobs { first_table_answer: true, ..Knobs::default() }),
        ("answer one object short", Knobs { object_one_short: true, ..Knobs::default() }),
        ("table 3 through callee 4", Knobs { callee_shift: true, ..Knobs::default() }),
        ("counts read once per table", Knobs { remap_reads_once: true, ..Knobs::default() }),
        ("third difference with the second bias", Knobs { remap_dz_bias1: true, ..Knobs::default() }),
    ];
    for (name, knobs) in cases {
        let differing = caught_by(Routine::Remap, SEED_REMAP, knobs);
        eprintln!("remap wrong version {name}: {differing} of {CASES} cases differ");
        assert!(differing > 0, "wrong remap not caught: {name}");
    }
}

#[test]
fn wrong_apply_versions_are_caught() {
    let _guard = TEST_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
    plant_stubs();
    let cases: [(&str, Knobs); 8] = [
        ("flag index without the table", Knobs { flag_slot_only: true, ..Knobs::default() }),
        ("tables walked backwards", Knobs { reverse_tables: true, ..Knobs::default() }),
        ("refresh hook skipped", Knobs { skip_refresh: true, ..Knobs::default() }),
        ("first table's answer kept", Knobs { first_table_answer: true, ..Knobs::default() }),
        ("answer one object short", Knobs { object_one_short: true, ..Knobs::default() }),
        ("counts re-read every iteration", Knobs { apply_reads_per_iteration: true, ..Knobs::default() }),
        ("first row's product with the first weight", Knobs { apply_row0_w0: true, ..Knobs::default() }),
        ("second row with the third bias", Knobs { apply_row1_bias2: true, ..Knobs::default() }),
    ];
    for (name, knobs) in cases {
        let differing = caught_by(Routine::Apply, SEED_APPLY, knobs);
        eprintln!("transform wrong version {name}: {differing} of {CASES} cases differ");
        assert!(differing > 0, "wrong transform not caught: {name}");
    }
    let spare = caught_by(
        Routine::Apply,
        SEED_APPLY,
        Knobs {
            apply_keep_spare: true,
            ..Knobs::default()
        },
    );
    eprintln!("transform wrong version spare word kept: {spare} of {CASES} cases differ");
    assert!(spare > 0, "wrong transform not caught: spare word kept");
}
