// original: 0x00DA23F0 set_stream_record_metadata
/// Stores a 32-bit leading field, five low-byte flags, and nine 32-bit metadata words, returning the last word.
lf_checker_rt::export!(thiscall, rw_s675_00da23f0(this: u32, leading_word: u32, flag_a: u32, flag_b: u32, flag_c: u32, flag_d: u32, flag_e: u32, word_a: u32, word_b: u32, word_c: u32, word_d: u32, word_e: u32, word_f: u32, word_g: u32, word_h: u32, word_i: u32) -> u32 {
    const LEADING_WORD: u32 = 0;
    const FLAG_BASE: u32 = 4;
    const WORD_BASE: u32 = 12;
    unsafe {
        (this.wrapping_add(LEADING_WORD) as *mut u32).write_unaligned(leading_word);
        (this.wrapping_add(FLAG_BASE) as *mut u8).write(flag_a as u8);
        (this.wrapping_add(FLAG_BASE + 1) as *mut u8).write(flag_b as u8);
        (this.wrapping_add(FLAG_BASE + 2) as *mut u8).write(flag_c as u8);
        (this.wrapping_add(FLAG_BASE + 3) as *mut u8).write(flag_d as u8);
        (this.wrapping_add(FLAG_BASE + 4) as *mut u8).write(flag_e as u8);
        for (index, value) in [word_a, word_b, word_c, word_d, word_e, word_f, word_g, word_h, word_i].into_iter().enumerate() {
            (this.wrapping_add(WORD_BASE + index as u32 * 4) as *mut u32).write_unaligned(value);
        }
    }
    word_i
});
