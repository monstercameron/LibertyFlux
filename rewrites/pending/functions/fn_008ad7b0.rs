// original: 0x008ad7b0 audio_curve_eval_b
/// Audio curve evaluator B: plain rational response in `x`.
///
/// The unsquared twin of evaluator A: `num = p*x + 1`, `base = 1/(p + 1)`,
/// returning `1/num` for nonnegative `q` and `(1/num - base)/(1 - base)` for
/// negative or unordered `q`.
export!(thiscall, rw_008ad7b0(this_: *const u8, x: f32) -> f32 {
    unsafe {
        const ONE: f32 = 1.0;
        let p = *(this_.add(4) as *const f32);
        let q = *(this_.add(8) as *const f32);
        let num = p * x + ONE;
        let base = ONE / (p + ONE);
        if !(q >= 0.0) {
            let t = ONE / num;
            (t - base) / (ONE - base)
        } else {
            ONE / num
        }
    }
});
