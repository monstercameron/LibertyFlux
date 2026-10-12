// original: 0x00C54F60 initialize_world_pool_record (proposed)
/// Initialize a fixed-size world-pool record. The five stack words provide
/// two dwords, a source pointer, a byte, and a float bit pattern. Three source
/// dwords are copied to `this + 0x50`; the float is stored at `+0x60`; the
/// byte and scalar fields are copied to their recorded offsets; and the
/// sixteen dwords from `+0x08` through `+0x44` are set to positive FLT_MAX.
/// `this` is in ECX, the callee removes all five stack words, and EAX returns
/// the original record pointer.
lf_checker_rt::export!(thiscall, rw_c54f60(this: u32, first_word: u32, source: u32, second_word: u32, flag: u32, scalar_bits: u32) -> u32 {
    unsafe {
        const FIRST_WORD: u32 = 0x00;
        const SECOND_WORD: u32 = 0x04;
        const RESET_WORD: u32 = 0x48;
        const FLAG_BYTE: u32 = 0x4c;
        const SOURCE_COPY: u32 = 0x50;
        const SCALAR: u32 = 0x60;
        const MAX_FINITE: u32 = 0x7f7f_ffff;
        (this.wrapping_add(FIRST_WORD) as *mut u32).write_unaligned(first_word);
        (this.wrapping_add(SECOND_WORD) as *mut u32).write_unaligned(second_word);
        (this.wrapping_add(FLAG_BYTE) as *mut u8).write(flag as u8);
        (this.wrapping_add(RESET_WORD) as *mut u32).write_unaligned(0);
        let source_words = source as *const u32;
        (this.wrapping_add(SOURCE_COPY) as *mut u32).write_unaligned(source_words.read_unaligned());
        (this.wrapping_add(SOURCE_COPY + 4) as *mut u32).write_unaligned(source_words.add(1).read_unaligned());
        (this.wrapping_add(SOURCE_COPY + 8) as *mut u32).write_unaligned(source_words.add(2).read_unaligned());
        (this.wrapping_add(SCALAR) as *mut u32).write_unaligned(scalar_bits);
        let mut offset = 0x08u32;
        while offset <= 0x44 {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(MAX_FINITE);
            offset += 4;
        }
        this
    }
});
