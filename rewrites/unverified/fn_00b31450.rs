// original: 0x00B31450 is_signed_value_greater

/// Compare two signed 32-bit values and return 1 when the first is greater,
/// 0 when equal, and 0xFFFFFFFF when the first is less. The original uses a
/// caller-cleaned cdecl frame with two word arguments.
lf_checker_rt::export!(cdecl, rw_b31450(left: u32, right: u32) -> u32 {
    if (left as i32) > (right as i32) {
        1
    } else if (left as i32) < (right as i32) {
        u32::MAX
    } else {
        0
    }
});
