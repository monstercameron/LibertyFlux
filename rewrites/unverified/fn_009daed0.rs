// original: 0x009DAED0 streaming_set_high_byte_flags

/// OR the top byte of `bits` into the flags word at `owner + 0x40` and return
/// the shifted value in EAX.
/// The function is cdecl with two 32-bit arguments; shifting by 24 discards
/// the input's low 24 bits, and the existing lower 24 flag bits are preserved.
lf_checker_rt::export!(cdecl, rw_009daed0(owner: u32, bits: u32) -> u32 {
    const FLAGS_OFFSET: u32 = 0x40;
    const HIGH_BYTE_SHIFT: u32 = 24;

    unsafe {
        let flags = owner.wrapping_add(FLAGS_OFFSET) as *mut u32;
        let previous = flags.read_unaligned();
        let shifted_bits = bits.wrapping_shl(HIGH_BYTE_SHIFT);
        flags.write_unaligned(previous | shifted_bits);
        shifted_bits
    }
});
