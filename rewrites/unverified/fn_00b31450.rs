// original: 0x00B31450 is_signed_value_greater

/// Compare two signed 32-bit values and return one only when the first is
/// strictly greater. Equal inputs and all remaining orderings return zero.
/// The original uses a caller-cleaned cdecl frame with two word arguments.
lf_checker_rt::export!(cdecl, rw_b31450(left: u32, right: u32) -> u32 {
    u32::from((left as i32) > (right as i32))
});
