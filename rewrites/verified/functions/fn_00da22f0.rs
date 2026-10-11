// original: 0x00DA22F0 set_stream_sample
/// Writes a byte flag, a 32-bit word, and a float bit pattern into a 12-byte sample record selected by slot index.
lf_checker_rt::export!(thiscall, rw_s675_00da22f0(this: u32, slot_index: u32, flag_word: u32, payload: u32, value_bits: u32) -> u32 {
    const TABLE_BASE: u32 = 0xC4;
    const ENTRY_STRIDE: u32 = 12;
    let entry = this.wrapping_add(TABLE_BASE).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe {
        (entry as *mut u8).write(flag_word as u8);
        (entry.wrapping_add(4) as *mut u32).write_unaligned(payload);
        (entry.wrapping_add(8) as *mut u32).write_unaligned(value_bits);
    }
    0
});
