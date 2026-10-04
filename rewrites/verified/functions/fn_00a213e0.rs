// original: 0x00a213e0 cam_angle_sweep_set (proposed)

/// Sweeps an angle through the shaper and stores the mapped result.
///
/// The float at `this + IN_OFF` is clamped at zero from below (a negative
/// becomes 0.0; NaN passes through), scaled by 180, subtracted from 270,
/// converted to radians, and passed to the shaper callee in `xmm0` (on
/// the rewrite side the bits travel as the callee's stack word and the
/// stub loads them into `xmm0`, exactly as the proven `xmm0_from_stack`
/// pattern). The answer is mapped as `((r + 1) * 0.5) * 40 + 45` in that
/// operation order and stored to `this + OUT_OFF`. Returns nothing.
///
/// Original: 0x00a213e0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a213e0(this: u32) -> u32 {
    unsafe {
        const C_SHAPER: u32 = 1;
        const IN_OFF: u32 = 0x28;
        const OUT_OFF: u32 = 0x60;
        const K_180: f32 = f32::from_bits(0x4334_0000); // 180.0
        const K_270: f32 = f32::from_bits(0x4387_0000); // 270.0
        const K_RAD: f32 = f32::from_bits(0x3c8e_fa35); // pi/180
        const K_ONE: f32 = f32::from_bits(0x3f80_0000); // 1.0
        const K_HALF: f32 = f32::from_bits(0x3f00_0000); // 0.5
        const K_40: f32 = f32::from_bits(0x4220_0000); // 40.0
        const K_45: f32 = f32::from_bits(0x4234_0000); // 45.0
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let raw = f32::from_bits(((this + IN_OFF) as *const u32).read_unaligned());
        let x1 = if 0.0 > raw { 0.0 } else { raw };
        let v = mul(sub(K_270, mul(x1, K_180)), K_RAD);
        let r: u32 = lf_checker_rt::callee_cdecl!(C_SHAPER, u32, v.to_bits());
        let s = mul(mul(add(f32::from_bits(r), K_ONE), K_HALF), K_40);
        let out = add(s, K_45);
        ((this + OUT_OFF) as *mut u32).write_unaligned(out.to_bits());
        0
    }
});
