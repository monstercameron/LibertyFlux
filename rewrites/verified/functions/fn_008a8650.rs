// original: 0x008a8650 audio_scale_and_forward_or_zero
/// Shape a control value and forward it to the audio worker, or return zero.
///
/// A null object or an exactly-zero rate returns 0.0. Otherwise the offset
/// input picks between itself and a rate-scaled curve, the rate picks
/// between that and a direct scaling, and the result goes to the worker
/// with the object as `this`; the worker's float answer is the return value.
/// NaN rates and offsets follow the unordered-comparison paths instead of
/// the zero shortcuts.
export!(stdcall, rw_008a8650(a: u32, b: f32, c: f32) -> f64 {
    unsafe {
        if a == 0 || b == 0.0 {
            return 0.0;
        }
        const ONE: f32 = 1.0;
        const FIVE: f32 = 5.0;
        let rate = ONE / b;
        let shifted = c - FIVE;
        let scaled = rate * c;
        let mut shaped = if !(shifted >= 0.0) {
            c
        } else {
            shifted * rate + FIVE
        };
        if !((b - ONE) >= 0.0) {
            shaped = scaled;
        }
        callee_thiscall!(1, f64, a, shaped.to_bits())
    }
});

