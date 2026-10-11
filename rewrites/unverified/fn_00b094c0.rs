// original: 0x00B094C0 net_lobby_state_update
//! Refresh lobby state from the current context, collect five indexed
//! state values, scan the context's linked lists, and merge the resulting
//! flags into the lobby byte before selecting any follow-up callbacks.
//!
//! This contract covers the null first query, context fallback, empty-list
//! path with a zero phase result, and the ordinary flag merge. It does not
//! cover the virtual state dispatch, nonzero result branches, the byte-pair
//! maze, or terminal callbacks. The function's return comparison is AL,
//! because only that byte is assigned on the return path. The differential
//! checks compare AL, heap writes, and three declared global words. Stack
//! contents and outgoing call traces are disabled, so this proof does not
//! establish stack-local or callee-call equivalence.

const C_GET_CONTEXT: u32 = 1;
const C_LOOKUP: u32 = 2;
const C_FROM_CONTEXT: u32 = 3;
const C_FROM_ZERO: u32 = 4;
const C_FROM_INDEX: u32 = 5;
const C_MODE_9: u32 = 6;
const C_MODE_27: u32 = 7;
const C_MODE_1: u32 = 8;
const C_MODE_2: u32 = 9;
const C_MODE_3: u32 = 10;
const C_PHASE: u32 = 11;
const C_LIST_QUERY: u32 = 12;
const C_VIRTUAL_MODE: u32 = 13;
const C_CONTEXT_QUERY: u32 = 14;
const C_LOBBY_QUERY_2: u32 = 15;
const C_AUX_TEST: u32 = 16;
const C_LOBBY_QUERY_3: u32 = 17;
const C_CONTEXT_QUERY_2: u32 = 18;
const C_STATE_QUERY: u32 = 19;
const C_PAIR_TEST: u32 = 20;
const C_GLOBAL_QUERY: u32 = 21;
const C_FINISH_A: u32 = 22;
const C_FINISH_B: u32 = 23;
const C_FINISH_C: u32 = 24;
const C_FINISH_D: u32 = 25;
const C_FINISH_E: u32 = 26;
const C_FINISH_F: u32 = 27;
const C_FINISH_G: u32 = 28;

const G_STATE_FLAGS: u32 = 0x0161_54D0;
const G_MODE: u32 = 0x0116_0E78;
const G_EXTERNAL_FLAG: u32 = 0x017F_5EB3;
const VT_MODE: u32 = 0x12C;

#[inline(always)]
unsafe fn rd32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn rd8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn wr8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

/// Search a forward-linked list for a tag, stopping on the same rising
/// state transition used by the original list walkers.
unsafe fn find_tag_with_rising_abort(mut node: u32, wanted: u32) -> bool {
    unsafe {
        if node == 0 {
            return false;
        }
        let mut previous = (rd32(node.wrapping_add(8)) >> 1) & 7;
        loop {
            let state = (rd32(node.wrapping_add(8)) >> 1) & 7;
            if previous < state && previous >= 2 {
                return false;
            }
            previous = state;
            if rd32(node.wrapping_add(4)) == wanted {
                return true;
            }
            node = rd32(node.wrapping_add(0x0C));
            if node == 0 {
                return false;
            }
        }
    }
}

unsafe fn virtual_mode(context: u32) -> u32 {
    unsafe {
        let table = rd32(context);
        let target = rd32(table.wrapping_add(VT_MODE));
        let function: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        function(context)
    }
}

unsafe fn run(this: u32, wrong_return: bool) -> u32 {
    unsafe {
        let context = lf_checker_rt::callee_thiscall!(C_GET_CONTEXT, u32, this);
        let list_owner = rd32(context.wrapping_add(0x224));
        let first_query = lf_checker_rt::callee_thiscall!(
            C_LOOKUP,
            u32,
            context.wrapping_add(0x2B0)
        );

        let mut saved_primary = 0;
        let mut saved_secondary;
        let mut state_record;
        if first_query != 0 {
            let record = rd32(first_query.wrapping_add(0x18));
            saved_primary = lf_checker_rt::callee_cdecl!(C_FROM_CONTEXT, u32, record);
            saved_secondary = 0;
            state_record = saved_primary;
        } else {
            saved_secondary = lf_checker_rt::callee_cdecl!(C_FROM_ZERO, u32, 0);
            let count = rd32(context.wrapping_add(0x2B0));
            let index = count.wrapping_add(3).wrapping_mul(12).wrapping_add(0x2B0);
            let indexed_value = rd32(context.wrapping_add(index));
            state_record = lf_checker_rt::callee_cdecl!(C_FROM_INDEX, u32, indexed_value);
            let special_record = rd32(state_record.wrapping_add(8)) == 3
                && find_tag_with_rising_abort(
                    rd32(list_owner.wrapping_add(0x2E0)),
                    0x40E,
                );
            if special_record || rd8(context.wrapping_add(0x274)) & 4 != 0 {
                saved_primary = state_record;
            } else {
                saved_primary = context;
            }
            // The fallback path uses the selected state as its active context.
            state_record = context;
        }

        let mode_9 = lf_checker_rt::callee_thiscall!(C_MODE_9, u32, this, 9, 0);
        let mode_27 = lf_checker_rt::callee_thiscall!(C_MODE_27, u32, this, 0x1B, 0);
        let mode_1 = lf_checker_rt::callee_thiscall!(C_MODE_1, u32, this, 1, 0);
        let mode_2 = lf_checker_rt::callee_thiscall!(C_MODE_2, u32, this, 2, 0);
        let mode_3 = lf_checker_rt::callee_thiscall!(C_MODE_3, u32, this, 3, 0);
        let mut out_mode = 0u32;
        let mut out_value = 0u32;
        let phase_result = lf_checker_rt::callee_thiscall!(
            C_PHASE,
            u32,
            list_owner,
            (&mut out_mode as *mut u32) as u32,
            (&mut out_value as *mut u32) as u32
        );

        let scan_414 = find_tag_with_rising_abort(
            rd32(list_owner.wrapping_add(0x2E0)),
            0x414,
        );
        let _phase_tag = mode_27 == 9 || mode_27 == 0x0C;
        let list_value = lf_checker_rt::callee_cdecl!(C_LIST_QUERY, u32, 0, 0);
        let list_value = if rd8(this.wrapping_add(0x1C4)) & 8 != 0 {
            0
        } else {
            list_value
        };

        let mut state_code = rd32(saved_primary.wrapping_add(8));
        let mut local_14 = false;
        let mut local_15 = false;
        let mut local_16 = false;
        if phase_result & 0xFF != 0 && state_code == 0 {
            local_16 = true;
        } else {
            match state_code {
                1 | 2 => {
                    local_15 = true;
                    local_14 = true;
                }
                3 => {
                    let table = rd32(context);
                    let value_ptr = rd32(context.wrapping_add(0x2C4));
                    if value_ptr != 0 {
                        let expected = (rd32(value_ptr.wrapping_add(0x2E)) as u16 as i16) as i32;
                        let observed = virtual_mode(context) as i32;
                        if expected == observed {
                            local_16 = true;
                        } else {
                            local_15 = true;
                            local_14 = true;
                        }
                    } else {
                        local_15 = true;
                        local_14 = true;
                    }
                    let _ = table;
                }
                5 => {
                    local_15 = false;
                }
                _ => {
                    local_15 = false;
                    local_14 = state_code == 0;
                }
            }
        }

        let mode_is_two = rd32(this.wrapping_add(0x144)) == 2;
        let stored_mode_is_zero = rd32(lf_checker_rt::global::<u32>(G_MODE) as u32) == 0;
        let context_has_flag = rd8(context.wrapping_add(0x26C)) & 4 != 0;
        let context_query = if context_has_flag {
            lf_checker_rt::callee_thiscall!(C_CONTEXT_QUERY, u32, context)
        } else {
            0
        };
        let query_flag = context_has_flag && (context_query & 0xFF) == 0;
        let lobby_query = lf_checker_rt::callee_thiscall!(C_LOBBY_QUERY_2, u32, this, 2, 0);
        let mut auxiliary = saved_secondary;
        let mut transient_flag = false;
        if lobby_query != 0 && auxiliary != 0 {
            if rd8(auxiliary.wrapping_add(0x2D9)) == 0 {
                transient_flag = query_flag;
            } else {
                transient_flag = true;
            }
        }

        let auxiliary = saved_secondary;
        let auxiliary_data = if auxiliary == 0 {
            0
        } else {
            rd32(auxiliary.wrapping_add(0xB30))
        };
        let auxiliary_pass = if auxiliary_data != 0 {
            lf_checker_rt::callee_cdecl!(C_AUX_TEST, u32, auxiliary_data, 1) & 0xFF != 0
        } else {
            false
        };
        let lobby_query_three = lf_checker_rt::callee_thiscall!(C_LOBBY_QUERY_3, u32, this, 3, 0);

        let initial_global = rd8(lf_checker_rt::global::<u8>(G_STATE_FLAGS) as u32);
        let state_low = context as u8;
        let mut next_global = if lobby_query_three != 0 {
            initial_global
        } else {
            state_low
        };
        let global_mode_six = rd32(context.wrapping_add(0x2B0)) == 6;
        let saved_gate = if global_mode_six { 1u8 } else { 0u8 };
        let state_query = lf_checker_rt::callee_thiscall!(C_STATE_QUERY, u32, list_owner);
        lf_checker_rt::global::<u8>(G_STATE_FLAGS).write(next_global);

        let external_flag = rd8(lf_checker_rt::global::<u8>(G_EXTERNAL_FLAG) as u32);
        let mut list_flag_b = scan_414;
        let mut list_flag_c = false;
        let mut byte_maze = false;
        let mut late_flag = false;
        if list_value != 0 {
            if let Some(object) = (list_value != 0).then_some(list_value) {
                byte_maze = rd8(object.wrapping_add(0x26FE))
                    ^ rd8(object.wrapping_add(0x26FC))
                    > 0x7F;
            }
        }
        if list_owner != 0 {
            list_flag_b |= find_tag_with_rising_abort(
                rd32(list_owner.wrapping_add(0x2E0)),
                0xD3,
            );
            list_flag_c = find_tag_with_rising_abort(
                rd32(list_owner.wrapping_add(0x2E0)),
                0x13F,
            );
        }
        if state_query != 0 && !query_flag {
            late_flag = true;
        }

        let _ = (
            mode_9,
            mode_1,
            mode_2,
            mode_3,
            scan_414,
            state_code,
            mode_is_two,
            stored_mode_is_zero,
            query_flag,
            transient_flag,
            auxiliary_pass,
            external_flag,
            list_flag_b,
            list_flag_c,
            byte_maze,
            late_flag,
            saved_gate,
            next_global,
        );

        let old_lobby_byte = rd8(this.wrapping_add(0x1C5));
        let merged_low_bit = saved_gate & 1;
        wr8(
            this.wrapping_add(0x1C5),
            (old_lobby_byte & !1) | merged_low_bit,
        );

        // The terminal callback matrix is reached only for the excluded
        // nonzero mode branches in this contract.
        if rd8(this.wrapping_add(0x1C4)) & 0x20 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(C_FINISH_A, u32, this);
        }
        if wrong_return { 0 } else { 1 }
    }
}

lf_checker_rt::export!(thiscall, rw_00B094C0(this: u32) -> u32 {
    unsafe { run(this, false) }
});

lf_checker_rt::export!(thiscall, rw_00B094C0_mut(this: u32) -> u32 {
    unsafe { run(this, true) }
});
