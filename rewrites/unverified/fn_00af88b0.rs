// original: 0x00AF88B0 veh_side_tests_2d (proposed)

/// Two chained 2D side tests over eight floats, returning a boolean.
///
/// From the arguments `(a0..a7)` the first test forms differences
/// `t3 = a4 - a0` and `t2 = a5 - a1`, then
/// `p1 = ((t3 + a6)*a3 - (t2 + a7)*a2) * ((t3*a3) - (t2*a2))`.
/// A strictly positive `p1` returns 0 at once. Otherwise the second test
/// mirrors the first with negated differences `s0 = a0 - a4`, `s1 = a1 - a5`:
/// `p2 = ((s0 + a2)*a7 - (s1 + a3)*a6) * ((s0*a7) - (s1*a6))`,
/// and the result is 1 unless `p2` is strictly positive. NaN never counts
/// as positive, matching the original's ordered-compare jumps. Every float
/// operation runs in the original's operand order with pinned order.
///
/// Original: 0x00AF88B0 (cdecl, eight stack arguments, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF88B0(a0: f32, a1: f32, a2: f32, a3: f32, a4: f32, a5: f32, a6: f32, a7: f32) -> u8 {
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
    let t3 = sub(a4, a0);
    let t2 = sub(a5, a1);
    let p1 = mul(sub(mul(add(t3, a6), a3), mul(add(t2, a7), a2)), sub(mul(t3, a3), mul(t2, a2)));
    if p1 > 0.0 {
        return 0;
    }
    let s0 = sub(a0, a4);
    let s1 = sub(a1, a5);
    let p2 = mul(
        sub(mul(add(s0, a2), a7), mul(add(s1, a3), a6)),
        sub(mul(s0, a7), mul(s1, a6)),
    );
    (!(p2 > 0.0)) as u8
});
