// original: 0x00ada4c0 ui_blend_routed (proposed)

/// Blend float pairs by one shared factor and route the blends to two
/// handler callees, taking one of two wirings chosen by comparing `a1`
/// with `a7`.
///
/// Integers `a0`, `a1`, `a2`, `a4`, `a8`; floats `a3`, `a6`, `a9`; `a5`
/// is unread. `t = (a0 - a1) / (a4 - a1)` in float32, `u = 1 - t`, and
/// every blend has the shape `u*x + t*y` (two multiplies then one add, in
/// that operand order). Both wirings also form an integer quotient by
/// signed division (`(a2 - a8) * (a0 - a1) / (a4 - a1)` on the `a1 == a7`
/// path, `(a8 - a2) * (a0 - a1) / (a4 - a1)` otherwise) and add it to an
/// integer argument; the divisor must be non-zero and the quotient must
/// fit, which the contract guarantees with small inputs.
///
/// Equal path: `r1 = blend(a9, a6)`, `r2 = blend(a9, a3)`,
/// nine-word call `(a1, ebx, r2, a0, ebx, r1, a7, a8, a9)` with
/// `ebx = a8 + quotient`; eight-word call `(a1, a0, a2, ebx, a3, r5, r4,
/// r3)` with `r3 = r1`, `r4 = r2`, `r5 = blend(a3, a6)` recomputed; final
/// nine-word call `(a0, a2, r7, a4, a2, a6, a0, ebx, r6)` with `r6 = r1`,
/// `r7 = r5` recomputed. Unequal path: `r1 = blend(a6, a9)`,
/// `r2 = blend(a3, a9)`, nine-word call `(a0, ebp, r2, a4, ebp, r1, a4,
/// a8, a9)` with `ebp = a2 + quotient`; eight-word call `(a0, a4, a2,
/// ebp, r5, a6, r4, r3)` with `r5 = blend(a3, a6)`; final nine-word call
/// `(a1, a2, a3, a0, a2, r7, a0, ebp, r6)` with `r6 = r2`, `r7 = r5`.
/// The original recomputes the repeated blends rather than reusing them,
/// keeps intermediates in its incoming `a1`/`a7` slots, and returns nothing.
///
/// Original: 0x00ada4c0 (cdecl, ten stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00ada4c0(
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, _a5: u32,
    a6: u32, a7: u32, a8: u32, a9: u32,
) -> u32 {
    unsafe {
        const NINE_CALLEE: u32 = 1; // 0xad98c0, nine words
        const EIGHT_CALLEE: u32 = 2; // 0xad97c0, eight words

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
        #[inline(always)]
        fn blend(u: f32, t: f32, x: f32, y: f32) -> f32 {
            add(mul(u, x), mul(y, t))
        }

        let dnum = (a0 as i32).wrapping_sub(a1 as i32);
        let dden = (a4 as i32).wrapping_sub(a1 as i32);
        let t = div(dnum as f32, dden as f32);
        let u = sub(1.0, t);
        let a3f = f32::from_bits(a3);
        let a6f = f32::from_bits(a6);
        let a9f = f32::from_bits(a9);

        if a1 == a7 {
            let prod = (a2 as i32)
                .wrapping_sub(a8 as i32)
                .wrapping_mul(dnum)
                .wrapping_div(dden);
            let ebx = (a8 as i32).wrapping_add(prod) as u32;
            let r1 = blend(u, t, a9f, a6f);
            let r2 = blend(u, t, a9f, a3f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                NINE_CALLEE, u32, a1, ebx, r2.to_bits(), a0, ebx, r1.to_bits(), a7, a8, a9
            );
            let r3 = blend(u, t, a9f, a6f);
            let r4 = blend(u, t, a9f, a3f);
            let r5 = blend(u, t, a3f, a6f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                EIGHT_CALLEE, u32, a1, a0, a2, ebx, a3, r5.to_bits(), r4.to_bits(), r3.to_bits()
            );
            let r6 = blend(u, t, a9f, a6f);
            let r7 = blend(u, t, a3f, a6f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                NINE_CALLEE, u32, a0, a2, r7.to_bits(), a4, a2, a6, a0, ebx, r6.to_bits()
            );
        } else {
            let prod = (a8 as i32)
                .wrapping_sub(a2 as i32)
                .wrapping_mul(dnum)
                .wrapping_div(dden);
            let ebp = (a2 as i32).wrapping_add(prod) as u32;
            let r1 = blend(u, t, a6f, a9f);
            let r2 = blend(u, t, a3f, a9f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                NINE_CALLEE, u32, a0, ebp, r2.to_bits(), a4, ebp, r1.to_bits(), a4, a8, a9
            );
            let r3 = blend(u, t, a6f, a9f);
            let r4 = blend(u, t, a3f, a9f);
            let r5 = blend(u, t, a3f, a6f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                EIGHT_CALLEE, u32, a0, a4, a2, ebp, r5.to_bits(), a6, r4.to_bits(), r3.to_bits()
            );
            let r6 = blend(u, t, a3f, a9f);
            let r7 = blend(u, t, a3f, a6f);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                NINE_CALLEE, u32, a1, a2, a3, a0, a2, r7.to_bits(), a0, ebp, r6.to_bits()
            );
        }
        0
    }
});
