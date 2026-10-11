// original: 0x009B5A00 script_state_initialize_fields

/// Initialize three script-state records through the manager lookup helper.
///
/// The routine obtains records for selector values one, two, and nine. For
/// each selector it first resolves the shared manager, then requests the
/// record with a zero secondary selector. A non-null first record receives a
/// sign-extended global byte at `+0x360`; the second receives one global byte
/// at `+0x22c`; and the third receives three global bytes at `+0x213..=0x215`.
/// Each lookup result is checked independently for null. The two helper calls
/// per record are thiscall calls with two stack arguments, and the function
/// itself is a zero-argument cdecl routine whose final EAX value is the last
/// copied byte when the third record exists, or zero otherwise.
lf_checker_rt::export!(cdecl, rw_009b5a00() -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e498;
        const FIRST_SOURCE: u32 = 0x0103_bff8;
        const SECOND_SOURCE: u32 = 0x0103_b924;
        const THIRD_SOURCE_A: u32 = 0x012d_d5ec;
        const THIRD_SOURCE_B: u32 = 0x0103_c0d9;
        const THIRD_SOURCE_C: u32 = 0x0103_c0da;
        const FIRST_MANAGER_LOOKUP: u32 = 1;
        const FIRST_RECORD_LOOKUP: u32 = 2;
        const SECOND_MANAGER_LOOKUP: u32 = 3;
        const SECOND_RECORD_LOOKUP: u32 = 4;
        const THIRD_MANAGER_LOOKUP: u32 = 5;
        const THIRD_RECORD_LOOKUP: u32 = 6;

        #[inline(always)]
        unsafe fn global_u8(file_va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(file_va).read() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let manager_address = lf_checker_rt::relocated(MANAGER_GLOBAL);
        let first_manager = lf_checker_rt::callee_thiscall!(FIRST_MANAGER_LOOKUP, u32, manager_address, 1u32, 0u32);
        let first_record = lf_checker_rt::callee_thiscall!(FIRST_RECORD_LOOKUP, u32, first_manager);
        if first_record != 0 {
            let signed_default = global_u8(FIRST_SOURCE) as i8 as i32 as u32;
            write_u32(first_record.wrapping_add(0x360), signed_default);
        }

        let second_manager = lf_checker_rt::callee_thiscall!(SECOND_MANAGER_LOOKUP, u32, manager_address, 2u32, 0u32);
        let second_record = lf_checker_rt::callee_thiscall!(SECOND_RECORD_LOOKUP, u32, second_manager);
        if second_record != 0 {
            write_u8(second_record.wrapping_add(0x22c), global_u8(SECOND_SOURCE));
        }

        let third_manager = lf_checker_rt::callee_thiscall!(THIRD_MANAGER_LOOKUP, u32, manager_address, 9u32, 0u32);
        let third_record = lf_checker_rt::callee_thiscall!(THIRD_RECORD_LOOKUP, u32, third_manager);
        if third_record != 0 {
            write_u8(third_record.wrapping_add(0x213), global_u8(THIRD_SOURCE_A));
            write_u8(third_record.wrapping_add(0x214), global_u8(THIRD_SOURCE_B));
            write_u8(third_record.wrapping_add(0x215), global_u8(THIRD_SOURCE_C));
            u32::from(global_u8(THIRD_SOURCE_C))
        } else {
            0
        }
    }
});
