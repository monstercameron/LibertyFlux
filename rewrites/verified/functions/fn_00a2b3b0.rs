// original: 0x00A2B3B0 task_accumulator_step (proposed)

/// Accumulator cell update with a sample path and a gated decrement path.
///
/// Arguments (thiscall: ECX holds `this`, two stack words): the low byte of
/// the first word is `cycle`, the second word is `arg1` as float bits.
/// Returns 1 on every gated return, 0 on the sample path and on the
/// below-floor return. `scaled0` is `(G_A * 50.0) * arg1`, computed first on
/// both paths.
///
/// When `cycle` is zero the sample callee runs with argument 7 and its x87
/// result is compared against the cell at `[sub+0x70+0x3B4]` where `sub` is
/// `[this+0x228]` (a null `sub` reads cell 0x3B4): a sample not above the
/// cell returns 0, else the cell becomes `((arg1 * 50.0) * G_A) * 0.5 + cell`
/// and the return is 0.
///
/// Otherwise the gate callee runs: a nonzero answer, a nonzero control byte
/// at `[sub+0x552]`, sampler bit 2 at `[sub+0x70+0x3D0]`, `arg1` exactly
/// +0.0/-0.0 (NaN passes), or a nonzero bypass global each return 1 with no
/// store. A cell at or below the floor global (NaN on either side included)
/// returns 0. Else the cell becomes `max(floor, cell - scaled0)`, where the
/// maximum keeps the floor only on an ordered strictly-greater comparison
/// (ties and NaN keep the decremented value), and the return is 1.

lf_checker_rt::export!(thiscall, rw_00a2b3b0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const G_A: u32 = 0x11735BC;
        const C50: u32 = 0xFE8B68;
        const C_ZERO: u32 = 0xFE8628;
        const G_BYPASS: u32 = 0x12DD62C;
        const G_LIM: u32 = 0xE9BD1C;
        const C_HALF: u32 = 0xFE8830;
        const GATE_CALLEE: u32 = 1;
        const SAMPLE_CALLEE: u32 = 2;
        const SUB_OFF: u32 = 0x228;
        const OUT_BIAS: u32 = 0x70;
        const CTRL_OFF: u32 = 0x552;
        const GATE_BIT_OFF: u32 = 0x3D0;
        const CELL_OFF: u32 = 0x3B4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let gain = f32::from_bits(g32(G_A));
        let c50 = f32::from_bits(g32(C50));
        let arg1f = f32::from_bits(arg1);
        let scaled0 = fmul(fmul(gain, c50), arg1f);
        if (arg0 & 0xff) == 0 {
            // Sampler path.
            let sub = rd32(this.wrapping_add(SUB_OFF));
            let out = if sub == 0 { 0 } else { sub.wrapping_add(OUT_BIAS) };
            let sample = f32::from_bits(lf_checker_rt::callee_cdecl!(SAMPLE_CALLEE, u32, 7u32));
            let cur = f32::from_bits(rd32(out.wrapping_add(CELL_OFF)));
            if !(sample > cur) {
                return 0;
            }
            let half = f32::from_bits(g32(C_HALF));
            let add = fmul(fmul(fmul(arg1f, c50), gain), half);
            let new = fadd(add, cur);
            unsafe { (out.wrapping_add(CELL_OFF) as *mut u32).write_unaligned(new.to_bits()) };
            return 0;
        }
        // Gate path.
        let gate = lf_checker_rt::callee_thiscall!(GATE_CALLEE, u8, this);
        if gate != 0 {
            return 1;
        }
        let sub = rd32(this.wrapping_add(SUB_OFF));
        if rd8(sub.wrapping_add(CTRL_OFF)) != 0 {
            return 1;
        }
        let out = if sub == 0 { 0 } else { sub.wrapping_add(OUT_BIAS) };
        if rd8(out.wrapping_add(GATE_BIT_OFF)) & 2 != 0 {
            return 1;
        }
        let czero = f32::from_bits(g32(C_ZERO));
        if arg1f == czero {
            return 1;
        }
        if rd8(lf_checker_rt::relocated(G_BYPASS)) != 0 {
            return 1;
        }
        let cur = f32::from_bits(rd32(out.wrapping_add(CELL_OFF)));
        let lim = f32::from_bits(g32(G_LIM));
        if !(cur > lim) {
            return 0;
        }
        let new = fsub(cur, scaled0);
        let capped = if lim > new { lim } else { new };
        unsafe { (out.wrapping_add(CELL_OFF) as *mut u32).write_unaligned(capped.to_bits()) };
        1
    }
});

// Wrong version: the dispatch condition is flipped, so every trial takes
// the other path (and calls the other callee). Must fail.
