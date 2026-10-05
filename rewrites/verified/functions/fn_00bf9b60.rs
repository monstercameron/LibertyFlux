// original: 0x00bf9b60 task_acquire_configured

/// Acquire a configured slot for the a0 task unless it is null: allocate
/// with an out-flag pointer (address skipped; the flag is scripted), bind,
/// seed the rate field from a flag bit, then either take the fast path or,
/// when the out-flag is clear, check a status bit, and stamp the generation.
///
/// Original: 0x00bf9b60 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9b60(this: u32, a0: u32) -> u32 {
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
        const SYS: u32 = 0x01394D60;
        const G1: u32 = 0x011F702C;
        const G2: u32 = 0x011F70C4;
        if a0 == 0 {
            return 0;
        }
        let mut scratch = [0u32; 4];
        let p = scratch.as_mut_ptr() as u32;
        let h: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(SYS),
            a0, r32u(this.wrapping_add(8)), p, 0, 0);
        if h == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, h, r32u(a0.wrapping_add(0x20)));
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(SYS), h, a0, 0);
        w32u(h.wrapping_add(0x1ec), 0x3f800000);
        if r8(this.wrapping_add(3)) & 2 != 0 {
            w32u(h.wrapping_add(0x1ec), 0x40400000);
        }
        if r8(p) == 0 && ((r32u(h.wrapping_add(0x254)) >> 3) & 1) != 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(4, u32, h);
        let t: u32 = lf_checker_rt::callee_stdcall!(5, u32,);
        let g1 = r32u(lf_checker_rt::relocated(G1));
        let g2 = r32u(lf_checker_rt::relocated(G2));
        w32u(h.wrapping_add(0x1d4), if g1 == t { g2.wrapping_add(1) } else { g2 });
        0
    }
});
