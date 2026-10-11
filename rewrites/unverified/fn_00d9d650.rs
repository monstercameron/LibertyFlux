// original: 0x00D9D650 degrees_to_radians_with_boundary_values

/// Convert degrees to radians, preserving the game's exact boundary cases:
/// exactly +180 degrees returns +pi and exactly -180 returns -pi. Every other
/// input, including signed zero, infinities and NaNs, is multiplied by the
/// radians-per-degree constant in that operand order. The result is returned
/// on the x87 floating-point stack.

lf_checker_rt::export!(cdecl, rw_00D9D650(degrees: f32) -> f32 {
    const POSITIVE_HALF_TURN: f32 = f32::from_bits(0x4334_0000);
    const NEGATIVE_HALF_TURN: f32 = f32::from_bits(0xC334_0000);
    const POSITIVE_PI: f32 = f32::from_bits(0x4049_0FDB);
    const NEGATIVE_PI: f32 = f32::from_bits(0xC049_0FDB);
    const RADIANS_PER_DEGREE: f32 = f32::from_bits(0x3C8E_FA35);

    if degrees == POSITIVE_HALF_TURN {
        POSITIVE_PI
    } else if degrees == NEGATIVE_HALF_TURN {
        NEGATIVE_PI
    } else {
        core::hint::black_box(degrees) * core::hint::black_box(RADIANS_PER_DEGREE)
    }
});
