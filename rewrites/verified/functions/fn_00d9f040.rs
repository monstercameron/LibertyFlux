// original: 0x00D9F040 stream_sample_float
/// Returns the float field at offset 0xCC in a 12-byte record table.
lf_checker_rt::export!(thiscall, rw_s675_00d9f040(this: u32, slot_index: u32) -> f32 {
    const FLOAT_TABLE: u32 = 0xCC;
    const ENTRY_STRIDE: u32 = 12;
    let address = this.wrapping_add(FLOAT_TABLE).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    f32::from_bits(unsafe { (address as *const u32).read_unaligned() })
});
