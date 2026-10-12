// original: 0x00B53320 swap_dword_values

/// Swap the 32-bit words addressed by two cdecl pointer arguments.
///
/// The function reads both pointed-to values before writing either one, then
/// stores the former right-hand value through `left` and the former
/// left-hand value through `right`. The machine return register also retains
/// the former right-hand word; the rewrite preserves that observable value.
lf_checker_rt::export!(cdecl, rw_b53320(left: *mut u32, right: *mut u32) -> u32 {
    unsafe {
        let right_value = right.read_unaligned();
        let left_value = left.read_unaligned();
        left.write_unaligned(right_value);
        right.write_unaligned(left_value);
        right_value
    }
});
