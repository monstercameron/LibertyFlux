// original: 0x00953C80 global_state_init
//! Reset global state and invoke the subsystem's initialization actions.
//!
//! A zero low byte in the sole stack flag exits early when the readiness byte
//! is set. Otherwise this routine clears several state groups, initializes
//! three fixed objects, conditionally refreshes a selected object, optionally
//! releases a saved handle, and sends a normalized event flag to a helper. It
//! clears two bounded byte tables, optionally updates heap-backed table slots,
//! resets several object records, stores two scripted 64-bit values, and
//! finishes with four reset helpers and two object actions.

use lf_checker_rt::{global, relocated};

const INIT_MARKER: u32 = 0x011F_6951;
const EARLY_RETURN_GATE: u32 = 0x0103_7868;

const DEVICE_A: u32 = 0x0103_77A8;
const DEVICE_B: u32 = 0x0103_7760;
const DEVICE_C: u32 = 0x0103_77C0;
const DEVICE_E: u32 = 0x0121_42B8;

const RESET_WORD_A: u32 = 0x0120_F290;
const RESET_WORD_B: u32 = 0x0120_F294;
const RESET_BYTE_A: u32 = 0x0120_F298;
const RESET_WORD_C: u32 = 0x0120_F29C;
const RESET_WORD_D: u32 = 0x0120_F2A0;
const RESET_BYTE_B: u32 = 0x0120_F2A4;
const RESET_WORD_E: u32 = 0x0121_42C0;
const RESET_WORD_F: u32 = 0x0121_42C4;
const RESET_BYTE_C: u32 = 0x0121_42C8;
const RESET_COUNTER: u32 = 0x011F_707C;
const RESET_WORD_G: u32 = 0x011F_A008;
const RESET_WORD_H: u32 = 0x011F_A00C;
const RESET_BYTE_D: u32 = 0x011F_A010;
const RESET_WORD_I: u32 = 0x011F_9FFC;
const RESET_WORD_J: u32 = 0x011F_A000;
const RESET_BYTE_E: u32 = 0x011F_A004;
const RESET_WORD_K: u32 = 0x011F_9FF0;
const RESET_WORD_L: u32 = 0x011F_9FF4;
const RESET_BYTE_F: u32 = 0x011F_9FF8;
const RESET_WORD_M: u32 = 0x0103_7810;
const RESET_WORD_N: u32 = 0x0103_7814;
const RESET_BYTE_G: u32 = 0x0103_7818;
const RESET_WORD_O: u32 = 0x0103_781C;
const RESET_WORD_P: u32 = 0x0120_CA30;
const RESET_WORD_Q: u32 = 0x0120_CA34;
const RESET_BYTE_H: u32 = 0x0120_CA38;
const RESET_WORD_R: u32 = 0x0120_CA44;
const RESET_WORD_S: u32 = 0x0120_CA3C;
const RESET_WORD_T: u32 = 0x0120_CA4C;
const RESET_WORD_U: u32 = 0x0120_CA50;
const RESET_WORD_V: u32 = 0x0120_CA54;
const RESET_BYTE_I: u32 = 0x0120_CA58;
const RESET_WORD_W: u32 = 0x0120_CA64;
const RESET_WORD_X: u32 = 0x0120_CA5C;
const RESET_WORD_Y: u32 = 0x0120_CA6C;
const RESET_WORD_Z: u32 = 0x011F_7028;
const RESET_WORD_AA: u32 = 0x011F_702C;
const RESET_WORD_AB: u32 = 0x011F_7030;
const RESET_WORD_AC0: u32 = 0x011F_7024;
const RESET_WORD_AD0: u32 = 0x011F_7078;
const RESET_WORD_AE0: u32 = 0x011F_70C4;
const RESET_BYTE_AA: u32 = 0x011F_703A;
const RESET_BYTE_AB: u32 = 0x011F_6953;
const RESET_BYTE_AC: u32 = 0x011F_7072;
const RESET_BYTE_AD: u32 = 0x011F_7073;
const RESET_WORD_AF0: u32 = 0x011F_70C0;

const CONDITION_GATE: u32 = 0x0103_7759;
const CONDITION_MODE: u32 = 0x011F_7060;
const COMPARED_VALUE: u32 = 0x0120_88B4;
const READ_ONLY_COMPARISON_VALUE: u32 = 0x00F1_C040;
const BUILD_MODE: u32 = 0x0103_7720;
const REFRESH_STATE: u32 = 0x011F_66A0;
const OBJECT_POINTER: u32 = 0x0120_88B8;
const OBJECT_GUARD: u32 = 0x0120_88BC;
const RELEASE_HANDLE: u32 = 0x011F_7068;
const EVENT_STATE: u32 = 0x0116_0EB8;
const RELEASE_STATE: u32 = 0x011F_70FC;

const BYTE_TABLE_A: u32 = 0x011F_6FF0;
const BYTE_TABLE_A_COUNT: u32 = 0x011F_6FFB;
const BYTE_TABLE_B: u32 = 0x011F_7018;
const BYTE_TABLE_B_COUNT: u32 = 0x011F_6FFD;
const POINTER_TABLE_A: u32 = 0x011F_6F7C;
const POINTER_TABLE_B: u32 = 0x011F_7000;
const OPTIONAL_TABLE_GATE: u32 = 0x011F_6FA8;
const OPTIONAL_TABLE_OFFSET: u32 = 0x0120_F290;
const OPTIONAL_TABLE_B_OFFSET: u32 = 0x011F_A010;
const OPTIONAL_TABLE_B_BASE: u32 = 0x011F_A00C;
const OPTIONAL_TABLE_C_BASE: u32 = 0x011F_A008;

const RESET_OBJECT_WORD_A: u32 = 0x0120_0870;
const RESET_OBJECT_WORD_B: u32 = 0x0120_0874;
const RESET_OBJECT_WORD_C: u32 = 0x0120_0878;
const RESET_OBJECT_WORD_D: u32 = 0x0120_0880;
const RESET_OBJECT_WORD_E: u32 = 0x0120_0884;
const RESET_OBJECT_WORD_F: u32 = 0x0120_0888;
const RESET_OBJECT_WORD_G: u32 = 0x0120_0890;
const RESET_OBJECT_WORD_H: u32 = 0x0120_0894;
const RESET_OBJECT_WORD_I: u32 = 0x0120_0898;
const RESET_OBJECT_WORD_J: u32 = 0x0120_08A8;
const RESET_OBJECT_WORD_K: u32 = 0x0120_08A4;
const RESET_OBJECT_WORD_L: u32 = 0x0120_08A0;

const RESET_FLOAT_BITS: u32 = 0x011F_7058;
const RESET_WORD_AC: u32 = 0x0103_7738;
const RESET_WORD_AD: u32 = 0x0103_773C;
const RESET_WORD_AE: u32 = 0x011F_706C;
const RESET_BYTE_J: u32 = 0x011F_7070;
const RESET_BYTE_K: u32 = 0x011F_7066;
const RESET_BYTE_L: u32 = 0x011F_7067;
const RESET_BYTE_M: u32 = 0x011F_7071;
const RESET_BYTE_N: u32 = 0x011F_701E;
const RESET_PAIR_A: u32 = 0x0120_F2A8;
const RESET_PAIR_B: u32 = 0x0120_F2AC;
const RESET_PAIR_C: u32 = 0x0120_F2B0;
const RESET_PAIR_D: u32 = 0x0120_F2B4;
const RESET_WORD_AF: u32 = 0x011F_7074;
const RESET_BYTE_O: u32 = 0x011F_6952;
const RESET_WORD_AG: u32 = 0x011F_70CC;
const RESET_WORD_AH: u32 = 0x011F_70C8;
const RESET_WORD_AI: u32 = 0x011F_70D4;
const RESET_WORD_AJ: u32 = 0x011F_70D0;
const RESET_WORD_AK: u32 = 0x011F_70DC;
const RESET_WORD_AL: u32 = 0x011F_70D8;
const FIXED_TABLE_A_LENGTH: u32 = 10;
const FIXED_TABLE_B_LENGTH: u32 = 5;
const OPTIONAL_TABLE_MARK: u8 = 2;
const SPECIAL_BUILD_MODE: u32 = 0x12;
const IDENTITY_SCALE_BITS: u32 = 0x3F80_0000;

const INIT_DEVICE_A: u32 = 0;
const INIT_DEVICE_B: u32 = 1;
const INIT_DEVICE_C: u32 = 2;
const REFRESH_OBJECT: u32 = 3;
const RELEASE_RESOURCE: u32 = 4;
const UPDATE_EVENT: u32 = 5;
const READ_CLOCK: u32 = 6;
const CLEAR_TIMER_SET: u32 = 7;
const CLEAR_TIMER_META: u32 = 8;
const CLEAR_OBJECT_LIST: u32 = 9;
const CLEAR_MODE_TABLE: u32 = 10;
const FINALIZE_OBJECT: u32 = 11;
const TAIL_FINALIZE_OBJECT: u32 = 12;

unsafe fn read_u8(file_va: u32) -> u8 {
    global::<u8>(file_va).read()
}

unsafe fn read_u32(file_va: u32) -> u32 {
    global::<u32>(file_va).read()
}

unsafe fn write_u8(file_va: u32, value: u8) {
    global::<u8>(file_va).write(value);
}

unsafe fn write_u32(file_va: u32, value: u32) {
    global::<u32>(file_va).write(value);
}

unsafe fn clear_bytes(file_va: u32, count: u32) {
    let start = global::<u8>(file_va);
    for index in 0..count {
        start.add(index as usize).write(0);
    }
}

/// Reset global state and invoke the subsystem's initialization actions.
///
/// A zero low byte in the sole stack flag exits early when the readiness byte
/// is set. Otherwise the routine clears several state groups, initializes
/// three fixed objects, conditionally refreshes a selected object, optionally
/// releases a saved handle, and passes a normalized event-state flag to a
/// helper. It clears two byte tables using bounded byte counts, optionally
/// clears one pointed-to byte in each runtime table, resets several object
/// records, obtains two scripted 64-bit values, and finishes with four reset
/// helpers and two object actions. The proof supplies table counts within the
/// original's valid limits and provides heap-backed values for the runtime
/// table pointers.
unsafe fn initialize_state(stack_flag: u32) -> u32 {
    write_u8(INIT_MARKER, 0);
    if stack_flag as u8 == 0 && read_u8(EARLY_RETURN_GATE) != 0 {
        return 0;
    }

    write_u32(RESET_WORD_A, 0);
    write_u32(RESET_WORD_B, 0);
    write_u8(RESET_BYTE_A, 0);
    write_u32(RESET_WORD_C, 0);
    write_u32(RESET_WORD_D, 0);
    write_u8(RESET_BYTE_B, 0);
    write_u32(RESET_WORD_E, 0);
    write_u32(RESET_WORD_F, 0);
    write_u8(RESET_BYTE_C, 0);
    write_u32(RESET_COUNTER, 0);
    write_u32(RESET_WORD_G, 0);
    write_u32(RESET_WORD_H, 0);
    write_u8(RESET_BYTE_D, 0);
    write_u32(RESET_WORD_I, 0);
    write_u32(RESET_WORD_J, 0);
    write_u8(RESET_BYTE_E, 0);
    write_u32(RESET_WORD_K, 0);
    write_u32(RESET_WORD_L, 0);
    write_u8(RESET_BYTE_F, 0);
    write_u32(RESET_WORD_M, 0);
    write_u32(RESET_WORD_N, 0);
    write_u8(RESET_BYTE_G, 0);
    write_u32(RESET_WORD_O, u32::MAX);
    write_u32(RESET_WORD_P, 0);
    write_u32(RESET_WORD_Q, 0);
    write_u8(RESET_BYTE_H, 0);
    write_u32(RESET_WORD_R, 0);
    write_u32(RESET_WORD_S, 0);
    write_u32(RESET_WORD_T, u32::MAX);
    write_u32(RESET_WORD_U, 0);
    write_u32(RESET_WORD_V, 0);
    write_u8(RESET_BYTE_I, 0);
    write_u32(RESET_WORD_W, 0);
    write_u32(RESET_WORD_X, 0);
    write_u32(RESET_WORD_Y, u32::MAX);
    write_u32(RESET_WORD_Z, 0);
    write_u32(RESET_WORD_AA, 0);
    write_u32(RESET_WORD_AB, 0);
    write_u32(RESET_WORD_AC0, 0);
    write_u32(RESET_WORD_AD0, 0);
    write_u32(RESET_WORD_AE0, 0);

    let _: u32 = lf_checker_rt::callee_thiscall!(
        INIT_DEVICE_A,
        u32,
        relocated(DEVICE_A)
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(
        INIT_DEVICE_B,
        u32,
        relocated(DEVICE_B)
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(
        INIT_DEVICE_C,
        u32,
        relocated(DEVICE_C)
    );

    let condition_is_ready = read_u8(CONDITION_GATE) != 0;
    if !condition_is_ready {
        let mode_is_initialized = read_u32(CONDITION_MODE) == 1;
        let compared_value = read_u32(COMPARED_VALUE);
        let matches_runtime_value = compared_value == read_u32(READ_ONLY_COMPARISON_VALUE);
        let build_mode_is_special = read_u32(BUILD_MODE) == SPECIAL_BUILD_MODE;
        let inspect_refresh_state =
            mode_is_initialized || !matches_runtime_value || build_mode_is_special;
        let refresh_state_is_set = read_u32(REFRESH_STATE) != 0;
        let object_value_is_null = read_u32(OBJECT_POINTER) == 0;
        let object_guard_is_clear = read_u32(OBJECT_GUARD) == 0;
        let should_refresh = (inspect_refresh_state && refresh_state_is_set)
            || object_value_is_null
            || object_guard_is_clear;

        if should_refresh {
            write_u32(REFRESH_STATE, 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                REFRESH_OBJECT,
                u32,
                relocated(OBJECT_POINTER)
            );
        }
    }

    write_u32(CONDITION_MODE, 0);
    write_u32(BUILD_MODE, 1);

    let release_handle = read_u32(RELEASE_HANDLE);
    if release_handle != 0 {
        let _: u32 =
            lf_checker_rt::callee_cdecl!(RELEASE_RESOURCE, u32, release_handle);
    }

    let event_is_set = read_u32(EVENT_STATE) != 0;
    let event_result: u8 = lf_checker_rt::callee_cdecl!(
        UPDATE_EVENT,
        u8,
        u32::from(event_is_set),
        1u32
    );
    write_u32(RELEASE_HANDLE, 0);
    write_u32(RELEASE_STATE, 0);
    if event_result == 0 {
        write_u32(EVENT_STATE, 0);
    }

    clear_bytes(BYTE_TABLE_A, FIXED_TABLE_A_LENGTH);
    clear_bytes(BYTE_TABLE_B, FIXED_TABLE_B_LENGTH);
    let first_table_count = u32::from(read_u8(BYTE_TABLE_A_COUNT));
    clear_bytes(BYTE_TABLE_A, first_table_count);
    let second_table_count = u32::from(read_u8(BYTE_TABLE_B_COUNT));
    clear_bytes(BYTE_TABLE_B, second_table_count);

    let source_pointer_a = read_u32(POINTER_TABLE_A);
    let source_offset_a = read_u32(OPTIONAL_TABLE_OFFSET);
    write_u8(RESET_BYTE_A, 0);
    write_u32(RESET_WORD_B, source_pointer_a);
    write_u8(RESET_BYTE_D, 0);
    write_u32(RESET_WORD_H, read_u32(POINTER_TABLE_B));

    if read_u32(OPTIONAL_TABLE_GATE) != 0 {
        global::<u8>(BYTE_TABLE_A).write(OPTIONAL_TABLE_MARK);
        let destination_a = source_pointer_a.wrapping_add(source_offset_a);
        (destination_a as *mut u8).write(0);

        let second_table_offset = read_u32(OPTIONAL_TABLE_B_OFFSET);
        let second_table_base = read_u32(OPTIONAL_TABLE_B_BASE);
        let second_table_slot = global::<u8>(BYTE_TABLE_B).add(second_table_offset as usize);
        second_table_slot.write(OPTIONAL_TABLE_MARK);

        let destination_b = read_u32(OPTIONAL_TABLE_C_BASE).wrapping_add(second_table_base);
        (destination_b as *mut u8).write(0);
    }

    write_u8(RESET_BYTE_AB, 0);
    write_u8(RESET_BYTE_AC, 0);
    write_u32(RESET_WORD_AF0, 0);
    write_u8(RESET_BYTE_AD, 0);

    write_u32(RESET_OBJECT_WORD_A, IDENTITY_SCALE_BITS);
    write_u32(RESET_OBJECT_WORD_B, 0);
    write_u32(RESET_OBJECT_WORD_C, 0);
    write_u32(RESET_OBJECT_WORD_D, 0);
    write_u32(RESET_OBJECT_WORD_E, IDENTITY_SCALE_BITS);
    write_u32(RESET_OBJECT_WORD_F, 0);
    write_u32(RESET_OBJECT_WORD_G, 0);
    write_u32(RESET_OBJECT_WORD_H, 0);
    write_u32(RESET_OBJECT_WORD_I, IDENTITY_SCALE_BITS);
    write_u32(RESET_OBJECT_WORD_J, 0);
    write_u32(RESET_OBJECT_WORD_K, 0);
    write_u32(RESET_OBJECT_WORD_L, 0);
    write_u8(RESET_BYTE_J, 0);
    write_u8(RESET_BYTE_AA, 0);

    write_u32(RESET_WORD_AC, u32::MAX);
    write_u32(RESET_WORD_AD, u32::MAX);
    write_u32(RESET_WORD_AE, 0);
    write_u8(RESET_BYTE_K, 0);
    write_u8(RESET_BYTE_L, 0);
    write_u8(RESET_BYTE_M, 0);
    write_u8(RESET_BYTE_N, 0);

    let first_clock: u64 = lf_checker_rt::callee_cdecl!(READ_CLOCK, u64,);
    write_u32(RESET_PAIR_A, first_clock as u32);
    write_u32(RESET_PAIR_B, (first_clock >> 32) as u32);
    let second_clock: u64 = lf_checker_rt::callee_cdecl!(READ_CLOCK, u64,);
    write_u32(RESET_PAIR_C, second_clock as u32);
    write_u32(RESET_PAIR_D, (second_clock >> 32) as u32);

    write_u32(RESET_FLOAT_BITS, IDENTITY_SCALE_BITS);
    write_u32(RESET_WORD_AF, 0);
    write_u8(RESET_BYTE_O, 0);
    write_u32(RESET_WORD_AG, 0);
    write_u32(RESET_WORD_AH, 0);
    write_u32(RESET_WORD_AI, 0);
    write_u32(RESET_WORD_AJ, 0);
    write_u32(RESET_WORD_AK, 0);
    write_u32(RESET_WORD_AL, 0);

    let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR_TIMER_SET, u32,);
    let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR_TIMER_META, u32,);
    let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR_OBJECT_LIST, u32,);
    let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR_MODE_TABLE, u32,);
    let _: u32 =
        lf_checker_rt::callee_thiscall!(FINALIZE_OBJECT, u32, relocated(DEVICE_E));
    let _: u32 = lf_checker_rt::callee_thiscall!(
        TAIL_FINALIZE_OBJECT,
        u32,
        relocated(DEVICE_E)
    );

    0
}

lf_checker_rt::export!(cdecl, rw_00953c80(stack_flag: u32) -> u32 {
    unsafe { initialize_state(stack_flag) }
});
