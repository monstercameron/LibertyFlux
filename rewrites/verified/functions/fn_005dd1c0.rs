// original: 0x005dd1c0 CTaskSimpleAssessInjuredPed::vf17

/// Assess an injured ped: refresh a sensed value, then run staged checks.
///
/// Returns 1 when there is nothing to assess (null subject) or the
/// hold flag (bit 1 of `+0x14`) is set; with the refresh flag (bit 0)
/// set, samples the slot-`+0xfc` method of the argument into `+0x24`
/// and clears it. A null worker at `+0x20` is reset for a 0 return;
/// a stage at `+0x18` other than `0x12a` returns 0. Otherwise emits a
/// scripted call; a clear flag byte at `+0x30` returns 0 at once,
/// else a gated check runs (a zero low byte returns 0), then the
/// subject is touched (keeping a bit on a nonzero low
/// byte), clears bit 5 of the worker's word at `+0x04` when set, and
/// returns 0.
///
/// Original: 0x005dd1c0 (thiscall, one pointer argument).
lf_checker_rt::export!(thiscall, rw_005dd1c0(this: u32, arg: u32) -> u32 {
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
        const FORM: u32 = 0xf91b64;
        const RATE: u32 = 0x3e4ccccd;
        const STAGE: u32 = 0x12a;
        const KEEP: u32 = 0x2000;
        const CLEAR_BIT: u32 = 0xffffffdf;
        const SENSE: u32 = 1;
        const RESET: u32 = 2;
        const EMIT: u32 = 3;
        const CHECK: u32 = 4;
        const TOUCH: u32 = 5;
        let o1 = rd32(this + 0x1c);
        if o1 == 0 {
            return 1;
        }
        if rd8(this + 0x14) & 1 != 0 {
            let vt = rd32(arg);
            let sense: extern "thiscall" fn(u32) -> f32 =
                unsafe { core::mem::transmute(rd32(vt + 0xfc) as usize) };
            let r = sense(arg);
            wr32(this + 0x24, r.to_bits());
            wr8(this + 0x14, rd8(this + 0x14) & 0xfe);
        }
        if rd8(this + 0x14) & 2 != 0 {
            return 1;
        }
        let o2 = rd32(this + 0x20);
        if o2 == 0 {
            // The pushed word is the callee's argument (it cleans the stack).
            lf_checker_rt::callee_thiscall!(RESET, u32, this, arg);
            return 0;
        }
        if rd32(this + 0x18) != STAGE {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(EMIT, u32, arg, lf_checker_rt::relocated(FORM), RATE, 0, 0);
        let q1 = rd32(this + 0x1c);
        if q1 != 0 && rd8(q1 + 0x211) != 0 {
            // A clear flag byte returns 0 at once, skipping the touch below.
            if rd8(this + 0x30) == 0 {
                return 0;
            }
            let r = lf_checker_rt::callee_thiscall!(CHECK, u32, this + 0x28);
            if (r as u8) == 0 {
                return 0;
            }
        }
        let w = rd32(this + 0x1c);
        let s = lf_checker_rt::callee_thiscall!(TOUCH, u32, w);
        if (s as u8) != 0 {
            wr32(w + 0x270, rd32(w + 0x270) | KEEP);
        }
        let d = rd32(this + 0x20);
        let e = rd32(d + 4);
        if (e >> 5) & 1 != 0 {
            wr32(d + 4, e & CLEAR_BIT);
        }
        0
    }
});
