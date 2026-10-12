// original: 0x00A848D0 quantize_float_to_2bit

/// Classifies one stack-passed single-precision value using four image
/// thresholds. The first threshold returns category 3 for ordered values
/// above it; exact equality with the second threshold also returns 3. Values
/// at or below the third threshold, including NaNs as the original's
/// unordered `jbe` does, return 1 only when greater than the fourth
/// threshold and otherwise return 0. Remaining ordered values return 2.
/// The comparisons use the original's strictness: only the first condition
/// and the second-threshold equality return category 3.
lf_checker_rt::export!(cdecl, rw_00A848D0(value: f32) -> u32 {
    const HIGH_CUTOFF_VA: u32 = 0x00FE_8B28;
    const SPECIAL_EQUAL_VA: u32 = 0x00FE_8628;
    const LOW_CUTOFF_VA: u32 = 0x00FE_8AFC;
    const SECONDARY_CUTOFF_VA: u32 = 0x00FE_8AB8;

    // These are read-only image values used by the original's comparisons.
    let high_cutoff = unsafe { lf_checker_rt::global::<f32>(HIGH_CUTOFF_VA).read_unaligned() };
    let special_equal = unsafe { lf_checker_rt::global::<f32>(SPECIAL_EQUAL_VA).read_unaligned() };
    let low_cutoff = unsafe { lf_checker_rt::global::<f32>(LOW_CUTOFF_VA).read_unaligned() };
    let secondary_cutoff = unsafe { lf_checker_rt::global::<f32>(SECONDARY_CUTOFF_VA).read_unaligned() };

    if value > high_cutoff {
        return 3;
    }
    if value == special_equal {
        return 3;
    }
    if value.is_nan() || value <= low_cutoff {
        return u32::from(value > secondary_cutoff);
    }
    2
});
