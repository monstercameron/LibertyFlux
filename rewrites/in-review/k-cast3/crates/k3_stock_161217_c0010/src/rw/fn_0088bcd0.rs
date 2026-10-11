// original: 0x0088bcd0 audio_scaled_product_to_int
/// Scales an unsigned product by 0.001 and takes the floor.
///
/// Computes `p = (float)a * (float)b * 0.001`, rounds with the 2^23
/// add/subtract trick, subtracts 1 when the rounded value overshoots `p`
/// (i.e. `floor(p)`), and truncates toward zero. The result always fits,
/// so no saturation or indefinite value can occur.
export!(cdecl, rw_0088bcd0(a: u32, b: u32) -> u32 {
    let mut p = (a as f32) * (b as f32);
    p *= 0.001f32;
    let q: f32 = if p < 8388608.0 { 8388608.0 } else { 0.0 };
    let r = (p + q) - q;
    let frac = r - p;
    // CMPNLESS is "not less-or-equal": adjust only when r is strictly above p.
    let adj: f32 = if frac > 0.0 { 1.0 } else { 0.0 };
    let y = r - adj;
    (y as i64) as u32
});
