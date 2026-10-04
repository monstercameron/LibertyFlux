// original: 0x008ad700 audio_curve_eval_a
/// Audio curve evaluator A: squared rational response in `x`.
///
/// With `p = this+4` and `q = this+8`: builds `num = p*x + 1`, `base =
/// 1/(p + 1)`, squares both, then returns `1/num^2` for nonnegative `q` and
/// `(1/num^2 - base^2)/(1 - base^2)` for negative or unordered `q`.
export!(thiscall, rw_008ad700(this_: *const u8, x: f32) -> f32 {
    unsafe {
        const ONE: f32 = 1.0;
        let p = *(this_.add(4) as *const f32);
        let q = *(this_.add(8) as *const f32);
        let num = p * x + ONE;
        let base = ONE / (p + ONE);
        let base2 = base * base;
        let num2 = num * num;
        if !(q >= 0.0) {
            let t = ONE / num2;
            (t - base2) / (ONE - base2)
        } else {
            ONE / num2
        }
    }
});
