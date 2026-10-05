// original: 0x00AC6FB0 stream_set_param_lo (proposed)

/// Store the float argument into the low parameter slot when it differs.
///
/// Same shape as its sibling setters, on the low slot (cdecl, one word).
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6FB0(arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0103F268;
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
