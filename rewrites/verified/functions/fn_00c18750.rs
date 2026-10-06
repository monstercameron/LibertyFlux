// original: 0x00c18750 clamped_scale_complement_st0

/// Map a scalar through a clamped, scaled helper call, returning ST0.
///
/// For `arg <= 0` (or NaN, which takes the same unordered jump) the result is
/// `1.0 - 1.0 = 0.0`; for `arg >= 1.0` it is `1.0 - 0.0 = 1.0`; between the
/// two the argument is scaled by pi then one half (in that order) and passed
/// in XMM0 to a helper returning a float in XMM0, and the result is
/// `1.0 - answer`. The original spills the result over its own argument slot
/// before loading it onto the x87 stack; the contract switches the stack
/// check off and observes the value through ST0 instead. Returns the f32
/// widened to f64, as the checker's ST0 proofs do.
///
/// Original: 0x00C18750 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c18750(arg_bits: u32) -> f64 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) * core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) + core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        const HELPER: u32 = 1;
        let one = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
        let arg = f32::from_bits(arg_bits);
        let r = if !(arg > 0.0) {
            one - one
        } else if !(one > arg) {
            one - 0.0
        } else {
            let pi = lf_checker_rt::global::<f32>(0x00fe_8aa0).read();
            let half = lf_checker_rt::global::<f32>(0x00fe_8830).read();
            let scaled = fmul(fmul(arg, pi), half);
            let ans: u32 = lf_checker_rt::callee_cdecl!(HELPER, u32, scaled.to_bits());
            one - f32::from_bits(ans)
        };
        r as f64
    }
});
