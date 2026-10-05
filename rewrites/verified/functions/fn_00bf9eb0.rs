// original: 0x00bf9eb0 task_ctor_44_trunc_flag

/// Initialise a task of kind 0x44, run the byte quantiser and the block copy,
/// then store a truncated float argument with one flag bit.
///
/// Original: 0x00bf9eb0 (thiscall, 6 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9eb0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn r8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn w8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn r32u(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32u(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x <= -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        const KIND: u32 = 0x44;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND,
            (lf_checker_rt::relocated(GG) as *const u32).read_unaligned(), a1, a0, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        lf_checker_rt::callee_thiscall!(3, u32, this, a3);
        let c = cvtt(f32::from_bits(a4));
        let cl = (c as u8).wrapping_mul(2) | ((a5 as u8) & 1);
        w8(this + 0x20, cl);
        w8(this + 2, 7);
        0
    }
});
