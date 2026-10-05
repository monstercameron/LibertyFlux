// original: 0x00bf97c0 task_build_flagged

/// Build the flagged slot for a0 unless zero: allocate with a flag pointer
/// eleven bytes past an aligned flag word (the stub writes the word; the
/// flag is its top byte and selects the tail), expand a packed vector
/// through a scripted buffer into three fields, solve, then release and
/// stamp the generation counter.
///
/// Original: 0x00bf97c0 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bf97c0(this: u32, a0: u32) -> u32 {
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
        let mut flag = [0u32; 8];
        let p = (flag.as_mut_ptr() as u32).wrapping_add(11);
        let h: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(SYS),
            a0, r32u(this.wrapping_add(8)), p, 0x40000000, 1);
        if h == 0 {
            return 0;
        }
        let mut buf_b = [0u32; 3];
        let mut buf_a = [0u32; 3];
        lf_checker_rt::callee_thiscall!(2, u32, this, buf_b.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(3, u32, this, buf_a.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(4, u32, h, buf_b.as_mut_ptr() as u32);
        w32u(h.wrapping_add(0x190), buf_a[0]);
        w32u(h.wrapping_add(0x194), buf_a[1]);
        w32u(h.wrapping_add(0x198), buf_a[2]);
        lf_checker_rt::callee_thiscall!(5, u32, h);
        let mut dummy = [0u32; 4];
        lf_checker_rt::callee_cdecl!(6, u32, h, dummy.as_mut_ptr() as u32, 0x40200000, 0xbf800000);
        if r8(p.wrapping_sub(8)) == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(7, u32, h);
        let t: u32 = lf_checker_rt::callee_stdcall!(8, u32,);
        let g1 = r32u(lf_checker_rt::relocated(G1));
        let g2 = r32u(lf_checker_rt::relocated(G2));
        w32u(h.wrapping_add(0x1d4), if g1 == t { g2.wrapping_add(1) } else { g2 });
        0
    }
});
