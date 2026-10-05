// original: 0x00bf8ff0 task_ctor_3b_compare_consts

/// Initialise a task of kind 0x3b through the 0x3e sub-initialiser, store
/// byte and float arguments, and set a flag bit when two shared float
/// constants compare equal (ordered equality; NaN never sets it).
///
/// Original: 0x00bf8ff0 (thiscall, 8 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8ff0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
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
        const F1: u32 = 0x0139C25C;
        const F2: u32 = 0x00FE8A94;
        lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a3);
        w8(this + 0x14, (a2 as u8).wrapping_add(2));
        w8(this + 0x20, a5 as u8);
        w8(this + 0x21, a6 as u8);
        let t = r8(this + 0x23) ^ (a7 as u8);
        w8(this, 0x3b);
        w8(this + 0x23, r8(this + 0x23) ^ (t & 1));
        w32u(this + 0x1c, a4);
        w8(this + 0x22, a2 as u8);
        let f1 = f32::from_bits((lf_checker_rt::relocated(F1) as *const u32).read_unaligned());
        let f2 = f32::from_bits((lf_checker_rt::relocated(F2) as *const u32).read_unaligned());
        w8(this + 2, 4);
        if f1 == f2 {
            w8(this + 3, r8(this + 3) | 2);
        }
        0
    }
});
