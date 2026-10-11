// original: 0x00953c80 global_state_init
//! Reset startup state and dispatch the initialization work selected by the
//! incoming byte and the current global settings.
//!
//! The zero-byte path returns early when the re-entry gate is set. Otherwise
//! the routine clears descriptor state, calls the selected setup helpers,
//! clears bounded byte tables, restores identity-like defaults, captures two
//! scripted 64-bit values, and finishes with two object helpers. Runtime
//! pointers read from global state are used only by the corresponding reset
//! branch.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const ENTRY_STATUS_VA: u32 = 0x011F_6951;
const REENTRY_GATE_VA: u32 = 0x0103_7868;
const INIT_CONTEXT_A_VA: u32 = 0x0103_77A8;
const INIT_CONTEXT_B_VA: u32 = 0x0103_7760;
const INIT_CONTEXT_C_VA: u32 = 0x0103_77C0;
const STARTUP_MODE_VA: u32 = 0x0103_7759;
const SETUP_STATE_VA: u32 = 0x011F_7060;
const STATE_VERSION_VA: u32 = 0x0103_7720;
const RESOURCE_CONTEXT_VA: u32 = 0x0120_88B8;
const RESOURCE_CONTEXT_VALUE_VA: u32 = 0x0120_88B4;
const RESOURCE_CONTEXT_MATCH_VA: u32 = 0x00F1_C040;
const RESOURCE_PENDING_VA: u32 = 0x011F_66A0;
const RESOURCE_FLAGS_VA: u32 = 0x0120_88BC;
const OPTIONAL_HANDLE_VA: u32 = 0x011F_7068;
const REQUIRED_FLAG_VA: u32 = 0x0116_0EB8;
const FIRST_BYTE_TABLE_VA: u32 = 0x011F_6FF0;
const FIRST_BYTE_COUNT_VA: u32 = 0x011F_6FFB;
const SECOND_BYTE_TABLE_VA: u32 = 0x011F_7018;
const SECOND_BYTE_COUNT_VA: u32 = 0x011F_6FFD;
const BYTE_TABLE_RESET_VA: u32 = 0x011F_6FA8;
const BYTE_TABLE_SOURCE_A_VA: u32 = 0x011F_6F7C;
const BYTE_TABLE_SOURCE_B_VA: u32 = 0x011F_7000;
const OUTPUT_OBJECT_VA: u32 = 0x0121_42B8;
const IDENTITY_BITS: u32 = 0x3F80_0000;

#[inline(always)]
unsafe fn read_u8_at(address: u32) -> u8 {
    // SAFETY: callers pass a mapped global address from this routine's data set.
    unsafe { core::ptr::read_volatile(global::<u8>(address)) }
}

#[inline(always)]
unsafe fn read_u32_at(address: u32) -> u32 {
    // SAFETY: callers pass a mapped global address from this routine's data set.
    unsafe { core::ptr::read_volatile(global::<u32>(address)) }
}

#[inline(always)]
unsafe fn write_u8_at(address: u32, value: u8) {
    // SAFETY: callers pass a mapped writable global address from this routine's data set.
    unsafe { core::ptr::write_volatile(global::<u8>(address), value) }
}

#[inline(always)]
unsafe fn write_u16_at(address: u32, value: u16) {
    // SAFETY: callers pass a mapped writable global address from this routine's data set.
    unsafe { core::ptr::write_volatile(global::<u16>(address), value) }
}

#[inline(always)]
unsafe fn write_u32_at(address: u32, value: u32) {
    // SAFETY: callers pass a mapped writable global address from this routine's data set.
    unsafe { core::ptr::write_volatile(global::<u32>(address), value) }
}

#[inline(always)]
unsafe fn clear_global_bytes(address: u32, count: usize) {
    let start = global::<u8>(address);
    for offset in 0..count {
        // SAFETY: the bounded offsets stay inside the declared byte table.
        unsafe { core::ptr::write_volatile(start.add(offset), 0) }
    }
}

#[inline(always)]
unsafe fn clear_pointer_byte(address: u32) {
    // SAFETY: the caller's startup state supplies a valid writable pointer.
    unsafe { core::ptr::write_volatile(address as *mut u8, 0) }
}

fn run_global_state_init(startup_flag: u8) {
    unsafe {
        write_u8_at(ENTRY_STATUS_VA, 0);
        if startup_flag == 0 && read_u8_at(REENTRY_GATE_VA) != 0 {
            return;
        }

        write_u32_at(0x0120_F290, 0);
        write_u32_at(0x0120_F294, 0);
        write_u8_at(0x0120_F298, 0);
        write_u32_at(0x0120_F29C, 0);
        write_u32_at(0x0120_F2A0, 0);
        write_u8_at(0x0120_F2A4, 0);
        write_u32_at(0x0121_42C0, 0);
        write_u32_at(0x0121_42C4, 0);
        write_u8_at(0x0121_42C8, 0);
        write_u32_at(0x011F_707C, 0);
        write_u32_at(0x011F_A008, 0);
        write_u32_at(0x011F_A00C, 0);
        write_u8_at(0x011F_A010, 0);
        write_u32_at(0x011F_9FFC, 0);
        write_u32_at(0x011F_A000, 0);
        write_u8_at(0x011F_A004, 0);
        write_u32_at(0x011F_9FF0, 0);
        write_u32_at(0x011F_9FF4, 0);
        write_u8_at(0x011F_9FF8, 0);
        write_u32_at(0x0103_7810, 0);
        write_u32_at(0x0103_7814, 0);
        write_u8_at(0x0103_7818, 0);
        write_u32_at(0x0103_781C, u32::MAX);
        write_u32_at(0x0120_CA30, 0);
        write_u32_at(0x0120_CA34, 0);
        write_u8_at(0x0120_CA38, 0);
        write_u32_at(0x0120_CA44, 0);
        write_u32_at(0x0120_CA3C, 0);
        write_u32_at(0x0120_CA4C, u32::MAX);
        write_u32_at(0x0120_CA50, 0);
        write_u32_at(0x0120_CA54, 0);
        write_u8_at(0x0120_CA58, 0);
        write_u32_at(0x0120_CA64, 0);
        write_u32_at(0x0120_CA5C, 0);
        write_u32_at(0x0120_CA6C, u32::MAX);
        write_u32_at(0x011F_7028, 0);
        write_u32_at(0x011F_702C, 0);
        write_u32_at(0x011F_7030, 0);

        let _: u32 = callee_thiscall!(1, u32, relocated(INIT_CONTEXT_A_VA));
        let _: u32 = callee_thiscall!(2, u32, relocated(INIT_CONTEXT_B_VA));
        let _: u32 = callee_thiscall!(3, u32, relocated(INIT_CONTEXT_C_VA));

        write_u32_at(0x011F_7024, 0);
        write_u32_at(0x011F_7078, 0);
        write_u32_at(0x011F_70C4, 0);

        if read_u8_at(STARTUP_MODE_VA) == 0 {
            let reaches_pending_test = read_u32_at(SETUP_STATE_VA) == 1
                || read_u32_at(RESOURCE_CONTEXT_VALUE_VA)
                    != read_u32_at(RESOURCE_CONTEXT_MATCH_VA)
                || read_u32_at(STATE_VERSION_VA) == 0x12;
            let release_resource = (reaches_pending_test
                && read_u32_at(RESOURCE_PENDING_VA) != 0)
                || read_u32_at(RESOURCE_CONTEXT_VA) == 0
                || read_u32_at(RESOURCE_FLAGS_VA) == 0;
            if release_resource {
                write_u32_at(RESOURCE_PENDING_VA, 0);
                let _: u32 = callee_thiscall!(4, u32, relocated(RESOURCE_CONTEXT_VA));
            }
        }

        let optional_handle = read_u32_at(OPTIONAL_HANDLE_VA);
        write_u32_at(SETUP_STATE_VA, 0);
        write_u32_at(STATE_VERSION_VA, 1);
        if optional_handle != 0 {
            let _: u32 = callee_cdecl!(5, u32, optional_handle);
        }

        let required_flag = u32::from(read_u32_at(REQUIRED_FLAG_VA) != 0);
        write_u32_at(OPTIONAL_HANDLE_VA, 0);
        write_u32_at(0x011F_70FC, 0);
        let setup_result: u8 = callee_cdecl!(6, u8, required_flag, 1);
        if setup_result == 0 {
            write_u32_at(REQUIRED_FLAG_VA, 0);
        }

        clear_global_bytes(FIRST_BYTE_TABLE_VA, 10);
        clear_global_bytes(SECOND_BYTE_TABLE_VA, 5);

        let first_count = usize::from(read_u8_at(FIRST_BYTE_COUNT_VA));
        clear_global_bytes(FIRST_BYTE_TABLE_VA, first_count);
        let second_count = usize::from(read_u8_at(SECOND_BYTE_COUNT_VA));
        clear_global_bytes(SECOND_BYTE_TABLE_VA, second_count);

        let byte_table_source_a = read_u32_at(BYTE_TABLE_SOURCE_A_VA);
        let byte_table_source_b = read_u32_at(BYTE_TABLE_SOURCE_B_VA);
        write_u8_at(0x0120_F298, 0);
        write_u32_at(0x0120_F294, byte_table_source_a);
        write_u8_at(0x011F_A010, 0);
        write_u32_at(0x011F_A00C, byte_table_source_b);
        if read_u32_at(BYTE_TABLE_RESET_VA) != 0 {
            let first_table_index = read_u32_at(0x0120_F290);
            write_u8_at(FIRST_BYTE_TABLE_VA, 2);
            clear_pointer_byte(byte_table_source_a.wrapping_add(first_table_index));
            let second_table_index = u32::from(read_u8_at(0x011F_A010));
            write_u8_at(SECOND_BYTE_TABLE_VA.wrapping_add(second_table_index), 2);
            let second_table_base = read_u32_at(0x011F_A008);
            clear_pointer_byte(second_table_base.wrapping_add(byte_table_source_b));
        }

        write_u8_at(0x011F_6953, 0);
        write_u8_at(0x011F_7072, 0);
        write_u32_at(0x011F_70C0, 0);
        write_u8_at(0x011F_7073, 0);
        write_u32_at(0x0120_0870, IDENTITY_BITS);
        write_u32_at(0x0120_0874, 0);
        write_u32_at(0x0120_0878, 0);
        write_u32_at(0x0120_0880, 0);
        write_u32_at(0x0120_0884, IDENTITY_BITS);
        write_u32_at(0x0120_0888, 0);
        write_u32_at(0x0120_0890, 0);
        write_u32_at(0x0120_0894, 0);
        write_u32_at(0x0120_0898, IDENTITY_BITS);
        write_u32_at(0x0120_08A8, 0);
        write_u32_at(0x0120_08A4, 0);
        write_u32_at(0x0120_08A0, 0);
        write_u8_at(0x011F_703A, 0);
        write_u32_at(0x0103_7738, u32::MAX);
        write_u32_at(0x0103_773C, u32::MAX);
        write_u32_at(0x011F_706C, 0);
        write_u8_at(0x011F_7070, 0);
        write_u8_at(0x011F_7066, 0);
        write_u8_at(0x011F_7067, 0);
        write_u8_at(0x011F_7071, 0);
        write_u8_at(0x011F_701E, 0);

        let first_counter: u64 = callee_cdecl!(7, u64,);
        write_u32_at(0x0120_F2A8, first_counter as u32);
        write_u32_at(0x0120_F2AC, (first_counter >> 32) as u32);
        let second_counter: u64 = callee_cdecl!(8, u64,);
        write_u32_at(0x0120_F2B0, second_counter as u32);
        write_u32_at(0x0120_F2B4, (second_counter >> 32) as u32);

        write_u32_at(0x011F_7058, IDENTITY_BITS);
        write_u16_at(0x011F_7074, 0);
        write_u8_at(0x011F_6952, 0);
        write_u32_at(0x011F_70CC, 0);
        write_u32_at(0x011F_70C8, 0);
        write_u32_at(0x011F_70D4, 0);
        write_u32_at(0x011F_70D0, 0);
        write_u32_at(0x011F_70DC, 0);
        write_u32_at(0x011F_70D8, 0);

        let _: u32 = callee_cdecl!(9, u32,);
        let _: u32 = callee_cdecl!(10, u32,);
        let _: u32 = callee_cdecl!(11, u32,);
        let _: u32 = callee_cdecl!(12, u32,);
        let _: u32 = callee_thiscall!(13, u32, relocated(OUTPUT_OBJECT_VA));
        let _: u32 = callee_thiscall!(14, u32, relocated(OUTPUT_OBJECT_VA));
    }
}

export!(cdecl, rw_00953c80(startup_flag: u8) -> () {
    run_global_state_init(startup_flag)
});

