// original: 0x009EEFD0 ped_task_iteration
/// Applies one keyed task entry to the ped's task state.
///
/// The original is a `thiscall` method with four 32-bit stack arguments:
/// a pointer to a task-pointer array, a task key, a caller slot reused as the
/// loop index, and a signed element count. The third argument is overwritten
/// with zero before the signed count check and is incremented after each
/// element. For a matching key, the method selects the context at `+0x38`,
/// compares the task's weight at `+0x48` with the ped threshold at `+0xB10`,
/// and, when accepted, copies the task's vectors and state words into the ped
/// record. The vector sign bits are flipped when the task key does not match
/// the context owner. A mode-1 entry also updates a reference slot, asks the
/// task-context callback for a state value, forwards that value and the
/// selected context to the ped callback, ticks the task context, and clears
/// bit zero of the task flag byte at `+0xA5`. A mode-3 entry consults the
/// task-type table and clears that flag for type 2 or when the x87-returned
/// task score is strictly greater than the configured threshold. Floating
/// comparisons retain ordered-comparison behavior: NaN never satisfies `>`.
/// The rewrite treats pointers stored in the original's 32-bit object layout
/// as 32-bit addresses, while keeping byte offsets and scalar values explicit.
///
/// The outgoing calls are checker stand-ins with per-callee answers. This
/// function does not execute any original code or perform inline assembly.
const TASK_CONTEXT: u32 = 0xA0;
const TASK_KIND: u32 = 0x08;
const TASK_PAYLOAD_MATCH: u32 = 0x40;
const TASK_PAYLOAD_OTHER: u32 = 0x30;
const TASK_FLAG: u32 = 0xA5;
const TASK_FLAG_CLEAR: u8 = 0xFE;
const TASK_LOW_INDEX: u32 = 0xA8;
const TASK_HIGH_INDEX: u32 = 0xAA;
const TASK_WEIGHT: u32 = 0x48;
const TASK_VECTOR_X: u32 = 0x50;
const TASK_VECTOR_Y: u32 = 0x54;
const TASK_VECTOR_Z: u32 = 0x58;
const CONTEXT_OWNER: u32 = 0x34;
const CONTEXT_SELECTED: u32 = 0x38;
const CONTEXT_LIMITS: u32 = 0x3C;
const CONTEXT_MODE_MATCH: u32 = 0x68;
const CONTEXT_MODE_OTHER: u32 = 0x70;
const PED_KIND_TABLE: u32 = 0x12B9_C78;
const TASK_TABLE_GATE: u32 = 0x103B_690;
const PED_WEIGHT: u32 = 0xB10;
const PED_STATE: u32 = 0x14C;
const PED_REFERENCE: u32 = 0xAB0;
const PED_VECTOR_A: u32 = 0xAC0;
const PED_VECTOR_B: u32 = 0xAC4;
const PED_VECTOR_C: u32 = 0xAC8;
const PED_VECTOR_D: u32 = 0xACC;
const PED_BLEND_X: u32 = 0xB00;
const PED_BLEND_Y: u32 = 0xB04;
const PED_BLEND_Z: u32 = 0xB08;
const PED_CALLBACK: u32 = 0x3C0;
const KIND_TABLE_ROWS: u32 = 0x70;
const KIND_ROW_STRIDE: u32 = 8;
const DATA_KIND: u32 = 0x04;
const DATA_COUNT: u32 = 0xCC;
const DATA_ITEMS: u32 = 0x8C;
const DATA_ITEM_STRIDE: u32 = 0x20;
const FIRST_TASK_SLOT: u32 = 0x04;
const CALLEE_TASK_QUERY: u32 = 1;
const CALLEE_TASK_STATE: u32 = 2;
const CALLEE_TASK_SCORE: u32 = 3;
const CALLEE_LOOKUP: u32 = 4;
const CALLEE_RELEASE: u32 = 5;
const CALLEE_RETAIN: u32 = 6;
const CALLEE_SELECT: u32 = 7;
const CALLEE_UPDATE: u32 = 8;
const CALLEE_TICK: u32 = 9;
const SCORE_THRESHOLD: u32 = 0xFE8B_F4;
const SIGN_MASK: u32 = 0xFE8F_A0;
const ZERO_FLOAT: u32 = 0xFE8_830;

lf_checker_rt::export!(thiscall, rw_009eefd0(this: u32, task_array: u32, task_key: u32, _loop_slot: u32, task_count: i32) -> () {
    fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }
    fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write_unaligned(value) };
    }
    fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) };
    }
    fn read_at_u8(base: u32, offset: u32) -> u8 {
        read_u8(base.wrapping_add(offset))
    }
    fn read_at_u16(base: u32, offset: u32) -> u16 {
        read_u16(base.wrapping_add(offset))
    }
    fn read_at_u32(base: u32, offset: u32) -> u32 {
        read_u32(base.wrapping_add(offset))
    }
    fn write_at_u8(base: u32, offset: u32, value: u8) {
        write_u8(base.wrapping_add(offset), value);
    }
    fn write_at_u32(base: u32, offset: u32, value: u32) {
        write_u32(base.wrapping_add(offset), value);
    }
    fn read_at_f32(base: u32, offset: u32) -> f32 {
        f32::from_bits(read_at_u32(base, offset))
    }

    let mut current_array = task_array;
    let mut index = 0i32;
    while index < task_count {
        let task_slot = current_array.wrapping_add((index as u32).wrapping_mul(4));
        let task = read_u32(task_slot);
        let context = read_at_u32(task, TASK_CONTEXT);
        let owner = read_at_u32(context, CONTEXT_OWNER);
        let key_matches = task_key == owner;
        let mode = read_at_u32(
            context,
            if key_matches { CONTEXT_MODE_MATCH } else { CONTEXT_MODE_OTHER },
        );

        if mode == 1 {
            let selected = if key_matches {
                read_at_u32(context, CONTEXT_SELECTED)
            } else {
                owner
            };
            let payload_offset = if key_matches { TASK_PAYLOAD_MATCH } else { TASK_PAYLOAD_OTHER };
            let payload = task.wrapping_add(payload_offset);
            let gate = lf_checker_rt::global::<u8>(TASK_TABLE_GATE) as u32;
            let table_enabled = read_u8(gate) != 0;
            let task_weight = read_at_f32(payload, 8);
            let current_weight = read_at_f32(this, PED_WEIGHT);

            if table_enabled && task_weight > current_weight && selected != 0 {
                let query_result: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_TASK_QUERY, u32, selected
                );
                if query_result != 0x0F {
                    let relation = read_at_u32(
                        context,
                        if key_matches { CONTEXT_LIMITS } else { 0x40 },
                    );
                    let source_x = read_at_u32(payload, 0x10);
                    let source_y = read_at_u32(payload, 0x14);
                    let source_z = read_at_u32(payload, 0x18);
                    let sign_bits = read_u32(lf_checker_rt::global::<u32>(SIGN_MASK) as u32);
                    let (blend_x, blend_y, blend_z) = if key_matches {
                        (source_x, source_y, source_z)
                    } else {
                        (source_x ^ sign_bits, source_y ^ sign_bits, source_z ^ sign_bits)
                    };
                    let kind_root = read_u32(lf_checker_rt::global::<u32>(PED_KIND_TABLE) as u32);
                    let kind_rows = read_at_u32(kind_root, KIND_TABLE_ROWS);
                    let type_index = read_at_u16(selected, TASK_KIND) as u32;
                    let kind_flags = read_at_u32(
                        kind_rows,
                        type_index.wrapping_mul(KIND_ROW_STRIDE).wrapping_add(4),
                    ) as u8 & 3;
                    let zero = f32::from_bits(read_u32(lf_checker_rt::global::<u32>(ZERO_FLOAT) as u32));
                    let source_height = f32::from_bits(blend_z);
                    let mut accepted = !(zero > source_height);
                    if accepted && relation != 0 {
                        let limit = read_at_f32(relation, 0x108);
                        if kind_flags == 2 {
                            accepted = !(task_weight > limit);
                        } else {
                            let adjusted_limit = core::hint::black_box(limit)
                                - core::hint::black_box(zero);
                            accepted = !(task_weight > adjusted_limit);
                        }
                    }

                    if accepted {
                        write_at_u32(this, PED_WEIGHT, read_at_u32(payload, 8));
                        write_at_u32(this, PED_VECTOR_A, read_at_u32(payload, 0));
                        write_at_u32(this, PED_VECTOR_B, read_at_u32(payload, 4));
                        write_at_u32(this, PED_VECTOR_C, read_at_u32(payload, 8));
                        write_at_u32(this, PED_VECTOR_D, read_at_u32(payload, 0x0C));
                        write_at_u32(this, PED_BLEND_X, blend_x);
                        write_at_u32(this, PED_BLEND_Y, blend_y);
                        write_at_u32(this, PED_BLEND_Z, blend_z);

                        let new_reference: u32 = lf_checker_rt::callee_cdecl!(
                            CALLEE_LOOKUP, u32, selected
                        );
                        if new_reference != 0 {
                            let ref_kind = (read_at_u32(new_reference, 0x28) >> 6) & 0x0F;
                            if (2..5).contains(&ref_kind) {
                                let reference_slot = this.wrapping_add(PED_REFERENCE);
                                let old_reference = read_u32(reference_slot);
                                if old_reference != 0 {
                                    let _: u32 = lf_checker_rt::callee_thiscall!(
                                        CALLEE_RELEASE, u32, old_reference, reference_slot
                                    );
                                }
                                write_u32(reference_slot, new_reference);
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    CALLEE_RETAIN, u32, new_reference, reference_slot
                                );
                            }
                        }

                        let child = read_at_u32(selected, FIRST_TASK_SLOT);
                        let data_object = read_at_u32(child, 0x0C);
                        let data_kind = read_at_u8(data_object.wrapping_add(DATA_KIND));
                        let state_value = if data_kind == 10 || data_kind == 4 || data_kind == 5 {
                            let task_index = if key_matches {
                                read_at_u16(task, TASK_HIGH_INDEX) as u32
                            } else {
                                read_at_u16(task, TASK_LOW_INDEX) as u32
                            };
                            let task_index_signed = task_index as i32;
                            let item_count = read_at_u32(data_object, DATA_COUNT) as i32;
                            let item = if task_index_signed >= 0 && task_index_signed < item_count {
                                let items = read_at_u32(data_object, DATA_ITEMS);
                                read_at_u8(items.wrapping_add(task_index.wrapping_mul(DATA_ITEM_STRIDE)).wrapping_add(0x0C)) as u32
                            } else {
                                0
                            };
                            lf_checker_rt::callee_thiscall!(
                                CALLEE_TASK_STATE, u32, data_object, item
                            )
                        } else {
                            let mut array_slot = current_array;
                            let state: u32 = lf_checker_rt::callee_thiscall!(
                                CALLEE_SELECT, u32,
                                &mut array_slot as *mut u32 as u32
                            );
                            current_array = array_slot;
                            state & 0xFF
                        };
                        write_at_u32(this, PED_STATE, state_value);
                        let callback_context = read_at_u32(
                            context,
                            if key_matches { CONTEXT_MODE_OTHER } else { CONTEXT_MODE_MATCH },
                        );
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CALLEE_UPDATE, u32, this.wrapping_add(PED_CALLBACK),
                            state_value, new_reference, callback_context
                        );
                    }
                }
            }

            if current_array != 0 {
                let live_task = read_u32(
                    current_array.wrapping_add((index as u32).wrapping_mul(4)),
                );
                let live_context = read_at_u32(live_task, TASK_CONTEXT);
                let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_TICK, u32, live_context);
                let old_flags = read_at_u8(live_task, TASK_FLAG);
                write_at_u8(live_task, TASK_FLAG, old_flags & TASK_FLAG_CLEAR);
            }
        } else if mode == 3 {
            let selected = if key_matches {
                read_at_u32(context, CONTEXT_SELECTED)
            } else {
                owner
            };
            let should_clear = if selected == 0 {
                true
            } else {
                let kind_root = read_u32(lf_checker_rt::global::<u32>(PED_KIND_TABLE) as u32);
                let kind_rows = read_at_u32(kind_root, KIND_TABLE_ROWS);
                let type_index = read_at_u16(selected, TASK_KIND) as u32;
                let kind_flags = read_at_u32(
                    kind_rows,
                    type_index.wrapping_mul(KIND_ROW_STRIDE).wrapping_add(4),
                ) as u8 & 3;
                if kind_flags == 2 {
                    true
                } else {
                    let child = read_at_u32(selected, FIRST_TASK_SLOT);
                    let score: f32 = lf_checker_rt::callee_thiscall!(
                        CALLEE_TASK_SCORE, f32, child
                    );
                    let threshold = f32::from_bits(
                        read_u32(lf_checker_rt::global::<u32>(SCORE_THRESHOLD) as u32),
                    );
                    score > threshold
                }
            };
            if should_clear {
                let old_flags = read_at_u8(task, TASK_FLAG);
                write_at_u8(task, TASK_FLAG, old_flags & TASK_FLAG_CLEAR);
            }
        }

        index = index.wrapping_add(1);
    }
});
