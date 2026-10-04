// original: 0x00b79210 float_bucket_3_1p5_0
/// Bucket a float into one of four bands.
///
/// Returns 4 for values at or above 3.0, 3 for values at or above 1.5,
/// 2 for values strictly above 0.0, and 1 otherwise (including NaN).
export!(cdecl, rw_00b79210(x: f32) -> u32 {
    if x >= 3.0 {
        4
    } else if x >= 1.5 {
        3
    } else if x > 0.0 {
        2
    } else {
        1
    }
});
