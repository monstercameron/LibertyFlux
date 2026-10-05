// original: 0x00882e50 stream_sincos4 (proposed)
/// Evaluate sine and cosine for four angles at once (vectorised minimax core).
///
/// Takes four angles as the lanes of `xmm0` and writes two four-float result
/// vectors: the sine vector to `out_sin`, the cosine vector to `out_cos`
/// (both must be 16-byte aligned; the original stores with `movaps`). The
/// range reduction multiplies by 2/pi, splits each lane into an octant index
/// (truncated to integer, then tested for oddness and masked to two bits)
/// and a fraction clamped to [0, 1], folds the octant into the fraction with
/// lane selection, evaluates one shared minimax polynomial in the squared
/// fraction, and re-applies the octant signs.
///
/// The rewrite performs the same lane operations in the same order with
/// scalar operators: every float step goes through a helper that pins both
/// operands with `core::hint::black_box` (so the compiler keeps the
/// original's operand order), integer lane steps are exact `u32` arithmetic,
/// truncation matches `cvttps2dq` (indefinite `0x80000000` out of range),
/// and the minimum matches `minps` exactly (a NaN first lane yields the
/// second lane, otherwise a NaN yields NaN, signed zeros yield the second
/// lane). Constants are read from the original's read-only table through
/// `relocated`, the same words the checked original reads.
///
/// Original: two stack arguments (callee cleans 8), angles in `xmm0`,
/// no return value.
lf_checker_rt::export!(cdecl, rw_00882e50(out_sin: u32, out_cos: u32) -> u32 {
    unsafe {
        const SCALE_ADDR: u32 = 0x00e7_5830; // 2/pi
        const ONE_ADDR: u32 = 0x00e7_5810; // 1.0
        const SIGN_ADDR: u32 = 0x00e7_5840; // sign mask
        const ABS_ADDR: u32 = 0x00e7_5850; // absolute-value mask
        const ODD_ADDR: u32 = 0x00e7_5860; // integer 1 per lane
        const PAIR_ADDR: u32 = 0x00e7_5870; // integer 2 per lane
        const POLY_B_ADDR: u32 = 0x00e7_58b0;
        const POLY_A_ADDR: u32 = 0x00e7_58a0;
        const POLY_C_ADDR: u32 = 0x00e7_5890;
        const POLY_D_ADDR: u32 = 0x00e7_5880;
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
        #[inline(always)]
        fn cvttps(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000
            } else {
                core::hint::black_box(x) as i32 as u32
            }
        }
        #[inline(always)]
        fn minps(a: f32, b: f32) -> f32 {
            if a.is_nan() {
                b
            } else if b.is_nan() {
                b
            } else if a == 0.0 && b == 0.0 {
                b
            } else if core::hint::black_box(a) < core::hint::black_box(b) {
                a
            } else {
                b
            }
        }
        let lane = |addr: u32| {
            (lf_checker_rt::relocated(addr) as *const u32).read_unaligned()
        };
        let scale = f32::from_bits(lane(SCALE_ADDR));
        let one = f32::from_bits(lane(ONE_ADDR));
        let sign_mask = lane(SIGN_ADDR);
        let abs_mask = lane(ABS_ADDR);
        let odd = lane(ODD_ADDR);
        let pair = lane(PAIR_ADDR);
        let poly_b = f32::from_bits(lane(POLY_B_ADDR));
        let poly_a = f32::from_bits(lane(POLY_A_ADDR));
        let poly_c = f32::from_bits(lane(POLY_C_ADDR));
        let poly_d = f32::from_bits(lane(POLY_D_ADDR));
        let mut sines = [0u32; 4];
        let mut cosines = [0u32; 4];
        for i in 0..4 {
            let x = f32::from_bits(lf_checker_rt::xmm_word(0, i));
            let x_bits = x.to_bits();
            let x7 = x_bits & sign_mask;
            let mut x0 = mul(f32::from_bits(x_bits & abs_mask), scale);
            let mut x2 = cvttps(x0);
            let x5 = if odd & x2 == 0 { 0xFFFF_FFFF } else { 0 };
            let mut x3 = odd.wrapping_add(x2);
            let x6f = x2 as i32 as f32;
            x2 &= pair;
            x3 &= pair;
            x0 = sub(x0, x6f);
            x2 <<= 30;
            x0 = minps(x0, one);
            let x4 = sub(one, x0);
            x3 <<= 30;
            x2 ^= x7;
            let mut x7b = x5;
            let mut x6 = f32::from_bits(x4.to_bits() & x7b);
            x7b = !x7b & x0.to_bits();
            x0 = f32::from_bits(x0.to_bits() & x5);
            let x5b = !x5 & x4.to_bits();
            x6 = f32::from_bits(x6.to_bits() | x7b);
            x0 = f32::from_bits(x0.to_bits() | x5b);
            let x1 = x0;
            x7b = x6.to_bits();
            x0 = mul(x0, x0);
            x6 = mul(x6, x6);
            let x1 = f32::from_bits(x1.to_bits() | x2);
            x7b |= x3;
            let x2f = x0;
            let x3f = x6;
            x0 = add(mul(x0, poly_b), poly_a);
            x6 = add(mul(x6, poly_b), poly_a);
            x0 = add(mul(x0, x2f), poly_c);
            x6 = add(mul(x6, x3f), poly_c);
            x0 = add(mul(x0, x2f), poly_d);
            x6 = add(mul(x6, x3f), poly_d);
            x0 = mul(x0, x1);
            x6 = mul(x6, f32::from_bits(x7b));
            sines[i] = x0.to_bits();
            cosines[i] = x6.to_bits();
        }
        for i in 0..4 {
            ((out_sin + (i * 4) as u32) as *mut u32).write_unaligned(sines[i]);
            ((out_cos + (i * 4) as u32) as *mut u32).write_unaligned(cosines[i]);
        }
        0
    }
});
