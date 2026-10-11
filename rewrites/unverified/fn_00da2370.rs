// original: 0x00DA2370 copy_stream_payload
/// Copies a byte, a 32-bit field, and three eight-byte source blocks into the record, then returns the source pointer.
lf_checker_rt::export!(thiscall, rw_s675_00da2370(this: u32, flag_word: u32, field_word: u32, source: u32) -> u32 {
    const FLAG: u32 = 0;
    const SOURCE_FIELD: u32 = 4;
    const PAYLOAD: u32 = 8;
    unsafe {
        (this.wrapping_add(FLAG) as *mut u8).write(flag_word as u8);
        (this.wrapping_add(SOURCE_FIELD) as *mut u32).write_unaligned(field_word);
        for word in 0..6u32 {
            let value = (source.wrapping_add(word * 4) as *const u32).read_unaligned();
            (this.wrapping_add(PAYLOAD + word * 4) as *mut u32).write_unaligned(value);
        }
    }
    source
});
