// original: 0x00CA2F20 task_update_threat_weight

/// Update a threat task's weight and its companion state from a target ped.
///
/// `this` points to the task object: kind at `+0x0c`, weight at `+4`, and a
/// helper argument at `+8`. `target` and `state` are the two stack arguments
/// in that order. The target supplies its weight threshold at `+0xb84`, child
/// link at `+0x6c`, player byte at `+0x219`, mode byte at `+0xa60`, component
/// pointer at `+0x224`, and a thiscall float query in vtable slot `0xfc`.
///
/// The routine first looks up task metadata. A positive target threshold with
/// metadata bit 11 clear may trigger a transition callback and update the
/// state's limit and delta. It then checks task eligibility and a target
/// classification. For the special classification it applies the configured
/// task multipliers, checks mode and cover state, and may compare random-scaled
/// time with two scripted vtable float queries. The fallback component check
/// applies its own multiplier. Float comparisons are ordered; NaN follows the
/// unordered branches of the original. This is a 32-bit thiscall with two
/// stack arguments and callee cleanup; it has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00ca2f20(this: u32, target: u32, state: u32) -> u32 {
    unsafe {
        const TASK_KIND: u32 = 0x0c;
        const TASK_WEIGHT: u32 = 0x04;
        const TASK_AUXILIARY: u32 = 0x08;
        const STATE_FLAGS: u32 = 0x04;
        const STATE_LIMIT: u32 = 0x0c;
        const TARGET_CHILD: u32 = 0x6c;
        const TARGET_THRESHOLD: u32 = 0xb84;
        const TARGET_PLAYER: u32 = 0x219;
        const TARGET_MODE: u32 = 0xa60;
        const TARGET_COMPONENT: u32 = 0x224;
        const TARGET_VTABLE: u32 = 0x00;
        const TARGET_QUERY_SLOT: u32 = 0xfc;
        const CHILD_BLOCKED: u32 = 0x0e;
        const INFO_FLAGS: u32 = 0x20;
        const FLAG_NO_TRANSITION: u32 = 1 << 11;
        const FLAG_EXTRA_WEIGHT: u32 = 1 << 9;
        const TASK_SPECIAL_RESULT: u32 = 0x4b5;
        const GLOBAL_OBJECT_VA: u32 = 0x11735b4;
        const GLOBAL_MODE_VA: u32 = 0x11d6fd4;
        const SCALE_SPECIAL_VA: u32 = 0x1050d5c;
        const SCALE_METADATA_VA: u32 = 0x1050d60;
        const SCALE_FALLBACK_VA: u32 = 0x1050d64;
        const WEIGHT_LIMIT_VA: u32 = 0x1050d68;
        const RANDOM_LIMIT_VA: u32 = 0x1050d6c;
        const RANDOM_SCALE_VA: u32 = 0x00fe8684;
        const TIME_GAP_VA: u32 = 0x00fe8bb0;
        const TIME_OFFSET_VA: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }

        #[inline(always)]
        unsafe fn read_static_f32(va: u32) -> f32 {
            unsafe { read_f32(lf_checker_rt::relocated(va)) }
        }

        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn subtract_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn multiply_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn query_target_time(target: u32) -> f32 {
            let vtable = unsafe { read_u32(target.wrapping_add(TARGET_VTABLE)) };
            let query = unsafe { read_u32(vtable.wrapping_add(TARGET_QUERY_SLOT)) };
            let method: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(query as usize) };
            method(target)
        }

        let task_kind = read_u32(this.wrapping_add(TASK_KIND));
        let metadata: u32 = lf_checker_rt::callee_cdecl!(1, u32, task_kind);
        let target_limit = read_f32(target.wrapping_add(TARGET_THRESHOLD));
        let metadata_flags = read_u32(metadata.wrapping_add(INFO_FLAGS));

        if target_limit > 0.0 && metadata_flags & FLAG_NO_TRANSITION == 0 {
            let current_target: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0u32);
            if current_target == target {
                let stored_object = read_u32(lf_checker_rt::relocated(GLOBAL_OBJECT_VA));
                let active_object: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0u32);
                let active_fields = read_u32(active_object.wrapping_add(0x228));
                write_u32(active_fields.wrapping_add(0x4f8), stored_object);
            }

            let child = read_u32(target.wrapping_add(TARGET_CHILD));
            let child_blocks_transition = child != 0
                && read_u8(child.wrapping_add(CHILD_BLOCKED)) != 0;
            if !child_blocks_transition {
                let task_weight_address = this.wrapping_add(TASK_WEIGHT);
                let task_weight = read_f32(task_weight_address);
                let delta = if target_limit < task_weight {
                    0.0
                } else {
                    subtract_f32(target_limit, task_weight)
                };
                write_f32(state.wrapping_add(STATE_LIMIT), target_limit);
                let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, target, delta.to_bits());
            }
        }

        let task_is_eligible: u8 =
            lf_checker_rt::callee_cdecl!(4, u8, task_kind, 0x38u32);
        if task_is_eligible == 0 {
            return 0;
        }

        let task_auxiliary = read_u32(this.wrapping_add(TASK_AUXILIARY));
        let event_result: u32 = lf_checker_rt::callee_cdecl!(5, u32, target, task_auxiliary);
        if event_result == TASK_SPECIAL_RESULT {
            let applies_special_scale: u8 =
                lf_checker_rt::callee_thiscall!(6, u8, target);
            if applies_special_scale != 0 {
                let task_weight_address = this.wrapping_add(TASK_WEIGHT);
                let task_weight = read_f32(task_weight_address);
                let scale = read_static_f32(SCALE_SPECIAL_VA);
                write_f32(task_weight_address, multiply_f32(scale, task_weight));
            }

            let global_mode = read_u32(lf_checker_rt::relocated(GLOBAL_MODE_VA));
            let is_player = read_u8(target.wrapping_add(TARGET_PLAYER)) != 0;
            let special_mode = global_mode == 2 && is_player;
            let metadata_has_extra_weight = metadata_flags & FLAG_EXTRA_WEIGHT != 0;
            if metadata_has_extra_weight && !special_mode {
                let task_weight_address = this.wrapping_add(TASK_WEIGHT);
                let task_weight = read_f32(task_weight_address);
                let scale = read_static_f32(SCALE_METADATA_VA);
                write_f32(task_weight_address, multiply_f32(scale, task_weight));
            }

            let target_cover_mode = read_u8(target.wrapping_add(TARGET_MODE)) == 2;
            let cover_suppresses_update = global_mode == 2 && target_cover_mode;
            let task_weight_address = this.wrapping_add(TASK_WEIGHT);
            let task_weight = read_f32(task_weight_address);
            let weight_limit = read_static_f32(WEIGHT_LIMIT_VA);
            if !is_player && task_weight > weight_limit && !cover_suppresses_update {
                let random: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
                let random_as_signed_float = random as i32 as f32;
                let random_scale = read_static_f32(RANDOM_SCALE_VA);
                let scaled_random = multiply_f32(random_as_signed_float, random_scale);
                let random_limit = read_static_f32(RANDOM_LIMIT_VA);
                if random_limit > scaled_random {
                    let current_time = query_target_time(target);
                    let time_gap = read_static_f32(TIME_GAP_VA);
                    if subtract_f32(current_time, task_weight) > time_gap {
                        let later_time = query_target_time(target);
                        let next_weight = add_f32(
                            subtract_f32(later_time, time_gap),
                            read_static_f32(TIME_OFFSET_VA),
                        );
                        write_f32(task_weight_address, next_weight);
                    }
                    let flags_address = state.wrapping_add(STATE_FLAGS);
                    let flags = read_u8(flags_address);
                    write_u8(flags_address, flags | 0x20);
                    return 0;
                }
            }
        }

        let component = read_u32(target.wrapping_add(TARGET_COMPONENT));
        let attachment: u32 = lf_checker_rt::callee_thiscall!(8, u32, component);
        if attachment != 0 {
            let can_apply_fallback: u8 = lf_checker_rt::callee_thiscall!(9, u8, attachment);
            if can_apply_fallback != 0 {
                let task_weight_address = this.wrapping_add(TASK_WEIGHT);
                let task_weight = read_f32(task_weight_address);
                let scale = read_static_f32(SCALE_FALLBACK_VA);
                write_f32(task_weight_address, multiply_f32(scale, task_weight));
            }
        }
        0
    }
});
