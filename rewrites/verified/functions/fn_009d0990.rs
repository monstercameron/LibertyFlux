// original: 0x009d0990 is_null_or_zero_word

/// Return one only when the pointer is nonnull and its first 32-bit word is zero; null and nonzero values return zero.
lf_checker_rt::export!(cdecl, rw_009d0990(value: u32) -> u32 {
    if value == 0 {
        return 0;
    }
    let value = value as *const u32;
    u32::from(unsafe { value.read_unaligned() == 0 })
});
