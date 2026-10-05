// original: 0x00AC6770 stream_publish_state (proposed)

/// Publish the constant state vector and notify three listeners.
///
/// The original copies the 16-byte constant vector into four state slots,
/// zeroes the pending vector and its mirror, applies the pending handle
/// through the apply callee, then notifies three listeners with
/// (handle, table, 1, 5) (cdecl, no arguments). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6770() -> u32 {
    unsafe {
        const CONST_VEC: u32 = 0x00FE8F20;
        const SLOT0: u32 = 0x0103F290;
        const SLOT1: u32 = 0x0103F2A0;
        const PENDING: u32 = 0x0154E0F0;
        const MIRROR: u32 = 0x0103F2B0;
        const SLOT2: u32 = 0x0103F260;
        const SLOT3: u32 = 0x0103F270;
        const TABLE2: u32 = 0x0103F2F0;
        const HANDLE_IN: u32 = 0x015B0E8C;
        const HANDLE0: u32 = 0x0154E014;
        const HANDLE1: u32 = 0x0154E018;
        const HANDLE2: u32 = 0x0154E034;
        const APPLY: u32 = 1;
        const NOTIFY: u32 = 2;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe {
            let cookie = rd(lf_checker_rt::relocated(HANDLE_IN));
            let c = lf_checker_rt::relocated(CONST_VEC);
            let (v0, v1, v2, v3) = (rd(c), rd(c + 4), rd(c + 8), rd(c + 12));
            let s0 = lf_checker_rt::relocated(SLOT0);
            wr(s0, v0);
            wr(s0 + 4, v1);
            wr(s0 + 8, v2);
            wr(s0 + 12, v3);
            let s1 = lf_checker_rt::relocated(SLOT1);
            wr(s1, v0);
            wr(s1 + 4, v1);
            wr(s1 + 8, v2);
            wr(s1 + 12, v3);
            let p = lf_checker_rt::relocated(PENDING);
            wr(p, 0);
            wr(p + 4, 0);
            wr(p + 8, 0);
            wr(p + 12, 0);
            let m = lf_checker_rt::relocated(MIRROR);
            wr(m, 0);
            wr(m + 4, 0);
            wr(m + 8, 0);
            wr(m + 12, 0);
            lf_checker_rt::callee_cdecl!(APPLY, u32, cookie);
            let s2 = lf_checker_rt::relocated(SLOT2);
            wr(s2, v0);
            wr(s2 + 4, v1);
            wr(s2 + 8, v2);
            wr(s2 + 12, v3);
            let s3 = lf_checker_rt::relocated(SLOT3);
            wr(s3, v0);
            wr(s3 + 4, v1);
            wr(s3 + 8, v2);
            wr(s3 + 12, v3);
            let h0 = rd(lf_checker_rt::relocated(HANDLE0));
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, h0, s2, 1u32, 5u32);
            let h1 = rd(lf_checker_rt::relocated(HANDLE1));
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, h1, s3, 1u32, 5u32);
            let h2 = rd(lf_checker_rt::relocated(HANDLE2));
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, h2, lf_checker_rt::relocated(TABLE2), 1u32, 5u32);
            0
        }
    }
});
