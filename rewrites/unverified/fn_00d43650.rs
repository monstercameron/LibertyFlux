// original: 0x00D43650 select_ambient_for_time

/// Select an ambient animation for one task from a ped's scenario state.
///
/// `task` is the task object in ECX. It contains the operation pointer list
/// (`+0x04`), a 16-bit operation count (`+0x08`), the final weight scale
/// (`+0x10`), scenario-table flags (`+0x1c`), a scenario-table index
/// (`+0x20`), and the task mode (`+0x24`). The stack arguments are the ped,
/// a pointer to the caller's output weight, an unused word, option flags,
/// and an operation context pointer. The output weight is initialized,
/// adjusted by successful operation handlers, then multiplied by the task
/// scale. The return value is a byte-sized success flag.
///
/// The ped's virtual slot at `+0xd4` yields the scenario context. Direct
/// helpers are intercepted by the checker. Operation codes 0 through 49
/// select one of seven handler groups; unsupported codes are skipped.
/// Counts, flags, and signed 16-bit scenario values retain the original
/// widths. Float additions and multiplication use the original operand
/// order. The original reuses its second incoming stack argument as a
/// scratch word and passes its address to a handler.
lf_checker_rt::export!(thiscall, rw_00d43650(
    task: u32,
    ped: u32,
    weight_slot: u32,
    _unused_word: u32,
    option_flags: u32,
    operation_context: u32
) -> u8 {
    unsafe {
        const PED_VTABLE: u32 = 0x00;
        const PED_SCENARIO_BITS: u32 = 0x28;
        const PED_SCENARIO_ENTITY: u32 = 0x6c;
        const PED_STATE_BITS: u32 = 0x26c;
        const PED_STATUS: u32 = 0xb30;
        const PED_ENTITY_READY: u32 = 0x0e;
        const SCENARIO_ENTITY_VALUE: u32 = 0x2e;
        const VTABLE_GET_SCENARIO: u32 = 0xd4;
        const TASK_OPERATIONS: u32 = 0x04;
        const TASK_OPERATION_COUNT: u32 = 0x08;
        const TASK_WEIGHT_SCALE: u32 = 0x10;
        const TASK_SCENARIO_FLAGS: u32 = 0x1c;
        const TASK_SCENARIO_INDEX: u32 = 0x20;
        const TASK_MODE: u32 = 0x24;
        const AMBIENT_TABLES: u32 = 0x017208f8;
        const OPTION_SKIP_SCENARIO_LOOKUP: u32 = 0x78;
        const PED_SCENARIO_MASK: u32 = 0x3c0;
        const PED_SCENARIO_MATCH: u32 = 0xc0;
        const PED_DISABLED_BIT: u32 = 0x04;
        const SCENARIO_MODE_TABLE: u32 = 2;
        const SCENARIO_MODE_REQUIRES_EMPTY_CONTEXT: u32 = 1;
        const SCENARIO_MODE_UNCONDITIONAL: u32 = 3;
        const TIME_FLAGS_MASK: u32 = 0x3e0;
        const WEIGHT_ONE_BITS: u32 = 0x3f800000;
        const CALLEE_GET_SCENARIO: u32 = 2;
        const CALLEE_GET_ENTITY_VALUE: u32 = 3;
        const CALLEE_RESOLVE_SCENARIO: u32 = 4;
        const CALLEE_AMBIENT_TABLE_CONTAINS: u32 = 5;
        const CALLEE_TIME_RANGE: u32 = 6;
        const CALLEE_DISPATCH_OPERATION: u32 = 7;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
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
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn multiply_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn operation_group(code: u32) -> u8 {
            match code {
                0 => 0,
                1..=8 | 12 | 13 | 15 | 18..=22 => 1,
                9..=11 | 14 => 2,
                16 | 17 | 23..=26 | 32..=35 | 38..=41 | 43..=49 => 3,
                27..=31 => 4,
                42 => 5,
                36 | 37 => 6,
                _ => 7,
            }
        }
        #[inline(always)]
        unsafe fn dispatch_operation(
            ped: u32,
            operation_context: u32,
            option_flags: u32,
            operation_entry: u32,
            operation_code: u32,
            scratch_word: *mut u32,
        ) -> u8 {
            let scratch_address = scratch_word as u32;
            lf_checker_rt::callee_cdecl!(
                CALLEE_DISPATCH_OPERATION,
                u32,
                ped,
                unsafe { read_u32(operation_entry.wrapping_add(4)) },
                operation_code,
                scratch_address,
                operation_entry,
                option_flags,
                operation_context
            ) as u8
        }

        let weight_output = weight_slot;
        let mut scratch_word = weight_slot;
        let ped_scenario_bits = read_u32(ped.wrapping_add(PED_SCENARIO_BITS)) & PED_SCENARIO_MASK;
        let mut scenario_index = if ped_scenario_bits == PED_SCENARIO_MATCH { ped } else { 0 };
        let mut context_allowed = 1u32;
        if scenario_index != 0
            && read_u8(ped.wrapping_add(PED_STATE_BITS)) & (PED_DISABLED_BIT as u8) != 0
            && read_u32(ped.wrapping_add(PED_STATUS)) != 0
        {
            context_allowed = 0;
        }

        let ped_vtable = read_u32(ped.wrapping_add(PED_VTABLE));
        let get_scenario_address = read_u32(ped_vtable.wrapping_add(VTABLE_GET_SCENARIO));
        let get_scenario: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(get_scenario_address as usize);
        let scenario_context = get_scenario(ped);

        let mut scenario_register = context_allowed;
        write_f32(weight_output, f32::from_bits(WEIGHT_ONE_BITS));
        if option_flags & OPTION_SKIP_SCENARIO_LOOKUP == 0 {
            scratch_word = lf_checker_rt::callee_thiscall!(
                CALLEE_GET_SCENARIO,
                u32,
                scenario_context,
                ped
            );
            scenario_register = scratch_word;
            let entity_value = lf_checker_rt::callee_thiscall!(
                CALLEE_GET_ENTITY_VALUE,
                u32,
                scenario_context,
                ped
            );
            if scratch_word == SCENARIO_ENTITY_VALUE && entity_value != 0 {
                scenario_index = (read_u16(entity_value.wrapping_add(SCENARIO_ENTITY_VALUE)) as i16 as i32) as u32;
            } else {
                let scenario_entity = read_u32(ped.wrapping_add(PED_SCENARIO_ENTITY));
                if scenario_entity == 0
                    || read_u8(scenario_entity.wrapping_add(PED_ENTITY_READY)) == 0
                {
                    scratch_word = lf_checker_rt::callee_thiscall!(
                        CALLEE_RESOLVE_SCENARIO,
                        u32,
                        scenario_context,
                        ped
                    );
                    scenario_index = scratch_word;
                }
            }
        }

        if option_flags & OPTION_SKIP_SCENARIO_LOOKUP == 0 {
            let mode = read_u32(task.wrapping_add(TASK_MODE));
            if mode == SCENARIO_MODE_TABLE {
                if scenario_index == u32::MAX {
                    return 0;
                }
                let table_array = read_u32(lf_checker_rt::global::<u32>(AMBIENT_TABLES) as u32);
                let table_entry = read_u32(table_array.wrapping_add(
                    read_u32(task.wrapping_add(TASK_SCENARIO_INDEX)).wrapping_mul(4),
                ));
                let found = lf_checker_rt::callee_thiscall!(
                    CALLEE_AMBIENT_TABLE_CONTAINS,
                    u32,
                    table_entry,
                    scenario_index
                ) as u8;
                if found == 0 {
                    return 0;
                }
            } else if mode == SCENARIO_MODE_REQUIRES_EMPTY_CONTEXT {
                if scenario_register != 0 {
                    return 0;
                }
            } else if mode != SCENARIO_MODE_UNCONDITIONAL && scenario_index != u32::MAX {
                return 0;
            }
        }

        let scenario_flags = read_u32(task.wrapping_add(TASK_SCENARIO_FLAGS));
        let low_time = scenario_flags & 0x1f;
        let high_time = (scenario_flags >> 5) & 0x1f;
        let mut time_allowed = 1u8;
        if low_time != 0 || scenario_flags & TIME_FLAGS_MASK != 0 {
            time_allowed = lf_checker_rt::callee_cdecl!(
                CALLEE_TIME_RANGE,
                u32,
                low_time,
                high_time
            ) as u8;
            if time_allowed == 0 {
                return 0;
            }
        }

        let mut saw_primary = false;
        let mut primary_repeat = false;
        let mut saw_group_four = false;
        let mut group_four_repeat = false;
        let mut saw_group_two = false;
        let mut group_five_seen = false;
        let mut group15_satisfied = false;
        let mut saw_required = true;
        let mut index = 0u32;
        loop {
            let operation_count = read_u16(task.wrapping_add(TASK_OPERATION_COUNT)) as u32;
            if index >= operation_count {
                break;
            }
            let operation_list = read_u32(task.wrapping_add(TASK_OPERATIONS));
            let operation_entry = read_u32(operation_list.wrapping_add(index.wrapping_mul(4)));
            let operation_code = read_u32(operation_entry);
            scratch_word = 0;
            match operation_group(operation_code) {
                0 => {
                    let result = dispatch_operation(
                        ped, operation_context, option_flags,
                        operation_entry, operation_code, &mut scratch_word,
                    );
                    saw_required = result != 0;
                    if result != 0 {
                        let converted = (scratch_word as i32) as f32;
                        let accumulated = read_f32(weight_output);
                        write_f32(weight_output, add_f32(converted, accumulated));
                    }
                }
                1 => {
                    if saw_primary {
                        primary_repeat = true;
                    } else {
                        primary_repeat = true;
                        let result = dispatch_operation(
                            ped, operation_context, option_flags,
                            operation_entry, operation_code, &mut scratch_word,
                        );
                        if result != 0 {
                            saw_primary = true;
                            let converted = (scratch_word as i32) as f32;
                            let accumulated = read_f32(weight_output);
                            write_f32(weight_output, add_f32(converted, accumulated));
                        }
                    }
                }
                2 => {
                    let result = dispatch_operation(
                        ped, operation_context, option_flags,
                        operation_entry, operation_code, &mut scratch_word,
                    );
                    if result != 0 {
                        context_allowed = 1;
                        saw_group_two = true;
                    } else {
                        context_allowed = u32::from(saw_group_two);
                    }
                }
                3 => {
                    let result = dispatch_operation(
                        ped, operation_context, option_flags,
                        operation_entry, operation_code, &mut scratch_word,
                    );
                    if result == 0 {
                        return 0;
                    }
                    group15_satisfied = true;
                    let converted = (scratch_word as i32) as f32;
                    let accumulated = read_f32(weight_output);
                    write_f32(weight_output, add_f32(converted, accumulated));
                }
                4 => {
                    if saw_group_four {
                        group_four_repeat = true;
                    } else {
                        group_four_repeat = true;
                        let result = dispatch_operation(
                            ped, operation_context, option_flags,
                            operation_entry, operation_code, &mut scratch_word,
                        );
                        if result != 0 {
                            saw_group_four = true;
                            let converted = (scratch_word as i32) as f32;
                            let accumulated = read_f32(weight_output);
                            write_f32(weight_output, add_f32(converted, accumulated));
                        }
                    }
                }
                5 => {
                    group_five_seen = true;
                    let result = dispatch_operation(
                        ped, operation_context, option_flags,
                        operation_entry, operation_code, &mut scratch_word,
                    );
                    if result != 0 {
                        group15_satisfied = true;
                    } else if read_u32(operation_entry.wrapping_add(4)) == 2 {
                        return 0;
                    }
                }
                _ => {}
            }
            index = index.wrapping_add(1);
            let current_count = read_u16(task.wrapping_add(TASK_OPERATION_COUNT)) as u32;
            if (index as i32) >= (current_count as i32) {
                break;
            }
        }

        let mut weight = read_f32(weight_output);
        let task_scale = read_f32(task.wrapping_add(TASK_WEIGHT_SCALE));
        weight = multiply_f32(task_scale, weight);
        write_f32(weight_output, weight);

        let accepted = (saw_primary || !primary_repeat)
            && (saw_group_four || !group_four_repeat)
            && (group15_satisfied || !group_five_seen)
            && saw_required
            && time_allowed != 0
            && context_allowed != 0;
        u8::from(accepted)
    }
});
