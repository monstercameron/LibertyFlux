// original: 0x00D9F060 stream_sample_word
/// Reads the 32-bit word field at offset 0xC8 in a 12-byte record table.
lf_checker_rt::export!(thiscall, rw_s675_00d9f060(this: u32, slot_index: u32) -> u32 {
    const WORD_TABLE: u32 = 0xC8;
    const ENTRY_STRIDE: u32 = 12;
    let address = this.wrapping_add(WORD_TABLE).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe { (address as *const u32).read_unaligned() }
});
