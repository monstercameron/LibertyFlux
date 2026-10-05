// original: 0x00AC6F70 stream_set_param_mid (proposed)

/// Store the float argument into the middle parameter slot when it differs.
///
/// Same shape as the high-slot setter, on the middle slot (cdecl, one word).
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6F70(arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0103F26C;
        const TABLE: u32 = 0x0103F260;
        const HANDLE: u32 = 0x0154E014;
        const NOTIFY: u32 = 1;
        unsafe {
            let cur: f32 = (lf_checker_rt::relocated(SLOT) as *const f32).read_unaligned();
            if cur == f32::from_bits(arg) {
                return 0;
            }
            (lf_checker_rt::relocated(SLOT) as *mut u32).write_unaligned(arg);
            let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, handle, lf_checker_rt::relocated(TABLE), 1u32, 5u32);
            0
        }
    }
});
