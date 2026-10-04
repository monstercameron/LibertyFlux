// original: 0x009e9900 ped_flag_bits_set
/// Test two flag bits in the word at field `0x20` of a ped-adjacent object.
///
/// Returns nonzero (1) only when both bit 5 and bit 12 of the word are set,
/// 0 otherwise. The full `eax` value keeps the original's shape: the word
/// shifted right by 5 with its low byte replaced by the boolean result.
export!(thiscall, rw_009e9900(this_ptr: u32) -> u32 {
    unsafe {
        let word = *((this_ptr + 0x20) as *const u32);
        let both_set = (word >> 5) & (word >> 12) & 1;
        ((word >> 5) & 0xffffff00) | both_set
    }
});
