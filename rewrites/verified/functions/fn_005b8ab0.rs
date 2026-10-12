// original: 0x005B8AB0 bitset_has_index

/// Test one bit in a 64-bit flag set stored as two consecutive 32-bit words.
///
/// `this` points to the low word followed by the high word, and the stack
/// argument is an unsigned bit index. Indices 0 through 31 select the low
/// word, 32 through 63 select the high word, and larger indices return false.
/// Both words are read before the range result is selected, matching the
/// original thiscall method's observable access and fault behavior.
lf_checker_rt::export!(thiscall, rw_005b8ab0(this: u32, bit_index: u32) -> u32 {
    const WORD_BITS: u32 = 32;
    const FLAG_WIDTH: u32 = 64;
    const HIGH_WORD_OFFSET: u32 = 4;

    unsafe {
        let low_word = (this as *const u32).read_unaligned();
        let high_word = (this.wrapping_add(HIGH_WORD_OFFSET) as *const u32).read_unaligned();

        let selected_word = if bit_index < WORD_BITS {
            low_word
        } else if bit_index < FLAG_WIDTH {
            high_word
        } else {
            return 0;
        };
        let bit_position = bit_index & (WORD_BITS - 1);
        u32::from((selected_word & (1u32 << bit_position)) != 0)
    }
});
