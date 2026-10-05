// original: 0x00c24860 cam_curve_eval (proposed)
/// Evaluate the global sample curve at `t`: scale by the sample count,
/// round to the segment index with the original's bit-trick rounding (sign
/// split, magic add/subtract, conditional unit step, all as both bits and
/// ordered float operations), linearly interpolate between the bracketing
/// samples with the fractional position divided by the count, then clamp
/// into [0, upper] where `upper` is the read-only bound. A NaN result
/// clamps to +0. Upper comparisons are strict (`ja`); the lower one takes
/// unordered (`jbe`).
///
/// Float operation order is the original's, pinned through `black_box`;
/// the truncation uses a cast, which matches `cvttss2si` for the
/// in-range values this contract feeds it (see contract: inputs are
/// bounded finite floats, counts small).
///
/// Original: 0x00c24860 (stdcall, one stack word; float result).
lf_checker_rt::export!(stdcall, rw_00c24860(tv: u32) -> f32 {
    unsafe {
        const COUNT: u32 = 0x016c86a8;
        const TABLE: u32 = 0x016c8658;
        const C_AND: u32 = 0x00fe8d1c;
        const C_CMP: u32 = 0x00fe8cf8;
        const C_UPPER: u32 = 0x00fe88e8;
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let r = lf_checker_rt::relocated;
        let count = (r(COUNT) as *const i32).read_unaligned();
        let count_f = count as f32;
        let t = f32::from_bits(tv);
        let mut b_and = (r(C_AND) as *const u32).read_unaligned();
        let mut b_cmp = (r(C_CMP) as *const u32).read_unaligned();
        let upper = (r(C_UPPER) as *const f32).read_unaligned();
        let scaled = mul(count_f, t);
        b_and &= scaled.to_bits();
        let x0 = f32::from_bits(scaled.to_bits() ^ b_and);
        let mut b0 = if x0 < f32::from_bits(b_cmp) { 0xffff_ffff } else { 0 };
        let mut f1 = scaled;
        b_cmp &= b0;
        b_cmp |= b_and;
        f1 = add(f1, f32::from_bits(b_cmp));
        f1 = sub(f1, f32::from_bits(b_cmp));
        let d = sub(f1, scaled);
        b0 = if !(d < f32::from_bits(b_and)) { 0xffff_ffff } else { 0 };
        b0 &= upper.to_bits();
        f1 = sub(f1, f32::from_bits(b0));
        let idx = f1 as i32;
        let idx_f = idx as f32;
        let base = r(TABLE).wrapping_add((idx as u32).wrapping_mul(4));
        let s0 = (base as *const f32).read_unaligned();
        let frac = sub(scaled, idx_f);
        let step = core::hint::black_box(upper) / core::hint::black_box(count_f);
        let frac = mul(frac, step);
        let s1 = (base.wrapping_add(4) as *const f32).read_unaligned();
        let span = sub(s1, s0);
        let y = add(mul(frac, span), s0);
        if y > upper {
            return upper;
        }
        if !(y > 0.0) {
            return 0.0;
        }
        y
    }
});
