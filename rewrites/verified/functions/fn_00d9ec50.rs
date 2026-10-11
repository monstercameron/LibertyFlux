// original: 0x00D9EC50 stream_entry_value_float
/// Returns the 32-bit float at offset 0x9C plus 16 bytes per slot index, preserving the x87 result channel.
lf_checker_rt::export!(thiscall, rw_s675_00d9ec50(this: u32, slot_index: u32) -> f32 {
    const FLOAT_FIELD: u32 = 0x9C;
    const ENTRY_STRIDE: u32 = 16;
    let address = this.wrapping_add(FLOAT_FIELD).wrapping_add(slot_index.wrapping_mul(ENTRY_STRIDE));
    f32::from_bits(unsafe { (address as *const u32).read_unaligned() })
});
