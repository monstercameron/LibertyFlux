// original: 0x0096D490 function_0096D490

//! Rewrite of the function at 0x0096D490, specification.
//!
//! The function first calls a status helper with the argument minus one and reads its answer
//! as the status. It then takes one of three routes:
//!   * the shared tail, when the status is 0x26 or 0x27, or when the shared-tail flag byte is set;
//!   * route A, when the shared-tail flag byte is clear and the A-route flag byte is set;
//!   * route B otherwise.
//!
//! The shared tail copies fifteen words of a source block (twelve are read, three are zero) into
//! a local buffer and makes two helper calls on it. It writes the five index-scaled table words
//! (one, then zeros). It then makes either the closing call with fixed arguments (return flag
//! clear) or the return calls (return flag set). Last, when the exit flag byte is clear, it writes
//! three source words and a zero into the object at offset 0x580.
//!
//! Route A asks a check helper for an answer. A nonzero answer goes to a helper that takes it as
//! its object, then a fill helper writes twelve words and the frame block is made from them (three
//! sign-flipped words and one copied word). A zero answer skips that helper: the fill helper writes
//! the source block instead and the frame block stays zero. Two helpers take the frame block, the
//! table is written as in the shared tail, the return calls run, and the exit-clear write-back
//! writes zeros.
//!
//! Route B runs its common part first. It takes a normalised direction from the twenty-word output
//! block, asks a check helper for an answer and clamps a running distance with the answer's
//! sub-object position when the answer is nonzero (status two may clear the clamp). It stores the
//! frame vector (position plus normal times clamp, with the fourth word from the output block),
//! and with the exit flag clear writes it into the object at 0x580. It calls two helpers, writes the
//! larger of two constants to the first table word, runs a four-step loop that zeroes four table
//! words, and sets two object words. The status then selects one of the status cases (1, 2, 3, 12,
//! 14, 22 to 24, and 0x24, which reads the field from a getter's object). Every other status, zero,
//! and every status outside the switch table takes no case body: it writes minus one to the field
//! at 0x31D8 (0x24 writes the getter's field), and runs the common tail.
//!
//! The common tail of route B writes a table word when the flag at 0x3211 is set and the zero-table
//! word at the index is positive, then calls the state helper and sets the flag byte at 0x2FA0.
//! When the global state word is not one, two sub-conditions decide whether the object route runs.
//! If the words at 0x12088B4 and 0xF1C040 differ, the object route runs. If they are equal, the
//! object route runs only when the word at 0x1037720 equals 0x12; otherwise the function returns
//! with nothing further written. The object route calls the object helper and branches on the
//! object's word at 0x38C: eight writes its own block (route B object eight), any value other than
//! seven writes the other-object block, and seven continues in the object-seven route, which
//! writes a table word and, with the global exit byte clear, four global words into the object.

use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global};

/// Byte flag: when set, the tail is taken on any status value.
const FLAG_SHARED_TAIL: u32 = 0x0121_8492;
/// Byte flag: when set, the tail takes the return path.
const FLAG_RETURN_PATH: u32 = 0x0121_8493;
/// Byte flag: when set (and the shared-tail flag is clear), route A runs.
const FLAG_A_ROUTE: u32 = 0x0121_849F;
/// Byte flag: when clear, the exit write-back runs.
const FLAG_EXIT: u32 = 0x0121_8497;
/// Object passed to the status helper.
const STATUS_HELPER_OBJECT: u32 = 0x0128_E310;
/// Source block of the snapshot and of the route-A and route-B fills.
const SNAPSHOT_SOURCE: u32 = 0x0128_E310;
/// Object passed to the two snapshot helpers.
const SNAPSHOT_HELPER_OBJECT: u32 = 0x0115_D9A0;
/// Object passed to the closing helper, and the object of the stdcall helper of the object-seven route.
const CLOSING_HELPER_OBJECT: u32 = 0x0115_DEF0;
/// Table index word; every table write is scaled by it.
const TABLE_INDEX: u32 = 0x0115_F5F4;
/// First table base (receives one).
const TABLE_ONE_BASE: u32 = 0x0115_F470;
/// Four further table bases (receive zero in the tail and route A).
const TABLE_ZERO_BASE_A: u32 = 0x0115_F480;
const TABLE_ZERO_BASE_B: u32 = 0x0115_F490;
const TABLE_ZERO_BASE_C: u32 = 0x0115_F4A0;
const TABLE_ZERO_BASE_D: u32 = 0x0115_F4B0;
/// Float constants of the return path and of the route-A sign mask.
const SCALE_CONSTANT: u32 = 0x00FE_8728;
const SIGN_MASK_CONSTANT: u32 = 0x00FE_8FA0;
const FLOAT_INPUT_FIRST: u32 = 0x0103_7A0C;
const FLOAT_INPUT_SECOND: u32 = 0x0103_7A10;
const FLOAT_INPUT_THIRD: u32 = 0x0103_7A14;
/// Source words of the exit write-back in the tail route.
const EXIT_SOURCE_FIRST: u32 = 0x0128_E340;
const EXIT_SOURCE_SECOND: u32 = 0x0128_E344;
const EXIT_SOURCE_THIRD: u32 = 0x0128_E348;
/// Offset in the object of the first exit word; the four words are consecutive.
const EXIT_OBJECT_OFFSET: u32 = 0x580;
/// Literal words passed to the helpers.
const FLOAT_ONE_BITS: u32 = 0x3F80_0000;
const CLOSING_ARG_FIRST: u32 = 0x3F34_FDF4;
const CLOSING_ARG_SECOND: u32 = 0xBF34_FDF4;
/// Callee ids of the stub table (the ids the stage contracts script).
const CALLEE_STATUS: u32 = 1;
const CALLEE_ROUTE_A_CHECK: u32 = 2;
const CALLEE_ROUTE_A_OUT: u32 = 3;
const CALLEE_ROUTE_A_FILL: u32 = 4;
const CALLEE_ROUTE_A_FIRST: u32 = 5;
const CALLEE_ROUTE_A_SECOND: u32 = 6;
const CALLEE_FLOAT: u32 = 7;
const CALLEE_CLOSE: u32 = 8;
const CALLEE_SNAPSHOT_A: u32 = 18;
const CALLEE_SNAPSHOT_B: u32 = 19;
const CALLEE_STATUS_TWO_HELPER: u32 = 9;
const CALLEE_STATUS_TWO_OBJECT: u32 = 10;
const CALLEE_STATUS_TWO_FILL: u32 = 25;
const CALLEE_ROUTE_B_STATE: u32 = 14;
const CALLEE_ROUTE_B_OBJECT: u32 = 16;
const CALLEE_ROUTE_B_FLAG: u32 = 15;
const CALLEE_ROUTE_B_FIRST_FILL: u32 = 24;
const CALLEE_OBJECT_HELPER: u32 = 17;
const CALLEE_GETTER: u32 = 21;
const CALLEE_STATUS_THREE_FLOAT: u32 = 20;
const CALLEE_STATUS_THREE_STEP: u32 = 11;

/// Field offsets of the object at `this` used by route B.
const FIELD_232C: u32 = 0x0000_232C;
const FIELD_31D8: u32 = 0x0000_31D8;
const FIELD_3211: u32 = 0x0000_3211;
const FIELD_2FA0: u32 = 0x0000_2FA0;
/// Offset of the call slot in the vtable of an object on the object route.
const VTABLE_SLOT_OFFSET: u32 = 0x0000_000C;
/// Four words written into the object on the exit-clear branch.
const FIELD_580: u32 = 0x0000_0580;
const FIELD_584: u32 = 0x0000_0584;
const FIELD_588: u32 = 0x0000_0588;
const FIELD_58C: u32 = 0x0000_058C;
/// Offset of the object word compared with seven and eight on the object route.
const OBJECT_WORD_OFFSET: u32 = 0x0000_038C;
/// Loop of route B: its first running offset, and its count (the step is one word).
const LOOP_OFFSET_START: u32 = 0x0000_0564;
const LOOP_ITERATIONS: u32 = 4;
/// Base of the global table written by the loop of route B.
const LOOP_TABLE_BASE: u32 = 0x0115_DEF0;
/// Object passed to the state helper, and to the object helper.
const STATE_HELPER_OBJECT: u32 = 0x0128_E400;
const OBJECT_HELPER_OBJECT: u32 = 0x0103_E498;
/// Global words read by route B.
const GLOBAL_STATE_WORD: u32 = 0x011F_7060;
const GLOBAL_OFFSET_WORD: u32 = 0x0103_7740;
const GLOBAL_TABLE_VALUE: u32 = 0x011F_70E8;
const GLOBAL_EXIT_BYTE: u32 = 0x011F_70EC;
/// Global words compared when the state word is not one: the first pair, and the third word
/// compared with the key.
const GLOBAL_WORD_A: u32 = 0x0120_88B4;
const GLOBAL_WORD_B: u32 = 0x00F1_C040;
const GLOBAL_WORD_C: u32 = 0x0103_7720;
const STATE_KEY_WORD: u32 = 0x12;
/// Global flag bytes read by the flag selection of the common tail.
const GLOBAL_FLAG_BYTE_A: u32 = 0x0128_3049;
const GLOBAL_FLAG_BYTE_B: u32 = 0x0129_576E;
/// Status value of the switch case with table entry zero (status one) and the other case statuses.
const STATUS_CASE_1: u32 = 1;
const STATUS_CASE_2: u32 = 2;
const STATUS_CASE_3: u32 = 3;
const STATUS_CASE_12: u32 = 12;
const STATUS_CASE_14: u32 = 14;
const STATUS_CASE_24: u32 = 0x24;
/// The getter is called with these arguments by the cases.
const STATUS_CASE_3_ARGUMENT: u32 = 3;
const STATUS_CASE_12_ARGUMENT: u32 = 0x0C;
const STATUS_CASE_14_ARGUMENT: u32 = 0x20;
/// Field of the getter's object read by status 0x24.
const FIELD_184: u32 = 0x0000_0184;
/// Words of the getter's object that the status-three case reads.
const GETTER_FIELD_WORD_A: u32 = 0x0000_014C;
const GETTER_FIELD_WORD_B: u32 = 0x0000_0150;
/// Offsets of the three getter-object floats read by the status-12 case (second, first, third).
const GETTER_FIELD_SECOND: u32 = 0x0000_0258;
const GETTER_FIELD_FIRST: u32 = 0x0000_025C;
const GETTER_FIELD_THIRD: u32 = 0x0000_0260;
/// Byte global that selects the step-loop shape of status three (zero: the two-step shape).
const CASE3_SELECT_BYTE: u32 = 0x0103_79D1;
/// Float constant of the status-three offset.
const CASE3_OFFSET_WORD: u32 = 0x0103_79D4;
/// Constant of the status-two and object routes (the complement of the scale is taken from it).
const CASE2_CONSTANT_WORD: u32 = 0x0103_7A24;
/// Value written to the object word at 0x232C by the status-22/23/24 cases and by object seven.
const TAIL_CASE_OBJECT_WORD: u32 = 0x3F28_F5C3;
/// Float constants of the normalisation, of the distance clamp and of the status cases.
const CLAMP_INITIAL_WORD: u32 = 0x0103_79D8;
const CLAMP_RADIUS_WORD: u32 = 0x00FE_87E4;
const NORMAL_SCALE_WORD: u32 = 0x00FE_88E8;
const CONSTANT_SECOND_WORD: u32 = 0x00FE_870C;
/// Field offsets of the distance block: the sub-object pointer and its three position floats.
const FIELD_SUB_OBJECT: u32 = 0x0000_0020;
const FIELD_SUB_POSITION_X: u32 = 0x0000_0030;
const FIELD_SUB_POSITION_Y: u32 = 0x0000_0034;
const FIELD_SUB_POSITION_Z: u32 = 0x0000_0038;
/// Object words written from the frame vector when the exit flag is clear.
const FIELD_VECTOR_WRITE_BACK: u32 = 0x0000_0580;
/// Fields of a slot answer object used by status two and by object eight.
const FIELD_ANSWER_FLAGS: u32 = 0x0000_0028;
const FIELD_ANSWER_KIND: u32 = 0x0000_1304;
const FIELD_ANSWER_SUB_OBJECT: u32 = 0x0000_0020;
const FIELD_ANSWER_SUB_BASE: u32 = 0x0000_0030;
const FIELD_ANSWER_OWN_BASE: u32 = 0x0000_0010;
/// Byte of the second object that the status-two clamp test reads.
const FIELD_STATUS_TWO_FLAG: u32 = 0x0000_022C;
/// Object whose value is the status-two helper's this pointer.
const STATUS_TWO_HELPER_OBJECT: u32 = 0x0103_E498;
/// Global whose value is the getter's this pointer.
const GETTER_GLOBAL: u32 = 0x0103_E49C;
/// Global of the object-seven route's stdcall helper argument.
const OBJECT_SEVEN_STDCALL_OBJECT: u32 = 0x011F_F030;
/// Globals written into the object by the object-seven exit-clear route.
const OBJECT_SEVEN_GLOBAL_FIRST: u32 = 0x0121_8390;
const OBJECT_SEVEN_GLOBAL_SECOND: u32 = 0x0121_8394;
const OBJECT_SEVEN_GLOBAL_THIRD: u32 = 0x0121_8398;
const OBJECT_SEVEN_GLOBAL_FOURTH: u32 = 0x0121_839C;

/// Reads one 32-bit word at a file virtual address.
#[inline(always)]
unsafe fn read_word(file_va: u32) -> u32 {
    unsafe { core::ptr::read_volatile(global::<u32>(file_va)) }
}

/// Reads one byte at a file virtual address.
#[inline(always)]
unsafe fn read_byte(file_va: u32) -> u8 {
    unsafe { core::ptr::read_volatile(global::<u8>(file_va)) }
}

/// Writes one word at `base + index * 4`; the index is read again for each write.
#[inline(always)]
unsafe fn write_indexed(base: u32, value: u32) {
    let index = unsafe { read_word(TABLE_INDEX) };
    let address = base.wrapping_add(index.wrapping_mul(4));
    unsafe { core::ptr::write_volatile(global::<u32>(address), value) };
}

/// Reads a word at a heap address.
#[inline(always)]
unsafe fn read_heap_word(address: u32) -> u32 {
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

/// Reads a byte at a heap address.
#[inline(always)]
unsafe fn read_heap_byte(address: u32) -> u8 {
    unsafe { core::ptr::read_volatile(address as *const u8) }
}

/// Writes a word at a heap address.
#[inline(always)]
unsafe fn write_heap_word(address: u32, value: u32) {
    unsafe { core::ptr::write_volatile(address as *mut u32, value) };
}

/// Writes a byte at a heap address.
#[inline(always)]
unsafe fn write_heap_byte(address: u32, value: u8) {
    unsafe { core::ptr::write_volatile(address as *mut u8, value) };
}

/// Float multiply, add, subtract and divide with the operands pinned in order (the first operand is
/// the destination on the original's SSE instructions, so NaN payloads match).
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fdiv(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

/// Bits of a float input multiplied by the scale constant. The input is the first operand.
#[inline(always)]
unsafe fn scaled_bits(file_va: u32) -> u32 {
    let input = core::hint::black_box(f32::from_bits(unsafe { read_word(file_va) }));
    let scale = core::hint::black_box(f32::from_bits(unsafe { read_word(SCALE_CONSTANT) }));
    (input * scale).to_bits()
}

/// Writes the four exit words into the object at `this`, consecutively from the exit offset.
unsafe fn exit_write_back(this: u32, words: [u32; 4]) {
    for (i, word) in words.iter().enumerate() {
        let address = this.wrapping_add(EXIT_OBJECT_OFFSET + 4 * i as u32);
        unsafe { write_heap_word(address, *word) };
    }
}

/// Return calls of the tail and of route A: the first float call takes the first product, the second
/// float call takes the second product, then the closing call takes the second product, the first
/// product and the first input word.
unsafe fn return_calls(product_first: u32, product_second: u32, first_input: u32) {
    unsafe {
        let _ = callee_cdecl!(CALLEE_FLOAT, u32, product_first);
        let _ = callee_cdecl!(CALLEE_FLOAT, u32, product_second);
        let _ = callee_thiscall!(
            CALLEE_CLOSE,
            u32,
            global::<u8>(CLOSING_HELPER_OBJECT) as u32,
            product_second,
            product_first,
            first_input,
        );
    }
}

/// Writes the table words of the tail and route A: one at the first base, zeros at the four others.
unsafe fn write_common_table(table_one_base: u32) {
    unsafe {
        write_indexed(table_one_base, FLOAT_ONE_BITS);
        write_indexed(TABLE_ZERO_BASE_A, 0);
        write_indexed(TABLE_ZERO_BASE_B, 0);
        write_indexed(TABLE_ZERO_BASE_C, 0);
        write_indexed(TABLE_ZERO_BASE_D, 0);
    }
}

/// Shared tail. Copies the fifteen-word source snapshot (three words left zero) into a local
/// buffer and makes the two snapshot calls. It then writes the five table words. Without the return
/// flag it makes the closing call with fixed arguments; with it, the return calls. The exit
/// write-back copies three source words and a zero when the exit flag is clear.
unsafe fn shared_tail(this: u32) {
    unsafe {
        let return_flag = read_byte(FLAG_RETURN_PATH);
        let mut snapshot = [0u32; 15];
        for index in [0usize, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14] {
            snapshot[index] = read_word(SNAPSHOT_SOURCE + 4 * index as u32);
        }
        let snapshot_ptr = snapshot.as_mut_ptr() as u32;

        let _ = callee_thiscall!(
            CALLEE_SNAPSHOT_A,
            u32,
            global::<u8>(SNAPSHOT_HELPER_OBJECT) as u32,
            snapshot_ptr,
            0u32,
            FLOAT_ONE_BITS,
        );
        let _ = callee_thiscall!(
            CALLEE_SNAPSHOT_B,
            u32,
            global::<u8>(SNAPSHOT_HELPER_OBJECT) as u32,
            snapshot_ptr,
            0u32,
            FLOAT_ONE_BITS,
        );

        write_common_table(TABLE_ONE_BASE);

        if return_flag == 0 {
            let _ = callee_thiscall!(
                CALLEE_CLOSE,
                u32,
                global::<u8>(CLOSING_HELPER_OBJECT) as u32,
                CLOSING_ARG_FIRST,
                CLOSING_ARG_SECOND,
                0u32,
            );
        } else {
            let first_input = read_word(FLOAT_INPUT_FIRST);
            let product_first = scaled_bits(FLOAT_INPUT_THIRD);
            let product_second = scaled_bits(FLOAT_INPUT_SECOND);
            return_calls(product_first, product_second, first_input);
        }

        if read_byte(FLAG_EXIT) == 0 {
            exit_write_back(
                this,
                [
                    read_word(EXIT_SOURCE_FIRST),
                    read_word(EXIT_SOURCE_SECOND),
                    read_word(EXIT_SOURCE_THIRD),
                    0,
                ],
            );
        }
    }
}

/// Route A. A check helper answers first. A nonzero answer goes to a helper that takes it as its
/// object, then a fill helper writes a block of twelve words. The frame block is made from that
/// block: three sign-flipped words and one copied word. A zero answer skips the first helper, and
/// the fill helper writes into the source block instead, leaving the frame block zero. Two further
/// helpers take the frame block, the table is written as in the tail, and the return calls and the
/// closing call run. The exit words are zeros, because this route never writes the frame words the
/// tail copies from.
unsafe fn route_a(this: u32) {
    unsafe {
        let mut out = [0u32; 12];
        let mut frame = [0u32; 4];
        let check = callee_cdecl!(CALLEE_ROUTE_A_CHECK, u32,);
        if check != 0 {
            let _ = callee_thiscall!(
                CALLEE_ROUTE_A_OUT,
                u32,
                check,
                out.as_mut_ptr() as u32,
                0x0000_04B5u32,
            );
            let _ = callee_thiscall!(
                CALLEE_ROUTE_A_FILL,
                u32,
                frame.as_mut_ptr() as u32,
                out.as_mut_ptr() as u32,
            );
            let sign = read_word(SIGN_MASK_CONSTANT);
            frame[0] = out[8] ^ sign;
            frame[1] = out[9] ^ sign;
            frame[2] = out[10] ^ sign;
            frame[3] = out[11];
        } else {
            // The fill helper writes into the source block itself; the frame stays zero.
            let _ = callee_thiscall!(
                CALLEE_ROUTE_A_FILL,
                u32,
                frame.as_mut_ptr() as u32,
                global::<u8>(SNAPSHOT_SOURCE) as u32,
            );
        }
        let _ = callee_stdcall!(
            CALLEE_ROUTE_A_FIRST,
            u32,
            frame.as_mut_ptr() as u32,
            0u32,
            FLOAT_ONE_BITS,
        );
        let _ = callee_stdcall!(
            CALLEE_ROUTE_A_SECOND,
            u32,
            frame.as_mut_ptr() as u32,
            0u32,
            FLOAT_ONE_BITS,
        );
        write_common_table(TABLE_ONE_BASE);

        let first_input = read_word(FLOAT_INPUT_FIRST);
        let product_first = scaled_bits(FLOAT_INPUT_THIRD);
        let product_second = scaled_bits(FLOAT_INPUT_SECOND);
        return_calls(product_first, product_second, first_input);

        if read_byte(FLAG_EXIT) == 0 {
            exit_write_back(this, [0, 0, 0, 0]);
        }
    }
}

/// Distance clamp of route B. The check answer is a pointer to an object whose sub-object (at
/// offset 0x20) has three position floats. The clamp is the distance from the frame position to
/// that point, less a radius, floored at zero, and it keeps the smaller of that and the running
/// clamp. A NaN distance is kept, as the original keeps it.
#[inline(always)]
unsafe fn distance_clamp(position: [f32; 3], check_object: u32, running_clamp: f32) -> f32 {
    unsafe {
        let sub_object = read_heap_word(check_object.wrapping_add(FIELD_SUB_OBJECT));
        let dx = fsub(position[0], f32::from_bits(read_heap_word(sub_object.wrapping_add(FIELD_SUB_POSITION_X))));
        let dy = fsub(position[1], f32::from_bits(read_heap_word(sub_object.wrapping_add(FIELD_SUB_POSITION_Y))));
        let dz = fsub(position[2], f32::from_bits(read_heap_word(sub_object.wrapping_add(FIELD_SUB_POSITION_Z))));
        let squared = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let distance = fsub(squared.sqrt(), f32::from_bits(read_word(CLAMP_RADIUS_WORD)));
        let floored = if 0.0 > distance { 0.0 } else { distance };
        if floored > running_clamp { running_clamp } else { floored }
    }
}

/// Unit-length words of out[4..6] (three words, each multiplied by the reciprocal length).
/// A zero squared length gives a zero reciprocal; a NaN length takes the square-root path.
#[inline(always)]
unsafe fn normalised_direction(out: &[u32; 20]) -> [f32; 3] {
    unsafe {
        let x = f32::from_bits(out[4]);
        let y = f32::from_bits(out[5]);
        let z = f32::from_bits(out[6]);
        let length_squared = fadd(fadd(fmul(y, y), fmul(x, x)), fmul(z, z));
        let reciprocal = if length_squared == 0.0 {
            0.0
        } else {
            fdiv(f32::from_bits(read_word(NORMAL_SCALE_WORD)), length_squared.sqrt())
        };
        [fmul(x, reciprocal), fmul(y, reciprocal), fmul(z, reciprocal)]
    }
}

/// Clamps a float answer to the range zero to one. A value below zero becomes +0.0 and a value above
/// one becomes one; any other value, a NaN included, is kept, as the original's two compares keep it.
#[inline(always)]
fn clamp_unit(value: f32, one: f32) -> f32 {
    if 0.0 > value {
        0.0
    } else if value > one {
        one
    } else {
        value
    }
}

/// Status 12 (table entry three): the getter is called with the argument 0x0C. The output helper
/// and the stdcall helper run as in the other cases. The weight comes from three floats of the
/// getter's object: when the first two are equal the weight is zero; otherwise t is the ratio of
/// the third-minus-second to the first-minus-second, a negative t gives weight one, and otherwise
/// the weight is one minus t clamped at one. The fallback constant times the weight goes to the
/// table word at the index. No exit check.
#[inline(never)]
unsafe fn route_b_status_twelve(this: u32, out_ptr: u32) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_12_ARGUMENT, 0u32);
        let mut frame_block = [0u32; 4];
        let frame_block_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(CALLEE_STATUS_TWO_FILL, u32, this, getter_object, out_ptr, frame_block_ptr);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_block_ptr, 1u32, FLOAT_ONE_BITS);
        let first = f32::from_bits(read_heap_word(getter_object.wrapping_add(GETTER_FIELD_FIRST)));
        let second = f32::from_bits(read_heap_word(getter_object.wrapping_add(GETTER_FIELD_SECOND)));
        let third = f32::from_bits(read_heap_word(getter_object.wrapping_add(GETTER_FIELD_THIRD)));
        let one = f32::from_bits(read_word(NORMAL_SCALE_WORD));
        let weight = if first == second {
            0.0
        } else {
            let ratio = fdiv(fsub(third, second), fsub(first, second));
            if 0.0 > ratio {
                fsub(one, 0.0)
            } else {
                let clamped = if ratio > one { one } else { ratio };
                fsub(one, clamped)
            }
        };
        let fallback = fsub(one, f32::from_bits(read_word(CASE3_OFFSET_WORD)));
        let value = fmul(fallback, weight);
        let index = read_word(TABLE_INDEX);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            value.to_bits(),
        );
    }
}

/// Status 14 (table entry four): the getter is called with the argument 0x20; a null answer
/// leaves the case at once (the common tail). Otherwise the output helper runs with the answer,
/// the stdcall helper runs on the frame block, and the fallback constant goes to the table word
/// at the index before the common tail. No exit check.
#[inline(never)]
unsafe fn route_b_status_fourteen(this: u32, out_ptr: u32) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_14_ARGUMENT, 0u32);
        if getter_object == 0 {
            return;
        }
        let mut frame_block = [0u32; 4];
        let frame_block_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(CALLEE_STATUS_TWO_FILL, u32, this, getter_object, out_ptr, frame_block_ptr);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_block_ptr, 1u32, FLOAT_ONE_BITS);
        let fallback = fsub(f32::from_bits(read_word(NORMAL_SCALE_WORD)), f32::from_bits(read_word(CASE3_OFFSET_WORD)));
        let index = read_word(TABLE_INDEX);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            fallback.to_bits(),
        );
    }
}

/// Status 3 (table entry two). `position` is the frame vector that the common prefix stores before
/// the switch; `frame_w` is the fourth word the prefix copies (output word nineteen).
/// The getter is called with the status as its argument, and its object's float answer (callee 20,
/// in ST0) is clamped to zero to one as t. Two getter words select the shape: a zero word gives the
/// zero shape (no step, no table word; with the exit flag clear the position and frame words are
/// written). Otherwise the position is scaled by the case constant and moved by two steps. Step k
/// calls the step helper with the k-th getter word, the output block and a vector block, then the
/// block's vector times the step weight (one minus t for step zero, t for step one) times (one minus
/// the case constant) is added to the position. The table words at the index receive (one minus t)
/// times A to the first zero table and A times t to the second, where A is one minus the offset
/// constant. The selector byte chooses the blend shape instead: the two steps' vectors are blended
/// with the weights and written into a frame block for the stdcall helper, and the table word takes
/// A. With the exit flag clear the accumulated position and the frame's fourth word are written into
/// the object at 0x580 to 0x58C.
#[inline(never)]
unsafe fn route_b_status_three(this: u32, out_ptr: u32, position: [f32; 3], frame_w: u32) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_3_ARGUMENT, 0u32);
        let answer = callee_thiscall!(CALLEE_STATUS_THREE_FLOAT, f32, getter_object);
        let word_a = read_heap_word(getter_object.wrapping_add(GETTER_FIELD_WORD_A));
        let word_b = read_heap_word(getter_object.wrapping_add(GETTER_FIELD_WORD_B));
        if word_a == 0 || word_b == 0 {
            // Zero getter word: no step, no table word; the exit-clear write-back takes the three
            // position words and the fourth word.
            if read_byte(FLAG_EXIT) == 0 {
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK), position[0].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4), position[1].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 8), position[2].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 12), frame_w);
            }
            return;
        }
        let one = f32::from_bits(read_word(NORMAL_SCALE_WORD));
        let case_constant = f32::from_bits(read_word(CASE2_CONSTANT_WORD));
        let offset = f32::from_bits(read_word(CASE3_OFFSET_WORD));
        let t = clamp_unit(answer, one);
        let a = fsub(one, offset);
        let complement = fsub(one, case_constant);
        let steps = [fsub(one, t), t];
        let words = [word_a, word_b];
        let mut acc = [
            fmul(position[0], case_constant),
            fmul(position[1], case_constant),
            fmul(position[2], case_constant),
        ];
        if read_byte(CASE3_SELECT_BYTE) != 0 {
            // Blend shape: two steps, each into its own vector block; the accumulated position takes
            // the block's vector times the step weight. Then the fill helper, a blend of the two
            // blocks' vectors written with the fourth word into a frame block, the stdcall helper on
            // that block, the table word at the index set to a, and the exit-clear write-back.
            let mut blocks = [[0u32; 16]; 2];
            for step in 0..2usize {
                let block_ptr = blocks[step].as_mut_ptr() as u32;
                let _ = callee_thiscall!(CALLEE_STATUS_THREE_STEP, u32, this, words[step], out_ptr, block_ptr);
                let scale = fmul(steps[step], complement);
                let vector_x = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 12)));
                let vector_y = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 13)));
                let vector_z = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 14)));
                acc[0] = fadd(fmul(vector_x, scale), acc[0]);
                acc[1] = fadd(fmul(vector_y, scale), acc[1]);
                acc[2] = fadd(fmul(scale, vector_z), acc[2]);
            }
            let mut fill_this = [0u32; 16];
            let _ = callee_thiscall!(CALLEE_ROUTE_A_FILL, u32, fill_this.as_mut_ptr() as u32, out_ptr);
            let first_block = blocks[0].as_mut_ptr() as u32;
            let second_block = blocks[1].as_mut_ptr() as u32;
            let b0x = f32::from_bits(read_heap_word(first_block.wrapping_add(4 * 12)));
            let b0y = f32::from_bits(read_heap_word(first_block.wrapping_add(4 * 13)));
            let b0z = f32::from_bits(read_heap_word(first_block.wrapping_add(4 * 14)));
            let b1x = f32::from_bits(read_heap_word(second_block.wrapping_add(4 * 12)));
            let b1y = f32::from_bits(read_heap_word(second_block.wrapping_add(4 * 13)));
            let b1z = f32::from_bits(read_heap_word(second_block.wrapping_add(4 * 14)));
            let blend = [
                fadd(fmul(b0x, steps[0]), fmul(b1x, steps[1])),
                fadd(fmul(b0y, steps[0]), fmul(b1y, steps[1])),
                fadd(fmul(b0z, steps[0]), fmul(b1z, steps[1])),
            ];
            let mut frame_block = [0u32; 16];
            let frame_ptr = frame_block.as_mut_ptr() as u32;
            write_heap_word(frame_ptr.wrapping_add(4 * 12), blend[0].to_bits());
            write_heap_word(frame_ptr.wrapping_add(4 * 13), blend[1].to_bits());
            write_heap_word(frame_ptr.wrapping_add(4 * 14), blend[2].to_bits());
            write_heap_word(frame_ptr.wrapping_add(4 * 15), frame_w);
            let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_ptr, 1u32, FLOAT_ONE_BITS);
            let index = read_word(TABLE_INDEX);
            core::ptr::write_volatile(
                global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
                a.to_bits(),
            );
            if read_byte(FLAG_EXIT) == 0 {
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK), acc[0].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4), acc[1].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 8), acc[2].to_bits());
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 12), frame_w);
            }
            return;
        }
        // Two-step shape.
        let mut block = [0u32; 16];
        let block_ptr = block.as_mut_ptr() as u32;
        for step in 0..2usize {
            let _ = callee_thiscall!(CALLEE_STATUS_THREE_STEP, u32, this, words[step], out_ptr, block_ptr);
            let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, block_ptr, (step + 1) as u32, FLOAT_ONE_BITS);
            let scale = fmul(steps[step], complement);
            let vector_x = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 12)));
            let vector_y = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 13)));
            let vector_z = f32::from_bits(read_heap_word(block_ptr.wrapping_add(4 * 14)));
            acc[0] = fadd(fmul(vector_x, scale), acc[0]);
            acc[1] = fadd(fmul(vector_y, scale), acc[1]);
            acc[2] = fadd(fmul(vector_z, scale), acc[2]);
        }
        let index = read_word(TABLE_INDEX);
        let first_value = fmul(steps[0], a);
        let second_value = fmul(a, t);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            first_value.to_bits(),
        );
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_B.wrapping_add(index.wrapping_mul(4))),
            second_value.to_bits(),
        );
        if read_byte(FLAG_EXIT) == 0 {
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK), acc[0].to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4), acc[1].to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 8), acc[2].to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 12), frame_w);
        }
    }
}

/// Status-22/23/24 case (one template). Getter (with the status as its argument), the output helper
/// with a frame block, the stdcall helper on that block, the fallback constant at the index in the
/// zero table, and the object word at 0x232C set to a fixed value; then the common tail.
#[inline(never)]
unsafe fn route_b_status_tail_case(this: u32, out_ptr: u32, status: u32) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, status, 0u32);
        let mut frame_block = [0u32; 4];
        let frame_block_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(CALLEE_STATUS_TWO_FILL, u32, this, getter_object, out_ptr, frame_block_ptr);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_block_ptr, 1u32, FLOAT_ONE_BITS);
        let fallback = fsub(f32::from_bits(read_word(NORMAL_SCALE_WORD)), f32::from_bits(read_word(CASE3_OFFSET_WORD)));
        let index = read_word(TABLE_INDEX);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            fallback.to_bits(),
        );
        write_heap_word(this.wrapping_add(FIELD_232C), TAIL_CASE_OBJECT_WORD);
    }
}

/// Status-one case (table entry zero). Calls the getter with status one, the fill helper with a
/// frame block, and writes the fallback constant to the table word at the index. Exit flag clear:
/// the four-word write-back of the frame vector with the case constants.
#[inline(never)]
unsafe fn route_b_status_one(this: u32, out_ptr: u32, frame: [f32; 4]) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_1, 0u32);
        let mut frame_block = [0u32; 4];
        let frame_block_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(CALLEE_STATUS_TWO_FILL, u32, this, getter_object, out_ptr, frame_block_ptr);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_block_ptr, 1u32, FLOAT_ONE_BITS);
        let scale_constant = f32::from_bits(read_word(NORMAL_SCALE_WORD));
        let offset_constant = f32::from_bits(read_word(CASE3_OFFSET_WORD));
        let fallback = fsub(scale_constant, offset_constant);
        let index = read_word(TABLE_INDEX);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            fallback.to_bits(),
        );
        if read_byte(FLAG_EXIT) == 0 {
            let k2 = f32::from_bits(read_word(CASE2_CONSTANT_WORD));
            let complement = fsub(scale_constant, k2);
            let zero_block = [0.0f32; 3];
            let mut written = [0u32; 4];
            for axis in 0..3 {
                written[axis] = fadd(fmul(frame[axis], k2), fmul(zero_block[axis], complement)).to_bits();
            }
            written[3] = frame[3].to_bits();
            for (i, word) in written.iter().enumerate() {
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4 * i as u32), *word);
            }
        }
    }
}

/// Status-two case. The getter's object is the helper receiver; a helper with the output block and a
/// four-word frame block runs first, the frame block then goes to a stdcall helper, and the getter
/// object's first slot (a vtable call) answers. A nonzero answer whose word at 0x28, masked with
/// 0x3C0, is 0x80 and whose word at 0x1304 is 3 gives the table value +0.0; any other answer gives
/// the fallback. The value goes to the table word at the index. With the exit flag clear, four words
/// go into the object from the frame vector and the two constants.
#[inline(never)]
unsafe fn route_b_status_two(this: u32, out_ptr: u32, frame: [f32; 4]) {
    unsafe {
        let getter_this = read_word(GETTER_GLOBAL);
        let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_2, 0u32);
        let mut frame_block = [0u32; 4];
        let frame_block_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(CALLEE_STATUS_TWO_FILL, u32, this, getter_object, out_ptr, frame_block_ptr);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, frame_block_ptr, 1u32, FLOAT_ONE_BITS);
        let scale_constant = f32::from_bits(read_word(NORMAL_SCALE_WORD));
        let offset_constant = f32::from_bits(read_word(CASE3_OFFSET_WORD));
        let fallback = fsub(scale_constant, offset_constant);
        let vtable = read_heap_word(getter_object);
        let slot = read_heap_word(vtable.wrapping_add(VTABLE_SLOT_OFFSET));
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let answer = method(getter_object);
        let zeroed = answer != 0
            && (read_heap_word(answer.wrapping_add(FIELD_ANSWER_FLAGS)) & 0x0000_03C0) == 0x0000_0080
            && read_heap_word(answer.wrapping_add(FIELD_ANSWER_KIND)) == 3;
        let value = if zeroed { 0.0f32 } else { fallback };
        let index = read_word(TABLE_INDEX);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            value.to_bits(),
        );
        if read_byte(FLAG_EXIT) == 0 {
            // Exit flag clear: four words go into the object from the frame vector and the two
            // constants. The frame words the original reads at its offsets 0x70 to 0x78 are modelled as
            // zero (see the stage notes).
            let k2 = f32::from_bits(read_word(CASE2_CONSTANT_WORD));
            let complement = fsub(scale_constant, k2);
            let zero_block = [0.0f32; 3];
            let mut written = [0u32; 4];
            for axis in 0..3 {
                written[axis] = fadd(fmul(frame[axis], k2), fmul(zero_block[axis], complement)).to_bits();
            }
            written[3] = frame[3].to_bits();
            for (i, word) in written.iter().enumerate() {
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4 * i as u32), *word);
            }
        }
    }
}

/// Route B, the common part and the object routes. The route takes its normalised direction and
/// clamp from the output block and the check answer, writes the frame vector, runs the status
/// case, and then the flag and state gates, and the object route selected by the object's word.
/// Statuses outside the switch table (zero, 0x19 to 0x23, 0x25, above 0x27) have no case body:
/// they write minus one to the field at 0x31D8 and take the common tail, as the original does.
#[allow(clippy::too_many_arguments)]
unsafe fn route_b(this: u32, status: u32) {
    unsafe {
        let product_first = scaled_bits(FLOAT_INPUT_THIRD);
        let product_second = scaled_bits(FLOAT_INPUT_SECOND);
        let first_input = read_word(FLOAT_INPUT_FIRST);
        return_calls(product_first, product_second, first_input);

        // Twenty words: out[0..11] are written through the second fill's argument, out[12..19]
        // through the first fill's object pointer (the words the normalisation reads).
        let mut out = [0u32; 20];
        let mut frame_block = [0u32; 4];
        let out_ptr = out.as_mut_ptr() as u32;
        let frame_ptr = frame_block.as_mut_ptr() as u32;
        let _ = callee_thiscall!(
            CALLEE_ROUTE_B_FIRST_FILL,
            u32,
            out_ptr,
            global::<u8>(SNAPSHOT_SOURCE) as u32,
        );
        let _ = callee_thiscall!(CALLEE_ROUTE_A_FILL, u32, frame_ptr, out_ptr);

        // Normalisation: pure arithmetic on out[4..6]; the frame position is out[12..14].
        let normal = normalised_direction(&out);
        let position = [f32::from_bits(out[12]), f32::from_bits(out[13]), f32::from_bits(out[14])];
        let check = callee_cdecl!(CALLEE_ROUTE_A_CHECK, u32,);
        let mut clamp = f32::from_bits(read_word(CLAMP_INITIAL_WORD));
        if check != 0 {
            clamp = distance_clamp(position, check, clamp);
        }
        // Status 2: two helpers before the switch; an object answer whose byte at 0x22C is four
        // clears the clamp. Both helpers take the status word as their first argument.
        if status == STATUS_CASE_2 {
            let helper_object = callee_thiscall!(
                CALLEE_STATUS_TWO_HELPER,
                u32,
                global::<u8>(STATUS_TWO_HELPER_OBJECT) as u32,
                status,
                0u32,
            );
            let second_object = callee_thiscall!(CALLEE_STATUS_TWO_OBJECT, u32, helper_object);
            if second_object != 0 && read_heap_byte(second_object.wrapping_add(FIELD_STATUS_TWO_FLAG)) == 4 {
                clamp = 0.0;
            }
        }
        // Frame vector: each position word gains its normal times the clamp; the fourth word is
        // copied from out[19]. Written to the object only when the exit flag is clear.
        let frame_x = fadd(position[0], fmul(normal[0], clamp));
        let frame_y = fadd(position[1], fmul(normal[1], clamp));
        let frame_z = fadd(position[2], fmul(normal[2], clamp));
        let frame_w = out[19];
        if read_byte(FLAG_EXIT) == 0 {
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK), frame_x.to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4), frame_y.to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 8), frame_z.to_bits());
            write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 12), frame_w);
        }

        let _ = callee_stdcall!(CALLEE_ROUTE_A_FIRST, u32, frame_ptr, 0u32, FLOAT_ONE_BITS);
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, out_ptr, 0u32, FLOAT_ONE_BITS);

        let constant_first = f32::from_bits(read_word(CASE3_OFFSET_WORD));
        let constant_second = f32::from_bits(read_word(CONSTANT_SECOND_WORD));
        let chosen = if constant_first > constant_second {
            constant_first
        } else {
            constant_second
        };
        write_indexed(TABLE_ONE_BASE, chosen.to_bits());

        let mut loop_argument: u32 = 1;
        let mut loop_offset: u32 = LOOP_OFFSET_START;
        for _ in 0..LOOP_ITERATIONS {
            let _ = callee_stdcall!(CALLEE_ROUTE_A_FIRST, u32, frame_ptr, loop_argument, FLOAT_ONE_BITS);
            let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, out_ptr, loop_argument, FLOAT_ONE_BITS);
            let index = read_word(TABLE_INDEX);
            let address = LOOP_TABLE_BASE.wrapping_add(index.wrapping_add(loop_offset).wrapping_mul(4));
            core::ptr::write_volatile(global::<u32>(address), 0);
            loop_argument = loop_argument.wrapping_add(1);
            loop_offset = loop_offset.wrapping_add(4);
        }

        write_heap_word(this.wrapping_add(FIELD_232C), FLOAT_ONE_BITS);
        // Status 0x24 writes the field from the object that the getter returns (its word at 0x184);
        // every other status writes minus one.
        let field_31d8 = if status == STATUS_CASE_24 {
            let getter_this = read_word(GETTER_GLOBAL);
            let getter_object = callee_thiscall!(CALLEE_GETTER, u32, getter_this, STATUS_CASE_24, 0u32);
            read_heap_word(getter_object.wrapping_add(FIELD_184))
        } else {
            0xFFFF_FFFF
        };
        write_heap_word(this.wrapping_add(FIELD_31D8), field_31d8);
        if status == STATUS_CASE_12 {
            route_b_status_twelve(this, out_ptr);
        } else if status == STATUS_CASE_14 {
            route_b_status_fourteen(this, out_ptr);
        } else if is_tail_case(status) {
            route_b_status_tail_case(this, out_ptr, status);
        } else if status == STATUS_CASE_3 {
            route_b_status_three(this, out_ptr, [frame_x, frame_y, frame_z], frame_w);
        } else if status == STATUS_CASE_1 {
            route_b_status_one(this, out_ptr, [frame_x, frame_y, frame_z, f32::from_bits(frame_w)]);
        } else if status == STATUS_CASE_2 {
            route_b_status_two(this, out_ptr, [frame_x, frame_y, frame_z, f32::from_bits(frame_w)]);
        }

        if read_heap_byte(this.wrapping_add(FIELD_3211)) != 0 {
            // Heap byte 0x3211 nonzero: the first table word at the index is zeroed when the
            // zero-table word at the same index is above zero. NaN and zero do not qualify.
            let index = read_word(TABLE_INDEX);
            let zero_table_word = f32::from_bits(read_word(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))));
            if zero_table_word > 0.0 {
                write_indexed(TABLE_ONE_BASE, 0);
            }
        }
        let state_answer = callee_thiscall!(CALLEE_ROUTE_B_STATE, u32, global::<u8>(STATE_HELPER_OBJECT) as u32);
        // The state answer's low byte, or either global flag byte, selects flag one; otherwise flag
        // zero. Flag zero makes one helper call with argument zero, and flag one makes one call with
        // argument one when the object's 0x2FA0 byte is zero (the call writes the object).
        let flag_bytes_set = read_byte(GLOBAL_FLAG_BYTE_A) != 0 || read_byte(GLOBAL_FLAG_BYTE_B) != 0;
        let new_flag: u8 = if (state_answer as u8) != 0 || flag_bytes_set { 1 } else { 0 };
        if new_flag == 0 {
            let _ = callee_thiscall!(CALLEE_ROUTE_B_FLAG, u32, this, 0u32);
        } else if read_heap_byte(this.wrapping_add(FIELD_2FA0)) == 0 {
            let _ = callee_thiscall!(CALLEE_ROUTE_B_FLAG, u32, this, 1u32);
        }
        write_heap_byte(this.wrapping_add(FIELD_2FA0), new_flag);

        // Global state word not one: two sub-conditions decide whether the object route runs. When
        // the two words at 0x12088B4 and 0xF1C040 are equal and the word at 0x1037720 is not the key,
        // the function returns here.
        if read_word(GLOBAL_STATE_WORD) != 1 {
            let words_equal = read_word(GLOBAL_WORD_A) == read_word(GLOBAL_WORD_B);
            let third_is_key = read_word(GLOBAL_WORD_C) == STATE_KEY_WORD;
            if words_equal && !third_is_key {
                return;
            }
        }

        let object = callee_thiscall!(CALLEE_ROUTE_B_OBJECT, u32, global::<u8>(OBJECT_HELPER_OBJECT) as u32);
        let object_word = read_heap_word(object.wrapping_add(OBJECT_WORD_OFFSET));
        if object_word == 8 {
            route_b_object_eight(this, object, out_ptr, [frame_x, frame_y, frame_z], normal, out[19]);
            return;
        }
        if object_word != 7 {
            route_b_object_other(object, out_ptr);
            return;
        }
        let offset_word = read_word(GLOBAL_OFFSET_WORD);
        if offset_word == 0xFFFF_FFFF {
            route_b_object_other(object, out_ptr);
            return;
        }
        if offset_word.wrapping_sub(0x16) <= 2 {
            write_heap_word(this.wrapping_add(FIELD_232C), TAIL_CASE_OBJECT_WORD);
        }
        let _ = callee_stdcall!(
            CALLEE_ROUTE_A_SECOND,
            u32,
            global::<u8>(OBJECT_SEVEN_STDCALL_OBJECT) as u32,
            1u32,
            FLOAT_ONE_BITS,
        );
        let index = read_word(TABLE_INDEX);
        let table_value = read_word(GLOBAL_TABLE_VALUE);
        core::ptr::write_volatile(
            global::<u32>(TABLE_ZERO_BASE_A.wrapping_add(index.wrapping_mul(4))),
            table_value,
        );
        if read_byte(GLOBAL_EXIT_BYTE) != 0 {
            return;
        }
        // Exit byte clear: four floats from the global block are written into the object, at
        // offsets 0x580 to 0x58C in order.
        write_heap_word(this.wrapping_add(FIELD_580), read_word(OBJECT_SEVEN_GLOBAL_FIRST));
        write_heap_word(this.wrapping_add(FIELD_584), read_word(OBJECT_SEVEN_GLOBAL_SECOND));
        write_heap_word(this.wrapping_add(FIELD_588), read_word(OBJECT_SEVEN_GLOBAL_THIRD));
        write_heap_word(this.wrapping_add(FIELD_58C), read_word(OBJECT_SEVEN_GLOBAL_FOURTH));
    }
}

/// Object word eight, on the route B object block. The object's first word is a vtable; its slot at
/// offset 0x0C is called with the object as the first argument. Then a fill call runs with a block,
/// and the stdcall helper runs on the same block. A nonzero answer puts the block's vector at the
/// answer's base position less the normal, and its fourth word is the last output word. Two table
/// words are written at the index (one and zero), and with the exit flag clear the object receives
/// the position times the case constant plus the block's vector times one minus the constant.
unsafe fn route_b_object_eight(this: u32, object: u32, out_ptr: u32, position: [f32; 3], normal: [f32; 3], out_last: u32) {
    unsafe {
        // Sixteen-word vector block: words 12 to 14 are the vector that the answer block writes and
        // the exit-clear write-back reads, word 15 is the last output word.
        let mut object_block = [0u32; 16];
        let object_block_ptr = object_block.as_mut_ptr() as u32;

        let vtable = read_heap_word(object);
        let slot = read_heap_word(vtable.wrapping_add(VTABLE_SLOT_OFFSET));
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let answer = method(object);

        let _ = callee_thiscall!(CALLEE_ROUTE_A_FILL, u32, object_block_ptr, out_ptr);
        if answer != 0 {
            // A nonzero answer: its sub-object pointer (word at 0x20) gives the position base; when that
            // pointer is zero the base is the answer's own word at 0x10.
            let sub_object = read_heap_word(answer.wrapping_add(FIELD_ANSWER_SUB_OBJECT));
            let base = if sub_object != 0 {
                sub_object.wrapping_add(FIELD_ANSWER_SUB_BASE)
            } else {
                answer.wrapping_add(FIELD_ANSWER_OWN_BASE)
            };
            let base_x = f32::from_bits(read_heap_word(base));
            let base_y = f32::from_bits(read_heap_word(base.wrapping_add(4)));
            let base_z = f32::from_bits(read_heap_word(base.wrapping_add(8)));
            write_heap_word(object_block_ptr.wrapping_add(4 * 12), fsub(base_x, normal[0]).to_bits());
            write_heap_word(object_block_ptr.wrapping_add(4 * 13), fsub(base_y, normal[1]).to_bits());
            write_heap_word(object_block_ptr.wrapping_add(4 * 14), fsub(base_z, normal[2]).to_bits());
            write_heap_word(object_block_ptr.wrapping_add(4 * 15), out_last);
        }
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, object_block_ptr, 0u32, FLOAT_ONE_BITS);
        write_indexed(TABLE_ONE_BASE, FLOAT_ONE_BITS);
        write_indexed(TABLE_ZERO_BASE_A, 0);
        if read_byte(FLAG_EXIT) == 0 {
            // Exit clear: each word is the position times the case constant plus the block's vector
            // times one minus the constant; the fourth word is the last output word.
            let one = f32::from_bits(read_word(NORMAL_SCALE_WORD));
            let k2 = f32::from_bits(read_word(CASE2_CONSTANT_WORD));
            let complement = fsub(one, k2);
            let block_x = f32::from_bits(read_heap_word(object_block_ptr.wrapping_add(4 * 12)));
            let block_y = f32::from_bits(read_heap_word(object_block_ptr.wrapping_add(4 * 13)));
            let block_z = f32::from_bits(read_heap_word(object_block_ptr.wrapping_add(4 * 14)));
            let written = [
                fadd(fmul(position[0], k2), fmul(block_x, complement)).to_bits(),
                fadd(fmul(position[1], k2), fmul(block_y, complement)).to_bits(),
                fadd(fmul(position[2], k2), fmul(block_z, complement)).to_bits(),
                out_last,
            ];
            for (i, word) in written.iter().enumerate() {
                write_heap_word(this.wrapping_add(FIELD_VECTOR_WRITE_BACK + 4 * i as u32), *word);
            }
        }
    }
}

/// Object word other than seven or eight, on the route B object block. The object's first word is a
/// vtable whose slot at 0x0C is called with the object as the first argument. A nonzero answer is the
/// helper object: a helper takes it and answers a block, which the fill call writes into the output
/// block. Then a stdcall helper runs on the output block, two table words are written at the index
/// (one and zero), and the route returns without the exit check.
unsafe fn route_b_object_other(object: u32, out_ptr: u32) {
    unsafe {
        let vtable = read_heap_word(object);
        let slot = read_heap_word(vtable.wrapping_add(VTABLE_SLOT_OFFSET));
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let answer = method(object);
        if answer != 0 {
            let filled = callee_thiscall!(CALLEE_OBJECT_HELPER, u32, answer);
            let _ = callee_thiscall!(CALLEE_ROUTE_A_FILL, u32, out_ptr, filled);
        }
        let _ = callee_stdcall!(CALLEE_ROUTE_A_SECOND, u32, out_ptr, 0u32, FLOAT_ONE_BITS);
        write_indexed(TABLE_ONE_BASE, FLOAT_ONE_BITS);
        write_indexed(TABLE_ZERO_BASE_A, 0);
    }
}

/// Statuses 22, 23 and 24 (decimal): three switch cases with one template.
#[inline(always)]
fn is_tail_case(status: u32) -> bool {
    status == 22 || status == 23 || status == 24
}

/// Dispatch of the export: the status helper answers first, then the shared tail, route A or route B.
unsafe fn dispatch(this: u32) {
    unsafe {
        let status = callee_thiscall!(
            CALLEE_STATUS,
            u32,
            global::<u8>(STATUS_HELPER_OBJECT) as u32,
            0xFFFF_FFFFu32,
        );
        if status == 0x27 || status == 0x26 || read_byte(FLAG_SHARED_TAIL) != 0 {
            shared_tail(this);
        } else if read_byte(FLAG_A_ROUTE) != 0 {
            route_a(this);
        } else {
            route_b(this, status);
        }
    }
}

export!(thiscall, rw_0096d490(this: u32) -> () {
    unsafe { dispatch(this) }
});
