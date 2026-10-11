// original: 0x00CA3150 task_update_response_weight

/// Update a task's response weight from metadata, ped class, and event tests.
///
/// `this` stores the weight at `+4`, an entity pointer at `+0`, the helper
/// argument at `+8`, and task kind at `+0x0c`. The first stack argument is the
/// ped; the second is retained by the caller but is not read here. The ped's
/// signed table index is at `+0x2e`, its player byte at `+0x219`, its mode byte
/// at `+0xa60`, and its special flags at `+0x26c`. Non-player weights use the
/// indexed table factor at entry `+0xf8`. The function calls the task weight
/// helper, then applies one of the class factors selected by task metadata and
/// the signed result from the event helper. Relation callbacks may apply a
/// final factor capped at the configured limit. All float products preserve
/// the original operand order; unordered comparisons follow the original
/// branches. This is a 32-bit thiscall with two stack words and no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00ca3150(this: u32, ped: u32, _context: u32) -> u32 {
    unsafe {
        const TASK_ENTITY: u32 = 0x00;
        const TASK_WEIGHT: u32 = 0x04;
        const TASK_AUXILIARY: u32 = 0x08;
        const TASK_KIND: u32 = 0x0c;
        const PED_INDEX: u32 = 0x2e;
        const PED_PLAYER: u32 = 0x219;
        const PED_MODE: u32 = 0xa60;
        const PED_SPECIAL_FLAGS: u32 = 0x26c;
        const ENTITY_FLAGS: u32 = 0x28;
        const ENTITY_PLAYER: u32 = 0x219;
        const INFO_KIND: u32 = 0x0c;
        const INFO_TYPE_FLAGS: u32 = 0x20;
        const SPECIAL_ENTITY_MASK: u32 = 0x3c0;
        const SPECIAL_ENTITY_KIND: u32 = 0xc0;
        const KIND_LOW_BASE: u32 = 0x1a2;
        const KIND_HIGH_BASE: u32 = 0x4c3;
        const KIND_HIGH_COUNT: u32 = 13;
        const KIND_UPPER_BOUND: i32 = 0x4c2;
        const KIND_LOWER_BOUND: i32 = 0x4c1;
        const CLASS_TABLE_VA: u32 = 0x1295cd8;
        const CLASS_TABLE_FACTOR: u32 = 0xf8;
        const START_WEIGHT_LIMIT_VA: u32 = 0x1050d58;
        const PLAYER_FACTOR_A_VA: u32 = 0x00fe8800;
        const PLAYER_FACTOR_B_VA: u32 = 0x00fe8870;
        const LOW_KIND_FACTOR_A_VA: u32 = 0x00fe8830;
        const LOW_KIND_FACTOR_B_VA: u32 = 0x00fe888c;
        const PED_MODE_FACTOR_VA: u32 = 0x00fe8960;
        const FINAL_FACTOR_VA: u32 = 0x00fe8800;
        const FINAL_CAP_VA: u32 = 0x00ed7534;

        const LOW_KIND_MAP: [u8; 8] = [0, 0, 1, 2, 2, 0, 0, 1];
        const HIGH_KIND_MAP: [u8; 14] = [0, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 0];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_i16(address: u32) -> i16 {
            unsafe { (address as *const i16).read_unaligned() }
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
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }

        #[inline(always)]
        unsafe fn static_f32(va: u32) -> f32 {
            unsafe { read_f32(lf_checker_rt::relocated(va)) }
        }

        #[inline(always)]
        fn multiply_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn apply_weight_factor(task: u32, factor_va: u32) {
            let weight_address = task.wrapping_add(TASK_WEIGHT);
            let weight = unsafe { read_f32(weight_address) };
            let factor = unsafe { static_f32(factor_va) };
            unsafe { write_f32(weight_address, multiply_f32(weight, factor)) };
        }

        let task_kind = read_u32(this.wrapping_add(TASK_KIND));
        let metadata: u32 = lf_checker_rt::callee_cdecl!(1, u32, task_kind);
        let metadata_kind = read_u32(metadata.wrapping_add(INFO_KIND));
        let initial_weight = read_f32(this.wrapping_add(TASK_WEIGHT));
        let initial_limit = static_f32(START_WEIGHT_LIMIT_VA);
        let weight_is_not_equal_to_limit = initial_weight != initial_limit;

        let kind_uses_player_factor = !matches!(metadata_kind, 3 | 4 | 5);
        let metadata_has_player_override = metadata_kind == 1;

        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, ped);
        let target_is_player = read_u8(ped.wrapping_add(PED_PLAYER)) != 0;
        if target_is_player {
            if kind_uses_player_factor && metadata_has_player_override {
                // This metadata kind leaves the helper-adjusted weight intact.
            } else if kind_uses_player_factor {
                unsafe { apply_weight_factor(this, PLAYER_FACTOR_A_VA) };
            } else {
                unsafe { apply_weight_factor(this, PLAYER_FACTOR_B_VA) };
            }
        } else {
            let signed_index = unsafe { read_i16(ped.wrapping_add(PED_INDEX)) } as i32;
            let table_base = lf_checker_rt::relocated(CLASS_TABLE_VA);
            let table_slot = table_base.wrapping_add((signed_index as u32).wrapping_mul(4));
            let class_data = unsafe { read_u32(table_slot) };
            let factor = unsafe { read_f32(class_data.wrapping_add(CLASS_TABLE_FACTOR)) };
            let weight_address = this.wrapping_add(TASK_WEIGHT);
            let weight = unsafe { read_f32(weight_address) };
            unsafe { write_f32(weight_address, multiply_f32(weight, factor)) };
        }

        let auxiliary = read_u32(this.wrapping_add(TASK_AUXILIARY));
        if (auxiliary as i32) > -1 {
            let event_code: u32 = lf_checker_rt::callee_cdecl!(3, u32, ped, auxiliary);
            if (event_code as i32) > KIND_UPPER_BOUND {
                let high_index = event_code.wrapping_sub(KIND_HIGH_BASE);
                if high_index <= KIND_HIGH_COUNT {
                    match HIGH_KIND_MAP[high_index as usize] {
                        0 => unsafe { apply_weight_factor(this, LOW_KIND_FACTOR_A_VA) },
                        1 => unsafe { apply_weight_factor(this, LOW_KIND_FACTOR_B_VA) },
                        _ => {}
                    }
                }
            } else if (event_code as i32) >= KIND_LOWER_BOUND {
                unsafe { apply_weight_factor(this, LOW_KIND_FACTOR_B_VA) };
            } else {
                let low_index = event_code.wrapping_sub(KIND_LOW_BASE);
                if low_index <= 7 {
                    match LOW_KIND_MAP[low_index as usize] {
                        0 => unsafe { apply_weight_factor(this, LOW_KIND_FACTOR_B_VA) },
                        1 => unsafe { apply_weight_factor(this, LOW_KIND_FACTOR_A_VA) },
                        _ => {}
                    }
                }
            }
        }

        if read_u8(ped.wrapping_add(PED_SPECIAL_FLAGS)) & 4 != 0
            && read_u8(ped.wrapping_add(PED_MODE)) == 1
        {
            unsafe { apply_weight_factor(this, PED_MODE_FACTOR_VA) };
        }

        let entity = read_u32(this.wrapping_add(TASK_ENTITY));
        if entity == 0 || entity == ped
            || read_u32(entity.wrapping_add(ENTITY_FLAGS)) & SPECIAL_ENTITY_MASK
                != SPECIAL_ENTITY_KIND
            || !kind_uses_player_factor
        {
            return 0;
        }

        let entity_is_player = read_u8(entity.wrapping_add(ENTITY_PLAYER)) != 0;
        let relation_is_valid = if entity_is_player {
            let accepted: u8 = lf_checker_rt::callee_cdecl!(4, u8, ped, entity);
            accepted != 0 && weight_is_not_equal_to_limit
        } else if target_is_player {
            let accepted: u8 = lf_checker_rt::callee_cdecl!(5, u8, ped, entity);
            accepted != 0
        } else if read_u8(ped.wrapping_add(PED_MODE)) == 2 {
            let accepted: u8 = lf_checker_rt::callee_cdecl!(4, u8, ped, entity);
            accepted != 0
        } else {
            false
        };

        if relation_is_valid {
            let task_weight_address = this.wrapping_add(TASK_WEIGHT);
            let weight = read_f32(task_weight_address);
            let scaled_weight = multiply_f32(weight, static_f32(FINAL_FACTOR_VA));
            let cap = static_f32(FINAL_CAP_VA);
            unsafe { write_f32(task_weight_address, if scaled_weight > cap { cap } else { scaled_weight }) };
        }
        0
    }
});
