// original: 0x00D849A0 input_ui_update_pass
/// Update the input object's cached direction and output angle.
///
/// The routine first asks whether the object is already handled. Otherwise
/// it reads two packed table entries, derives a direction from the optional
/// anchor, and normalizes that direction against a virtual table result. A
/// close, aligned target may delegate the update to the solver. The regular
/// path evaluates the selected table slots, lets the object-specific helpers
/// adjust the angle and output value, clears the output flag, and negates the
/// two output floats when the object's mirror bit is set.
///
/// The handled path returns the first helper's answer without touching the
/// output pointers. The solver path returns the solver's answer. The regular
/// path returns the output-flag pointer.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Address of the runtime-populated table of packed input entries.
const ENTRY_TABLE: u32 = 0x0117_8284;
/// Address of the object-category helper's implicit object.
const CATEGORY_OBJECT: u32 = 0x0117_7A80;
/// Address of the global switch used by the optional score helper.
const SCORE_SWITCH: u32 = 0x0179_7754;
/// Offset of the object's optional anchor pointer.
const ANCHOR_OFFSET: u32 = 0x20;
/// Offset of the first packed table pair.
const FIRST_PAIR_OFFSET: u32 = 0xDE0;
/// Offset of the second packed table pair.
const SECOND_PAIR_OFFSET: u32 = 0xDE4;
/// Byte stride between table records.
const ENTRY_STRIDE: u32 = 0x20;
/// Signed-coordinate fields in each record.
const ENTRY_X_OFFSET: u32 = 0x14;
const ENTRY_Y_OFFSET: u32 = 0x16;
/// A scalar one in the image, used by the direction fallback.
const DIRECTION_THRESHOLD: u32 = 0x00FE_863C;
/// Constant used to scale packed table coordinates.
const ENTRY_SCALE: u32 = 0x00FE_87A4;
/// Constant used by the angle helper.
const MIRROR_ANGLE: u32 = 0x00FE_8AA0;
/// Low limit used by the angle adjustment.
const ANGLE_FLOOR: u32 = 0x00FE_8DC4;
/// Angle adjustment step.
const ANGLE_STEP: u32 = 0x00FE_8AEC;
/// Upper limit used by the angle adjustment.
const ANGLE_CEILING: u32 = 0x00FE_8AA0;
/// Factor used when mirroring the two output values.
const MIRROR_FACTOR: u32 = 0x00FE_8D94;
/// The object flag that selects the alternate angle constants.
const ANGLE_MODE_OFFSET: u32 = 0xF19;
/// The object byte used as a scale for solver and output helpers.
const SCALE_OFFSET: u32 = 0xE6F;
/// The object's mirrored-state flag.
const MIRROR_OFFSET: u32 = 0xE73;
/// The angle field used by the sine and cosine helpers.
const STORED_ANGLE_OFFSET: u32 = 0x1C;

/// Keep the first source operand as the SSE destination for NaN payloads.
#[inline(always)]
fn fadd1(left: f32, right: f32) -> f32 {
    let left = core::hint::black_box(left);
    let right = core::hint::black_box(right);
    if left.is_nan() {
        left
    } else if right.is_nan() {
        right
    } else {
        core::hint::black_box(left + right)
    }
}

/// Keep subtraction's destination operand and NaN payload order exact.
#[inline(always)]
fn fsub1(left: f32, right: f32) -> f32 {
    let left = core::hint::black_box(left);
    let right = core::hint::black_box(right);
    if left.is_nan() {
        left
    } else if right.is_nan() {
        right
    } else {
        core::hint::black_box(left - right)
    }
}

/// Keep multiplication's destination operand and NaN payload order exact.
#[inline(always)]
fn fmul1(left: f32, right: f32) -> f32 {
    let left = core::hint::black_box(left);
    let right = core::hint::black_box(right);
    if left.is_nan() {
        left
    } else if right.is_nan() {
        right
    } else {
        core::hint::black_box(left * right)
    }
}

/// Preserve the source and divisor order used by scalar division.
#[inline(always)]
fn fdiv1(left: f32, right: f32) -> f32 {
    let left = core::hint::black_box(left);
    let right = core::hint::black_box(right);
    if left.is_nan() {
        left
    } else if right.is_nan() {
        right
    } else {
        core::hint::black_box(left / right)
    }
}

#[inline(always)]
fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read() }
}

#[inline(always)]
fn read_u16(address: u32) -> u16 {
    unsafe { (address as *const u16).read() }
}

#[inline(always)]
fn read_i16(address: u32) -> i16 {
    unsafe { (address as *const i16).read() }
}

#[inline(always)]
fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
fn read_f32(address: u32) -> f32 {
    f32::from_bits(read_u32(address))
}

#[inline(always)]
fn read_f32_volatile(address: u32) -> f32 {
    f32::from_bits(unsafe { (address as *const u32).read_volatile() })
}

#[inline(always)]
fn entry_base(index: u16, subindex: u16) -> u32 {
    let table = global::<u32>(ENTRY_TABLE);
    let base = unsafe { table.add(index as usize).read() };
    base.wrapping_add(u32::from(subindex).wrapping_mul(ENTRY_STRIDE))
}

#[inline(always)]
fn scaled_coordinate(entry: u32, offset: u32, scale: f32) -> f32 {
    fmul1(read_i16(entry.wrapping_add(offset)) as f32, scale)
}

#[inline(always)]
fn table_result(object: u32, output: *mut u32) -> u32 {
    let vtable = read_u32(object);
    let target = read_u32(vtable.wrapping_add(0xEC));
    let call: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    call(object, output as u32)
}

#[inline(always)]
fn atan2_result(first: f32, second: f32) -> f32 {
    callee_cdecl!(7, f32, first.to_bits(), second.to_bits())
}

#[inline(always)]
fn table_pair_is_live(packed: u32) -> bool {
    let index = packed as u16;
    if index == u16::MAX {
        return false;
    }
    unsafe { global::<u32>(ENTRY_TABLE).add(index as usize).read() != 0 }
}

fn rewrite(
    object: u32,
    output_angle: u32,
    output_value: u32,
    value_c: u32,
    output_flag: u32,
    extra: u32,
    mutant: bool,
) -> u32 {
    let already_handled = callee_cdecl!(1, u32, object);
    if already_handled as u8 != 0 {
        return already_handled;
    }

    let table_scale = read_f32(relocated(ENTRY_SCALE));
    let first_index = read_u16(object.wrapping_add(FIRST_PAIR_OFFSET));
    let first_subindex = read_u16(object.wrapping_add(FIRST_PAIR_OFFSET + 2));
    let first_entry = entry_base(first_index, first_subindex);
    let first_x = scaled_coordinate(first_entry, ENTRY_X_OFFSET, table_scale);
    let first_y = scaled_coordinate(first_entry, ENTRY_Y_OFFSET, table_scale);

    let second_index = read_u16(object.wrapping_add(SECOND_PAIR_OFFSET));
    let second_subindex = read_u16(object.wrapping_add(SECOND_PAIR_OFFSET + 2));
    let second_entry = entry_base(second_index, second_subindex);
    let second_x = scaled_coordinate(second_entry, ENTRY_X_OFFSET, table_scale);
    let second_y = scaled_coordinate(second_entry, ENTRY_Y_OFFSET, table_scale);

    let mirrored = read_u8(object.wrapping_add(MIRROR_OFFSET)) & 1 != 0;
    let anchor = read_u32(object.wrapping_add(ANCHOR_OFFSET));
    let mut slot_10 = 0.0f32;
    let mut slot_2c = 0.0f32;
    let mut slot_28 = 0.0f32;
    let mut slot_30 = 0.0f32;
    let slot_34 = 0.0f32;
    let mut slot_38 = second_x;
    let slot_3c = second_y;
    let mut slot_40 = first_x;
    let mut slot_58 = first_y;
    let mut slot_5c = 0.0f32;

    let (direction_x, direction_y, direction_z) = if anchor != 0 {
        let x = read_f32(anchor.wrapping_add(0x10));
        let y = read_f32(anchor.wrapping_add(0x14));
        let z = read_f32(anchor.wrapping_add(0x18));
        if mirrored {
            (-x, -y, -z)
        } else {
            (x, y, z)
        }
    } else {
        let angle = read_u32(object.wrapping_add(STORED_ANGLE_OFFSET));
        let sine = f32::from_bits(callee_cdecl!(2, u32, angle));
        let cosine = f32::from_bits(callee_cdecl!(3, u32, angle));
        if mirrored {
            (sine, -cosine, -0.0)
        } else {
            (-sine, cosine, 0.0)
        }
    };
    slot_10 = direction_x;
    slot_30 = direction_y;
    slot_28 = direction_z;

    let mut table_output = [0u32; 3];
    let returned_vector = table_result(object, table_output.as_mut_ptr());
    let vector_x = read_f32(returned_vector);
    let vector_y = read_f32(returned_vector.wrapping_add(4));
    let vector_z = read_f32(returned_vector.wrapping_add(8));
    let dot_y = fmul1(vector_y, direction_y);
    let dot_x = fmul1(vector_x, direction_x);
    slot_5c = fadd1(fadd1(dot_y, dot_x), fmul1(vector_z, direction_z));

    let direction_length_sq = fadd1(
        fmul1(direction_y, direction_y),
        fmul1(direction_x, direction_x),
    );
    let direction_threshold = read_f32(relocated(DIRECTION_THRESHOLD));
    if direction_threshold > direction_length_sq {
        slot_28 = 1.0;
        slot_2c = 0.0;
        slot_30 = 0.0;
    } else if !(direction_length_sq <= 0.0) {
        let length = direction_length_sq.sqrt();
        let reciprocal = 1.0 / length;
        slot_28 = fmul1(reciprocal, direction_x);
        slot_30 = fmul1(reciprocal, direction_y);
    } else {
        slot_28 = 0.0;
        slot_30 = 0.0;
    }

    // These volatile reads preserve the null-anchor fault after the virtual
    // table and trigonometric helpers have already run.
    let anchor_x = read_f32_volatile(anchor.wrapping_add(0x30));
    let anchor_y = read_f32_volatile(anchor.wrapping_add(0x34));
    let target_dx = fsub1(second_x, first_x);
    let target_dy = fsub1(second_y, first_y);
    let from_anchor_x = fsub1(anchor_x, second_x);
    let from_anchor_y = fsub1(anchor_y, second_y);
    let target_length_sq = fadd1(fmul1(target_dy, target_dy), fmul1(target_dx, target_dx));
    let target_length = target_length_sq.sqrt();
    let target_inverse = 1.0 / target_length;
    let alignment_y = fmul1(fmul1(target_inverse, target_dy), from_anchor_y);
    let alignment_x = fmul1(fmul1(target_inverse, target_dx), from_anchor_x);
    let alignment = fadd1(alignment_y, alignment_x);
    let align_limit = read_f32(relocated(0x00FE_8DE8));
    let target_distance_sq = fadd1(
        fmul1(from_anchor_y, from_anchor_y),
        fmul1(from_anchor_x, from_anchor_x),
    );
    let target_distance = target_distance_sq.sqrt();
    let close_limit = read_f32(relocated(0x00FE_8B38));
    if alignment > align_limit && close_limit > target_distance {
        if callee_cdecl!(5, u32, object, 1u32, 0u32) != 0 {
            let scale_byte = read_u8(object.wrapping_add(SCALE_OFFSET));
            return callee_cdecl!(
                6,
                u32,
                object,
                0u32,
                read_u32(object.wrapping_add(0xE4C)),
                read_u32(object.wrapping_add(0xE50)),
                output_angle,
                output_value,
                value_c,
                output_flag,
                extra,
                (scale_byte as f32).to_bits()
            );
        }
    }

    let mut angle = atan2_result(slot_28, slot_30);
    if mirrored {
        angle = fadd1(angle, read_f32(relocated(MIRROR_ANGLE)));
    }
    slot_10 = angle;
    slot_30 = angle;

    let mut best_score = read_f32(relocated(0x00FE_88E8));
    for ordinal in 4u32..12 {
        let packed_first = read_u32(object.wrapping_add(0xDD4 + ordinal * 4));
        let packed_second = read_u32(object.wrapping_add(0xDD8 + ordinal * 4));
        slot_58 = f32::from_bits(packed_first);
        if !table_pair_is_live(packed_first) || !table_pair_is_live(packed_second) {
            continue;
        }
        let first_record = entry_base(packed_first as u16, (packed_first >> 16) as u16);
        let second_record = entry_base(packed_second as u16, (packed_second >> 16) as u16);
        let candidate_x = scaled_coordinate(first_record, ENTRY_X_OFFSET, table_scale);
        let candidate_y = scaled_coordinate(first_record, ENTRY_Y_OFFSET, table_scale);
        let next_x = scaled_coordinate(second_record, ENTRY_X_OFFSET, table_scale);
        let next_y = scaled_coordinate(second_record, ENTRY_Y_OFFSET, table_scale);
        let anchor_dx = fsub1(candidate_x, anchor_x);
        let anchor_dy = fsub1(candidate_y, anchor_y);
        let mut distance = fsub1(
            fadd1(fmul1(anchor_dy, anchor_dy), fmul1(anchor_dx, anchor_dx)).sqrt(),
            read_f32(relocated(0x00FE_8B38)),
        );
        if !(distance > 0.0) {
            distance = 0.0;
        }
        let distance_cap = read_f32(relocated(0x0105_6D84));
        if distance > distance_cap {
            break;
        }
        let delta_x = fsub1(next_x, candidate_x);
        let delta_y = fsub1(next_y, candidate_y);
        let delta_sq = fadd1(fmul1(delta_x, delta_x), fmul1(delta_y, delta_y));
        let inverse_delta = 1.0 / delta_sq.sqrt();
        let candidate_angle = atan2_result(
            fmul1(inverse_delta, delta_x),
            fmul1(inverse_delta, delta_y),
        );
        slot_58 = candidate_angle;
        let angle_difference = fsub1(slot_58, slot_30);
        let mode = read_u8(object.wrapping_add(ANGLE_MODE_OFFSET)) & 1;
        let mode_a = if mode != 0 {
            read_f32(relocated(0x00FE_87F4))
        } else {
            read_f32(relocated(0x00FE_8844))
        };
        let first_probe = callee_cdecl!(
            8,
            f32,
            angle_difference.to_bits(),
            0x3E20_D97Cu32,
            0x3FA0_D97Cu32,
            mode_a.to_bits(),
            0x3F80_0000u32
        );
        slot_58 = first_probe;
        let distance_factor = fsub1(
            1.0,
            fdiv1(distance, read_f32(relocated(0x0105_6D84))),
        );
        slot_38 = distance_factor;
        let first_score = fsub1(
            1.0,
            fmul1(fsub1(1.0, first_probe), distance_factor),
        );
        if !(first_score > best_score) {
            best_score = first_score;
        }
        let mode_b = if mode != 0 {
            read_f32(relocated(0x00E9_AF9C))
        } else {
            read_f32(relocated(0x00FE_8B30))
        };
        let second_probe = callee_cdecl!(
            8,
            f32,
            angle_difference.to_bits(),
            0x3E20_D97Cu32,
            0x3FA0_D97Cu32,
            mode_b.to_bits(),
            read_u32(relocated(0x0105_6D88))
        );
        let scaled_second_probe = fdiv1(
            second_probe,
            read_u8(object.wrapping_add(SCALE_OFFSET)) as f32,
        );
        let second_score = fsub1(
            1.0,
            fmul1(fsub1(1.0, scaled_second_probe), distance_factor),
        );
        if !(second_score > best_score) {
            best_score = second_score;
        }
    }
    slot_28 = best_score;

    let mut packed_address = object.wrapping_add(0xE04);
    let mut ordinal = 12i32;
    while ordinal >= 4 {
        let packed = read_u32(packed_address);
        if table_pair_is_live(packed) {
            let record = entry_base(packed as u16, (packed >> 16) as u16);
            if callee_thiscall!(
                9,
                u32,
                relocated(CATEGORY_OBJECT),
                record
            ) == 2
            {
                let _ = callee_cdecl!(
                    10,
                    u32,
                    object,
                    ordinal as u32,
                    &mut slot_10 as *mut f32 as u32
                );
            }
        }
        ordinal -= 1;
        packed_address = packed_address.wrapping_sub(4);
    }

    let switch = unsafe { global::<u8>(SCORE_SWITCH).read() };
    if switch == 0 {
        slot_10 = callee_cdecl!(
            11,
            f32,
            object,
            0u32,
            slot_10.to_bits(),
            slot_30.to_bits(),
            0x4000_0000u32,
            2u32,
            1u32,
            1u32,
            extra
        );
    }

    let first_packed = read_u32(object.wrapping_add(FIRST_PAIR_OFFSET));
    if table_pair_is_live(first_packed) {
        let second_index = read_u16(object.wrapping_add(0xDDC));
        if second_index != u16::MAX {
            let second_table = unsafe { global::<u32>(ENTRY_TABLE).add(second_index as usize).read() };
            if second_table != 0 {
                let record = entry_base(first_packed as u16, (first_packed >> 16) as u16);
                if callee_thiscall!(
                    9,
                    u32,
                    relocated(CATEGORY_OBJECT),
                    record
                ) == 2
                {
                    let _ = callee_cdecl!(
                        10,
                        u32,
                        object,
                        3u32,
                        &mut slot_10 as *mut f32 as u32
                    );
                }
            }
        }
    }

    let angle_before_wrap = slot_10;
    slot_10 = fsub1(angle_before_wrap, slot_30);
    let scale_byte = read_u8(object.wrapping_add(SCALE_OFFSET));
    let scale_for_call = fmul1(scale_byte as f32, slot_28);
    slot_38 = scale_for_call;
    let angle_answer = callee_cdecl!(
        12,
        u32,
        object,
        angle_before_wrap.to_bits(),
        slot_30.to_bits(),
        &mut slot_10 as *mut f32 as u32,
        &mut slot_38 as *mut f32 as u32
    );
    if angle_answer as u8 == 0 {
        let lower = read_f32(relocated(ANGLE_FLOOR));
        let step = read_f32(relocated(ANGLE_STEP));
        let upper = read_f32(relocated(ANGLE_CEILING));
        while lower > slot_10 {
            slot_10 = fadd1(slot_10, step);
        }
        while slot_10 > upper {
            slot_10 = fsub1(slot_10, step);
        }
    }

    slot_40 = if mirrored {
        read_f32(relocated(ANGLE_FLOOR))
    } else {
        0.0
    };
    let _ = callee_cdecl!(
        13,
        u32,
        object,
        &mut slot_10 as *mut f32 as u32,
        &mut (slot_40) as *mut f32 as u32,
        &mut slot_58 as *mut f32 as u32
    );
    let second_float = slot_5c;
    let first_float = slot_38;
    let _ = slot_2c;
    let _ = slot_34;
    let _ = slot_3c;
    let _ = callee_cdecl!(
        14,
        u32,
        first_float.to_bits(),
        second_float.to_bits(),
        value_c,
        output_value,
        object,
        &mut (slot_40) as *mut f32 as u32
    );

    unsafe {
        (output_angle as *mut f32).write(slot_10);
        (output_flag as *mut u8).write(0);
    }
    if mirrored {
        let mirror = read_f32(relocated(MIRROR_FACTOR));
        if !mutant {
            let value = fmul1(unsafe { (output_angle as *const f32).read() }, mirror);
            unsafe { (output_angle as *mut f32).write(value) };
        }
        let value = fmul1(unsafe { (output_value as *const f32).read() }, mirror);
        unsafe { (output_value as *mut f32).write(value) };
    }
    output_flag
}

export!(cdecl, rw_d849a0(
    object: u32,
    output_angle: u32,
    output_value: u32,
    value_c: u32,
    output_flag: u32,
    extra: u32,
) -> u32 {
    rewrite(object, output_angle, output_value, value_c, output_flag, extra, false)
});

export!(cdecl, rw_d849a0_mut(
    object: u32,
    output_angle: u32,
    output_value: u32,
    value_c: u32,
    output_flag: u32,
    extra: u32,
) -> u32 {
    rewrite(object, output_angle, output_value, value_c, output_flag, extra, true)
});
