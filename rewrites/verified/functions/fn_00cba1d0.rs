// original: 0x00CBA1D0 CTaskSimpleMoveSlideToCoord::vf17
/// Clears transient entity fields, advances task timing and position state,
/// optionally updates an attached object, normalizes the target heading, and
/// returns one when the task reports completion in AL. The heading helper
/// receives π in ST0 and the widened heading in ST1; the proof compares ST0
/// and the same widened heading through the caller's stack window.
mod cba1 {
    use core::hint::black_box;
    use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

    const ENTITY_POS: usize = 0x20;
    const ENTITY_OBJECT: usize = 0x78;
    const ENTITY_TRANSIENT_0: usize = 0xE0;
    const ENTITY_TRANSIENT_1: usize = 0xE4;
    const ENTITY_TRANSIENT_2: usize = 0xE8;
    const ENTITY_ENTRY_FLOAT: usize = 0xEC;
    const ENTITY_GATE: usize = 0x219;
    const ENTITY_EXTENSION: usize = 0x224;
    const ENTITY_FLAGS: usize = 0x29C;
    const ENTITY_HEADING: usize = 0xAA0;

    const TASK_POSITION_X: usize = 0x20;
    const TASK_POSITION_Y: usize = 0x24;
    const TASK_DELTA_X: usize = 0x30;
    const TASK_DELTA_Y: usize = 0x34;
    const TASK_HEADING: usize = 0x38;
    const TASK_HEADING_STEP: usize = 0x3C;
    const TASK_RADIUS: usize = 0x40;
    const TASK_TICK: usize = 0x48;
    const TASK_FLAGS: usize = 0x4C;
    const TASK_COUNTER_A: usize = 0x50;
    const TASK_COUNTER_B: usize = 0x54;

    const ENTITY_VTABLE_SLOT: usize = 0x0C;
    const OBJECT_FLAGS: usize = 0x0C;
    const OBJECT_VTABLE_SLOT: usize = 0x14;

    const CALLEE_EARLY_UPDATE: u32 = 1;
    const CALLEE_POSITION_UPDATE: u32 = 2;
    const CALLEE_EXTENSION_QUERY: u32 = 3;
    const CALLEE_HEADING_DOUBLE: u32 = 5;
    const CALLEE_HEADING_TRANSFORM_A: u32 = 6;
    const CALLEE_HEADING_TRANSFORM_B: u32 = 7;
    const CALLEE_ENTITY_ADJUST_A: u32 = 9;
    const CALLEE_ENTITY_ADJUST_B: u32 = 10;
    const CALLEE_ANGLE_MAGNITUDE_A: u32 = 11;
    const CALLEE_COMPLETE: u32 = 12;
    const CALLEE_ANGLE_MAGNITUDE_B: u32 = 13;
    const CALLEE_FINAL_UPDATE: u32 = 14;

    const FILE_TICK_VA: u32 = 0x0117_35B4;
    const FILE_SCALE_VA: u32 = 0x0117_35BC;
    const FILE_MODE_VA: u32 = 0x011D_6FD4;
    const FILE_PI_F64_VA: u32 = 0x00FE_8A50;
    const FILE_PI_F32_VA: u32 = 0x00FE_8AA0;
    const FILE_NEG_PI_F32_VA: u32 = 0x00FE_8DC4;
    const FILE_ONE_F32_VA: u32 = 0x00FE_88E8;
    const FILE_NEG_ONE_F32_VA: u32 = 0x00FE_8D94;
    const FILE_ZERO_POINT_ONE_VA: u32 = 0x0105_0E68;
    const FILE_ABS_MASK_VA: u32 = 0x00FE_8F80;

    #[inline(always)]
    unsafe fn read_u32(base: *mut u8, offset: usize) -> u32 {
        // SAFETY: the contract maps the object and the named field.
        unsafe { base.add(offset).cast::<u32>().read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(base: *mut u8, offset: usize, value: u32) {
        // SAFETY: the contract maps the object and the named field as writable.
        unsafe { base.add(offset).cast::<u32>().write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u8(base: *mut u8, offset: usize) -> u8 {
        // SAFETY: the contract maps the object and the named byte.
        unsafe { base.add(offset).read() }
    }

    #[inline(always)]
    unsafe fn read_ptr(base: *mut u8, offset: usize) -> *mut u8 {
        unsafe { read_u32(base, offset) as usize as *mut u8 }
    }

    #[inline(always)]
    unsafe fn read_f32(base: *mut u8, offset: usize) -> f32 {
        f32::from_bits(unsafe { read_u32(base, offset) })
    }

    #[inline(always)]
    unsafe fn write_f32(base: *mut u8, offset: usize, value: f32) {
        unsafe { write_u32(base, offset, value.to_bits()) }
    }

    #[inline(always)]
    unsafe fn read_global_u32(file_va: u32) -> u32 {
        // SAFETY: the checker maps the original image at the relocated base.
        unsafe { global::<u32>(file_va).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_global_f32(file_va: u32) -> f32 {
        f32::from_bits(unsafe { read_global_u32(file_va) })
    }

    #[inline(never)]
    fn add_f32(left: f32, right: f32) -> f32 {
        black_box(black_box(left) + black_box(right))
    }

    #[inline(never)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        black_box(black_box(left) - black_box(right))
    }

    #[inline(never)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        black_box(black_box(left) * black_box(right))
    }

    /// COMISS/UCOMISS followed by JB also takes the unordered path.
    #[inline(always)]
    fn below(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left < right
    }

    /// COMISS followed by JBE takes below, equal, and unordered values.
    #[inline(always)]
    fn below_equal(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left <= right
    }

    /// Exact binary32-to-binary64 widening, including a quieted NaN payload.
    fn widen_f32(bits: u32) -> u64 {
        let sign = u64::from(bits >> 31) << 63;
        let exponent = (bits >> 23) & 0xFF;
        let fraction = bits & 0x007F_FFFF;
        if exponent == 0xFF {
            return sign | 0x7FF0_0000_0000_0000 | (u64::from(fraction) << 29)
                | if fraction == 0 { 0 } else { 0x0008_0000_0000_0000 };
        }
        if exponent == 0 {
            if fraction == 0 {
                return sign;
            }
            let leading = 31 - fraction.leading_zeros();
            let shifted = fraction << (23 - leading);
            let biased = (leading as i32 - 149 + 1023) as u64;
            return sign | (biased << 52) | (u64::from(shifted & 0x007F_FFFF) << 29);
        }
        sign | (u64::from(exponent + 896) << 52) | (u64::from(fraction) << 29)
    }

    fn round_shift(value: u64, shift: u32) -> u64 {
        if shift == 0 {
            return value;
        }
        if shift >= 64 {
            return 0;
        }
        let quotient = value >> shift;
        let remainder = value & ((1u64 << shift) - 1);
        let halfway = 1u64 << (shift - 1);
        if remainder > halfway || (remainder == halfway && quotient & 1 != 0) {
            quotient + 1
        } else {
            quotient
        }
    }

    /// Binary64-to-binary32 round-to-nearest-even conversion used by CVTPD2PS.
    fn narrow_f64(bits: u64) -> u32 {
        let sign = (bits >> 32) as u32 & 0x8000_0000;
        let exponent = ((bits >> 52) & 0x7FF) as u32;
        let fraction = bits & 0x000F_FFFF_FFFF_FFFF;
        if exponent == 0x7FF {
            if fraction == 0 {
                return sign | 0x7F80_0000;
            }
            return sign | 0x7F80_0000 | ((fraction >> 29) as u32) | 0x0040_0000;
        }
        if exponent == 0 {
            return sign;
        }
        let mut f32_exp = exponent as i32 - 896;
        let significand = (1u64 << 52) | fraction;
        if f32_exp >= 255 {
            return sign | 0x7F80_0000;
        }
        if f32_exp > 0 {
            let mut rounded = round_shift(significand, 29);
            if rounded >= 1 << 24 {
                rounded >>= 1;
                f32_exp += 1;
            }
            if f32_exp >= 255 {
                return sign | 0x7F80_0000;
            }
            return sign | ((f32_exp as u32) << 23) | (rounded as u32 & 0x007F_FFFF);
        }
        let rounded = round_shift(significand, (30 - f32_exp) as u32);
        sign | rounded as u32
    }

    #[inline(always)]
    fn split_f64(bits: u64) -> (u32, u32) {
        (bits as u32, (bits >> 32) as u32)
    }

    fn double_helper(angle: f32, pi_bits: u64, mutate_second_input: bool) -> f64 {
        let mut second = widen_f32(angle.to_bits());
        if mutate_second_input {
            second ^= 1u64 << 63;
        }
        let (pi_low, pi_high) = split_f64(pi_bits);
        let (angle_low, angle_high) = split_f64(second);
        // The original passes the second double in a local qword at caller
        // stack bytes +0x30. The checker logs its words as arguments 12 and 13.
        // The lower words are frame contents and are deliberately skipped.
        let answer: f64 = callee_cdecl!(
            CALLEE_HEADING_DOUBLE,
            f64,
            pi_low,
            pi_high,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            angle_low,
            angle_high
        );
        answer
    }

    #[inline(always)]
    fn f32_helper(id: u32, value: f32) -> f32 {
        let bits = value.to_bits();
        match id {
            CALLEE_HEADING_TRANSFORM_A => callee_cdecl!(id, f32, bits),
            CALLEE_HEADING_TRANSFORM_B => callee_cdecl!(id, f32, bits),
            CALLEE_ANGLE_MAGNITUDE_A => callee_cdecl!(id, f32, bits),
            CALLEE_ANGLE_MAGNITUDE_B => callee_cdecl!(id, f32, bits),
            _ => unreachable!(),
        }
    }

    unsafe fn run(task: *mut u8, entity: *mut u8, mutate_second_input: bool) -> u32 {
        // The original reads a caller scratch word at entry. Contracts define
        // this otherwise-uninitialised word as zero on both sides.
        unsafe {
            write_u32(entity, ENTITY_TRANSIENT_0, 0);
            write_u32(entity, ENTITY_TRANSIENT_1, 0);
            write_u32(entity, ENTITY_TRANSIENT_2, 0);
            write_u32(entity, ENTITY_ENTRY_FLOAT, 0);
        }

        let task_flags = unsafe { read_u32(task, TASK_FLAGS) };
        if task_flags & 1 != 0 {
            let _: u32 = callee_thiscall!(
                CALLEE_EARLY_UPDATE,
                u32,
                task as u32,
                entity as u32
            );

            let elapsed = unsafe { read_u32(task, TASK_TICK) };
            if elapsed != u32::MAX {
                let tick = unsafe { read_global_u32(FILE_TICK_VA) };
                unsafe { write_u32(task, TASK_TICK, elapsed.wrapping_add(tick)) };
            }

            let anchor = unsafe { read_ptr(entity, ENTITY_POS) };
            let dx = sub_f32(
                unsafe { read_f32(task, TASK_POSITION_X) },
                unsafe { read_f32(anchor, 0x30) },
            );
            let dy = sub_f32(
                unsafe { read_f32(task, TASK_POSITION_Y) },
                unsafe { read_f32(anchor, 0x34) },
            );
            unsafe {
                write_f32(task, TASK_DELTA_X, dx);
                write_f32(task, TASK_DELTA_Y, dy);
            }
            let _: u32 = callee_thiscall!(CALLEE_POSITION_UPDATE, u32, entity as u32, 1);

            if unsafe { read_u8(entity, ENTITY_GATE) } != 0 {
                let extension = unsafe { read_ptr(entity, ENTITY_EXTENSION) };
                let query = unsafe { extension.add(0x44) };
                let query_result: u32 = callee_thiscall!(
                    CALLEE_EXTENSION_QUERY,
                    u32,
                    query as u32
                );
                if query_result != 4 {
                    let object = unsafe { read_ptr(extension, 0x54) };
                    if unsafe { read_u8(object, OBJECT_FLAGS) } & 1 == 0 {
                        let object_vtable = unsafe { read_ptr(object, 0) };
                        let function_address = unsafe {
                            read_u32(object_vtable, OBJECT_VTABLE_SLOT)
                        };
                        let function: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
                            core::mem::transmute(function_address as usize)
                        };
                        let result = function(object as u32, entity as u32, 2, 0);
                        if result & 0xFF != 0 {
                            let flags = unsafe { read_u32(object, OBJECT_FLAGS) };
                            unsafe { write_u32(object, OBJECT_FLAGS, flags | 2) };
                        }
                    }
                }
            }
        }
        unsafe { write_u32(task, TASK_FLAGS, task_flags & !1) };

        let heading_step = unsafe { read_f32(task, TASK_HEADING_STEP) };
        let mut value = unsafe { read_f32(task, TASK_HEADING) };

        // UCOMISS + LAHF + (an instruction of the original)
        // adjustment path; unordered and nonzero values normalize the angle.
        if heading_step != 0.0 {
            let entity_heading = unsafe { read_f32(entity, ENTITY_HEADING) };
            let heading_delta = sub_f32(entity_heading, value);
            let pi = unsafe { read_global_f32(FILE_PI_F32_VA) };
            let negative_pi = unsafe { read_global_f32(FILE_NEG_PI_F32_VA) };
            let wrap = if heading_delta < 0.0 { negative_pi } else { pi };
            let reduced_input = add_f32(heading_delta, wrap);
            let pi_bits = unsafe {
                (global::<u64>(FILE_PI_F64_VA)).read_unaligned()
            };
            let reduced = double_helper(reduced_input, pi_bits, mutate_second_input);
            let narrowed = f32::from_bits(narrow_f64(reduced.to_bits()));
            value = sub_f32(narrowed, wrap);

            if value == negative_pi {
                value = pi;
            }
            value = f32::from_bits(value.to_bits() ^ 0x8000_0000);

            let factor = if value < 0.0 {
                unsafe { read_global_f32(FILE_NEG_ONE_F32_VA) }
            } else if value == 0.0 {
                0.0
            } else {
                unsafe { read_global_f32(FILE_ONE_F32_VA) }
            };
            let scale = unsafe { read_global_f32(FILE_SCALE_VA) };
            let scaled_step = mul_f32(heading_step, scale);
            let signed_step = mul_f32(scaled_step, factor);
            let target_heading = unsafe { read_f32(entity, ENTITY_HEADING) };
            value = add_f32(signed_step, target_heading);

            let mode = unsafe { read_global_u32(FILE_MODE_VA) } as i32;
            let mut transformed = value;
            let comparison_bound = unsafe { read_f32(entity, ENTITY_HEADING) };
            if mode < 2 {
                transformed = f32_helper(CALLEE_HEADING_TRANSFORM_A, transformed);
            }

            let bound = unsafe { read_f32(task, TASK_HEADING) };
            let clamp = if below(bound, comparison_bound) {
                if below(comparison_bound, bound) || below(bound, transformed) {
                    false
                } else {
                    true
                }
            } else if !below(transformed, bound) {
                true
            } else if below(comparison_bound, bound) || below(bound, transformed) {
                false
            } else {
                true
            };

            if clamp {
                transformed = bound;
                unsafe { write_u32(task, TASK_HEADING_STEP, 0) };
            } else if mode >= 2 {
                transformed = f32_helper(CALLEE_HEADING_TRANSFORM_B, transformed);
            }
            value = transformed;

            let entity_vtable = unsafe { read_ptr(entity, 0) };
            let function_address = unsafe { read_u32(entity_vtable, ENTITY_VTABLE_SLOT) };
            let function: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                core::mem::transmute(function_address as usize)
            };
            let _: u32 = function(entity as u32, value.to_bits());
            let _: u32 = callee_thiscall!(CALLEE_ENTITY_ADJUST_A, u32, entity as u32, value.to_bits());
        }

        // The zero step path skips angle normalization and forwards the task
        // heading directly to the second adjustment call.
        let adjustment = if heading_step != 0.0 {
            value
        } else {
            unsafe { read_f32(task, TASK_HEADING) }
        };
        let _: u32 = callee_thiscall!(
            CALLEE_ENTITY_ADJUST_B,
            u32,
            entity as u32,
            adjustment.to_bits()
        );

        let anchor = unsafe { read_ptr(entity, ENTITY_POS) };
        let dx = sub_f32(
            unsafe { read_f32(task, TASK_POSITION_X) },
            unsafe { read_f32(anchor, 0x30) },
        );
        let dy = sub_f32(
            unsafe { read_f32(task, TASK_POSITION_Y) },
            unsafe { read_f32(anchor, 0x34) },
        );

        let elapsed = unsafe { read_u32(task, TASK_TICK) };
        let tick = unsafe { read_global_u32(FILE_TICK_VA) };
        if elapsed != u32::MAX && (elapsed as i32) < (tick as i32) {
            let difference = sub_f32(
                unsafe { read_f32(task, TASK_HEADING) },
                unsafe { read_f32(entity, ENTITY_HEADING) },
            );
            let magnitude = f32_helper(CALLEE_ANGLE_MAGNITUDE_A, difference);
            let absolute = f32::from_bits(
                magnitude.to_bits() & unsafe { read_global_u32(FILE_ABS_MASK_VA) },
            );
            let threshold = unsafe { read_global_f32(FILE_ZERO_POINT_ONE_VA) };
            if !below_equal(threshold, absolute) {
                let _: u32 = callee_thiscall!(CALLEE_COMPLETE, u32, task as u32, entity as u32);
                return 1;
            }
        }

        let radius = unsafe { read_f32(task, TASK_RADIUS) };
        let distance_squared = add_f32(mul_f32(dy, dy), mul_f32(dx, dx));
        let radius_squared = mul_f32(radius, radius);
        if below_equal(radius_squared, distance_squared) {
            unsafe {
                let flags = read_u32(entity, ENTITY_FLAGS);
                write_u32(entity, ENTITY_FLAGS, flags | 0x2000);
            }
            let first = unsafe { read_u32(task, TASK_COUNTER_A) };
            let second = unsafe { read_u32(task, TASK_COUNTER_B) };
            if first != u32::MAX || second != u32::MAX {
                let object = unsafe { read_ptr(entity, ENTITY_OBJECT) };
                let _: u32 = callee_thiscall!(
                    CALLEE_FINAL_UPDATE,
                    u32,
                    object as u32,
                    first,
                    0x4B,
                    0x4020,
                    3,
                    0x4080_0000,
                    second
                );
            }
        } else {
            let difference = sub_f32(
                unsafe { read_f32(task, TASK_HEADING) },
                unsafe { read_f32(entity, ENTITY_HEADING) },
            );
            let magnitude = f32_helper(CALLEE_ANGLE_MAGNITUDE_B, difference);
            let absolute = f32::from_bits(
                magnitude.to_bits() & unsafe { read_global_u32(FILE_ABS_MASK_VA) },
            );
            let threshold = unsafe { read_global_f32(FILE_ZERO_POINT_ONE_VA) };
            if threshold > absolute {
                let _: u32 = callee_thiscall!(CALLEE_COMPLETE, u32, task as u32, entity as u32);
                return 1;
            }
            unsafe {
                let flags = read_u32(entity, ENTITY_FLAGS);
                write_u32(entity, ENTITY_FLAGS, flags | 0x2000);
            }
            let first = unsafe { read_u32(task, TASK_COUNTER_A) };
            let second = unsafe { read_u32(task, TASK_COUNTER_B) };
            if first != u32::MAX || second != u32::MAX {
                let object = unsafe { read_ptr(entity, ENTITY_OBJECT) };
                let _: u32 = callee_thiscall!(
                    CALLEE_FINAL_UPDATE,
                    u32,
                    object as u32,
                    first,
                    0x4B,
                    0x4020,
                    3,
                    0x4080_0000,
                    second
                );
            }
        }
        0
    }

    export!(thiscall, rw_fn_00cba1d0(task: *mut u8, entity: *mut u8) -> u32 {
        unsafe { run(task, entity, false) }
    });

    export!(thiscall, mut_fn_00cba1d0(task: *mut u8, entity: *mut u8) -> u32 {
        unsafe { run(task, entity, true) }
    });
}
