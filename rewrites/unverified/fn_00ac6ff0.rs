// original: 0x00AC6FF0 stream_set_param_pair (proposed)

/// Store a two-float vector into its slots when either word differs.
///
/// The original compares both stored slots with the words at `vec` (ordered
/// comparison) and returns on full equality (cdecl, one pointer). Otherwise
/// it stores both words and notifies with (handle, table, 1, 5).
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6FF0(vec: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103F2F0;
        const HANDLE: u32 = 0x0154E034;
        const NOTIFY: u32 = 1;
        unsafe {
            let n0 = f32::from_bits((vec as *const u32).read_unaligned());
            let n1 = f32::from_bits((vec.wrapping_add(4) as *const u32).read_unaligned());
            let c0: f32 = (lf_checker_rt::relocated(TABLE) as *const f32).read_unaligned();
            let c1: f32 = (lf_checker_rt::relocated(TABLE).wrapping_add(4) as *const f32).read_unaligned();
            if c0 == n0 && c1 == n1 {
                return 0;
            }
            (lf_checker_rt::relocated(TABLE) as *mut u32).write_unaligned(n0.to_bits());
            (lf_checker_rt::relocated(TABLE).wrapping_add(4) as *mut u32).write_unaligned(n1.to_bits());
            let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, handle, lf_checker_rt::relocated(TABLE), 1u32, 5u32);
            0
        }
    }
});
