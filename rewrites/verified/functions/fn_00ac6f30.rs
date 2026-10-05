// original: 0x00AC6F30 stream_set_param_hi (proposed)

/// Store the float argument into the high parameter slot when it differs.
///
/// Same shape as the scaled setter without the multiply: ordered compare of
/// the stored slot with the argument, store and notify (handle, table
/// address, 1, 5) on difference (cdecl, one word). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6F30(arg: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x0103F278;
        const TABLE: u32 = 0x0103F270;
        const HANDLE: u32 = 0x0154E018;
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
