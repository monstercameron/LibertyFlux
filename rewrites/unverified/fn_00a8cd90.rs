// original: 0x00a8cd90 pool_blend_weights (proposed)

/// Blend two occupancy bytes from a pair of floats, storing the leftover.
///
/// `this` takes the leftover weight at +0x10C; `f1`, `f2` are the range
/// ends, `f3` the divisor, and `p4`/`p5` receive one byte each. When `f2`
/// exceeds `f1`, `p4` takes 0xFF, `p5` takes 0 and the leftover is 1.0,
/// returning `p5`. Otherwise the ratio `(f1-f2)/f3` feeds `t = 1-ratio`:
/// both bytes start at 0xFF, then when 0.5 exceeds `t` the leftover is
/// `1-t` and `p4` takes `(t*2*255)` truncated, else the leftover is `t`
/// and `p5` takes `((1-t)*2*255)` truncated. Returns the truncated
/// integer. All float operations run in the original's operand order; the
/// first branch falls through exactly when `f2 > f1` (ordered greater),
/// the second takes the below-or-equal path, as `comiss` plus `jbe` does.
///
/// Original: 0x00A8CD90 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00a8cd90(
    this: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    p4: u32,
    p5: u32,
) -> u32 {
    unsafe {
        const LEFTOVER: u32 = 0x10c;
        const ONE_BITS: u32 = 0x3f800000;
        const HALF_BITS: u32 = 0x3f000000;
        const TWO_BITS: u32 = 0x40000000;
        const MAX_BYTE_BITS: u32 = 0x437f0000;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Truncate like `cvttss2si`: out-of-range (including NaN and
        /// infinite) yields `i32::MIN`, matching the hardware conversion
        /// where Rust's saturating `as` cast would differ.
        #[inline(always)]
        fn cvt(x: f32) -> i32 {
            if x > -2147483648.0f32 && x < 2147483648.0f32 {
                x.trunc() as i32
            } else {
                i32::MIN
            }
        }
        let f1 = f32::from_bits(a1);
        let f2 = f32::from_bits(a2);
        if f2 > f1 {
            (p4 as *mut u8).write(0xff);
            (p5 as *mut u8).write(0);
            ((this + LEFTOVER) as *mut u32).write_unaligned(ONE_BITS);
            return p5;
        }
        let f3 = f32::from_bits(a3);
        let ratio = div(sub(f1, f2), f3);
        let one = f32::from_bits(ONE_BITS);
        let t = sub(one, ratio);
        let compl = sub(one, t);
        (p4 as *mut u8).write(0xff);
        (p5 as *mut u8).write(0xff);
        let half = f32::from_bits(HALF_BITS);
        if half > t {
            let two = f32::from_bits(TWO_BITS);
            let scaled = mul(t, two);
            ((this + LEFTOVER) as *mut f32).write_unaligned(compl);
            let max = f32::from_bits(MAX_BYTE_BITS);
            let v = cvt(mul(scaled, max));
            (p4 as *mut u8).write(v as u8);
            v as u32
        } else {
            let two = f32::from_bits(TWO_BITS);
            let scaled = mul(compl, two);
            ((this + LEFTOVER) as *mut f32).write_unaligned(t);
            let max = f32::from_bits(MAX_BYTE_BITS);
            let v = cvt(mul(scaled, max));
            (p5 as *mut u8).write(v as u8);
            v as u32
        }
    }
});
