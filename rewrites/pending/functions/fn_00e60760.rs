// original: 0x00e60760 zero_timing_table_and_forward
/// Zeroes sixteen timing table entries, then forwards one fixed callback
/// address to the shared mainloop helper.
///
/// Each entry is seven words; the sixteen entries fill `0x1c0` bytes from
/// `0x019f36a0`. Returns the helper's answer unchanged.
export!(cdecl, rw_00e60760() -> u32 {
    unsafe {
        let mut p = relocated(0x019F36A0) as *mut u32;
        let mut left = 16u32 * 7;
        while left > 0 {
            *p = 0;
            p = p.add(1);
            left -= 1;
        }
        callee_cdecl!(1, u32, relocated(0x00E6FA00))
    }
});
