// original: 0x00899800 audio_indexed_ptr_or_default
/// Returns the indexed entry of an audio table, or a default object.
///
/// Yields 0 when the table is disabled, the shared default object when the
/// index is past the end, and the base-plus-index entry otherwise.
export!(thiscall, rw_00899800(this: u32, index: u32) -> u32 {
    unsafe {
        let enabled = core::ptr::read_unaligned(this.wrapping_add(0x44) as *const u8);
        if enabled == 0 {
            return 0;
        }
        let limit = core::ptr::read_unaligned(this.wrapping_add(0x2c) as *const u32);
        if index >= limit {
            return relocated(0xE79780);
        }
        core::ptr::read_unaligned(this.wrapping_add(0x34) as *const u32).wrapping_add(index)
    }
});
