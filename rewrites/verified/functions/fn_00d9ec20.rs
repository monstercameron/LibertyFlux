// original: 0x00D9EC20 stream_entry_word
/// Reads the 32-bit word at offset 0x98 plus 16 bytes per signed-independent slot index. The index multiplication wraps as a 32-bit unsigned offset.
lf_checker_rt::export!(thiscall, rw_s675_00d9ec20(this: u32, slot_index: u32) -> u32 {
    const WORD_FIELD: u32 = 0x98;
    const ENTRY_STRIDE: u32 = 16;
    let address = this.wrapping_add(WORD_FIELD).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe { (address as *const u32).read_unaligned() }
});
