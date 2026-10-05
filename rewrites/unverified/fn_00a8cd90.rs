// original: 0x00A8CD90 fade_flags_from_range (proposed)

/// Blend two flag bytes from where a value falls in a range.
///
/// Given `lo`, `hi` and a `divisor`, plus two output bytes: when `hi` is not
/// above `lo` the outputs are `0xFF` and `0` and the stored factor at
/// `this+0x10C` is 1. Otherwise `t = (lo - hi) / divisor` and
/// `v = 1 - t`: both outputs start at `0xFF`, the stored factor is the
/// smaller of `t` and `v`, and the output on the larger side is rescaled to
/// `side * 2 * 255` truncated toward zero (out-of-range or NaN yields
/// `0x80000000`, whose low byte is stored). NaN inputs take the first
/// branch whose comparison is unordered. No calls.
///
/// Original: thiscall, five stack words (two floats, divisor, two pointers),
/// no return value.
lf_checker_rt::export!(thiscall, rw_00A8CD90(
    this: u32,
    lo: u32,
    hi: u32,
    divisor: u32,
    out_a: u32,
    out_b: u32,
) -> u32 {
    unsafe {
        const FACTOR_OFF: u32 = 0x10c;
        const ONE_BITS: u32 = 0xfe88e8;
        const HALF_BITS: u32 = 0xfe8830;
        const TWO_BITS: u32 = 0xfe8a24;
        const SCALE_BITS: u32 = 0xfe8c08;
        #[inline(always)]
        fn c(va: u32) -> f32 {
            unsafe {
                f32::from_bits(
                    (lf_checker_rt::relocated(va) as *const u32).read_unaligned(),
                )
            }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// x86 `cvttss2si`: truncate toward zero; NaN, infinities and
        /// out-of-range values yield `i32::MIN` (Rust's `as` saturates
        /// instead, so the edges are handled explicitly).
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        let fa = f32::from_bits(lo);
        let fb = f32::from_bits(hi);
        let dv = f32::from_bits(divisor);
        if !(fb > fa) {
            (out_a as *mut u8).write(0xff);
            (out_b as *mut u8).write(0);
            ((this + FACTOR_OFF) as *mut u32).write_unaligned(0x3f800000);
            return 0;
        }
        let t = div(sub(fa, fb), dv);
        let one = c(ONE_BITS);
        let v = sub(one, t);
        (out_a as *mut u8).write(0xff);
        (out_b as *mut u8).write(0xff);
        let w = sub(one, v);
        if c(HALF_BITS) > v {
            let u = mul(v, c(TWO_BITS));
            ((this + FACTOR_OFF) as *mut f32).write_unaligned(w);
            let s = mul(u, c(SCALE_BITS));
            (out_a as *mut u8).write(cvtt(s) as u8);
        } else {
            let u = mul(w, c(TWO_BITS));
            ((this + FACTOR_OFF) as *mut f32).write_unaligned(v);
            let s = mul(u, c(SCALE_BITS));
            (out_b as *mut u8).write(cvtt(s) as u8);
        }
        0
    }
});
