// original: 0x00bf7980 task_ctor_2f_vec5_flags

/// Initialise a task of kind 0x2f: run the base, main and block initialisers,
/// store five float arguments, and pack five flag arguments into two bytes.
///
/// Original: 0x00bf7980 (thiscall, 13 stack words).
lf_checker_rt::export!(thiscall, rw_00bf7980(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32) -> u32 {
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
        const KIND: u32 = 0x2f;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND);
        let g = (lf_checker_rt::relocated(GG) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(2, u32, this, KIND, g, a1, a0, 1);
        lf_checker_rt::callee_thiscall!(3, u32, this, a2);
        let mut cl = ((a11 as u8) & 1) | (a12 as u8).wrapping_mul(2);
        cl = (cl << 2) | ((a9 as u8) & 3);
        cl = cl.wrapping_add(cl) | ((a10 as u8) & 1);
        w32u(this + 0x18, a4);
        w32u(this + 0x1c, a5);
        w32u(this + 0x20, a6);
        w32u(this + 0x24, a7);
        w32u(this + 0x28, a8);
        cl = (cl << 3) | ((a3 as u8) & 7);
        w8(this + 0x3c, cl);
        let mut al = cl & 7;
        w8(this + 2, 2);
        if cl & 8 != 0 {
            al = al.wrapping_add(2);
        }
        w8(this + 0x14, al);
        0
    }
});
