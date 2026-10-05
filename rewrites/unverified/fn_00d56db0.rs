// original: 0x00d56db0 ccam_polar_from_vec

/// Decompose an (x, y, z) vector into a biased pair-op of (x, y) and a
/// pair-op of (hypot(x, y), z).
///
/// `vec` points to three floats. The shared two-float routine (cdecl, two
/// stack arguments, single-precision result in ST0) first runs on (x, y);
/// its result plus the constant `BIAS` (single-precision pi) is stored to
/// `out_biased`. Then the
/// same routine runs on (sqrt(x*x + y*y), z) and its result is stored to
/// `out_plain`. The sum is formed as x*x + y*y and the bias is added as
/// result + BIAS, in the original's operand order. Returns `out_plain`.
///
/// The original spills the first result through its own incoming `vec` slot
/// and reads it back; the slot is popped on return, so no caller can observe
/// the clobber. The rewrite leaves incoming stack intact, and the contract
/// switches the stack check off for that reason (a recorded narrowing).
///
/// Original: 0x00d56db0 (stdcall, three stack arguments, two calls).
lf_checker_rt::export!(stdcall, rw_00d56db0(vec: u32, out_plain: u32, out_biased: u32) -> u32 {
    unsafe {
        /// Read-only bias added to the first result.
        const BIAS: u32 = 0x00fe8aa0;
        /// Shared two-float routine, first call (intercepted; cdecl).
        const PAIR_OP_A: u32 = 1;
        /// Shared two-float routine, second call (intercepted; cdecl).
        const PAIR_OP_B: u32 = 2;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let x = f32::from_bits((vec as *const u32).read_unaligned());
        let y = f32::from_bits(((vec.wrapping_add(4)) as *const u32).read_unaligned());
        let z = f32::from_bits(((vec.wrapping_add(8)) as *const u32).read_unaligned());
        let first: f32 = lf_checker_rt::callee_cdecl!(PAIR_OP_A, f32, x.to_bits(), y.to_bits());
        let bias = f32::from_bits((lf_checker_rt::global::<u32>(BIAS) as *const u32).read_unaligned());
        (out_biased as *mut u32).write_unaligned(add(first, bias).to_bits());
        let hyp = core::hint::black_box(add(mul(x, x), mul(y, y))).sqrt();
        let second: f32 = lf_checker_rt::callee_cdecl!(PAIR_OP_B, f32, hyp.to_bits(), z.to_bits());
        (out_plain as *mut u32).write_unaligned(second.to_bits());
        out_plain
    }
});
