// original: 0x00bfa240 task_attach_or_reuse

/// Attach the a0 task to a fresh slot: allocate through the system object,
/// bind, clear the reuse flag when the current occupant is a0 itself, then
/// release and stamp the generation counter (incremented when it matches).
///
/// Original: 0x00bfa240 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bfa240(this: u32, a0: u32) -> u32 {
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
        let h: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(SYS),
            r32u(this.wrapping_add(8)), 0, 0);
        if h == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(2, u32, h, r32u(a0.wrapping_add(0x20)));
        let r: u32 = lf_checker_rt::callee_stdcall!(3, u32,);
        if a0 == r {
            w8(h.wrapping_add(0x1d9), 0);
        }
        lf_checker_rt::callee_thiscall!(4, u32, h);
        let t: u32 = lf_checker_rt::callee_stdcall!(5, u32,);
        let g1 = r32u(lf_checker_rt::relocated(G1));
        let g2 = r32u(lf_checker_rt::relocated(G2));
        w32u(h.wrapping_add(0x1d4), if g1 == t { g2.wrapping_add(1) } else { g2 });
        0
    }
});
