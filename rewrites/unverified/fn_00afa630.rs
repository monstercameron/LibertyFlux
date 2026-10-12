// original: 0x00AFA630 circular_index_with_direction

/// Return the signed remainder after applying a directional offset to the
/// shared circular range. The one stack argument is a signed step. The
/// function reads the enable byte, signed range length, and signed base value
/// from the game's global state. With the enable byte set it adds the step to
/// the base; otherwise it subtracts the step from the range length and then
/// adds the base. Both intermediate operations wrap as 32-bit integers before
/// signed remainder is computed. A zero divisor, or the signed overflow case
/// `i32::MIN % -1`, is outside the checked inputs because the original faults
/// on those divisions.
lf_checker_rt::export!(cdecl, rw_00afa630(step_bits: u32) -> u32 {
    const RANGE_LENGTH: u32 = 0x01600140;
    const BASE_VALUE: u32 = 0x01600144;
    const ENABLED: u32 = 0x01600148;

    unsafe {
        let enabled = lf_checker_rt::global::<u8>(ENABLED).read_volatile() != 0;
        let range = lf_checker_rt::global::<i32>(RANGE_LENGTH).read_volatile();
        let base = lf_checker_rt::global::<i32>(BASE_VALUE).read_volatile();
        let step = step_bits as i32;
        let numerator = if enabled {
            base.wrapping_add(step)
        } else {
            range.wrapping_sub(step).wrapping_add(base)
        };
        numerator.wrapping_rem(range) as u32
    }
});
