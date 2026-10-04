// original: 0x00b39610 task_blend_accumulate (proposed)

/// Accumulate a task blend into `obj`: scale the counter global through the
/// two factor globals and the bias global (float order pinned to the
/// original's), feed the result to the two evaluator callees (which take it
/// in the vector register and return floats the same way; the contract moves
/// the value across on the stack and compares the register contents), then
/// combine the two answers with `rate`, the anchor triple at `anchor` and
/// the anchor address reinterpreted as a float, storing the triple into
/// `obj` plus the anchor, and the scaled delta into `out`. The computed
/// factor is also spilled over the incoming `obj` argument slot, which the
/// contract's stack check therefore ignores. Returns `out`, matching the
/// original's exit register. Original: 0x00b39610 (cdecl, four stack words:
/// anchor, rate bits, obj, out).
lf_checker_rt::export!(cdecl, rw_00b39610(
    anchor: u32,
    rate_bits: u32,
    obj: u32,
    out: u32,
) -> u32 {
    unsafe {
        const EVAL_A: u32 = 1;
        const EVAL_B: u32 = 2;
        const COUNTER: u32 = 0x016624b4;
        const FACTOR_A: u32 = 0x00fe8790;
        const FACTOR_B: u32 = 0x00fe8aec;
        const BIAS: u32 = 0x016624b8;
        const DELTA_SCALE: u32 = 0x00fe8830;
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
        let counter =
            lf_checker_rt::global::<u32>(COUNTER).read() as i32 as f32;
        let fa = f32::from_bits(lf_checker_rt::global::<u32>(FACTOR_A).read());
        let fb = f32::from_bits(lf_checker_rt::global::<u32>(FACTOR_B).read());
        let bias = f32::from_bits(lf_checker_rt::global::<u32>(BIAS).read());
        let factor = add(mul(mul(counter, fa), fb), bias);
        let rate = f32::from_bits(rate_bits);
        (obj as *mut u32).write_unaligned(0);
        ((obj + 4) as *mut u32).write_unaligned(0x3f80_0000);
        ((obj + 8) as *mut u32).write_unaligned(0);
        let answer_a = f32::from_bits(lf_checker_rt::callee_cdecl!(
            EVAL_A,
            u32,
            factor.to_bits()
        ));
        let answer_b = f32::from_bits(lf_checker_rt::callee_cdecl!(
            EVAL_B,
            u32,
            factor.to_bits()
        ));
        let anchor_ptr_bits = f32::from_bits(anchor);
        let anchor0 =
            f32::from_bits((anchor as *const u32).read_unaligned());
        let anchor1 =
            f32::from_bits(((anchor + 4) as *const u32).read_unaligned());
        let anchor2 =
            f32::from_bits(((anchor + 8) as *const u32).read_unaligned());
        let scale =
            f32::from_bits(lf_checker_rt::global::<u32>(DELTA_SCALE).read());
        let delta = mul(sub(rate, anchor_ptr_bits), scale);
        let mix = add(delta, anchor_ptr_bits);
        let zero = 0.0f32;
        let x = mul(sub(mul(answer_b, zero), answer_a), mix);
        let y = mul(add(mul(answer_a, zero), answer_b), mix);
        let z = mul(mix, zero);
        (obj as *mut u32).write_unaligned(add(anchor0, x).to_bits());
        ((obj + 4) as *mut u32).write_unaligned(add(anchor1, y).to_bits());
        ((obj + 8) as *mut u32).write_unaligned(add(anchor2, z).to_bits());
        (out as *mut u32).write_unaligned(delta.to_bits());
        out
    }
});
