// original: 0x00a35830 vehicle_phase_index (proposed)

/// Map a phase float to an integer index through range folding and
/// round-to-nearest with a sign-dependent adjustment.
///
/// Negative inputs gain 2π, then the value is folded into `[0, 2π)` by
/// subtracting 2π when `2π - x + π/8 >= 2π`, scaled by `8/(2π)`, rounded
/// with the 2^23 magic constant, and adjusted by -1 when the rounded
/// value's fraction is ordered-greater than the value's sign bit pattern
/// (`cmpnless` is predicate NLE, not NLT: equal values, including +0.0
/// against +0.0, give no adjustment, while unordered always adjusts).
/// The final conversion truncates toward zero and yields `i32::MIN` for
/// NaN or out-of-range values, exactly like `cvttss2si`. Cdecl/1 (float
/// bits), returns the index in EAX. All float operations keep the
/// original's operand order.
lf_checker_rt::export!(cdecl, rw_00a35830(arg_bits: u32) -> u32 {
    unsafe {
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const PI_8: f32 = f32::from_bits(0x3EC9_0FDB);
        const INV_2PI: f32 = f32::from_bits(0x3E22_F983);
        const EIGHT: f32 = f32::from_bits(0x4100_0000);
        const TWO_POW_23: f32 = f32::from_bits(0x4B00_0000);
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        const SIGN_BIT: u32 = 0x8000_0000;
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
        let mut x = f32::from_bits(arg_bits);
        if x < 0.0 {
            x = add(x, TWO_PI);
        }
        let mut y = add(sub(TWO_PI, x), PI_8);
        if !(y < TWO_PI) {
            y = sub(y, TWO_PI);
        }
        y = mul(mul(y, INV_2PI), EIGHT);
        let sign = f32::from_bits(y.to_bits() & SIGN_BIT);
        let abs = f32::from_bits(y.to_bits() ^ sign.to_bits());
        let magic = if abs < TWO_POW_23 {
            f32::from_bits(TWO_POW_23.to_bits() | sign.to_bits())
        } else {
            sign
        };
        let mut r = sub(add(y, magic), magic);
        let frac = sub(r, y);
        // cmpnless is predicate NLE: true unless frac is less-or-equal to
        // the sign pattern (equal values give 0.0; unordered gives 1.0).
        let adj = if !(frac <= sign) { ONE } else { 0.0 };
        r = sub(r, adj);
        // cvttss2si: truncate; NaN or out of i32 range gives i32::MIN.
        if r.is_nan() || r < -2147483648.0 || r >= 2147483648.0 {
            0x8000_0000
        } else {
            (r as i32) as u32
        }
    }
});
