// original: 0x00a35910 vehicle_quadrant_blend (proposed)

/// Blend two delta floats into one result across four sign quadrants.
///
/// Divides `dx = c - a` by a divisor that is `1e-4` when `d - b` is
/// exactly zero (a divide-by-zero guard) and `d - b` itself otherwise,
/// calls the range observer (id 1, which takes the quotient in XMM0 via
/// transport), and combines the quotient with π/2 by quadrant. The first
/// branch tests `dx` itself: the compare runs BEFORE the division, so a
/// negative divisor flips the quadrant relative to the quotient's sign.
/// `dx` and divisor both positive gives `π - q`, a non-positive `dx` with
/// a positive divisor gives `-π - q`, and the other two give `-q` through
/// different operand orders. Cdecl/4 (float bits), returns on x87 ST0.
lf_checker_rt::export!(cdecl, rw_00a35910(a_bits: u32, b_bits: u32, c_bits: u32, d_bits: u32) -> f32 {
    unsafe {
        const TINY: f32 = f32::from_bits(0x38D1_B717); // 1e-4
        const PI_2: f32 = f32::from_bits(0x3FC9_0FDB);
        const NEG_PI_2: f32 = f32::from_bits(0xBFC9_0FDB);
        const OBSERVER: u32 = 1;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let a = f32::from_bits(a_bits);
        let b = f32::from_bits(b_bits);
        let c = f32::from_bits(c_bits);
        let d = f32::from_bits(d_bits);
        let dx = sub(c, a);
        let denom_raw = sub(d, b);
        // Divide-by-zero guard: an exact zero (either sign) is replaced by
        // the tiny value; anything else, NaN included, divides as itself.
        // (The ucomiss/lahf/test/jp sequence jumps unless ordered-equal.)
        let denom = if denom_raw == 0.0 { TINY } else { denom_raw };
        let q = div(dx, denom);
        let _: u32 = lf_checker_rt::callee_cdecl!(OBSERVER, u32, q.to_bits());
        // The original compares dx against zero BEFORE dividing, so the
        // first branch follows dx, not the quotient.
        if dx > 0.0 {
            if denom > 0.0 {
                add(sub(PI_2, q), PI_2)
            } else {
                sub(PI_2, add(q, PI_2))
            }
        } else if denom > 0.0 {
            sub(NEG_PI_2, add(q, PI_2))
        } else {
            sub(sub(PI_2, q), PI_2)
        }
    }
});
