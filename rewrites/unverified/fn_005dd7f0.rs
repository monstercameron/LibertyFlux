// original: 0x005dd7f0 CTaskComplexTreatAccident::vf19

/// Run a treat-accident task step, or clone a none task when childless.
///
/// With a child at `+0x14`: stores the global at `EXTRA` into the
/// child's word at `+0xd3c`, issues two ten-word scripted calls against
/// the argument object (at `+0x570` and `+0xbb0`), then a final call
/// with `(0x3a6, arg)` whose result is returned. Without a child:
/// allocates through the pool at `POOL` and default-constructs a none
/// task (null stays null). The original aligns its frame for SSE; the
/// rewrite needs no frame.
///
/// Original: 0x005dd7f0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dd7f0(this: u32, arg: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const POOL: u32 = 0x167e2a0;
        const EXTRA: u32 = 0x11735b4;
        const VTABLE_NONE: u32 = 0xeb38c4;
        const FORM_A: u32 = 0xf91b68;
        const FORM_B: u32 = 0xf91b4c;
        const ONE_F: u32 = 0x3f800000;
        const STAGE: u32 = 1;
        const KIND: u32 = 0x3a6;
        const CALL_A: u32 = 1;
        const CALL_B: u32 = 2;
        const CALL_C: u32 = 3;
        const ALLOC: u32 = 4;
        const CTOR: u32 = 5;
        let child = rd32(this + 0x14);
        if child != 0 {
            let g = rd32(lf_checker_rt::relocated(EXTRA));
            wr32(child + 0xd3c, g);
            lf_checker_rt::callee_thiscall!(CALL_A, u32, arg + 0x570,
                lf_checker_rt::relocated(FORM_A), 1, 0, 0, 0xffffffff, 0, 0, ONE_F, 0, 0);
            lf_checker_rt::callee_thiscall!(CALL_B, u32, arg + 0xbb0,
                lf_checker_rt::relocated(FORM_B), 0, child, 0x1388, 0x4b5, 0, 0, 0x1f4, 0x1f4, 1);
            return lf_checker_rt::callee_thiscall!(CALL_C, u32, this, KIND, arg);
        }
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, new);
        wr32(new, lf_checker_rt::relocated(VTABLE_NONE));
        new
    }
});
