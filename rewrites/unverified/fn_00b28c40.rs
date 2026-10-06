// original: 0x00b28c40 slot_any_active

/// Reports whether any of the 24 file slots is active.
///
/// Reads the slot flag bytes (one byte per slot, 24 slots) and returns 1 as
/// soon as a flag equal to 1 is found, scanning slots 0 upward. Returns 0
/// when no flag is 1. Takes no arguments (cdecl, no stack words); the value
/// is in EAX with the upper bytes clear on both paths.
lf_checker_rt::export!(cdecl, rw_00b28c40() -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_COUNT: u32 = 24;
        const ACTIVE: u8 = 1;
        let base = lf_checker_rt::relocated(SLOT_FLAGS);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let flag = ((base + i) as *const u8).read();
            if flag == ACTIVE {
                return 1;
            }
            i += 1;
        }
        0
    }
});
