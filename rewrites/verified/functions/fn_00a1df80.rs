// original: 0x00A1DF80 task_gated_direction_solve
/// Applies the gated direction update for an owner and target. It exits without
/// touching the output while the owner, target, child, or child state gate is
/// closed. On the active path it selects a mode-dependent angle bias, derives
/// an angle from the target's direction vector or fallback angle, stores the
/// combined angle on the owner, asks the direction solver for a four-word
/// record, and copies that record to the caller's output.
const OWNER_FLAGS_OFFSET: u32 = 0x38e;
const OWNER_MODE_OFFSET: u32 = 0x130;
const OWNER_KIND_OFFSET: u32 = 0x394;
const OWNER_INPUT_OFFSET: u32 = 0x304;
const OWNER_ANGLE_OFFSET: u32 = 0x308;
const TARGET_FLAGS_OFFSET: u32 = 0x26c;
const TARGET_CHILD_OFFSET: u32 = 0xb30;
const CHILD_KIND_OFFSET: u32 = 0x1304;
const CHILD_TYPE_OFFSET: u32 = 0x2e;
const CHILD_VECTOR_OFFSET: u32 = 0x20;
const FALLBACK_ANGLE_OFFSET: u32 = 0x1c;
const VECTOR_X_OFFSET: u32 = 0x10;
const VECTOR_Y_OFFSET: u32 = 0x14;
const OUTPUT_FLAG: u8 = 0x04;
const DISABLED_CHILD_KIND: u32 = 2;
const NEGATE_SIGN_BIT: u32 = 0x8000_0000;
const CHILD_TYPE_A: u32 = 0x012f_a65c;
const CHILD_TYPE_B: u32 = 0x012f_a290;
const ANGLE_BIAS_A: u32 = 0x00fe_8da0;
const ANGLE_BIAS_B: u32 = 0x00fe_8978;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u16(address: u32) -> u16 {
    unsafe { (address as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
fn add_f32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
unsafe fn read_global_u32(file_address: u32) -> u32 {
    unsafe { lf_checker_rt::global::<u32>(file_address).read_unaligned() }
}

#[inline(always)]
unsafe fn read_global_f32(file_address: u32) -> f32 {
    f32::from_bits(unsafe { read_global_u32(file_address) })
}

lf_checker_rt::export!(thiscall, rw_00A1DF80(this: u32, target: u32, output: u32) -> u32 {
    unsafe {
        if read_u8(this + OWNER_FLAGS_OFFSET) & OUTPUT_FLAG == 0
            || read_u32(this + OWNER_MODE_OFFSET) > 1
            || read_u8(target + TARGET_FLAGS_OFFSET) & OUTPUT_FLAG == 0
        {
            return 0;
        }

        let child = read_u32(target + TARGET_CHILD_OFFSET);
        if child == 0 || read_u32(child + CHILD_KIND_OFFSET) == DISABLED_CHILD_KIND {
            return 0;
        }

        let owner_kind = read_u32(this + OWNER_KIND_OFFSET);
        let mut use_bias_a = owner_kind == 5 || owner_kind == 6;
        let mut use_bias_b = owner_kind == 7 || owner_kind == 8;
        let child_type = read_u16(child + CHILD_TYPE_OFFSET) as i16 as i32 as u32;
        let global_type_a = read_global_u32(CHILD_TYPE_A);
        let global_type_b = read_global_u32(CHILD_TYPE_B);
        if child_type == global_type_a || child_type == global_type_b {
            core::mem::swap(&mut use_bias_a, &mut use_bias_b);
        }

        let angle_bias = if use_bias_a {
            read_global_f32(ANGLE_BIAS_A)
        } else if use_bias_b {
            read_global_f32(ANGLE_BIAS_B)
        } else {
            0.0
        };

        let vector = read_u32(child + CHILD_VECTOR_OFFSET);
        let angle = if vector == 0 {
            f32::from_bits(read_u32(child + FALLBACK_ANGLE_OFFSET))
        } else {
            let x_bits = read_u32(vector + VECTOR_X_OFFSET) ^ NEGATE_SIGN_BIT;
            let y_bits = read_u32(vector + VECTOR_Y_OFFSET);
            let x = f64::from(f32::from_bits(x_bits)).to_bits();
            let y = f64::from(f32::from_bits(y_bits)).to_bits();
            let answer = lf_checker_rt::callee_cdecl!(
                1,
                u64,
                x as u32,
                (x >> 32) as u32,
                y as u32,
                (y >> 32) as u32,
            );
            f64::from_bits(answer) as f32
        };

        let combined_angle = add_f32(angle, angle_bias);
        write_u32(this + OWNER_ANGLE_OFFSET, combined_angle.to_bits());

        let mut frame_output = [0u32; 4];
        let answer_record = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            frame_output.as_mut_ptr() as u32,
            read_u32(this + OWNER_INPUT_OFFSET),
            combined_angle.to_bits(),
        );
        write_u32(output, read_u32(answer_record));
        write_u32(output + 4, read_u32(answer_record + 4));
        write_u32(output + 8, read_u32(answer_record + 8));
        write_u32(output + 12, read_u32(answer_record + 12));
        0
    }
});
