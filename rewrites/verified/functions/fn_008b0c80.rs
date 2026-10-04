// original: 0x008b0c80 audio_poly_eval_clamped
/// Polynomial evaluator with range selection and output clamps.
///
/// Combines three inputs through a reciprocal stage, evaluates a degree-7
/// polynomial in the intermediate value, and clamps: non-positive adjusted
/// inputs yield zero and large inputs saturate at -100.
export!(stdcall, rw_008b0c80(_a: u32, b: f32, c: f32, d: f32) -> f64 {
    unsafe {
        let mut x0 = b * d;
        let one = *global::<f32>(0xFE88E8);
        let x4 = c;
        let five = *global::<f32>(0xFE8AD8);
        let x1 = one / x0;
        let mut x3 = x4 - five;
        let zero = 0.0f32;
        let x2 = x1 * x4;
        if x3 >= 0.0 {
            x3 = x3 * x1;
            x3 = x3 + five;
        } else {
            x3 = x4;
        }
        x0 = x0 - one;
        if !(x0 >= 0.0) {
            x3 = x2;
        }
        let mut px2 = x3 - *global::<f32>(0xE7C9FC);
        px2 = px2 * *global::<f32>(0xE7C9D8);
        let mut px4 = px2 * px2;
        let mut px0 = px2 * *global::<f32>(0xE7CA10);
        let mut px1 = px4 * *global::<f32>(0xE7C9F0);
        px0 = px0 - *global::<f32>(0xE7C9F8);
        px4 = px4 * px2;
        px1 = px1 + px0;
        px0 = px4 * *global::<f32>(0xE7CA0C);
        px4 = px4 * px2;
        px0 = px0 + px1;
        px1 = px4 * *global::<f32>(0xE7C9E0);
        px4 = px4 * px2;
        px1 = px1 + px0;
        px0 = px4 * *global::<f32>(0xE7C9E4);
        px4 = px4 * px2;
        px0 = px0 + px1;
        px1 = px4 * *global::<f32>(0xE7CA04);
        px4 = px4 * px2;
        px1 = px1 + px0;
        px0 = px4 * *global::<f32>(0xE7CA08);
        px4 = px4 * px2;
        px0 = px0 + px1;
        px4 = px4 * *global::<f32>(0xE7C9DC);
        px4 = px4 + px0;
        let t0 = x3 - *global::<f32>(0xE7C9F4);
        if !(t0 >= 0.0) {
            px4 = zero;
        }
        let x3b = x3 - *global::<f32>(0xE7CA00);
        if !(x3b >= 0.0) {
            px4 as f64
        } else {
            f32::from_bits(0xC2C80000u32) as f64
        }
    }
});
