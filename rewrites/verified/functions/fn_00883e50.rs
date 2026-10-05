// original: 0x00883e50 stream_blend_vectors (proposed)
/// Blend a strided vector array toward a target over `ceil(count / 8)` blocks.
///
/// Advances a four-lane ramp (`ramp = [fa, step + fa, 0, 0]`, grown by
/// `4 * step` twice per block) where `step = (fb - fa) / count_f32` and
/// `count_f32` converts the unsigned `count` exactly (through 64-bit float,
/// as the original does). Each block reads two vectors from `dst` (at
/// `dst + 32k` and `dst + 16 + 32k`) and one from `src` (at `src + 16 +
/// 32k`), scales the destination vectors by the ramp, adds the scaled second
/// vector into the source vector, accumulates the scaled first vector into
/// the output at `src + 32k`, and stores the summed vector at `src + 16 +
/// 32k`. A zero count stores nothing.
///
/// Lanes 2 and 3 of the ramp are `2 * step + fa` and `3 * step + fa` (the
/// unpack stages also shuffle entry upper lanes, but into positions the
/// final unpack discards, so no entry state leaks in). Alignment: every
/// vector access is `movaps`, so `dst` must be 16-byte aligned and `src`
/// 16-byte aligned after its 32-byte header.
///
/// Original: cdecl, five stack arguments (`dst`, `src`, `count`, `fa`, `fb`),
/// no return value.
lf_checker_rt::export!(cdecl, rw_00883e50(dst: u32, src: u32, count: u32, fa: u32, fb: u32) -> u32 {
    unsafe {
        const ONE_ADDR: u32 = 0x00fe_88e8; // 1.0
        const THREE_ADDR: u32 = 0x00fe_8a94; // 3.0
        const TWO_ADDR: u32 = 0x00fe_8a24; // 2.0
        const FOUR_ADDR: u32 = 0x00fe_8ab8; // 4.0
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let lane = |addr: u32| {
            (lf_checker_rt::relocated(addr) as *const u32).read_unaligned()
        };
        let one = f32::from_bits(lane(ONE_ADDR));
        let three = f32::from_bits(lane(THREE_ADDR));
        let two = f32::from_bits(lane(TWO_ADDR));
        let four = f32::from_bits(lane(FOUR_ADDR));
        let fa = f32::from_bits(fa);
        let fb = f32::from_bits(fb);
        let count_f = (count as f64) as f32;
        let step = mul(div(one, count_f), sub(fb, fa));
        // The two unpack/shuffle stages select lanes [fa, step+fa, 2step+fa,
        // 3step+fa]; the entry upper lanes they also shuffle are discarded.
        let mut ramp = [fa, add(step, fa), add(mul(step, two), fa), add(mul(step, three), fa)];
        let grow = mul(step, four);
        if count == 0 {
            return 0;
        }
        let mut k = 0u32;
        let mut done = 0u32;
        while done < count {
            let dst_a = dst.wrapping_add(k);
            let dst_b = dst_a.wrapping_add(16);
            let src_v = src.wrapping_add(16).wrapping_add(k);
            let out_a = src.wrapping_add(k);
            let out_b = src.wrapping_add(16).wrapping_add(k);
            let mut va = [0.0f32; 4];
            let mut vb = [0.0f32; 4];
            let mut vs = [0.0f32; 4];
            for i in 0..4 {
                va[i] = f32::from_bits(
                    ((dst_a + (i * 4) as u32) as *const u32).read_unaligned(),
                );
                vb[i] = f32::from_bits(
                    ((dst_b + (i * 4) as u32) as *const u32).read_unaligned(),
                );
                vs[i] = f32::from_bits(
                    ((src_v + (i * 4) as u32) as *const u32).read_unaligned(),
                );
            }
            for i in 0..4 {
                va[i] = mul(va[i], ramp[i]);
            }
            for i in 0..4 {
                ramp[i] = add(ramp[i], grow);
            }
            for i in 0..4 {
                vb[i] = mul(vb[i], ramp[i]);
            }
            for i in 0..4 {
                ramp[i] = add(ramp[i], grow);
            }
            for i in 0..4 {
                vs[i] = add(vs[i], vb[i]);
            }
            for i in 0..4 {
                let acc = f32::from_bits(
                    ((out_a + (i * 4) as u32) as *const u32).read_unaligned(),
                );
                ((out_a + (i * 4) as u32) as *mut u32)
                    .write_unaligned(add(acc, va[i]).to_bits());
                ((out_b + (i * 4) as u32) as *mut u32).write_unaligned(vs[i].to_bits());
            }
            k = k.wrapping_add(32);
            done = done.wrapping_add(8);
        }
        0
    }
});
