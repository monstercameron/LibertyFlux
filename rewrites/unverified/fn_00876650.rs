// original: 0x00876650 crmt_blend_weight_clamp

/// Clamp a request weight stored at +0x1c to the inclusive range from zero
/// through the static maximum weight. The thiscall receiver is in ECX and
/// the stack argument is the incoming f32 bit pattern. Negative values,
/// including negative infinity, become positive zero; values above the
/// maximum become that maximum. Negative zero and NaNs retain their input
/// bits because neither ordered comparison selects a clamp branch. The
/// function has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00876650(this: u32, weight_bits: u32) -> () {
    const WEIGHT_FIELD: u32 = 0x1c;
    const MAX_WEIGHT_GLOBAL_VA: u32 = 0x00fe88e8;

    unsafe {
        let maximum_bits = lf_checker_rt::global::<u32>(MAX_WEIGHT_GLOBAL_VA).read_unaligned();
        let weight = f32::from_bits(weight_bits);
        let maximum = f32::from_bits(maximum_bits);
        let result_bits = if weight < 0.0 {
            0.0f32.to_bits()
        } else if weight > maximum {
            maximum_bits
        } else {
            weight_bits
        };
        ((this + WEIGHT_FIELD) as *mut u32).write_unaligned(result_bits);
    }
});
