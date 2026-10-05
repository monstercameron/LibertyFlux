// original: 0x00A4D460 vehicle_lerp_f34 (proposed)

/// Blends the table entry selected by a global index toward 120.0 by the
/// argument factor and stores the result at `this + OUT`.
///
/// `i` is the signed global index, `k` the float at `TABLE + i * STRIDE`
/// (0x210), `c = 120.0`, `t` the argument: stores `k + (c - k) * t` computed
/// strictly as `d = c - k`, `m = d * t`, `o = m + k` (the original's SSE
/// order, pinned with `black_box` so NaN payloads propagate identically).
/// Returns nothing.
///
/// Original: 0x00A4D460 (thiscall, one float stack word), leaf, SSE lerp.
lf_checker_rt::export!(thiscall, rw_00A4D460(this: u32, tbits: u32) -> u32 {
    unsafe {
        const OUT: u32 = 0x0F34;
        const INDEX: u32 = 0x01174790;
        const TABLE: u32 = 0x015E89E4;
        const STRIDE: i32 = 0x210;
        const C: f32 = f32::from_bits(0x42F0_0000); // 120.0
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let idx = lf_checker_rt::global::<i32>(INDEX).read_unaligned();
        let addr = lf_checker_rt::relocated(TABLE)
            .wrapping_add(idx.wrapping_mul(STRIDE) as u32);
        let k = f32::from_bits((addr as *const u32).read_unaligned());
        let t = f32::from_bits(tbits);
        let o = add(mul(sub(C, k), t), k);
        ((this + OUT) as *mut u32).write_unaligned(o.to_bits());
        0
    }
});
