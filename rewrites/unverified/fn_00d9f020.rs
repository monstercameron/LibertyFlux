// original: 0x00D9F020 stream_sample_flag
/// Reads the flag byte from a 12-byte record table at offset 0xC4 and returns it in AL.
lf_checker_rt::export!(thiscall, rw_s675_00d9f020(this: u32, slot_index: u32) -> u32 {
    const FLAG_TABLE: u32 = 0xC4;
    const ENTRY_STRIDE: u32 = 12;
    let address = this.wrapping_add(FLAG_TABLE).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe { (address as *const u8).read() as u32 }
});
