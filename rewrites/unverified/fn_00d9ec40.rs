// original: 0x00D9EC40 stream_entry_flag
/// Reads the low-byte flag at offset 0x94 plus 16 bytes per slot index and returns it in AL.
lf_checker_rt::export!(thiscall, rw_s675_00d9ec40(this: u32, slot_index: u32) -> u32 {
    const FLAG_FIELD: u32 = 0x94;
    const ENTRY_STRIDE: u32 = 16;
    let address = this.wrapping_add(FLAG_FIELD).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    unsafe { (address as *const u8).read() as u32 }
});
