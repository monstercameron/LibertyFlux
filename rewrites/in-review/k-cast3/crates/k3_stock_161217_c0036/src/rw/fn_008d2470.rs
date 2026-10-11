// original: 0x008D2470 PlayerUpdateTick
//! Updates the player's counters and cash, publishes the active position,
//! and evaluates the player's movement state. The linked manager, entity,
//! and position objects are supplied by the caller.

#![allow(unsafe_code)]
#![allow(clippy::pedantic)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![cfg(target_arch = "x86")]

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

const OBJECT_MANAGER: u32 = 2;
const COUNT_ACTIVE_RECORDS: u32 = 3;
const GET_FRAME_COUNT: u32 = 4;
const GET_FRAME_DELTA: u32 = 7;
const FIND_CONTROL_RECORD: u32 = 6;
const REPORT_TIMER: u32 = 5;
const REPORT_COUNTER: u32 = 8;
const UPDATE_PLAYER: u32 = 9;
const REPORT_CASH_DECREASE: u32 = 10;
const SHOW_CASH_COUNTER: u32 = 11;
const STORE_CASH: u32 = 12;
const CASH_POPUP_ACTIVE: u32 = 13;
const FRONTEND_BUSY: u32 = 14;
const OPEN_CASH_POPUP: u32 = 15;
const REPORT_EVENT: u32 = 16;
const GET_MANAGER: u32 = 17;
const GET_ACTIVE_ENTITY: u32 = 18;
const GET_ACTIVE_ENTITY_STATE: u32 = 19;
const REPORT_ENTITY: u32 = 20;
const IS_POSITION_TRACKED: u32 = 21;
const GET_ACTIVE_ACTION: u32 = 22;
const IS_ACTION_ACTIVE: u32 = 23;
const GET_NEXT_ACTION: u32 = 24;
const GET_ACTION_DATA: u32 = 25;
const IS_POSITION_IDLE: u32 = 26;
const PROJECT_POSITION: u32 = 27;
const REFRESH_PLAYER: u32 = 28;
const FINISH_PLAYER_TICK: u32 = 29;

const GLOBAL_PLAYER_INDEX: u32 = 0x0103_6F14;
const GLOBAL_MODE: u32 = 0x0103_7720;
const GLOBAL_CLOCK: u32 = 0x0117_35B4;
const GLOBAL_SCALE: u32 = 0x0117_35BC;
const GLOBAL_TICK: u32 = 0x0117_3604;
const GLOBAL_POSITION_X: u32 = 0x0117_36B0;
const GLOBAL_POSITION_Y: u32 = 0x0117_36B4;
const GLOBAL_POSITION_Z: u32 = 0x0117_36B8;
const GLOBAL_POSITION_W: u32 = 0x0117_36BC;
const GLOBAL_NOTIFICATION_STATE: u32 = 0x0117_36C0;
const GLOBAL_PLAYER_TABLE: u32 = 0x011A_8808;
const GLOBAL_TICK_LOCK: u32 = 0x011F_7060;
const GLOBAL_CUTSCENE_STATE: u32 = 0x0120_88B4;
const GLOBAL_CASH_BASELINE: u32 = 0x012B_6250;
const GLOBAL_TIME_SCALE: u32 = 0x00FE_8C58;
const GLOBAL_SIGN_ADJUST: u32 = 0x00FE_8F50;
const FRONTEND_CASH_OBJECT: u32 = 0x0117_6888;
const FRONTEND_CASH_LABEL: u32 = 0x00E8_0C24;

const LINK_MANAGER: u32 = 0x598;
const LINK_ENTITY: u32 = 0xB30;
const ENTITY_MODE: u32 = 0x1300;
const ENTITY_TIMER: u32 = 0x170;
const ENTITY_OBJECT: u32 = 0x20;
const ENTITY_RECORDS: u32 = 0xF80;
const ENTITY_RECORD_COUNT: u32 = 0xF84;
const RECORD_FLAGS: u32 = 0x164;
const CALL_VECTOR_SLOT: u32 = 0xEC;

#[inline(always)]
unsafe fn word(base: u32, offset: u32) -> u32 {
    ((base.wrapping_add(offset)) as *const u32).read()
}

#[inline(always)]
unsafe fn dword(base: u32, offset: u32, value: u32) {
    ((base.wrapping_add(offset)) as *mut u32).write(value);
}

#[inline(always)]
unsafe fn byte(base: u32, offset: u32) -> u8 {
    ((base.wrapping_add(offset)) as *const u8).read()
}

#[inline(always)]
unsafe fn store_byte(base: u32, offset: u32, value: u8) {
    ((base.wrapping_add(offset)) as *mut u8).write(value);
}

#[inline(always)]
unsafe fn half(base: u32, offset: u32, value: i16) {
    ((base.wrapping_add(offset)) as *mut i16).write(value);
}

#[inline(always)]
unsafe fn float_word(base: u32, offset: u32) -> f32 {
    f32::from_bits(word(base, offset))
}

#[inline(always)]
unsafe fn store_float_word(base: u32, offset: u32, value: f32) {
    dword(base, offset, value.to_bits());
}

#[inline(always)]
unsafe fn global_word(address: u32) -> u32 {
    lf_checker_rt::global::<u32>(address).read()
}

#[inline(always)]
unsafe fn store_global(address: u32, value: u32) {
    lf_checker_rt::global::<u32>(address).write(value);
}

#[inline(always)]
unsafe fn native_member_u32_0(address: u32, this: u32) -> u32 {
    callee_thiscall!(address, u32, this)
}

#[inline(always)]
unsafe fn native_member_void_0(address: u32, this: u32) {
    let _ = callee_thiscall!(address, u32, this);
}

#[inline(always)]
unsafe fn native_member_void_1(address: u32, this: u32, arg: u32) {
    let _ = callee_thiscall!(address, u32, this, arg);
}

#[inline(always)]
unsafe fn native_member_u32_1(address: u32, this: u32, arg: u32) -> u32 {
    callee_thiscall!(address, u32, this, arg)
}

#[inline(always)]
unsafe fn native_member_f32_0(address: u32, this: u32) -> f32 {
    callee_thiscall!(address, f32, this)
}

#[inline(always)]
unsafe fn native_cdecl_u32_0(address: u32) -> u32 {
    callee_cdecl!(address, u32,)
}

#[inline(always)]
unsafe fn native_cdecl_u32_1(address: u32, arg: u32) -> u32 {
    callee_cdecl!(address, u32, arg)
}

#[inline(always)]
unsafe fn native_cdecl_u32_2(address: u32, first: u32, second: u32) -> u32 {
    callee_cdecl!(address, u32, first, second)
}

#[inline(always)]
unsafe fn native_cdecl_bool_0(address: u32) -> u8 {
    callee_cdecl!(address, u8,)
}

#[inline(always)]
unsafe fn native_cdecl_bool_1(address: u32, arg: u32) -> u8 {
    callee_cdecl!(address, u8, arg)
}

#[inline(always)]
unsafe fn native_cdecl_void_0(address: u32) {
    let _ = callee_cdecl!(address, u32,);
}

#[inline(always)]
unsafe fn native_cdecl_void_1(address: u32, arg: u32) {
    let _ = callee_cdecl!(address, u32, arg);
}

#[inline(always)]
unsafe fn native_cdecl_void_2_f32(address: u32, kind: u32, value: f32) {
    let _ = callee_cdecl!(address, u32, kind, value.to_bits());
}

#[inline(always)]
unsafe fn native_cdecl_f32_6(
    address: u32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> f32 {
    callee_cdecl!(address, f32, first, second, third, fourth, fifth, sixth)
}

#[inline(always)]
fn fadd(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn fmul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
unsafe fn record_flag(entity: u32, selector: u32) -> bool {
    let record = native_member_u32_1(FIND_CONTROL_RECORD, entity, selector);
    record != 0 && (byte(record, RECORD_FLAGS) & 1) != 0
}

#[inline(always)]
unsafe fn record_missing_or_active(entity: u32, selector: u32) -> bool {
    let found = native_member_u32_1(FIND_CONTROL_RECORD, entity, selector);
    if found == 0 {
        true
    } else {
        // The original performs a separate lookup for the flag after its
        // null check, so both helper calls are observable in the contract.
        let record = native_member_u32_1(FIND_CONTROL_RECORD, entity, selector);
        (byte(record, RECORD_FLAGS) & 1) != 0
    }
}

#[inline(always)]
unsafe fn vector_query(subject: u32) -> u32 {
    let vtable = word(subject, 0);
    let target = word(vtable, CALL_VECTOR_SLOT);
    let function: extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(target as usize);
    let mut scratch = [0_u32; 7];
    function(subject, scratch.as_mut_ptr() as u32)
}

#[inline(always)]
unsafe fn vector_length_squared(vector: u32) -> f32 {
    let x = float_word(vector, 0);
    let y = float_word(vector, 4);
    let z = float_word(vector, 8);
    let x_squared = fmul(x, x);
    let y_squared = fmul(y, y);
    let xy_squared = fadd(x_squared, y_squared);
    let z_squared = fmul(z, z);
    fadd(xy_squared, z_squared)
}

#[inline(always)]
unsafe fn vector_length(vector: u32) -> f32 {
    vector_length_squared(vector).sqrt()
}

#[inline(always)]
unsafe fn event_count_float(count: u32) -> f32 {
    let signed_count = count as i32;
    let table_index = (-(signed_count >> 31)) as usize;
    let adjustment = ((relocated(GLOBAL_SIGN_ADJUST) as *const f64).add(table_index)).read();
    let sum = core::hint::black_box(signed_count as f64) + core::hint::black_box(adjustment);
    sum as f32
}

#[inline(always)]
unsafe fn log_counter_pair(event: u32, count: u32, value_offset: u32, this: u32) {
    native_cdecl_void_2_f32(REPORT_COUNTER, event, event_count_float(count));
    native_cdecl_void_2_f32(REPORT_COUNTER, event.wrapping_sub(0x1F), float_word(this, value_offset));
}

unsafe fn reset_control_counters(this: u32) {
    dword(this, 0x508, 0);
    dword(this, 0x510, 0);
}

unsafe fn reset_idle_counters(this: u32) {
    // The idle-mode paths clear the accumulated control count but preserve
    // the independent active-record timer at +0x510.
    dword(this, 0x508, 0);
}

unsafe fn update_idle_counter(this: u32, entity: u32) {
    let active_for_9_and_13 =
        record_missing_or_active(entity, 9) && record_missing_or_active(entity, 13);
    let active_for_10_and_14 =
        record_missing_or_active(entity, 10) && record_missing_or_active(entity, 14);

    if active_for_9_and_13 && active_for_10_and_14 {
        let accumulated = word(this, 0x508);
        if accumulated != 0 {
            if accumulated > 999 {
                dword(this, 0x528, accumulated);
                dword(this, 0x52C, word(this, 0x50C));
                native_cdecl_void_2_f32(REPORT_COUNTER, 0x71, event_count_float(accumulated));
                native_cdecl_void_2_f32(REPORT_COUNTER, 0x52, float_word(this, 0x50C));
            }
            dword(this, 0x50C, 0);
            reset_idle_counters(this);
        }
        dword(this, 0x514, 0);
        dword(this, 0x51C, 0);
        return;
    }

    let entity_timer = float_word(entity, ENTITY_TIMER);
    let one_side_active = active_for_9_and_13 ^ active_for_10_and_14;
    if one_side_active && entity_timer == 0.0 {
        let elapsed = native_cdecl_u32_0(GET_FRAME_COUNT);
        dword(this, 0x508, word(this, 0x508).wrapping_add(elapsed));
        let elapsed_seconds = native_member_f32_0(GET_FRAME_DELTA, entity);
        store_float_word(this, 0x50C, fadd(elapsed_seconds, float_word(this, 0x50C)));
        let elapsed_again = native_cdecl_u32_0(GET_FRAME_COUNT) >> 1;
        let hold = word(this, 0x524);
        if elapsed_again < hold {
            dword(this, 0x524, hold.wrapping_sub(elapsed_again));
        } else {
            dword(this, 0x524, 0);
        }
        dword(this, 0x514, 0);
        dword(this, 0x51C, 0);
        return;
    }

    let count = word(this, 0x508);
    let hold = word(this, 0x524);
    if count != 0 && hold <= 499 {
        dword(
            this,
            0x524,
            hold.wrapping_add(native_cdecl_u32_0(GET_FRAME_COUNT)),
        );
        dword(this, 0x514, 0);
        dword(this, 0x51C, 0);
        return;
    }

    if count > 999 {
        dword(this, 0x528, count);
        dword(this, 0x52C, word(this, 0x50C));
        native_cdecl_void_2_f32(REPORT_COUNTER, 0x71, event_count_float(count));
        native_cdecl_void_2_f32(REPORT_COUNTER, 0x52, float_word(this, 0x50C));
    }
    dword(this, 0x50C, 0);
    reset_idle_counters(this);
    dword(this, 0x524, 0);
    dword(this, 0x514, 0);
    dword(this, 0x51C, 0);
}

unsafe fn update_active_counter(this: u32, entity: u32) {
    let timer = float_word(entity, ENTITY_TIMER);
    let state_9_active = record_flag(entity, 9);
    let vector_magnitude_sq = || unsafe { vector_length_squared(vector_query(entity)) };

    if !state_9_active && word(this, 0x51C) == 0 {
        let state_13_active = record_flag(entity, 13);
        if !state_13_active || (timer >= 5.0 && vector_magnitude_sq() <= 0.1) {
            let accumulated = word(this, 0x514);
            let hold = word(this, 0x524);
            if accumulated == 0 || hold > 499 {
                if accumulated > 2999 {
                    dword(this, 0x530, accumulated);
                    dword(this, 0x534, word(this, 0x518));
                    native_cdecl_void_2_f32(REPORT_COUNTER, 0x70, event_count_float(accumulated));
                    native_cdecl_void_2_f32(REPORT_COUNTER, 0x54, float_word(this, 0x518));
                }
                dword(this, 0x514, 0);
                dword(this, 0x518, 0);
            } else {
                let elapsed = native_cdecl_u32_0(GET_FRAME_COUNT);
                dword(this, 0x524, hold.wrapping_add(elapsed));
            }
            dword(this, 0x524, 0);
            reset_control_counters(this);
            return;
        }

        let elapsed = native_cdecl_u32_0(GET_FRAME_COUNT);
        dword(this, 0x514, word(this, 0x514).wrapping_add(elapsed));
        let seconds = native_member_f32_0(GET_FRAME_DELTA, entity);
        store_float_word(this, 0x518, fadd(seconds, float_word(this, 0x518)));
        let hold_step = native_cdecl_u32_0(GET_FRAME_COUNT) / 5;
        let hold = word(this, 0x524);
        if hold <= hold_step {
            dword(this, 0x524, 0);
        } else {
            dword(this, 0x524, hold - hold_step);
        }
        reset_control_counters(this);
        return;
    }

    let accumulated = word(this, 0x514);
    if accumulated == 0 {
        let state_13_active = record_flag(entity, 13);
        let state_9_active_again = !state_13_active && record_flag(entity, 9);
        if !state_13_active
            && state_9_active_again
            && (timer < 5.0 || vector_magnitude_sq() > 0.1)
        {
            let elapsed = native_cdecl_u32_0(GET_FRAME_COUNT);
            dword(this, 0x51C, word(this, 0x51C).wrapping_add(elapsed));
            let seconds = native_member_f32_0(GET_FRAME_DELTA, entity);
            store_float_word(this, 0x520, fadd(seconds, float_word(this, 0x520)));
            let hold_step = native_cdecl_u32_0(GET_FRAME_COUNT) / 5;
            let hold = word(this, 0x524);
            if hold <= hold_step {
                dword(this, 0x524, 0);
            } else {
                dword(this, 0x524, hold - hold_step);
            }
            reset_control_counters(this);
            return;
        }

        let state_time = word(this, 0x51C);
        let hold = word(this, 0x524);
        if state_time != 0 && hold < 500 {
            dword(
                this,
                0x524,
                hold.wrapping_add(native_cdecl_u32_0(GET_FRAME_COUNT)),
            );
            reset_control_counters(this);
            return;
        }

        if state_time > 999 {
            dword(this, 0x538, state_time);
            dword(this, 0x53C, word(this, 0x520));
            native_cdecl_void_2_f32(REPORT_COUNTER, 0x6F, event_count_float(state_time));
            native_cdecl_void_2_f32(REPORT_COUNTER, 0x53, float_word(this, 0x520));
        }
        dword(this, 0x520, 0);
        dword(this, 0x51C, 0);
    } else {
        if accumulated > 2999 {
            dword(this, 0x530, accumulated);
            dword(this, 0x534, word(this, 0x518));
            native_cdecl_void_2_f32(REPORT_COUNTER, 0x70, event_count_float(accumulated));
            native_cdecl_void_2_f32(REPORT_COUNTER, 0x54, float_word(this, 0x518));
        }
        dword(this, 0x514, 0);
        dword(this, 0x518, 0);
    }
    dword(this, 0x524, 0);
    reset_control_counters(this);
}

unsafe fn update_cash(this: u32) {
    let current = word(this, 0x5C8);
    let target = word(this, 0x5C4);
    if current == target {
        return;
    }

    let delta = target.wrapping_sub(current) as i32;
    let magnitude = delta.wrapping_abs() as u32;
    let step = if magnitude < 0x186A1 {
        if magnitude < 0x2711 {
            if magnitude < 0x3E9 {
                if magnitude > 0x32 { 0x2A } else { 1 }
            } else {
                0x7B
            }
        } else {
            0x4D2
        }
    } else {
        0x3039
    };

    let next = if delta < 0 {
        native_cdecl_void_2_f32(REPORT_CASH_DECREASE, 0x6C, step as f32);
        current.wrapping_sub(step)
    } else {
        native_cdecl_void_2_f32(REPORT_TIMER, 0x6C, step as f32);
        let next = current.wrapping_add(step);
        let global_tick = global_word(GLOBAL_TICK) as i32;
        if global_tick % 3 == 0 {
            native_member_void_1(
                SHOW_CASH_COUNTER,
                relocated(FRONTEND_CASH_OBJECT),
                relocated(FRONTEND_CASH_LABEL),
            );
        }
        next
    };
    native_member_void_1(STORE_CASH, this, next);

    let baseline = global_word(GLOBAL_CASH_BASELINE) as i32;
    if (next as i32).wrapping_sub(baseline) > 499_999
        && native_cdecl_bool_1(CASH_POPUP_ACTIVE, 0x19) == 0
        && native_cdecl_bool_0(FRONTEND_BUSY) == 0
    {
        native_cdecl_void_1(OPEN_CASH_POPUP, 0x19);
    }
}

unsafe fn update_player_position(this: u32, manager: u32, entity: u32) {
    let last_tick = word(this, 0x4F0);
    if byte(this, 0x4EC) != 0 && global_word(GLOBAL_CLOCK).wrapping_sub(last_tick) > 2000 {
        store_byte(this, 0x4EC, 0);
        let active = native_cdecl_u32_1(GET_ACTIVE_ENTITY, 0);
        if active != 0 {
            let bits = word(active, 0x28);
            if bits & 0x7C00 != 0x0C00 {
                dword(active, 0x28, bits & 0xFFFF_83FF);
            }
        }
    }

    let active = entity;
    let tick = global_word(GLOBAL_TICK);
    if tick & 0x1F == 0 {
        if active != 0
            && byte(manager, 0x26C) & 4 != 0
            && float_word(word(active, ENTITY_OBJECT), 0x28) <= 0.0
            && float_word(word(active, ENTITY_OBJECT), 0x28) != 0.0
        {
            let position = vector_query(active);
            let z = float_word(word(active, ENTITY_OBJECT), 0x28);
            if vector_length(position) < 0.05
                && (word(active, ENTITY_MODE) == 0
                    || word(native_cdecl_u32_2(GET_ACTIVE_ENTITY_STATE, 0, 0), ENTITY_MODE) == 2)
                && byte(active, 0x118) & 1 == 0
            {
                let increment = if z > -0.5 || z == -0.5 { 1 } else { 2 };
                dword(this, 0x500, word(this, 0x500).wrapping_add(increment));
            } else {
                dword(this, 0x500, 0);
            }
        } else {
            dword(this, 0x500, 0);
        }
    }

    if active == 0 {
        let position = vector_query(manager);
        if vector_length_squared(position) > 0.0 {
            let list_context = word(manager, 0x224);
            if native_member_u32_0(IS_POSITION_TRACKED, list_context) == 0 {
                if native_member_u32_0(IS_POSITION_IDLE, list_context) != 0 {
                    let points = word(manager, 0x20);
                    let height = float_word(points, 0x38);
                    let projected = native_cdecl_f32_6(
                        PROJECT_POSITION,
                        word(points, 0x30),
                        word(points, 0x34),
                        word(points, 0x38),
                        0,
                        0,
                        4,
                    );
                    let drop = height - projected;
                    if drop > 2.0 {
                        native_cdecl_void_2_f32(REPORT_COUNTER, 0x59, drop);
                    }
                } else {
                    let speed = native_member_f32_0(GET_FRAME_DELTA, manager);
                    if speed > 0.005 {
                        let speed_again = native_member_f32_0(GET_FRAME_DELTA, manager);
                        native_cdecl_void_2_f32(REPORT_TIMER, 0x50, speed_again);
                        native_cdecl_void_2_f32(
                            REPORT_TIMER,
                            0x46,
                            fmul(
                                f32::from_bits(global_word(GLOBAL_SCALE)),
                                f32::from_bits(global_word(GLOBAL_TIME_SCALE)),
                            ),
                        );
                    }
                }
            } else {
                let active_action = native_member_u32_0(GET_ACTIVE_ACTION, active);
                let action_active = native_cdecl_u32_1(IS_ACTION_ACTIVE, active_action);
                if action_active == 0 {
                    let next_action = native_member_u32_0(GET_NEXT_ACTION, active);
                    let next_active = native_cdecl_u32_1(IS_ACTION_ACTIVE, next_action);
                    if next_active == 0 {
                        let action_data = native_member_u32_0(GET_ACTION_DATA, active);
                        if (byte(action_data, 6) ^ byte(action_data, 4)) < 0x80 {
                            return;
                        }
                    }
                }
                let speed = native_member_f32_0(GET_FRAME_DELTA, manager);
                native_cdecl_void_2_f32(REPORT_TIMER, 0x51, speed);
                native_cdecl_void_2_f32(
                    REPORT_TIMER,
                    0x4B,
                    fmul(
                        f32::from_bits(global_word(GLOBAL_SCALE)),
                        f32::from_bits(global_word(GLOBAL_TIME_SCALE)),
                    ),
                );
            }
        }
    } else {
        native_cdecl_void_1(REPORT_ENTITY, active);
    }
    let _ = entity;
}

unsafe fn update(this: u32, mutant: bool) {
    let manager_link = word(this, LINK_MANAGER);
    if manager_link != 0 {
        let player_index = global_word(GLOBAL_PLAYER_INDEX) as i32;
        let tracked_player = if player_index == -1 {
            0
        } else {
            global_word(GLOBAL_PLAYER_TABLE.wrapping_add((player_index as u32).wrapping_mul(4)))
        };
        if tracked_player == this {
            let position = word(manager_link, 0x20);
            store_global(GLOBAL_POSITION_X, word(position, 0x30));
            store_global(GLOBAL_POSITION_Y, word(position, 0x34));
            store_global(GLOBAL_POSITION_Z, word(position, 0x38));
            store_global(GLOBAL_POSITION_W, word(position, 0x3C));
        }
    }

    if global_word(GLOBAL_TICK_LOCK) == 1
        || global_word(GLOBAL_CUTSCENE_STATE) != u32::MAX
        || global_word(GLOBAL_MODE) == 0x12
    {
        return;
    }

    if manager_link != 0 {
        let _ = native_member_u32_0(OBJECT_MANAGER, manager_link);
    }
    let manager = word(this, LINK_MANAGER);
    dword(this, 0x4DC, global_word(GLOBAL_CLOCK));

    let list_context = word(manager, 0x224);
    let mut finalize_now = false;
    if list_context != 0 {
        let mut node = word(list_context, 0x2E0);
        if node != 0 {
            let mut previous_state = (word(node, 8) >> 1) & 7;
            loop {
                let current_state = (word(node, 8) >> 1) & 7;
                if previous_state < current_state && previous_state > 1 {
                    break;
                }
                if word(node, 4) == 0x41E {
                    let scaled_time = fmul(
                        f32::from_bits(global_word(GLOBAL_SCALE)),
                        f32::from_bits(global_word(GLOBAL_TIME_SCALE)),
                    );
                    native_cdecl_void_2_f32(REPORT_TIMER, 0x33, scaled_time);
                    finalize_now = true;
                    break;
                }
                node = word(node, 0xC);
                previous_state = current_state;
                if node == 0 {
                    break;
                }
            }
        }
    }

    let entity = word(manager, LINK_ENTITY);
    if !finalize_now {
        if byte(manager, 0x26C) & 4 == 0 || entity == 0 {
            dword(this, 0x510, 0);
            dword(this, 0x508, 0);
            dword(this, 0x524, 0);
            dword(this, 0x514, 0);
            dword(this, 0x51C, 0);
        } else {
            match word(entity, ENTITY_MODE) {
                0 => {
                    if native_member_u32_0(COUNT_ACTIVE_RECORDS, entity) < 3 {
                        let elapsed = native_cdecl_u32_0(GET_FRAME_COUNT);
                        dword(this, 0x510, word(this, 0x510).wrapping_add(elapsed));
                    } else {
                        dword(this, 0x510, 0);
                    }
                    update_idle_counter(this, entity);
                }
                1 => update_active_counter(this, entity),
                _ => {
                    dword(this, 0x510, 0);
                    dword(this, 0x508, 0);
                    dword(this, 0x524, 0);
                    dword(this, 0x514, 0);
                    dword(this, 0x51C, 0);
                }
            }
        }
    }

    native_member_void_0(UPDATE_PLAYER, this);
    update_cash(this);

    let cap = if mutant { 199.0_f32 } else { 200.0_f32 };
    store_float_word(manager, 0xA84, cap);
    half(this, 0x556, cap as i16);
    if byte(manager, 0x26C) & 4 != 0 {
        native_cdecl_void_1(REPORT_EVENT, 0x1A6);
    }

    let active_manager = native_cdecl_u32_0(GET_MANAGER);
    let active_entity = native_cdecl_u32_1(GET_ACTIVE_ENTITY, 0);
    if byte(this, 0x4EC) != 0
        && global_word(GLOBAL_CLOCK).wrapping_sub(word(this, 0x4F0)) > 2000
        && active_entity != 0
    {
        store_byte(this, 0x4EC, 0);
        let flags = word(active_entity, 0x28);
        if flags & 0x7C00 != 0x0C00 {
            dword(active_entity, 0x28, flags & 0xFFFF_83FF);
        }
    }

    update_player_position(this, active_manager, active_entity);

    native_member_void_0(REFRESH_PLAYER, this);
    native_member_void_0(FINISH_PLAYER_TICK, this);
}

/// Player update. This is a void routine; its caller observes memory and
/// outgoing helper effects rather than an EAX return value.
export!(thiscall, rw_008d2470(this: u32) -> () {
    unsafe { update(this, false); }
});

/// Deliberately wrong sibling: changes the fixed movement cap by one unit.
export!(thiscall, mut_008d2470(this: u32) -> () {
    unsafe { update(this, true); }
});
