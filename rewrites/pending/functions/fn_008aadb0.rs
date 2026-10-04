// original: 0x008aadb0 audio_lerp_clamped
/// Clamped linear interpolation between two floats.
///
/// Blends `a` towards `b` by `t = (e - c) / (d - c)`, where the factor is
/// 0 when `c >= e` and 1 when `e >= d` (comparisons with `comiss` order and
/// NaN semantics: either side NaN takes the "below" branch). The original
/// also spills the result into its own third stack slot, which a rewrite
/// cannot reproduce; the contract skips the stack comparison and the value
/// is verified exactly through the ST0 return channel.
export!(cdecl, rw_008aadb0(a: f32, b: f32, c: f32, d: f32, e: f32) -> f32 {
    let t = if !(c >= e) {
        if !(e >= d) {
            (e - c) / (d - c)
        } else {
            1.0
        }
    } else {
        0.0
    };
    (b - a) * t + a
});
