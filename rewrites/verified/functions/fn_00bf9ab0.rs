// original: 0x00bf9ab0 task_ensure_bind_stamp

/// Ensure the a0 task is bound unless null (creating the binding when the
/// slot is empty), transform through two scratch buffers (frame addresses
/// skipped; never read back), allocate, bind twice more, then release and
/// stamp the generation counter.
///
/// Original: 0x00bf9ab0 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9ab0(this: u32, a0: u32) -> u32 {
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
        if r32u(a0.wrapping_add(0x20)) == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, a0);
            lf_checker_rt::callee_thiscall!(2, u32, a0.wrapping_add(0x10), r32u(a0.wrapping_add(0x20)));
        }
        let saved = r32u(a0.wrapping_add(0x20));
        let mut scratch = [0u32; 8];
        let p = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(3, u32, this, p);
        let h: u32 = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(SYS),
            r32u(this.wrapping_add(8)), 0, 0);
        if h == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(5, u32, h, saved);
        lf_checker_rt::callee_thiscall!(6, u32, h, p);
        lf_checker_rt::callee_thiscall!(7, u32, lf_checker_rt::relocated(SYS), h, a0, 0);
        lf_checker_rt::callee_thiscall!(8, u32, h);
        let t: u32 = lf_checker_rt::callee_stdcall!(9, u32,);
        let g1 = r32u(lf_checker_rt::relocated(G1));
        let g2 = r32u(lf_checker_rt::relocated(G2));
        w32u(h.wrapping_add(0x1d4), if g1 == t { g2.wrapping_add(1) } else { g2 });
        0
    }
});
