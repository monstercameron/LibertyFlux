// original: 0x00bf8f90 task_ctor_3a_sub_flags

/// Initialise a task of kind 0x3a through the 0x3e sub-initialiser and the
/// block initialiser, then pack flag bits from three arguments.
///
/// Original: 0x00bf8f90 (thiscall, 7 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8f90(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
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
        lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a3);
        w8(this, 0x3a);
        lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        let mut cl = ((a6 as u8) & 1).wrapping_mul(2) | ((a5 as u8) & 1);
        cl = cl.wrapping_mul(2) | (r8(this + 0x2c) & 0xf8);
        cl |= (a4 as u8) & 1;
        w8(this + 0x14, 2);
        w8(this + 0x2c, cl);
        w8(this + 2, 4);
        0
    }
});
