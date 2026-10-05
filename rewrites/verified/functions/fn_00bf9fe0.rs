// original: 0x00bf9fe0 task_ctor_43_chain

/// Initialise a task of kind 0x43 through a chain of four calls.
///
/// Original: 0x00bf9fe0 (thiscall, 4 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9fe0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
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
        const KIND: u32 = 0x43;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND);
        let g = (lf_checker_rt::relocated(GG) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, this, KIND, g, a1, a0, 1);
        lf_checker_rt::callee_thiscall!(3, u32, this, a2);
        lf_checker_rt::callee_thiscall!(4, u32, this, a3);
        w8(this + 2, 7);
        0
    }
});
