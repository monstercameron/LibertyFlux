// original: 0x008dfd30 quantize_sample_pairs (proposed)

/// Quantize six paired float samples into twelve bytes.
///
/// For each lane `k` in 0..6 reads `a = arg0[k]` and `b = arg1[k]`. The
/// first byte is `a` clamped to `[0, HI]` (a value below zero becomes zero,
/// above `HI` becomes `HI`, NaN passes through) times `SCALE`, converted
/// with truncation toward zero (`cvttss2si`: NaN or out-of-range gives
/// `0x80000000`, whose low byte is stored). The second byte is `b + HI`
/// through the same clamp after scaling by `PRE`, times `SCALE` likewise.
/// The bytes land at `this + 0x48 + 2 * k`, bit 2 of `this + 0x59` is set,
/// and the full conversion of the last second byte is returned. The float
/// operation order is the original's. Thiscall, two stack arguments.
lf_checker_rt::export!(thiscall, rw_008dfd30(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const HI: u32 = 0x00fe_88e8;
        const SCALE: u32 = 0x00fe_8c08;
        const PRE: u32 = 0x00fe_8830;
        const LANES: u32 = 6;
        const OUT_OFF: u32 = 0x48;
        const FLAG_OFF: u32 = 0x59;
        const FLAG_BIT: u8 = 4;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// The original's clamp: below zero to zero, above `hi` to `hi`,
        /// NaN untouched (both compare-and-branch pairs fall through).
        #[inline(always)]
        fn clamp01(v: f32, hi: f32) -> f32 {
            let mut t = v;
            if t < 0.0 {
                t = 0.0;
            }
            if t > hi {
                t = hi;
            }
            t
        }
        /// `cvttss2si` in full, computed with integers: truncate toward zero,
        /// `0x80000000` for NaN and out-of-range inputs. Deliberately not
        /// written as a guarded `x as i32`: the compiler merges such a guard
        /// with the saturating cast into saturation fixups (clamping toward
        /// `i32::MAX`/zero) instead of preserving invalid-to-`0x80000000`.
        #[inline(always)]
        fn cvtt(x: f32) -> u32 {
            let b = x.to_bits();
            let exp = (((b >> 23) & 0xff) as i32) - 127;
            if exp < 0 {
                // |x| < 1 (zeros, denormals, fractions) truncates to 0.
                return 0;
            }
            if exp >= 31 {
                // NaN, infinities and |x| >= 2^31 are all invalid; -2^31
                // itself also encodes as 0x80000000, so no special case.
                return 0x8000_0000;
            }
            let mag = if exp >= 23 {
                (0x0080_0000 | (b & 0x007f_ffff)) << (exp - 23)
            } else {
                (0x0080_0000 | (b & 0x007f_ffff)) >> (23 - exp)
            };
            if b & 0x8000_0000 != 0 {
                mag.wrapping_neg()
            } else {
                mag
            }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }

        let hi = f32::from_bits(lf_checker_rt::global::<u32>(HI).read_unaligned());
        let scale = f32::from_bits(lf_checker_rt::global::<u32>(SCALE).read_unaligned());
        let pre = f32::from_bits(lf_checker_rt::global::<u32>(PRE).read_unaligned());
        let mut last: u32 = 0;
        let mut k: u32 = 0;
        while k < LANES {
            let a = rdf(arg0.wrapping_add(k.wrapping_mul(4)));
            let first = cvtt(mul(clamp01(a, hi), scale));
            ((this + OUT_OFF + k.wrapping_mul(2)) as *mut u8).write(first as u8);
            let b = rdf(arg1.wrapping_add(k.wrapping_mul(4)));
            let grown = mul(add(b, hi), pre);
            last = cvtt(mul(clamp01(grown, hi), scale));
            ((this + OUT_OFF + k.wrapping_mul(2) + 1) as *mut u8).write(last as u8);
            k += 1;
        }
        let flag = (this + FLAG_OFF) as *mut u8;
        flag.write(flag.read() | FLAG_BIT);
        last
    }
});
