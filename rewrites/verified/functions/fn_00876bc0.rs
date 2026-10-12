// original: 0x00876BC0 normalize_angle_radians

/// Normalize the angle at `this + 0x14` into the half-open interval [lower, upper).
///
/// The lower bound, period, and upper bound are read from the referenced image
/// data words. Values below the lower bound receive repeated period additions;
/// values at or above the upper bound receive repeated period subtractions.
/// A NaN passes through unchanged. Arithmetic uses the original operand order.
///
/// Original calling convention: thiscall with no stack arguments; the float
/// result is returned in XMM0. The proof tests finite inputs only because the
/// original loop does not terminate on either infinity.
lf_checker_rt::export!(thiscall, rw_00876bc0(this: u32) -> core::arch::x86::__m128 {
    const INPUT_OFFSET: u32 = 0x14;
    const LOWER_BOUND_VA: u32 = 0x00fe8dc4;
    const PERIOD_VA: u32 = 0x00fe8aec;
    const UPPER_BOUND_VA: u32 = 0x00fe8aa0;
    unsafe {
        let input_bits = (this.wrapping_add(INPUT_OFFSET) as *const u32).read_unaligned();
        let mut value = f32::from_bits(input_bits);
        let lower = f32::from_bits(lf_checker_rt::global::<u32>(LOWER_BOUND_VA).read_unaligned());
        let period = f32::from_bits(lf_checker_rt::global::<u32>(PERIOD_VA).read_unaligned());
        let upper = f32::from_bits(lf_checker_rt::global::<u32>(UPPER_BOUND_VA).read_unaligned());
        if !value.is_nan() {
            while value < lower {
                value = core::hint::black_box(value) + core::hint::black_box(period);
            }
            while value >= upper {
                value = core::hint::black_box(value) - core::hint::black_box(period);
            }
        }
        core::mem::transmute::<[f32; 4], core::arch::x86::__m128>([value, 0.0, 0.0, 0.0])
    }
});
