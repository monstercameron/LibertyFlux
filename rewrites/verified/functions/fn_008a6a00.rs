// original: 0x008a6a00 audio_round_half_away_to_i32
/// Round a float to `i32`, halves away from zero.
///
/// Biases the value by +0.5 (non-negative inputs, including NaN) or -0.5
/// (negative inputs) and truncates toward zero, exactly like `cvttss2si`:
/// NaN, infinities and out-of-range values all yield `i32::MIN`, and values
/// with magnitude below 1 yield 0.
export!(cdecl, rw_008a6a00(x: f32) -> i32 {
    let truncate = |v: f32| -> i32 {
        let bits = v.to_bits();
        let exp = (((bits >> 23) & 0xFF) as i32) - 127;
        if exp < 0 {
            return 0;
        }
        if exp >= 31 {
            return i32::MIN;
        }
        let mantissa = (bits & 0x7F_FFFF) | 0x80_0000;
        let magnitude = if exp >= 23 {
            mantissa << (exp - 23)
        } else {
            mantissa >> (23 - exp)
        };
        if bits & 0x8000_0000 == 0 {
            magnitude as i32
        } else {
            (magnitude as i32).wrapping_neg()
        }
    };
    const HALF: f32 = 0.5;
    if x < 0.0 {
        truncate(x - HALF)
    } else {
        truncate(x + HALF)
    }
});

