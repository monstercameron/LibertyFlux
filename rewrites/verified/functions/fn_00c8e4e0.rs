// original: 0x00c8e4e0 fn_00c8e4e0_classify_motion_speed

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

/// Classify a floating-point speed against two read-only game thresholds.
/// COMISS leaves an unordered comparison on the non-early path and the second
/// SETBE maps it to class three; values below the lower threshold map to zero.
const SPEED_LOW_SOURCE: u32 = 0x00FE891C;
const SPEED_HIGH_SOURCE: u32 = 0x00FE893C;

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

unsafe fn classify_motion_speed(input_bits: u32, wrong_lower_bound: bool) -> u32 {
    let input = f32::from_bits(input_bits);
    let lower = unsafe { read_global_f32(SPEED_LOW_SOURCE) };
    let upper = unsafe { read_global_f32(SPEED_HIGH_SOURCE) };
    if if wrong_lower_bound { lower >= input } else { lower > input } {
        0
    } else if upper <= input || input.is_nan() || upper.is_nan() {
        3
    } else {
        2
    }
}

lf_checker_rt::export!(cdecl, rw_00c8e4e0_classify_motion_speed(input_bits: u32) -> u32 {
    unsafe { classify_motion_speed(input_bits, false) }
});
