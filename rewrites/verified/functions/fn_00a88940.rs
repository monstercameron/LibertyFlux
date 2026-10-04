// original: 0x00a88940 status_bits_to_mask
/// Build a status mask from three flag bits.
///
/// Reads the byte at `this + 0x1480`: bit 0 selects 0x18, bit 1 adds 0x80,
/// bit 2 adds 6. Returns the combined mask.
export!(thiscall, rw_00a88940(this_obj: u32) -> u32 {
    unsafe {
        let flags = *((this_obj.wrapping_add(0x1480)) as *const u8);
        let mut out = 0u32;
        if flags & 1 != 0 {
            out = 0x18;
        }
        if flags & 2 != 0 {
            out |= 0x80;
        }
        if flags & 4 != 0 {
            out |= 6;
        }
        out
    }
});
