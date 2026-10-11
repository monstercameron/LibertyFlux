// original: 0x00C5D420 wrap_phase_clamp_normalize

/// Converts an angle into a normalized phase, constrains that phase to a
/// requested interval, writes the resulting fraction, and returns the
/// original low-byte adjustment flag. The seven cdecl stack words are
/// `(unused, angle_bits, unused, unused, output, lower_bits, upper_bits)`;
/// only the angle, output pointer, and two bounds are read. The angle is
/// scaled by the exact single-precision reciprocal of one turn. Bounds in
/// ascending order use a linear interval; descending bounds describe an
/// interval that crosses the phase boundary. Out-of-range values choose the
/// nearer endpoint and set the flag. Comparisons preserve x86 `COMISS`
/// unordered behavior, and arithmetic retains the original operand order.
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_c5d420(
    _unused_first: u32,
    angle_bits: u32,
    _unused_third: u32,
    _unused_fourth: u32,
    output_address: u32,
    lower_bits: u32,
    upper_bits: u32,
) -> u32 {
    // Keep the runtime's patched mirror symbols in this DLL even though this
    // leaf has no intercepted calls or register-entry reads.
    let _runtime_exports = core::hint::black_box((
        core::ptr::addr_of!(lf_checker_rt::CHECKER_XBASE),
        core::ptr::addr_of!(lf_checker_rt::CHECKER_CTABLE),
        core::ptr::addr_of!(lf_checker_rt::CHECKER_XMM),
        core::ptr::addr_of!(lf_checker_rt::CHECKER_TLS),
    ));

    const ONE: f32 = f32::from_bits(0x3f80_0000);
    const INV_TURN: f32 = f32::from_bits(0x3e22_f983);
    const NEG_INV_TURN: f32 = f32::from_bits(0xbe22_f983);
    const HALF: f32 = f32::from_bits(0x3f00_0000);
    const SIGN_MASK: u32 = 0x8000_0000;

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn div(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) / core::hint::black_box(right)
    }
    #[inline(always)]
    fn comiss_jb(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left < right
    }
    #[inline(always)]
    fn comiss_jbe(left: f32, right: f32) -> bool {
        left.is_nan() || right.is_nan() || left <= right
    }
    #[inline(always)]
    fn comiss_ja(left: f32, right: f32) -> bool {
        !left.is_nan() && !right.is_nan() && left > right
    }
    #[inline(always)]
    fn flip_sign(value: f32) -> f32 {
        f32::from_bits(value.to_bits() ^ SIGN_MASK)
    }
    #[inline(always)]
    unsafe fn store_f32(address: u32, value: f32) {
        unsafe { (address as *mut u32).write_unaligned(value.to_bits()) };
    }
    unsafe {
        let angle = f32::from_bits(angle_bits);
        let lower = f32::from_bits(lower_bits);
        let upper = f32::from_bits(upper_bits);
        let mut phase = if comiss_jb(angle, 0.0) {
            mul(angle, NEG_INV_TURN)
        } else {
            sub(ONE, mul(angle, INV_TURN))
        };
        store_f32(output_address, phase);
        let mut adjusted = false;

        if !comiss_jbe(lower, upper) {
            // The interval crosses the phase boundary. Values in either
            // included segment are retained; values in the gap snap to the
            // nearer boundary, with ties selecting the upper boundary.
            let below_lower = comiss_jbe(lower, phase);
            let above_upper = comiss_jbe(phase, upper);
            if !below_lower && !above_upper {
                let lower_delta = sub(phase, lower);
                let lower_distance = if comiss_jbe(0.0, lower_delta) {
                    lower_delta
                } else {
                    flip_sign(lower_delta)
                };
                let upper_delta = sub(phase, upper);
                let upper_distance = if comiss_jbe(0.0, upper_delta) {
                    upper_delta
                } else {
                    flip_sign(upper_delta)
                };
                if comiss_jbe(upper_distance, lower_distance) {
                    phase = upper;
                } else {
                    phase = lower;
                }
                store_f32(output_address, phase);
                adjusted = true;
            }

            if comiss_jb(phase, lower) {
                let numerator = add(sub(ONE, lower), phase);
                let denominator = sub(ONE, sub(lower, upper));
                phase = div(numerator, denominator);
            } else {
                let numerator = sub(phase, lower);
                let denominator = sub(ONE, sub(lower, upper));
                phase = div(numerator, denominator);
            }
            store_f32(output_address, phase);
        } else {
            // The ordinary interval is ascending. Values outside its ends
            // snap to the nearer endpoint across the unused part of a turn.
            let width = sub(upper, lower);
            let complement = sub(ONE, width);
            if !comiss_jbe(lower, phase) {
                let half_gap = mul(complement, HALF);
                let distance = sub(lower, phase);
                adjusted = true;
                if comiss_ja(distance, half_gap) {
                    phase = upper;
                    store_f32(output_address, phase);
                } else {
                    let zero = sub(lower, lower);
                    phase = div(zero, width);
                    store_f32(output_address, phase);
                    return (output_address & 0xffff_ff00) | u32::from(adjusted);
                }
            } else if !comiss_jbe(phase, upper) {
                let half_gap = mul(complement, HALF);
                let distance = sub(phase, upper);
                adjusted = true;
                if comiss_ja(distance, half_gap) {
                    let zero = sub(lower, lower);
                    phase = div(zero, width);
                    store_f32(output_address, phase);
                    return (output_address & 0xffff_ff00) | u32::from(adjusted);
                }
                phase = upper;
                store_f32(output_address, phase);
            }

            let numerator = sub(phase, lower);
            phase = div(numerator, width);
            store_f32(output_address, phase);
        }

        (output_address & 0xffff_ff00) | u32::from(adjusted)
    }
}
