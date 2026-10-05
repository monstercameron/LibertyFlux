// original: 0x00bfa0b0 task_ctor_3f_toggle

/// Initialise a task of kind 0x3f and toggle one flag bit from an argument.
///
/// Original: 0x00bfa0b0 (thiscall, 3 stack words).
lf_checker_rt::export!(thiscall, rw_00bfa0b0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
        const KIND: u32 = 0x3f;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND);
        let g = (lf_checker_rt::relocated(GG) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, this, KIND, g, a1, a0, 1);
        let t = (a2 as u8).wrapping_mul(2) ^ r8(this + 3);
        w8(this + 2, 9);
        w8(this + 3, r8(this + 3) ^ (t & 2));
        0
    }
});
