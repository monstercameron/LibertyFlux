// original: 0x00bf7860 task_ctor_35_flags_packed

/// Initialise a task of kind 0x35, copy a 3-dword block, then pack flag bits
/// from four arguments and the old flag byte into the flag byte.
///
/// Original: 0x00bf7860 (thiscall, 7 stack words).
lf_checker_rt::export!(thiscall, rw_00bf7860(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
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
        const KIND: u32 = 0x35;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND,
            (lf_checker_rt::relocated(GG) as *const u32).read_unaligned(), a1, a0, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        let mut cl = (a6 as u8) & 1;
        cl = cl.wrapping_add(cl) | ((a5 as u8) & 1);
        cl = cl.wrapping_add(cl) | ((a4 as u8) & 1);
        cl = (cl << 3) | (((a3 as u8).wrapping_sub(1)) & 7);
        cl = cl.wrapping_add(cl) | (r8(this + 0x1c) & 0x81);
        w8(this + 0x1c, cl);
        w8(this + 2, 1);
        0
    }
});
