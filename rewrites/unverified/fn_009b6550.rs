// original: 0x009B6550 script_check_signed_component_modes

/// Check either of two signed-byte component modes against the expected code.
/// The stdcall arguments are an expected dword, a second-check enable byte,
/// and a first-check enable byte. When the first flag is nonzero, the routine
/// asks the manager for the current action and compares its sign-extended byte
/// at `+0x38f` with the expected dword. If that does not match, the second flag
/// may query another object and compare its sign-extended byte at `+0x22e`.
/// Either match returns one in AL; all other paths return zero.
lf_checker_rt::export!(stdcall, rw_009b6550(expected_mode: u32, second_enabled: u32, first_enabled: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e498;
        const FIRST_MODE: u32 = 0x38f;
        const SECOND_MODE: u32 = 0x22e;
        const FIRST_QUERY: u32 = 1;
        const SECOND_QUERY: u32 = 2;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let manager_slot = lf_checker_rt::relocated(MANAGER_GLOBAL);
        if first_enabled as u8 != 0 {
            let first_object = lf_checker_rt::callee_thiscall!(FIRST_QUERY, u32, manager_slot);
            if first_object != 0 {
                let first_mode = read_u8(first_object.wrapping_add(FIRST_MODE)) as i8 as i32 as u32;
                if first_mode == expected_mode {
                    return 1;
                }
            }
        }

        if second_enabled as u8 != 0 {
            let second_object = lf_checker_rt::callee_thiscall!(SECOND_QUERY, u32, manager_slot);
            if second_object != 0 {
                let second_mode = read_u8(second_object.wrapping_add(SECOND_MODE)) as i8 as i32 as u32;
                if second_mode == expected_mode {
                    return 1;
                }
            }
        }
        0
    }
});
