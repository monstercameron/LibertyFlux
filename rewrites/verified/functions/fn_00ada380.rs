// original: 0x00ada380 ui_blend_pair_b (proposed)

/// Blend two value pairs by one shared factor and hand both blends to the
/// pair handler, twice with different argument orders.
///
/// Twin of `rw_00ada240` with different wiring: the integers are `a0`, `a3`,
/// `a4`, the floats `a5`..`a8`, `t = (a0 - a3) / (a4 - a3)`, `u = 1 - t`,
/// `r1 = u*a6 + t*a8`, `r2 = u*a5 + t*a7` (two multiplies then one add, in
/// that operand order). The handler callee takes 8 words: first
/// `(a1, a2, a3, a0, a5, a6, r2, r1)`, then, after recomputing the same two
/// blends, `(a1, a2, a0, a4, r2, r1, a7, a8)`. The original reuses its
/// incoming `a0`/`a3` slots as float scratch; only the call arguments are
/// observable.
///
/// Original: 0x00ada380 (cdecl, nine stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00ada380(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32) -> u32 {
    unsafe {
        const PAIR_CALLEE: u32 = 1;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
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

        let num = (a0 as i32).wrapping_sub(a3 as i32) as f32;
        let den = (a4 as i32).wrapping_sub(a3 as i32) as f32;
        let t = div(num, den);
        let u = sub(1.0, t);
        let a5f = f32::from_bits(a5);
        let a6f = f32::from_bits(a6);
        let a7f = f32::from_bits(a7);
        let a8f = f32::from_bits(a8);
        let r1 = add(mul(u, a6f), mul(a8f, t));
        let r2 = add(mul(u, a5f), mul(a7f, t));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            PAIR_CALLEE, u32, a1, a2, a3, a0, a5, a6, r2.to_bits(), r1.to_bits()
        );
        let r3 = add(mul(u, a6f), mul(a8f, t));
        let r4 = add(mul(u, a5f), mul(a7f, t));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            PAIR_CALLEE, u32, a1, a2, a0, a4, r4.to_bits(), r3.to_bits(), a7, a8
        );
        0
    }
});
