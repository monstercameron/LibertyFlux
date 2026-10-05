// original: 0x00AC6DB0 stream_set_param_with_complement (proposed)

/// Store the argument and its complement `1 - arg` into four slots.
///
/// The original compares the stored slot with the float argument (ordered)
/// and returns on equality (cdecl, one word). Otherwise it stores `arg`,
/// `1 - arg`, `arg`, `1 - arg` into the four slots and notifies with
/// (handle, table, 1, 5). The subtraction order is the original's.
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6DB0(arg: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0103F2E0;
        const SLOT: u32 = 0x0103F2E8;
        const HANDLE: u32 = 0x0154E03C;
        const ONE_ADDR: u32 = 0x00FE88E8;
        const NOTIFY: u32 = 1;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        unsafe {
            let cur: f32 = (lf_checker_rt::relocated(SLOT) as *const f32).read_unaligned();
            let new = f32::from_bits(arg);
            if cur == new {
                return 0;
            }
            let one: f32 = (lf_checker_rt::relocated(ONE_ADDR) as *const f32).read_unaligned();
            let comp = sub(one, new);
            let t = lf_checker_rt::relocated(TABLE);
            (t as *mut u32).write_unaligned(arg);
            (t.wrapping_add(4) as *mut f32).write_unaligned(comp);
            (t.wrapping_add(8) as *mut u32).write_unaligned(arg);
            (t.wrapping_add(12) as *mut f32).write_unaligned(comp);
            let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, handle, t, 1u32, 5u32);
            0
        }
    }
});
