// original: 0x00A1EC90 tbd
/// Adjust a pointed-to scalar toward a target selected from two angular
/// inputs, using a double helper result as the blend fraction.
///
/// The routine returns without a helper call when the absolute input
/// difference is within the image threshold or the weight is at most its
/// image cutoff. Otherwise it derives a bounded helper parameter from the
/// excess difference, calls the double helper, and uses the returned value to
/// blend the pointed-to scalar toward the positive target or its negative.
/// It writes only when the current value lies outside that interval.
///
/// The helper receives the image's double constant in XMM0 and a stack float
/// promoted to `f64` in XMM1. The call's vector comparison covers the low
/// eight bytes of each register. The original also uses the second incoming
/// scalar argument as a temporary target slot on the helper path.
///
/// Original: `0x00A1EC90` (stdcall, output pointer and four float words).
lf_checker_rt::export!(stdcall, rw_00a1ec90(output: u32, first_bits: u32, second_bits: u32, alpha_bits: u32, helper_input_bits: u32) -> () {
    unsafe {
        const ABS_MASK_VA: u32 = 0x00FE_8F80;
        const DIFFERENCE_THRESHOLD_VA: u32 = 0x00FE_8978;
        const ALPHA_CUTOFF_VA: u32 = 0x00FE_8764;
        const ONE_VA: u32 = 0x00FE_88E8;
        const EXCESS_SCALE_VA: u32 = 0x00FE_891C;
        const NORMALIZE_SCALE_VA: u32 = 0x00E9_8888;
        const HELPER_SCALE_VA: u32 = 0x00E9_B4E4;
        const HELPER_DOUBLE_VA: u32 = 0x00E9_B4F0;
        const SIGN_MASK_VA: u32 = 0x00FE_8FA0;

        let first = f32::from_bits(first_bits);
        let second = f32::from_bits(second_bits);
        let alpha = f32::from_bits(alpha_bits);
        let difference = sub32(first, second);
        let absolute_mask = lf_checker_rt::global::<u32>(ABS_MASK_VA).read_unaligned();
        let absolute_difference = f32::from_bits(difference.to_bits() & absolute_mask);
        let threshold = lf_checker_rt::global::<f32>(DIFFERENCE_THRESHOLD_VA).read_unaligned();
        if jbe32(absolute_difference, threshold) {
            return;
        }

        let alpha_cutoff = lf_checker_rt::global::<f32>(ALPHA_CUTOFF_VA).read_unaligned();
        if jbe32(alpha, alpha_cutoff) {
            return;
        }

        let excess = sub32(absolute_difference, threshold);
        let excess_scale = lf_checker_rt::global::<f32>(EXCESS_SCALE_VA).read_unaligned();
        let normalized_scale = lf_checker_rt::global::<f32>(NORMALIZE_SCALE_VA).read_unaligned();
        let scaled_excess = mul32(excess, excess_scale);
        let normalized_excess = mul32(scaled_excess, normalized_scale);
        let one = lf_checker_rt::global::<f32>(ONE_VA).read_unaligned();
        let capped_excess = if jbe32(normalized_excess, one) {
            normalized_excess
        } else {
            one
        };
        let helper_scale = lf_checker_rt::global::<f32>(HELPER_SCALE_VA).read_unaligned();
        let helper_parameter = mul32(capped_excess, helper_scale);
        let target = sub32(threshold, helper_parameter);

        let helper_constant = lf_checker_rt::global::<f64>(HELPER_DOUBLE_VA).read_unaligned();
        let helper_input = f64::from(f32::from_bits(helper_input_bits));
        let helper_constant_bits = helper_constant.to_bits();
        let helper_input_double_bits = helper_input.to_bits();
        let blend_bits: u64 = lf_checker_rt::callee_cdecl!(
            1,
            u64,
            helper_constant_bits as u32,
            (helper_constant_bits >> 32) as u32,
            helper_input_double_bits as u32,
            (helper_input_double_bits >> 32) as u32,
        );
        let blend_as_double = f64::from_bits(blend_bits);
        let blend = core::hint::black_box(blend_as_double) as f32;

        let output_pointer = output as *mut f32;
        let current = output_pointer.read_unaligned();
        if jbe32(current, target) {
            let sign_mask = lf_checker_rt::global::<u32>(SIGN_MASK_VA).read_unaligned();
            let negative_target = f32::from_bits(target.to_bits() ^ sign_mask);
            if jbe32(negative_target, current) {
                return;
            }

            let one_minus_blend = sub32(one, blend);
            let weighted_current = mul32(current, blend);
            let weighted_target = mul32(one_minus_blend, target);
            let adjusted = sub32(weighted_current, weighted_target);
            output_pointer.write_unaligned(adjusted);
        } else {
            let one_minus_blend = sub32(one, blend);
            let weighted_current = mul32(current, blend);
            let weighted_target = mul32(one_minus_blend, target);
            let adjusted = add32(weighted_target, weighted_current);
            output_pointer.write_unaligned(adjusted);
        }
    }
});

#[inline(never)]
fn add32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(never)]
fn sub32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn mul32(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(always)]
fn jbe32(left: f32, right: f32) -> bool {
    left.is_nan() || right.is_nan() || left <= right
}
