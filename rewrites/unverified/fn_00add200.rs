// original: 0x00ADD200 CRenderPhaseDrawScene::render
#![allow(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(dead_code)]

//! Scene-draw dispatcher: the ordered steps of the scene-draw render phase, run
//! on one object. The method returns nothing.
//!
//! The steps are chosen by three fields of the object: a guard word at +0x938,
//! a flag word at +0x8E8 whose bit 0x1000 (draw bit A) and bit 0x2000 (draw
//! bit B) are tested at several points, and a byte at +0x940.
//!
//! - If the guard word is -1 the method returns at once, making no call.
//! - Otherwise a block of 0x0C bytes is allocated and registered, then the base
//!   step runs on the object.
//! - If draw bit A is set, a first group runs: a 0x10-byte block is allocated,
//!   stamped with the constructor table word and given a flag byte of one, and
//!   registered; then a 0x410-byte block is allocated and passed to the
//!   sub-object constructor, whose answer is registered.
//! - If the byte at +0x940 is zero, a probe group runs: the probe answers a
//!   byte; a zero answer selects a float constant read from the image, a
//!   nonzero answer selects +0.0. A 0x18-byte block then receives the
//!   six-argument constructor with that float as its fourth argument.
//! - If draw bit B is set, two stamped blocks are registered before the tail.
//! - The byte at +0x940 selects the tail: zero takes the short tail, nonzero
//!   the long tail. Each tail is a fixed sequence of allocations, stamped
//!   blocks, registrations and callee steps. The long tail carries a middle
//!   group under draw bit A: a stamped block, a probe on a frame byte, a
//!   constructor block, and a float group (four float answers stored into a
//!   four-float block, then a five-argument call on a 0x30-byte block).
//! - Closing: if draw bit A is set, a closing group runs (on the zero tail the
//!   float group comes first; then a stamped block, a probe on a frame byte
//!   loaded from the scene-global object, and a constructor block). If draw bit
//!   B is set, one stamped block is registered. Then the exit step.
//!
//! A stamped block gets its first word set to a table word; its second word is
//! XORed with the low 14 bits of a draw counter, the counter then advancing by
//! one; its first word and its word at +8 are then set. Every allocation result
//! that is NULL is registered as NULL. Every value a callee answers and the
//! original passes on is passed on unchanged.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Guard word: when it equals minus one the method returns at once.
const OFF_GUARD: u32 = 0x938;
/// Flag word holding the two draw bits tested along the way.
const OFF_FLAGS: u32 = 0x8E8;
/// Byte tested against zero to choose between the long and short tails.
const OFF_SOURCE_BYTE: u32 = 0x940;
/// Sub-object passed to the ctor pair and the probe.
const OFF_SUB_OBJECT: u32 = 0xB0;
/// Fields passed by address to the middle and closing float calls.
const OFF_SCALE_A: u32 = 0x95C;
const OFF_SCALE_B: u32 = 0x958;
const OFF_SCALE_C: u32 = 0x960;
/// Offset of the guard word copied into some blocks.
const OFF_BLOCK_GUARD: u32 = 0xC;
/// Offset of the byte read from the scene-global object (closing group).
const OFF_GLOBAL_OBJECT_BYTE: u32 = 0x10A;
/// First draw bit.
const FLAG_DRAW_A: u32 = 0x1000;
/// Second draw bit.
const FLAG_DRAW_B: u32 = 0x2000;

/// File address of the draw counter (one dword, advanced by each stamp).
const DRAW_COUNTER_VA: u32 = 0x010327A0;
/// File address of the scene object pointer read by the float group.
const SCENE_OBJECT_VA: u32 = 0x011A28F4;
/// File address of the global object pointer read by the closing group.
const GLOBAL_OBJECT_VA: u32 = 0x01593B6C;
/// Constant table words and address words written by the stamps.
const TABLE_WORD_FIRST: u32 = 0x00E7E048;
const TABLE_WORD_SECOND: u32 = 0x00E7E080;
const TABLE_WORD_THIRD: u32 = 0x00E81E0C;
const TABLE_WORD_CTOR: u32 = 0x00E84C94;
const WORD8_BLOCK_A: u32 = 0x00432BE0;
const WORD8_BLOCK_B: u32 = 0x009CB6A0;
const WORD8_BLOCK_C: u32 = 0x00C107A0;
const WORD8_BLOCK_D: u32 = 0x00AD7EF0;
const WORD8_BLOCK_E: u32 = 0x00B59900;
const WORD8_BLOCK_F: u32 = 0x00D64820;
const WORD8_BLOCK_G: u32 = 0x00C106F0;
const WORD8_BLOCK_H: u32 = 0x00ABD880;
const WORD8_BLOCK_I: u32 = 0x00ABEC20;
const WORD8_BLOCK_CTOR: u32 = 0x005E16E0;
const WORD8_BLOCK_MIDDLE: u32 = 0x005E16D0;
const WORD8_BLOCK_T3C: u32 = 0x00925D20;
const WORD8_BLOCK_T3A: u32 = 0x0092D7E0;
const WORD8_BLOCK_T3B: u32 = 0x00926600;
/// Constants passed as arguments by the middle and closing groups.
const ARG_MIDDLE_POINTER: u32 = 0x005E16E0;
const ARG_CLOSING_POINTER: u32 = 0x00ADE5F0;
const ARG_FLOAT_GROUP: u32 = 0x00ADE560;

/// Callee ids, as declared in the contracts.
const CALLEE_ALLOC: u32 = 1; // allocator, cdecl, arguments (size, zero)
const CALLEE_REGISTER: u32 = 2; // registration of one pointer, cdecl, 1 argument
const CALLEE_BASE_STEP: u32 = 3; // base step on the object, thiscall, no argument
const CALLEE_STEP_4: u32 = 4; // cdecl, 1 argument (0, then 1)
const CALLEE_FILL_TABLE: u32 = 5; // cdecl, 3 arguments (guard, size, 4)
const CALLEE_STEP_6: u32 = 6; // cdecl, no argument
const CALLEE_STEP_7: u32 = 7; // cdecl, 1 argument (guard word)
const CALLEE_STEP_8: u32 = 8; // cdecl, no argument
const CALLEE_EXIT_STEP: u32 = 9; // cdecl, no argument, the closing call
const CALLEE_DRAW_CTOR: u32 = 15; // thiscall, 1 argument (object sub-field)
const CALLEE_MIDDLE_PROBE: u32 = 16; // thiscall, 2 arguments (constant, frame byte pointer)
const CALLEE_CLOSING_PROBE: u32 = 17; // thiscall, 2 arguments (constant, frame byte pointer)
const CALLEE_FLOAT_0: u32 = 18; // thiscall, no argument, answers st0 (float)
const CALLEE_FLOAT_1: u32 = 19; // thiscall, no argument, answers st0 (float)
const CALLEE_FLOAT_2: u32 = 20; // thiscall, no argument, answers st0 (float)
const CALLEE_FLOAT_3: u32 = 21; // thiscall, no argument, answers st0 (float)
const CALLEE_FLOAT_BLOCK: u32 = 22;
const CALLEE_DRAW_CTOR_SIX: u32 = 11; // thiscall, 6 arguments, answers a pointer (zero-byte probe group) // thiscall, 5 arguments (constant, float block pointer, 3 fields)

/// File address of the float constant read when the probe answers zero.
const FLOAT_CONST_VA: u32 = 0x00FE88E8;
/// Callee ids of the zero-byte path.
const CALLEE_PROBE: u32 = 10; // thiscall, no argument, answers a byte (on sub-object)
const CALLEE_SCENE_STEP: u32 = 12; // cdecl, 3 arguments (guard, a, b), answers a word
const CALLEE_SCENE_FILL: u32 = 14; // cdecl, 1 argument (the word answered above)


/// Read a 32-bit word of the object.
fn read_u32(obj: u32, off: u32) -> u32 {
    // SAFETY: the object is the checker's fabricated segment for the trial.
    unsafe { (obj.wrapping_add(off) as *const u32).read_unaligned() }
}

/// Read a byte of the object.
fn read_u8(obj: u32, off: u32) -> u8 {
    // SAFETY: as read_u32.
    unsafe { (obj.wrapping_add(off) as *const u8).read_unaligned() }
}

/// Stamp a block: the first word becomes the first table word, the second word
/// is mixed with the draw counter, the counter advances, then the first word
/// becomes `table_word` and the word at +8 becomes `word8`.
fn stamp(block: u32, table_word: u32, word8: u32) {
    let counter_ptr = global::<u32>(DRAW_COUNTER_VA);
    let second_ptr = block.wrapping_add(4) as *mut u32;
    // SAFETY: the block is a non-NULL pointer the allocator returned for the
    // trial; it spans at least 0x10 bytes in the fabricated heap.
    unsafe {
        let old = second_ptr.read_unaligned();
        (block as *mut u32).write_unaligned(relocated(TABLE_WORD_FIRST));
        let counter = counter_ptr.read_unaligned();
        let mix = (old ^ counter) & 0x3FFF;
        second_ptr.write_unaligned(old ^ mix);
        counter_ptr.write_unaligned(counter.wrapping_add(1));
        (block as *mut u32).write_unaligned(relocated(table_word));
        (block.wrapping_add(8) as *mut u32).write_unaligned(relocated(word8));
    }
}

/// Allocation step: the allocator is called with the size first and a zero
/// second. A non-NULL block is stamped. Returns the block (or NULL).
fn alloc_stamp(size: u32, table_word: u32, word8: u32) -> u32 {
    let p = callee_cdecl!(CALLEE_ALLOC, u32, size, 0);
    if p != 0 {
        stamp(p, table_word, word8);
    }
    p
}

/// As alloc_stamp, and the guard word is then written at +0xC of the block.
fn alloc_stamp_guarded(obj: u32, size: u32, table_word: u32, word8: u32) -> u32 {
    let p = alloc_stamp(size, table_word, word8);
    if p != 0 {
        // SAFETY: the block is non-NULL and at least 0x10 bytes long.
        unsafe {
            (p.wrapping_add(OFF_BLOCK_GUARD) as *mut u32)
                .write_unaligned(read_u32(obj, OFF_GUARD));
        }
    }
    p
}

/// Registration of one block (or NULL, or a value a callee answered).
fn register(p: u32) {
    callee_cdecl!(CALLEE_REGISTER, (), p);
}

/// Draw-bit-A first group: a block with the constructor table word, a byte
/// flag set in it, then a constructor on a larger block.
fn first_group_a(obj: u32) {
    let block = callee_cdecl!(CALLEE_ALLOC, u32, 0x10, 0);
    if block != 0 {
        stamp(block, TABLE_WORD_CTOR, WORD8_BLOCK_CTOR);
        // SAFETY: the block is non-NULL and at least 0x10 bytes long.
        unsafe { (block.wrapping_add(0xC) as *mut u8).write_unaligned(1) };
    }
    register(block);

    let big = callee_cdecl!(CALLEE_ALLOC, u32, 0x410, 0);
    let answer = if big != 0 {
        callee_thiscall!(CALLEE_DRAW_CTOR, u32, big, obj.wrapping_add(OFF_SUB_OBJECT))
    } else {
        0
    };
    register(answer);
}

/// Draw-bit-A middle group (long tail): two blocks and a probe on a frame
/// byte, a block with a constructor, and the float group with its closing
/// block of three fields.
fn middle_group_a(obj: u32) {
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_MIDDLE));

    let block = callee_cdecl!(CALLEE_ALLOC, u32, 0x10, 0);
    let answer = if block != 0 {
        let mut frame_byte: u8 = 0;
        callee_thiscall!(
            CALLEE_MIDDLE_PROBE, u32, block,
            relocated(ARG_MIDDLE_POINTER), (&mut frame_byte as *mut u8) as u32
        )
    } else {
        0
    };
    register(answer);

    let big = callee_cdecl!(CALLEE_ALLOC, u32, 0x410, 0);
    let answer_ctor = if big != 0 {
        callee_thiscall!(CALLEE_DRAW_CTOR, u32, big, obj.wrapping_add(OFF_SUB_OBJECT))
    } else {
        0
    };
    register(answer_ctor);

    // Float group: four st0 answers stored in order into a four-float block,
    // whose address the closing call receives.
    let scene_object = unsafe { global::<u32>(SCENE_OBJECT_VA).read_unaligned() };
    let block_floats: [f32; 4] = [
        callee_thiscall!(CALLEE_FLOAT_0, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_1, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_2, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_3, f32, scene_object),
    ];
    let scale_block = callee_cdecl!(CALLEE_ALLOC, u32, 0x30, 0);
    let answer_block = if scale_block != 0 {
        callee_thiscall!(
            CALLEE_FLOAT_BLOCK, u32, scale_block,
            relocated(ARG_FLOAT_GROUP), block_floats.as_ptr() as u32,
            obj.wrapping_add(OFF_SCALE_A),
            obj.wrapping_add(OFF_SCALE_B),
            obj.wrapping_add(OFF_SCALE_C)
        )
    } else {
        0
    };
    register(answer_block);
}

/// Draw-bit-A closing group (after the long tail): one stamped block, a block
/// whose byte comes from the scene-global object, and one with a constructor.
fn closing_group_a(obj: u32) {
    let _ = obj;
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_MIDDLE));

    let block = callee_cdecl!(CALLEE_ALLOC, u32, 0x10, 0);
    let answer = if block != 0 {
        // The global object's byte is read only on this non-NULL path.
        let global_object = unsafe { global::<u32>(GLOBAL_OBJECT_VA).read_unaligned() };
        let mut frame_byte: u8 = if global_object != 0 {
            read_u8(global_object, OFF_GLOBAL_OBJECT_BYTE)
        } else {
            0
        };
        callee_thiscall!(
            CALLEE_CLOSING_PROBE, u32, block,
            relocated(ARG_CLOSING_POINTER), (&mut frame_byte as *mut u8) as u32
        )
    } else {
        0
    };
    register(answer);
}

/// Draw-bit-B group: two stamped blocks before the tail split.
fn draw_b_pair() {
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_T3A));
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_T3B));
}

/// Long tail (the byte at +0x940 is nonzero).
fn long_tail(obj: u32) {
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_B));
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_C));
    callee_cdecl!(CALLEE_STEP_4, (), 0);
    register(alloc_stamp_guarded(obj, 0x10, TABLE_WORD_THIRD, WORD8_BLOCK_D));
    callee_cdecl!(CALLEE_STEP_4, (), 1);
    register(alloc_stamp_guarded(obj, 0x10, TABLE_WORD_THIRD, WORD8_BLOCK_E));
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_F));
    callee_cdecl!(CALLEE_FILL_TABLE, (), read_u32(obj, OFF_GUARD), 0xC, 4);

    // Draw-bit group A (middle group) is tested again here.
    if read_u32(obj, OFF_FLAGS) & FLAG_DRAW_A != 0 {
        middle_group_a(obj);
    }
    callee_cdecl!(CALLEE_STEP_6, (),);
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_G));
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_H));
    callee_cdecl!(CALLEE_FILL_TABLE, (), read_u32(obj, OFF_GUARD), 0xA, 4);
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_I));
    callee_cdecl!(CALLEE_STEP_7, (), read_u32(obj, OFF_GUARD));
    callee_cdecl!(CALLEE_STEP_8, (),);
}

/// Probe-and-float group of the zero-byte path. The probe answers a byte; a zero
/// byte selects the float constant from the image, a nonzero byte +0.0. An
/// allocation of 0x18 bytes then receives the constructor pair (which takes the
/// float as its fourth argument) when non-NULL; its answer is registered.
fn zero_byte_probe_group(obj: u32) {
    let answer = callee_thiscall!(CALLEE_PROBE, u32, obj.wrapping_add(OFF_SUB_OBJECT));
    let float_bits = if (answer as u8) == 0 {
        // SAFETY: a read of the float constant from the image, through the
        // relocated address of the file's own value.
        unsafe { relocated_float(FLOAT_CONST_VA) }
    } else {
        0.0f32
    };
    let block = callee_cdecl!(CALLEE_ALLOC, u32, 0x18, 0);
    let result = if block != 0 {
        callee_thiscall!(
            CALLEE_DRAW_CTOR_SIX, u32, block,
            0, 0, 1, float_bits.to_bits(), 1, 0
        )
    } else {
        0
    };
    register(result);
}

/// Read a float constant from the image at a file address.
///
/// # Safety
/// The address must be a readable float in the mapped image.
unsafe fn relocated_float(file_va: u32) -> f32 {
    (relocated(file_va) as *const f32).read_unaligned()
}

/// Short tail (the byte at +0x940 is zero).
fn zero_tail(obj: u32) {
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_B));
    callee_cdecl!(CALLEE_STEP_4, (), 0);
    register(alloc_stamp_guarded(obj, 0x10, TABLE_WORD_THIRD, WORD8_BLOCK_D));
    callee_cdecl!(CALLEE_STEP_4, (), 1);
    register(alloc_stamp_guarded(obj, 0x10, TABLE_WORD_THIRD, WORD8_BLOCK_E));
    callee_cdecl!(CALLEE_STEP_6, (),);
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_F));
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_G));
    let answer_a = callee_cdecl!(CALLEE_SCENE_STEP, u32, read_u32(obj, OFF_GUARD), 1, 1);
    callee_cdecl!(CALLEE_SCENE_FILL, (), answer_a);
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_C));
    let answer_b = callee_cdecl!(CALLEE_SCENE_STEP, u32, read_u32(obj, OFF_GUARD), 0xE, 1);
    callee_cdecl!(CALLEE_SCENE_FILL, (), answer_b);
    callee_cdecl!(CALLEE_STEP_7, (), read_u32(obj, OFF_GUARD));
}

/// Draw-bit-A closing group of the zero-byte path: the float group, then the
/// stamped block with the scene-global byte, then the constructor block.
fn zero_closing_a(obj: u32) {
    let scene_object = unsafe { global::<u32>(SCENE_OBJECT_VA).read_unaligned() };
    let block_floats: [f32; 4] = [
        callee_thiscall!(CALLEE_FLOAT_0, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_1, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_2, f32, scene_object),
        callee_thiscall!(CALLEE_FLOAT_3, f32, scene_object),
    ];
    let scale_block = callee_cdecl!(CALLEE_ALLOC, u32, 0x30, 0);
    let answer_block = if scale_block != 0 {
        callee_thiscall!(
            CALLEE_FLOAT_BLOCK, u32, scale_block,
            relocated(ARG_FLOAT_GROUP), block_floats.as_ptr() as u32,
            obj.wrapping_add(OFF_SCALE_A),
            obj.wrapping_add(OFF_SCALE_B),
            obj.wrapping_add(OFF_SCALE_C)
        )
    } else {
        0
    };
    register(answer_block);

    closing_group_a(obj);

    let big = callee_cdecl!(CALLEE_ALLOC, u32, 0x410, 0);
    let answer_ctor = if big != 0 {
        callee_thiscall!(CALLEE_DRAW_CTOR, u32, big, obj.wrapping_add(OFF_SUB_OBJECT))
    } else {
        0
    };
    register(answer_ctor);
}
export!(thiscall, rw_00add200_s7(obj: u32) -> () {
    // Entry guard: the method returns at once, making no call.
    if read_u32(obj, OFF_GUARD) == 0xFFFF_FFFF {
        return;
    }

    // First allocation group, then the base step on the object.
    register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_A));
    callee_thiscall!(CALLEE_BASE_STEP, (), obj);

    // Draw-bit group A (first group).
    if read_u32(obj, OFF_FLAGS) & FLAG_DRAW_A != 0 {
        first_group_a(obj);
    }
    // Zero byte: the probe-and-float group runs before the draw-bit-B test.
    if read_u8(obj, OFF_SOURCE_BYTE) == 0 {
        zero_byte_probe_group(obj);
    }
    // Draw-bit group B: two stamped blocks before the tail split.
    if read_u32(obj, OFF_FLAGS) & FLAG_DRAW_B != 0 {
        draw_b_pair();
    }
    // The byte is tested again; zero leads to the short tail, nonzero to the long tail.
    if read_u8(obj, OFF_SOURCE_BYTE) == 0 {
        zero_tail(obj);
    } else {
        long_tail(obj);
    }

    // Closing checks: draw bit A runs the closing group; draw bit B adds one
    // stamped block before the exit step.
    if read_u32(obj, OFF_FLAGS) & FLAG_DRAW_A != 0 {
        if read_u8(obj, OFF_SOURCE_BYTE) == 0 {
            zero_closing_a(obj);
        } else {
            closing_group_a(obj);
        }
    }
    if read_u32(obj, OFF_FLAGS) & FLAG_DRAW_B != 0 {
        register(alloc_stamp(0xC, TABLE_WORD_SECOND, WORD8_BLOCK_T3C));
    }
    callee_cdecl!(CALLEE_EXIT_STEP, (),);
});
