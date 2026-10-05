// original: 0x00AF8800 veh_point_line_distance (proposed)

/// Length of the rejection of `b - c` from `a - c`, clamped at zero.
///
/// Treats the three arguments as three-float vectors `c` (origin), `a` and
/// `b`. With `u = a - c` and `v = b - c` it forms `m = |v|^2 - (u.v)^2/|u|^2`
/// exactly as the original's SSE sequence does (dot product accumulated as
/// `((v1*u1) + (u0*v0)) + (v2*u2)`, norms as `((x0^2 + x1^2) + x2^2)`, every
/// operation in the original's operand order). When `m` is positive or NaN
/// the result is `sqrt(m)`; when `m` is zero or negative (float roundoff
/// around a degenerate configuration) the result is +0.0. The original also
/// spills into its incoming first-argument stack slot; that scratch write is
/// not modelled (the contract runs with the stack check off).
///
/// Original: 0x00AF8800 (cdecl, three stack arguments, result in ST0).
lf_checker_rt::export!(cdecl, rw_00AF8800(c: u32, a: u32, b: u32) -> f32 {
    unsafe {
        #[inline(always)]
        unsafe fn lane(p: u32, i: u32) -> f32 {
            unsafe { f32::from_bits(((p + i * 4) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        let (u0, u1, u2) = (sub(lane(a, 0), lane(c, 0)), sub(lane(a, 1), lane(c, 1)), sub(lane(a, 2), lane(c, 2)));
        let (v0, v1, v2) = (sub(lane(b, 0), lane(c, 0)), sub(lane(b, 1), lane(c, 1)), sub(lane(b, 2), lane(c, 2)));
        let dot = add(add(mul(v1, u1), mul(u0, v0)), mul(v2, u2));
        let unorm = add(add(mul(u0, u0), mul(u1, u1)), mul(u2, u2));
        let along = div(mul(dot, dot), unorm);
        let vnorm = add(add(mul(v1, v1), mul(v0, v0)), mul(v2, v2));
        let m = sub(vnorm, along);
        // The original branches on `0.0 < m` with an unordered-compare jump:
        // NaN takes the square-root side, like a positive value.
        if m <= 0.0 { 0.0 } else { m.sqrt() }
    }
});
