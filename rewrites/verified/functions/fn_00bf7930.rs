// original: 0x00bf7930 task_ctor_33_quantize

/// Initialise a task of kind 0x33, copy a block, run the vector quantiser,
/// then fold one byte argument into flag bits.
///
/// Original: 0x00bf7930 (thiscall, 5 stack words).
lf_checker_rt::export!(thiscall, rw_00bf7930(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
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
        const KIND: u32 = 0x33;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND,
            (lf_checker_rt::relocated(GG) as *const u32).read_unaligned(), a1, a0, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        lf_checker_rt::callee_thiscall!(3, u32, this, a3);
        let t = ((a4 as u8).wrapping_sub(1)).wrapping_mul(2) ^ r8(this + 0x1c);
        w8(this + 2, 1);
        w8(this + 0x1c, r8(this + 0x1c) ^ (t & 0x0e));
        0
    }
});
