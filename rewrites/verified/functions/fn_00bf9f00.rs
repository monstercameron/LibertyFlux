// original: 0x00bf9f00 task_ctor_45_branch_flags

/// Initialise a task of kind 0x45: the main initialiser, then either the
/// block initialiser or the quantiser-plus-copy pair depending on a1,
/// then flag packing with a second branch on a7.
///
/// Original: 0x00bf9f00 (thiscall, 10 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9f00(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32) -> u32 {
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
        const KIND: u32 = 0x45;
        const GG: u32 = 0x011735A4;
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND,
            (lf_checker_rt::relocated(GG) as *const u32).read_unaligned(), a1, a0, 0);
        if a1 != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        } else {
            lf_checker_rt::callee_thiscall!(3, u32, this, a3);
            lf_checker_rt::callee_thiscall!(4, u32, this, a4);
        }
        w32u(this + 0x20, a9);
        if (a7 as u8).wrapping_sub(1) > 2 {
            w8(this + 0x28, r8(this + 0x28) & 0xf3);
        } else {
            let t = ((a7 as u8) << 2) ^ r8(this + 0x28);
            w8(this + 0x28, r8(this + 0x28) ^ (t & 0x0c));
        }
        let cl = (r8(this + 0x28) & 0xfc) | ((((a6 as u8) & 1) << 1) | ((a5 as u8) & 1));
        w8(this + 0x28, cl);
        w32u(this + 0x24, a8);
        w8(this + 2, 7);
        0
    }
});
