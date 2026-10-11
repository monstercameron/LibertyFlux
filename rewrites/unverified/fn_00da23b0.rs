// original: 0x00DA23B0 set_stream_record_fields
/// Stores two byte-sized values and five 32-bit fields at record offsets 0 through 0x18, returning the final field.
lf_checker_rt::export!(thiscall, rw_s675_00da23b0(this: u32, first_byte: u32, first_word: u32, second_byte: u32, word_c: u32, word_d: u32, word_e: u32, word_f: u32) -> u32 {
    const FIRST_BYTE: u32 = 0;
    const FIRST_WORD: u32 = 4;
    const SECOND_BYTE: u32 = 8;
    const WORD_C: u32 = 12;
    const WORD_D: u32 = 16;
    const WORD_E: u32 = 20;
    const WORD_F: u32 = 24;
    unsafe {
        (this.wrapping_add(FIRST_BYTE) as *mut u8).write(first_byte as u8);
        (this.wrapping_add(FIRST_WORD) as *mut u32).write_unaligned(first_word);
        (this.wrapping_add(SECOND_BYTE) as *mut u8).write(second_byte as u8);
        (this.wrapping_add(WORD_C) as *mut u32).write_unaligned(word_c);
        (this.wrapping_add(WORD_D) as *mut u32).write_unaligned(word_d);
        (this.wrapping_add(WORD_E) as *mut u32).write_unaligned(word_e);
        (this.wrapping_add(WORD_F) as *mut u32).write_unaligned(word_f);
    }
    word_f
});
