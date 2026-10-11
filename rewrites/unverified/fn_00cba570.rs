// original: 0x00CBA570 CTaskSimpleSlideToCoord::vf17

/// Update the slide task's movement fields and dispatch its state callbacks.
/// A zero movement delta skips the angle path; otherwise the angle helper's
/// double result is rounded to a single-precision target value. The routine
/// returns one when either completion-distance check succeeds, and otherwise
/// sets the state object's completion bit and returns zero.
///
/// The proof fixes the aligned incoming scratch word to zero. The angle
/// helper's first x87 double is checked; its second x87 double is outside the
/// stock worker's call-log channel.
unsafe fn update_slide_task(this: u32, state: u32, wrong_scratch: bool) -> u8 {
    const TASK_POINT: u32 = 0x20;
    const TASK_OBJECT: u32 = 0x78;
    const TASK_NESTED: u32 = 0x224;
    const TASK_SECONDARY: u32 = 0xA80;
    const TASK_TARGET: u32 = 0xAA0;
    const TASK_GATE: u32 = 0x219;
    const TASK_COMPLETION: u32 = 0x29C;
    const TASK_SCRATCH: u32 = 0xEC;

    const THIS_SOURCE_X: u32 = 0xC0;
    const THIS_SOURCE_Y: u32 = 0xC4;
    const THIS_DELTA_X: u32 = 0xD0;
    const THIS_DELTA_Y: u32 = 0xD4;
    const THIS_CURRENT: u32 = 0xD8;
    const THIS_DELTA: u32 = 0xDC;
    const THIS_RADIUS: u32 = 0xE4;
    const THIS_TICK: u32 = 0xE8;
    const THIS_FLAGS: u32 = 0xEC;

    const POINT_X: u32 = 0x30;
    const POINT_Y: u32 = 0x34;
    const NESTED_QUERY: u32 = 0x44;
    const NESTED_OBJECT: u32 = 0x54;
    const SECONDARY_VALUE: u32 = 0x3C;
    const OBJECT_FLAGS: u32 = 0x0C;

    const RVA_SIGN_MASK: u32 = 0x00BE_8FA0;
    const RVA_ABS_MASK: u32 = 0x00BE_8F80;
    const RVA_GLOBAL_TICK: u32 = 0x00D7_35B4;
    const RVA_FLOAT_SCALE: u32 = 0x00D7_35BC;
    const RVA_ONE: u32 = 0x00BE_88E8;
    const RVA_NEG_ONE: u32 = 0x00BE_8D94;
    const RVA_PI_F32: u32 = 0x00BE_8AA0;
    const RVA_NEG_PI_F32: u32 = 0x00BE_8DC4;
    const RVA_PI_F64: u32 = 0x00BE_8A50;
    const RVA_COMPLETION_EPSILON: u32 = 0x0065_0E68;

    const CALLEE_TASK_HOOK: u32 = 0;
    const CALLEE_INITIALIZE: u32 = 1;
    const CALLEE_NESTED_QUERY: u32 = 2;
    const CALLEE_NESTED_UPDATE: u32 = 3;
    const CALLEE_STATE_QUERY: u32 = 4;
    const CALLEE_STATE_SET: u32 = 5;
    const CALLEE_SECONDARY_QUERY: u32 = 6;
    const CALLEE_SECONDARY_UPDATE: u32 = 7;
    const CALLEE_ANGLE: u32 = 8;
    const CALLEE_FLOAT_MAP: u32 = 9;
    const CALLEE_VTABLE_FLOAT: u32 = 10;
    const CALLEE_FLOAT_UPDATE: u32 = 11;
    const CALLEE_FINAL_FLOAT: u32 = 12;

    const INITIALIZE_MODE: u32 = 1;
    const PAIRED_IDS: [u32; 6] = [0x26, 0x27, 0x30, 0x31, 0x35, 0x36];
    const CLEAR_TASK_BIT: u32 = !1;
    const SET_COMPLETION_BIT: u32 = 0x2000;
    const INITIAL_TICK_OFFSET: u32 = 0x1F40;
    const ACTIVE_TICK_OFFSET: u32 = 0x01F4;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }

    #[inline(always)]
    fn float_add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    #[inline(always)]
    fn float_sub(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    #[inline(always)]
    fn float_mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    #[inline(always)]
    fn unordered_or_below(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left < right
    }

    unsafe {
        write_word(state + 0xE0, 0);
        write_word(state + 0xE4, 0);
        write_word(state + 0xE8, 0);
        write_word(
            state + TASK_SCRATCH,
            if wrong_scratch { 1.0f32.to_bits() } else { 0 },
        );

        let task_flags = read_word(this + THIS_FLAGS);
        let mut finish_gate = 1u8;
        if task_flags & 2 != 0 {
            finish_gate = lf_checker_rt::callee_thiscall!(
                CALLEE_TASK_HOOK,
                u8,
                this,
                state
            );
        }

        let task_tick_address = this + THIS_TICK;
        let mut task_tick = read_word(task_tick_address);
        if task_tick == u32::MAX {
            if read_word(this + THIS_FLAGS) & 2 == 0 {
                task_tick = read_word(lf_checker_rt::relocated(RVA_GLOBAL_TICK))
                    .wrapping_add(INITIAL_TICK_OFFSET);
                write_word(task_tick_address, task_tick);
            } else if finish_gate != 0 {
                task_tick = read_word(lf_checker_rt::relocated(RVA_GLOBAL_TICK))
                    .wrapping_add(ACTIVE_TICK_OFFSET);
                write_word(task_tick_address, task_tick);
            }
        }

        let state_object = read_word(state + TASK_OBJECT);
        if read_word(this + THIS_FLAGS) & 1 != 0 {
            let point = read_word(state + TASK_POINT);
            let source_x = read_float(this + THIS_SOURCE_X);
            let source_y = read_float(this + THIS_SOURCE_Y);
            let point_x = read_float(point + POINT_X);
            let point_y = read_float(point + POINT_Y);
            write_word(this + THIS_DELTA_X, float_sub(source_x, point_x).to_bits());
            write_word(this + THIS_DELTA_Y, float_sub(source_y, point_y).to_bits());

            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_INITIALIZE,
                u32,
                state,
                INITIALIZE_MODE
            );

            if read_byte(state + TASK_GATE) != 0 {
                let nested = read_word(state + TASK_NESTED);
                let query = lf_checker_rt::callee_thiscall!(
                    CALLEE_NESTED_QUERY,
                    u32,
                    nested + NESTED_QUERY
                );
                if query != 4 {
                    let nested_object = read_word(nested + NESTED_OBJECT);
                    if read_word(nested_object + OBJECT_FLAGS) & 1 == 0 {
                        let updated = lf_checker_rt::callee_thiscall!(
                            CALLEE_NESTED_UPDATE,
                            u8,
                            nested_object,
                            state,
                            2u32,
                            0u32
                        );
                        if updated != 0 {
                            let flags = read_word(nested_object + OBJECT_FLAGS);
                            write_word(nested_object + OBJECT_FLAGS, flags | 2);
                        }
                    }
                }
            }

            for state_id in PAIRED_IDS {
                let previous = lf_checker_rt::callee_thiscall!(
                    CALLEE_STATE_QUERY,
                    u32,
                    state_object,
                    state_id
                );
                if previous != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CALLEE_STATE_SET,
                        u32,
                        previous,
                        0xBF80_0000u32
                    );
                }
            }

            let secondary = read_word(state + TASK_SECONDARY);
            let secondary_value = read_word(secondary + SECONDARY_VALUE);
            let next_state = lf_checker_rt::callee_thiscall!(
                CALLEE_SECONDARY_QUERY,
                u32,
                state
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_SECONDARY_UPDATE,
                u32,
                state_object,
                secondary_value,
                0x0Fu32,
                1.0f32.to_bits(),
                next_state
            );

            let flags = read_byte(this + THIS_FLAGS);
            write_byte(this + THIS_FLAGS, flags & (CLEAR_TASK_BIT as u8));
        }

        let movement_delta = read_float(this + THIS_DELTA);
        let final_float = if movement_delta == 0.0 {
            read_float(this + THIS_CURRENT)
        } else {
            let target = read_float(state + TASK_TARGET);
            let current = read_float(this + THIS_CURRENT);
            let displacement = float_sub(target, current);
            let pi = read_float(lf_checker_rt::relocated(RVA_PI_F32));
            let negative_pi = read_float(lf_checker_rt::relocated(RVA_NEG_PI_F32));
            let boundary = if displacement < 0.0 { negative_pi } else { pi };
            let _unobserved_second_argument = f64::from(float_add(boundary, displacement));

            let pi_bits = unsafe {
                core::ptr::read_unaligned(
                    lf_checker_rt::relocated(RVA_PI_F64) as *const u64
                )
            };
            let angle_result: f64 = lf_checker_rt::callee_cdecl!(
                CALLEE_ANGLE,
                f64,
                pi_bits as u32,
                (pi_bits >> 32) as u32
            );
            let rounded_angle = angle_result as f32;
            let adjusted_angle = if rounded_angle == negative_pi {
                pi
            } else {
                rounded_angle
            };
            let sign_mask = read_word(lf_checker_rt::relocated(RVA_SIGN_MASK));
            let negative_angle = f32::from_bits(adjusted_angle.to_bits() ^ sign_mask);
            let mut direction = read_float(lf_checker_rt::relocated(RVA_NEG_ONE));
            if !(0.0 > negative_angle) {
                direction = if negative_angle == 0.0 {
                    0.0
                } else {
                    read_float(lf_checker_rt::relocated(RVA_ONE))
                };
            }

            let scale = read_float(lf_checker_rt::relocated(RVA_FLOAT_SCALE));
            let scaled_delta = float_mul(movement_delta, scale);
            let directed_delta = float_mul(scaled_delta, direction);
            let mut next_current = float_add(directed_delta, target);
            let current_vs_target = unordered_or_below(current, target);
            let reset_to_current = if current_vs_target {
                let target_vs_current = unordered_or_below(target, current);
                let current_vs_candidate = unordered_or_below(current, next_current);
                !target_vs_current && !current_vs_candidate
            } else {
                !unordered_or_below(next_current, current)
            };
            if reset_to_current {
                write_word(this + THIS_DELTA, 0);
                next_current = current;
            }

            let mapped_value: f32 = lf_checker_rt::callee_cdecl!(
                CALLEE_FLOAT_MAP,
                f32,
                next_current.to_bits()
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_VTABLE_FLOAT,
                u32,
                state_object,
                mapped_value.to_bits()
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_FLOAT_UPDATE,
                u32,
                state_object,
                direction.to_bits()
            );
            direction
        };

        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_FINAL_FLOAT,
            u32,
            state_object,
            final_float.to_bits()
        );

        let point = read_word(state + TASK_POINT);
        let source_x = read_float(this + THIS_SOURCE_X);
        let source_y = read_float(this + THIS_SOURCE_Y);
        let point_x = read_float(point + POINT_X);
        let point_y = read_float(point + POINT_Y);
        let offset_x = float_sub(source_x, point_x);
        let offset_y = float_sub(source_y, point_y);
        let task_tick = read_word(this + THIS_TICK);
        let global_tick = read_word(lf_checker_rt::relocated(RVA_GLOBAL_TICK));

        if (task_tick as i32) < (global_tick as i32) {
            let target_delta = float_sub(
                read_float(this + THIS_CURRENT),
                read_float(state + TASK_TARGET),
            );
            let distance_value: f32 = lf_checker_rt::callee_cdecl!(
                CALLEE_FLOAT_MAP,
                f32,
                target_delta.to_bits()
            );
            let abs_mask = read_word(lf_checker_rt::relocated(RVA_ABS_MASK));
            let absolute_distance = f32::from_bits(distance_value.to_bits() & abs_mask);
            let epsilon = read_float(lf_checker_rt::relocated(RVA_COMPLETION_EPSILON));
            if epsilon > absolute_distance {
                return 1;
            }
        }

        if finish_gate != 0 {
            let radius = read_float(this + THIS_RADIUS);
            let radius_squared = float_mul(radius, radius);
            let x_squared = float_mul(offset_x, offset_x);
            let y_squared = float_mul(offset_y, offset_y);
            let distance_squared = float_add(x_squared, y_squared);
            if radius_squared > distance_squared {
                let target_delta = float_sub(
                    read_float(this + THIS_CURRENT),
                    read_float(state + TASK_TARGET),
                );
                let distance_value: f32 = lf_checker_rt::callee_cdecl!(
                    CALLEE_FLOAT_MAP,
                    f32,
                    target_delta.to_bits()
                );
                let abs_mask = read_word(lf_checker_rt::relocated(RVA_ABS_MASK));
                let absolute_distance = f32::from_bits(distance_value.to_bits() & abs_mask);
                let epsilon = read_float(lf_checker_rt::relocated(RVA_COMPLETION_EPSILON));
                if epsilon > absolute_distance {
                    return 1;
                }
            }
        }

        let completion = read_word(state + TASK_COMPLETION);
        write_word(state + TASK_COMPLETION, completion | SET_COMPLETION_BIT);
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00CBA570(this: u32, state: u32) -> u8 {
    unsafe { update_slide_task(this, state, false) }
});

lf_checker_rt::export!(thiscall, mut_00CBA570(this: u32, state: u32) -> u8 {
    unsafe { update_slide_task(this, state, true) }
});
