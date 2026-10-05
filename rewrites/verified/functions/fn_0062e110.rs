// original: 0x0062E110 rage::ProceduralTextureSkyhat::vf3 (symbols)

/// Advance one sky-hat filter stage, then tail to the next stage.
///
/// Scales the feed at `+0x2c` by the global gain, adds the bias at `+0x28`,
/// and publishes the result to the parameter block (at `PARAMS`, `+0x44`)
/// slot `+0x10`; the delta against the previous value there is scaled by the
/// factor at `+0x18`, added to the accumulator at `+0x14` and stored back.
/// Finally tails to the parameter block's slot `+0x10` with the block as the
/// object, whose answer is the answer. All float operations keep the
/// original's operand order (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_0062e110(this: u32) -> u32 {
    unsafe {
        const PARAMS: u32 = 0x44;
        const GAIN: u32 = 0xFE8B3C;
        const TAIL_SLOT: u32 = 0x10;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let feed = f32::from_bits(rd(this, 0x2c));
        let gain = f32::from_bits(*lf_checker_rt::global::<u32>(GAIN));
        let mut stage = mul(feed, gain);
        stage = add(stage, f32::from_bits(rd(this, 0x28)));
        let params = rd(this, PARAMS);
        let mut acc = sub(stage, f32::from_bits(rd(params, 0x10)));
        ((params + 0x10) as *mut u32).write_unaligned(stage.to_bits());
        acc = mul(acc, f32::from_bits(rd(params, 0x18)));
        acc = add(acc, f32::from_bits(rd(params, 0x14)));
        ((params + 0x14) as *mut u32).write_unaligned(acc.to_bits());
        let vt = (params as *const u32).read_unaligned();
        let tgt = ((vt + TAIL_SLOT) as *const u32).read_unaligned();
        let next: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt as usize);
        next(params)
    }
});
