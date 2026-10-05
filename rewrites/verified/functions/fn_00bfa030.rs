// original: 0x00bfa030 task_ctor_40_toggle_float

/// Initialise a task of kind 0x40, toggle one flag bit from an argument,
/// store one float argument, and run the word quantiser.
///
/// Original: 0x00bfa030 (thiscall, 5 stack words).
lf_checker_rt::export!(thiscall, rw_00bfa030(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
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
        const KIND: u32 = 0x40;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND);
        let g = (lf_checker_rt::relocated(GG) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, this, KIND, g, a1, a0, 1);
        let t = r8(this + 0x22) ^ (a4 as u8);
        w8(this + 0x22, r8(this + 0x22) ^ (t & 1));
        w32u(this + 0x18, a2);
        lf_checker_rt::callee_thiscall!(3, u32, this, a3);
        w8(this + 2, 8);
        0
    }
});
