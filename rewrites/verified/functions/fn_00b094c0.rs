// original: 0x00B094C0 net_lobby_state_update
//! Refreshes the lobby state from the active network objects, merges the
//! activity byte into the global state, updates the low state bit on the
//! receiver object, conditionally notifies the lobby, and returns one.

const THIS_ACTIVE_MODE: u32 = 0x144;
const THIS_FLAGS: u32 = 0x1c4;
const THIS_STATE: u32 = 0x1c5;
const OBJECT_VTABLE: u32 = 0;
const OBJECT_LINK: u32 = 0x224;
const OBJECT_FLAGS: u32 = 0x26c;
const OBJECT_STATE: u32 = 0x2b0;
const OBJECT_LIST: u32 = 0x2e0;
const OBJECT_MODE_SOURCE: u32 = 0x2c4;
const OBJECT_ACTIVITY_SOURCE: u32 = 0xb30;
const OBJECT_STATUS: u32 = 0x210;
const OBJECT_TYPE: u32 = 0xa70;
const OBJECT_PHASE: u32 = 0xa74;
const OBJECT_ACTION: u32 = 0x398;
const OBJECT_CHILD: u32 = 0x228;
const MODE_FLAGS: u8 = 0x20;
const CANDIDATE_STATE: u32 = 0x8;
const CANDIDATE_BITS: u32 = 0x20;
const CANDIDATE_ACTIVE: u32 = 0x2d9;
const NODE_TAG: u32 = 0x4;
const NODE_STATE: u32 = 0x8;
const NODE_NEXT: u32 = 0xc;
const TAG_INITIAL: u32 = 0x40e;
const TAG_READY: u32 = 0x414;
const TAG_ACTIVITY: u32 = 0xd3;
const TAG_PENDING: u32 = 0x13f;
const GLOBAL_IDLE: u32 = 0x01160e78;
const GLOBAL_ACTIVITY: u32 = 0x016154d0;
const GLOBAL_BLOCKED: u32 = 0x017f5eb3;

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    core::ptr::read_unaligned(address as *const u8)
}

#[inline(always)]
unsafe fn read_i16(address: u32) -> i16 {
    core::ptr::read_unaligned(address as *const i16)
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    core::ptr::read_unaligned(address as *const u32)
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    core::ptr::write_unaligned(address as *mut u8, value);
}

/// Search a linked state list in order. A rising three-bit state of two or
/// greater ends the search before that node's tag is considered.
#[inline(always)]
unsafe fn list_has_tag(mut node: u32, tag: u32) -> bool {
    if node == 0 {
        return false;
    }

    let mut previous_state = (read_u32(node.wrapping_add(NODE_STATE)) >> 1) & 7;
    loop {
        let current_state = (read_u32(node.wrapping_add(NODE_STATE)) >> 1) & 7;
        if previous_state < current_state && previous_state >= 2 {
            return false;
        }
        if read_u32(node.wrapping_add(NODE_TAG)) == tag {
            return true;
        }
        previous_state = current_state;
        node = read_u32(node.wrapping_add(NODE_NEXT));
        if node == 0 {
            return false;
        }
    }
}

#[inline(always)]
unsafe fn call_object_method_12c(object: u32) -> u32 {
    let table = read_u32(object.wrapping_add(OBJECT_VTABLE));
    let method = read_u32(table.wrapping_add(0x12c));
    let target: extern "thiscall" fn(u32) -> u32 =
        core::mem::transmute(method as usize);
    target(object)
}

#[inline(always)]
unsafe fn call_cdecl_byte_no_args(id: u32) -> u8 {
    let target: extern "cdecl" fn() -> u8 =
        core::mem::transmute(lf_checker_rt::callee_addr(id) as usize);
    target()
}

unsafe fn lobby_state_update(this: u32) -> u8 {
    const CALL_GET_OBJECT: u32 = 1;
    const CALL_LOOKUP_ACTIVE: u32 = 2;
    const CALL_RESOLVE_FIRST: u32 = 3;
    const CALL_RESOLVE_DEFAULT: u32 = 4;
    const CALL_RESOLVE_INDEXED: u32 = 5;
    const CALL_QUERY_9: u32 = 6;
    const CALL_QUERY_1B: u32 = 7;
    const CALL_QUERY_1: u32 = 8;
    const CALL_QUERY_2: u32 = 9;
    const CALL_QUERY_3: u32 = 10;
    const CALL_REFRESH_FLAGS: u32 = 11;
    const CALL_GET_ACTIVITY_OBJECT: u32 = 12;
    const CALL_TEST_ACTIVE: u32 = 13;
    const CALL_QUERY_VISIBLE: u32 = 14;
    const CALL_IS_ACTIVITY_SOURCE: u32 = 15;
    const CALL_QUERY_MODE: u32 = 16;
    const CALL_TEST_ACTIVITY: u32 = 17;
    const CALL_CHECK_OBJECT: u32 = 18;
    const CALL_CHECK_STATE_BYTES: u32 = 19;
    const CALL_CAN_REFRESH: u32 = 20;
    const CALL_PREPARE_ACTION: u32 = 21;
    const CALL_NOTIFY_ACTIVE: u32 = 22;
    const CALL_NOTIFY_PENDING: u32 = 23;
    const CALL_PREPARE_PENDING: u32 = 24;
    const CALL_CAN_NOTIFY: u32 = 25;
    const CALL_COMMIT_ACTION: u32 = 26;
    const CALL_NOTIFY_IDLE: u32 = 27;

    let object = lf_checker_rt::callee_thiscall!(CALL_GET_OBJECT, u32, this);
    let initial_object = read_u32(object.wrapping_add(OBJECT_LINK));
    let lookup = lf_checker_rt::callee_thiscall!(
        CALL_LOOKUP_ACTIVE,
        u32,
        object.wrapping_add(OBJECT_STATE)
    );

    let mut primary_candidate = 0u32;
    let secondary_candidate;
    if lookup != 0 {
        let value = read_u32(lookup.wrapping_add(0x18));
        primary_candidate = lf_checker_rt::callee_cdecl!(CALL_RESOLVE_FIRST, u32, value);
        secondary_candidate = 0;
    } else {
        secondary_candidate = lf_checker_rt::callee_cdecl!(CALL_RESOLVE_DEFAULT, u32, 0);
        let index = read_u32(object.wrapping_add(OBJECT_STATE)).wrapping_add(3);
        let table_index = index.wrapping_mul(3);
        let indexed_value = read_u32(
            object
                .wrapping_add(OBJECT_STATE)
                .wrapping_add(table_index.wrapping_mul(4)),
        );
        let indexed_candidate =
            lf_checker_rt::callee_cdecl!(CALL_RESOLVE_INDEXED, u32, indexed_value);
        let has_initial_tag = read_u32(indexed_candidate.wrapping_add(CANDIDATE_STATE)) == 3
            && list_has_tag(
                read_u32(initial_object.wrapping_add(OBJECT_LIST)),
                TAG_INITIAL,
            );
        if has_initial_tag || (read_u8(object.wrapping_add(0x274)) & 4) != 0 {
            primary_candidate = indexed_candidate;
        }
    }

    let query_9 = lf_checker_rt::callee_thiscall!(CALL_QUERY_9, u32, this, 9, 0);
    let _query_1b = lf_checker_rt::callee_thiscall!(CALL_QUERY_1B, u32, this, 0x1b, 0);
    let query_1 = lf_checker_rt::callee_thiscall!(CALL_QUERY_1, u32, this, 1, 0);
    let query_2 = lf_checker_rt::callee_thiscall!(CALL_QUERY_2, u32, this, 2, 0);
    let query_3 = lf_checker_rt::callee_thiscall!(CALL_QUERY_3, u32, this, 3, 0);

    let mut first_out = 0u32;
    let mut second_out = 0u32;
    let refresh_result = lf_checker_rt::callee_thiscall!(
        CALL_REFRESH_FLAGS,
        u8,
        initial_object,
        &mut first_out as *mut u32 as u32,
        &mut second_out as *mut u32 as u32
    );
    let mut query_matches = query_1 == 9 || query_1 == 12;
    let ready_tag = list_has_tag(
        read_u32(initial_object.wrapping_add(OBJECT_LIST)),
        TAG_READY,
    );
    let mut activity_object =
        lf_checker_rt::callee_cdecl!(CALL_GET_ACTIVITY_OBJECT, u32, 0, 0);
    if (read_u8(this.wrapping_add(THIS_FLAGS)) & 8) != 0 {
        activity_object = 0;
    }

    let candidate_mode = read_u32(primary_candidate.wrapping_add(CANDIDATE_STATE));
    let early_mode = refresh_result != 0 && candidate_mode == 0;
    let mut mode_gate_a = false;
    let mut mode_gate_b = false;
    if early_mode {
        mode_gate_a = true;
    } else {
        match candidate_mode {
            1 | 2 | 5 => {
                mode_gate_a = true;
                mode_gate_b = true;
            }
            3 => {
                let mode_source = read_u32(object.wrapping_add(OBJECT_MODE_SOURCE));
                let matches = if mode_source == 0 {
                    false
                } else {
                    let dynamic_value = call_object_method_12c(object);
                    let expected = read_i16(mode_source.wrapping_add(0x2e)) as i32 as u32;
                    dynamic_value == expected
                };
                if !matches {
                    mode_gate_a = true;
                    mode_gate_b = true;
                }
            }
            _ => {
                mode_gate_a = candidate_mode == 0;
            }
        }
    }

    let active_mode = read_u8(this.wrapping_add(THIS_ACTIVE_MODE)) == 2;
    let idle_global = read_u32(lf_checker_rt::global::<u32>(GLOBAL_IDLE) as u32) == 0;
    let active_object = if (read_u8(object.wrapping_add(OBJECT_FLAGS)) & 4) != 0 {
        lf_checker_rt::callee_thiscall!(CALL_TEST_ACTIVE, u8, object) == 0
    } else {
        false
    };
    let mode_query = lf_checker_rt::callee_thiscall!(CALL_QUERY_VISIBLE, u32, this, 2, 0);
    let flag18 = if mode_query != 0 {
        true
    } else if active_object
        && secondary_candidate != 0
        && read_u8(secondary_candidate.wrapping_add(CANDIDATE_ACTIVE)) != 0
        && idle_global
    {
        true
    } else {
        false
    };

    let activity_source = read_u32(object.wrapping_add(OBJECT_ACTIVITY_SOURCE));
    let source_is_active = if activity_source != 0 {
        lf_checker_rt::callee_cdecl!(CALL_IS_ACTIVITY_SOURCE, u8, activity_source, 1) != 0
    } else {
        false
    };
    let state_query = lf_checker_rt::callee_thiscall!(CALL_QUERY_MODE, u32, this, 3, 0);
    let write_source_flag = state_query != 0;
    let mut state_probe_qualifies = false;
    if object as u8 != 0 {
        let ready = lf_checker_rt::callee_thiscall!(CALL_TEST_ACTIVITY, u8, object) != 0;
        if !ready {
            // This probe loop is only entered when the readiness query fails.
            // The worker contract pins its pointer and flag inputs so that a
            // retrying source probe cannot become an unbounded trial.
            if activity_object != 0 {
                let state_changed = read_u8(activity_object.wrapping_add(0x328d)) != 0;
                let delta_a = read_u8(activity_object.wrapping_add(0x26fe))
                    ^ read_u8(activity_object.wrapping_add(0x26fc));
                let delta_b = read_u8(activity_object.wrapping_add(0x2c0e))
                    ^ read_u8(activity_object.wrapping_add(0x2c0c));
                state_probe_qualifies |= (state_changed && !idle_global)
                    || delta_a > 0x7f
                    || delta_b > 0x7f;
                if !state_probe_qualifies && idle_global && secondary_candidate != 0 {
                    state_probe_qualifies = read_u8(secondary_candidate.wrapping_add(CANDIDATE_ACTIVE)) != 0;
                }
                if !state_probe_qualifies {
                    let delta_c = read_u8(activity_object.wrapping_add(0x26de))
                        ^ read_u8(activity_object.wrapping_add(0x26dc));
                    let delta_d = read_u8(activity_object.wrapping_add(0x28fe))
                        ^ read_u8(activity_object.wrapping_add(0x28fc));
                    state_probe_qualifies = delta_c > 0x7f || delta_d > 0x7f;
                    if !state_probe_qualifies {
                        state_probe_qualifies = lf_checker_rt::callee_thiscall!(
                            CALL_CHECK_STATE_BYTES,
                            u8,
                            activity_object.wrapping_add(0x2c08)
                        ) != 0;
                    }
                }
            }
        }
    }

    let old_activity = read_u8(lf_checker_rt::global::<u8>(GLOBAL_ACTIVITY) as u32);
    let activity_byte = if write_source_flag { old_activity } else { 0 };
    write_u8(lf_checker_rt::global::<u8>(GLOBAL_ACTIVITY) as u32, activity_byte);
    let state_is_six = read_u32(object.wrapping_add(OBJECT_STATE)) == 6;
    let state_bytes = lf_checker_rt::callee_thiscall!(CALL_CHECK_OBJECT, u8, initial_object);
    let probe_active = active_object
        && idle_global
        && secondary_candidate != 0
        && read_u8(secondary_candidate.wrapping_add(CANDIDATE_ACTIVE)) != 0;

    let platform_active = call_cdecl_byte_no_args(CALL_CAN_REFRESH) != 0;
    let blocked = read_u8(lf_checker_rt::global::<u8>(GLOBAL_BLOCKED) as u32) != 0;
    let pending_level = read_u8(object.wrapping_add(OBJECT_STATUS));
    let phase = read_u32(object.wrapping_add(OBJECT_PHASE));
    let mut abort_notification = platform_active && blocked;
    if !abort_notification && state_bytes == 0 && pending_level != 0 {
        abort_notification = phase == 1 || phase == 2;
    }
    if !abort_notification && read_u32(object.wrapping_add(OBJECT_LINK)) == 0 {
        abort_notification = true;
    }
    if !abort_notification && mode_gate_a && read_u32(object.wrapping_add(OBJECT_ACTION)) != 0 {
        abort_notification = true;
    }
    let child = read_u32(object.wrapping_add(OBJECT_CHILD));
    if !abort_notification && mode_gate_b {
        let child_flags = read_u8(child.wrapping_add(0x70 + 0x3d0));
        abort_notification = child_flags & 8 != 0;
    }
    if !abort_notification && early_mode {
        let child_flags = read_u8(child.wrapping_add(0x70 + 0x3d0));
        abort_notification = child_flags & 8 != 0;
    }
    if !abort_notification && (read_u8(this.wrapping_add(THIS_FLAGS)) & 0x10) != 0 {
        abort_notification = true;
    }
    if !abort_notification && activity_byte != (if probe_active { 1 } else { 0 }) {
        abort_notification = true;
    }

    // The original reads this frame word before the later state transition.
    // The contract supplies a zero stack fill, which is the transport used
    // here because a portable Rust export cannot inspect its incoming stack.
    let incoming_frame_word = 0u32;
    let mut local_state = if incoming_frame_word != 0 || secondary_candidate != 0 {
        1u32
    } else {
        0u32
    };
    let mut notify_gate = false;
    if abort_notification && local_state == 0 && ready_tag != (refresh_result != 0) {
        let candidate_bit = (read_u32(primary_candidate.wrapping_add(CANDIDATE_BITS)) >> 3) & 1;
        let object_state_bit = (read_u32(primary_candidate.wrapping_add(CANDIDATE_BITS)) >> 3) & 1;
        local_state = if candidate_bit != 0 && secondary_candidate != 0 && refresh_result == 0 {
            1
        } else {
            object_state_bit
        };
        if local_state == 0 && (read_u8(this.wrapping_add(THIS_FLAGS)) & 0x10) != 0 {
            local_state = 1;
        }
        notify_gate = local_state != 0;
    }

    if abort_notification && local_state == 0 && ready_tag != (refresh_result != 0) {
        // The selected frame value is zero under the defined stack fill.
        query_matches = false;
    }
    let mut notify_status = abort_notification && local_state == 0 && ready_tag == (refresh_result != 0);
    if !abort_notification && local_state != 0 {
        notify_status = true;
    }
    let local_status = if notify_status { 1u32 } else { 0u32 };

    let query_list = read_u32(query_9.wrapping_add(OBJECT_LIST));
    let found_activity = list_has_tag(query_list, TAG_ACTIVITY);
    let found_pending = list_has_tag(query_list, TAG_PENDING);
    let mut activity_flag = found_activity;
    let mut pending_flag = found_pending;
    if !found_activity {
        activity_flag = false;
    }
    if !found_pending {
        pending_flag = false;
    }

    let object_status = read_u8(object.wrapping_add(OBJECT_STATUS)) != 0;
    let object_type = read_u32(object.wrapping_add(OBJECT_TYPE));
    let (final_gate, transition_state) = if object_type == 1 || object_status {
        (false, 1u32)
    } else {
        (query_matches, if query_matches { 1u32 } else { 0u32 })
    };
    let candidate_bits = read_u32(primary_candidate.wrapping_add(CANDIDATE_BITS));
    let mut computed_state = if candidate_bits & 8 != 0
        && secondary_candidate != 0
        && refresh_result == 0
    {
        1u32
    } else {
        transition_state
    };
    let current_state = read_u8(this.wrapping_add(THIS_STATE));
    if refresh_result == 0 && current_state & 1 == 0 && state_is_six && secondary_candidate != 0 {
        computed_state = 1;
    }
    if activity_flag {
        computed_state = 1;
    }

    let next_low_bit = u8::from(state_is_six);
    write_u8(
        this.wrapping_add(THIS_STATE),
        (current_state & !1) | next_low_bit,
    );

    if (read_u8(this.wrapping_add(THIS_FLAGS)) & MODE_FLAGS) != 0 {
        let pending = 0u32;
        if pending != 0 && secondary_candidate == 0 {
            let can_prepare =
                lf_checker_rt::callee_thiscall!(CALL_PREPARE_ACTION, u32, this) != 0;
            if can_prepare {
                lf_checker_rt::callee_thiscall!(
                    CALL_COMMIT_ACTION,
                    u32,
                    this,
                    object,
                    pending,
                    u32::MAX
                );
            }
        }
    } else if flag18 {
        if activity_byte == 0 && query_3 == 0 && !active_mode && computed_state != 0 {
            lf_checker_rt::callee_thiscall!(CALL_NOTIFY_ACTIVE, u32, this, query_9);
        }
    } else if query_3 == 0 && !active_mode {
        if computed_state != 0 {
            lf_checker_rt::callee_thiscall!(CALL_NOTIFY_PENDING, u32, this, query_9);
        } else if notify_gate {
            lf_checker_rt::callee_thiscall!(CALL_NOTIFY_PENDING, u32, this, query_9);
        } else if pending_flag && !activity_flag {
            let can_prepare =
                lf_checker_rt::callee_thiscall!(CALL_PREPARE_PENDING, u32, this) != 0;
            if can_prepare {
                lf_checker_rt::callee_thiscall!(
                    CALL_COMMIT_ACTION,
                    u32,
                    this,
                    query_9,
                    0,
                    u32::MAX
                );
            }
        } else if active_object {
            let can_notify = lf_checker_rt::callee_thiscall!(CALL_CAN_NOTIFY, u32, this) != 0;
            if can_notify {
                lf_checker_rt::callee_thiscall!(
                    CALL_COMMIT_ACTION,
                    u32,
                    this,
                    0,
                    0,
                    secondary_candidate
                );
            }
        }
    }

    if active_mode && secondary_candidate != 0 {
        lf_checker_rt::callee_thiscall!(CALL_NOTIFY_IDLE, u32, this, 0);
    }

    let _ = (
        query_2,
        source_is_active,
        final_gate,
        computed_state,
        local_status,
        state_probe_qualifies,
    );
    1
}

lf_checker_rt::export!(thiscall, rw_00B094C0(this: u32) -> u8 {
    unsafe { lobby_state_update(this) }
});
