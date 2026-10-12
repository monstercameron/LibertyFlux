// original: 0x00B514E0 smooth_pitch_rate
/// Update the stored pitch toward an angle derived from a vector, clamping
/// the change by the frame-scaled writable rate.
///
/// The routine negates the vector's X component, converts X and Y to doubles,
/// and gets an angle from the double helper. It subtracts the current pitch,
/// sends that difference through the scalar helper, and clamps the result to
/// the positive and negative rate limit. The adjusted pitch is then passed
/// through the scalar helper once more and stored at `this + 0x1498`.
///
/// The global rate is read from writable image data. The double helper takes
/// two `f64` values in XMM0/XMM1 and returns an `f64` in XMM0. Each scalar
/// helper call takes one stack `f32` and returns an `f32` in the x87 channel.
/// The contract scripts both helpers and compares their arguments.
///
/// Original: `0x00B514E0` (thiscall, one vector pointer and one frame float).
lf_checker_rt::export!(thiscall, rw_00b514e0(this: u32, vector: u32, frame_bits: u32) -> () {
    unsafe {
        const CURRENT_PITCH_OFFSET: u32 = 0x1498;
        const SIGN_MASK_VA: u32 = 0x00FE_8FA0;
        const RATE_SCALE_VA: u32 = 0x0117_35BC;

        let sign_mask = lf_checker_rt::global::<u32>(SIGN_MASK_VA).read_unaligned();
        let vector_x = (vector as *const f32).read_unaligned();
        let vector_y = ((vector + 4) as *const f32).read_unaligned();
        let negated_x = f32::from_bits(vector_x.to_bits() ^ sign_mask);
        let x_as_double = f64::from(negated_x).to_bits();
        let y_as_double = f64::from(vector_y).to_bits();
        let angle_bits: u64 = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            x_as_double as u32,
            (x_as_double >> 32) as u32,
            y_as_double as u32,
            (y_as_double >> 32) as u32,
        );
        let angle_as_double = f64::from_bits(angle_bits);
        let angle = core::hint::black_box(angle_as_double) as f32;

        let current_pitch = ((this + CURRENT_PITCH_OFFSET) as *const f32).read_unaligned();
        let difference = sub32(angle, current_pitch);
        let shaped_difference: f32 =
            lf_checker_rt::callee_cdecl!(2, f32, difference.to_bits());

        let frame = f32::from_bits(frame_bits);
        let rate_scale = lf_checker_rt::global::<f32>(RATE_SCALE_VA).read_unaligned();
        let limit = mul32(rate_scale, frame);
        let negative_limit = f32::from_bits(limit.to_bits() ^ sign_mask);
        let adjustment = if ja32(negative_limit, shaped_difference) {
            negative_limit
        } else if jbe32(shaped_difference, limit) {
            shaped_difference
        } else {
            limit
        };

        let advanced_pitch = add32(current_pitch, adjustment);
        let stored_pitch: f32 =
            lf_checker_rt::callee_cdecl!(2, f32, advanced_pitch.to_bits());
        ((this + CURRENT_PITCH_OFFSET) as *mut f32).write_unaligned(stored_pitch);
    }
});

#[inline(never)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(never)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(always)]
fn jbe32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left <= right
}

#[inline(always)]
fn ja32(left: f32, right: f32) -> bool {
    !left.is_nan() && !right.is_nan() && left > right
}
