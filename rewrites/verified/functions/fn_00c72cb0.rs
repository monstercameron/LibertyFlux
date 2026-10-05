// original: 0x00c72cb0 ped_task_params_rescale (proposed)

/// Rescale the timing fields of a ped task parameter block.
///
/// `this` points to the block (at least 0xFC bytes). The routine divides 100
/// by the tick count at `+0x30` (read as a signed integer) and stores the
/// rate at `+0x34`; converts the duration at `+0x9C` from degrees to radians
/// and stores its reciprocal at `+0xA0`; derives three more reciprocals at
/// `+0xA8`, `+0x90`, `+0x98` (the last over the difference of `+0x8C` and
/// `+0x94`); multiplies the pair at `+0xB8`/`+0xBC` by the game speed factor
/// (0.1, or 0.3 when flag bit 0x20000000 at `+0xF0` is set); multiplies `+0x14`
/// by a third game factor; converts the speed at `+0x50` from km/h to m/s and
/// then scales it by 1.2 into `+0x4C`; clamps the level byte at `+0x40` down
/// to 7; reports the folded speed, the clamped level and a pointer at `+0x54`
/// to the parameter consumer (intercepted cdecl callee of three arguments);
/// and finally converts `+0x84` from degrees to radians with its reciprocal
/// at `+0x88`. All arithmetic is single precision in the original's operand
/// order.
///
/// Original: 0x00C72CB0 (thiscall, no stack arguments; no defined return).
lf_checker_rt::export!(thiscall, rw_00C72CB0(this: u32) -> u32 {
    unsafe {
        const HUNDRED: u32 = 0xFE8BB0; // 100.0
        const ONE: u32 = 0xFE88E8; // 1.0
        const DEG_TO_RAD: u32 = 0xFE8728; // pi/180
        const KMH_TO_MS: u32 = 0xED3760; // 5/18
        const HEADROOM: u32 = 0xFE891C; // 1.2
        const G_SPEED_LO: u32 = 0x104B6E8; // 0.1
        const G_SPEED_HI: u32 = 0x104B6EC; // 0.3
        const G_MISC: u32 = 0x104B6F0;
        const SPEED_FLAG: u32 = 0x2000_0000;
        const LEVEL_MAX: u8 = 7;
        const CONSUMER: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { ((a) as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gk(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(va) as *const u32).read()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let speed_hi = rd32(this + 0xF0) & SPEED_FLAG != 0;
        let one = gk(ONE);

        let ticks = rd32(this + 0x30) as i32 as f32;
        wrf(this + 0x34, div(gk(HUNDRED), ticks));

        let dur = mul(rdf(this + 0x9C), gk(DEG_TO_RAD));
        wrf(this + 0x9C, dur);
        wrf(this + 0xA0, div(one, dur));

        wrf(this + 0xA8, div(one, rdf(this + 0xA4)));

        let v = rdf(this + 0x8C);
        wrf(this + 0x90, div(one, v));
        wrf(this + 0x98, div(one, sub(v, rdf(this + 0x94))));

        let speed = gk(if speed_hi { G_SPEED_HI } else { G_SPEED_LO });
        wrf(this + 0xB8, mul(rdf(this + 0xB8), speed));
        wrf(this + 0xBC, mul(rdf(this + 0xBC), speed));

        let over = ((this + 0x40) as *const u8).read() > LEVEL_MAX;
        wrf(this + 0x14, mul(rdf(this + 0x14), gk(G_MISC)));

        let folded = mul(rdf(this + 0x50), gk(KMH_TO_MS));
        wrf(this + 0x50, folded);
        let folded = mul(folded, gk(HEADROOM));
        wrf(this + 0x4C, folded);

        if over {
            ((this + 0x40) as *mut u8).write(LEVEL_MAX);
        }
        let level = ((this + 0x40) as *const u8).read() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(CONSUMER, u32, folded.to_bits(), level, this + 0x54);

        let tail = mul(rdf(this + 0x84), gk(DEG_TO_RAD));
        wrf(this + 0x84, tail);
        wrf(this + 0x88, div(one, tail));
        0
    }
});
