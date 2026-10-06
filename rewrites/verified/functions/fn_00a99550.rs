// original: 0x00a99550 filemem_blend_rgba

/// Blend two packed 32-bit colors lane by lane and store the result.
///
/// `src_a` and `src_b` point to the two source colors, `t` is the blend
/// factor, and the blended color is written to `out`. Each of the four
/// bytes is blended independently as `trunc((b - a) * t + a)`, where the
/// byte difference is exacted as a signed 32-bit integer, converted exactly
/// to float, multiplied by `t` then added to `a` in that order (single
/// precision), and truncated toward zero; a result that does not fit a
/// signed 32-bit integer (including NaN and infinities) contributes byte
/// 0x00, matching the original's truncate-convert instruction. The high
/// byte is blended first and ends up high. Returns `out`.
///
/// Original: 0x00A99550 (cdecl, four stack words, leaf, SSE float math).
lf_checker_rt::export!(cdecl, rw_00a99550(out: u32, src_a: u32, src_b: u32, t: f32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// One blended lane: truncate-convert semantics of cvttss2si,
        /// keeping only the low byte the original keeps.
        #[inline(always)]
        fn lane(a: u8, b: u8, t: f32) -> u8 {
            let d = (b as i32).wrapping_sub(a as i32);
            let v = add(mul(d as f32, t), a as f32);
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                0
            } else {
                v as i32 as u8
            }
        }

        let a = (src_a as *const u32).read_unaligned();
        let b = (src_b as *const u32).read_unaligned();
        let mut r: u32 = 0;
        for i in [3u32, 2, 1, 0] {
            let la = ((a >> (i * 8)) & 0xff) as u8;
            let lb = ((b >> (i * 8)) & 0xff) as u8;
            r = (r << 8) | (lane(la, lb, t) as u32);
        }
        (out as *mut u32).write_unaligned(r);
        out
    }
});
