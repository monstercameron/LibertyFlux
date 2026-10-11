// original: 0x00DA22A0 set_stream_entry
/// Writes a byte flag, a 32-bit word, and two float bit patterns into a 16-byte record selected by slot index.
lf_checker_rt::export!(thiscall, rw_s675_00da22a0(this: u32, slot_index: u32, flag_word: u32, payload: u32, first_value_bits: u32, second_value_bits: u32) -> u32 {
    const TABLE_BASE: u32 = 0x94;
    const ENTRY_STRIDE: u32 = 16;
    let entry = this.wrapping_add(TABLE_BASE).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe {
        (entry as *mut u8).write(flag_word as u8);
        (entry.wrapping_add(4) as *mut u32).write_unaligned(payload);
        (entry.wrapping_add(8) as *mut u32).write_unaligned(first_value_bits);
        (entry.wrapping_add(12) as *mut u32).write_unaligned(second_value_bits);
    }
    0
});
