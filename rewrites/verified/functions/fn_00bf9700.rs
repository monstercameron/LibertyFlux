// original: 0x00bf9700 task_rebuild_slot

/// Rebuild this slot: probe the registry, copy fields through a scratch
/// buffer, allocate, solve over three scratch buffers, bind, report the
/// +0x1c field to the field writer, then release and stamp the generation.
/// All scratch-buffer addresses are skipped; the buffers are never read.
///
/// Original: 0x00bf9700 (thiscall, 0 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9700(this: u32) -> u32 {
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
        const C0: u32 = 0x00EBC794;
        const C1: u32 = 0x00EBC7A4;
        const SYS: u32 = 0x01394D60;
        const G1: u32 = 0x011F702C;
        const G2: u32 = 0x011F70C4;
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(C0), 0);
        let mut scratch = [0u32; 8];
        let p = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(2, u32, this, p);
        let h: u32 = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(SYS),
            r32u(this.wrapping_add(8)), 0, 0);
        if h == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(4, u32, p, p, p, 0);
        lf_checker_rt::callee_thiscall!(5, u32, h, p);
        lf_checker_rt::callee_thiscall!(6, u32, h, lf_checker_rt::relocated(C1), r32u(this.wrapping_add(0x1c)));
        lf_checker_rt::callee_thiscall!(7, u32, h);
        let t: u32 = lf_checker_rt::callee_stdcall!(8, u32,);
        let g1 = r32u(lf_checker_rt::relocated(G1));
        let g2 = r32u(lf_checker_rt::relocated(G2));
        w32u(h.wrapping_add(0x1d4), if g1 == t { g2.wrapping_add(1) } else { g2 });
        0
    }
});
