// original: 0x00AC6E50 stream_set_param_vec4 (proposed)

/// Store a four-float vector into its slots when any word differs.
///
/// The original compares all four stored slots with the words at `vec`
/// (ordered comparison) and returns on full equality (cdecl, one pointer).
/// Otherwise it stores all four words and notifies with (handle, table,
/// 1, 5). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6E50(vec: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103F2C0;
        const HANDLE: u32 = 0x0154E030;
        const NOTIFY: u32 = 1;
        unsafe {
            let mut new = [0u32; 4];
            let mut same = true;
            for i in 0..4u32 {
                let n = (vec.wrapping_add(i * 4) as *const u32).read_unaligned();
                new[i as usize] = n;
                let c: f32 = (lf_checker_rt::relocated(TABLE).wrapping_add(i * 4) as *const f32)
                    .read_unaligned();
                same = same && c == f32::from_bits(n);
            }
            if same {
                return 0;
            }
            for i in 0..4u32 {
                (lf_checker_rt::relocated(TABLE).wrapping_add(i * 4) as *mut u32)
                    .write_unaligned(new[i as usize]);
            }
            let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, handle, lf_checker_rt::relocated(TABLE), 1u32, 5u32);
            0
        }
    }
});
