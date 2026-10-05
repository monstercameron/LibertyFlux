// original: 0x00AF8660 veh_float_mix2 (proposed)

/// Bilinear-style blend of two axis differences: `(e-a)*c + (f-b)*d`.
///
/// Takes six floats `(a, b, c, d, e, f)` and returns the sum of the two
/// scaled differences. The subtraction, scaling and final addition run in
/// the original's order with pinned operand order, so results agree bit for
/// bit including NaN payloads. The original also spills the result into its
/// incoming fifth-argument stack slot; that scratch write is not modelled
/// (the contract runs with the stack check off).
///
/// Original: 0x00AF8660 (cdecl, six stack arguments, result in ST0).
lf_checker_rt::export!(cdecl, rw_00AF8660(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> f32 {
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
    add(mul(sub(e, a), c), mul(sub(f, b), d))
});
