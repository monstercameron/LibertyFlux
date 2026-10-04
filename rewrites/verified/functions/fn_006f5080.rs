// original: 0x006f5080 read_field_11bit_b3
/// Read an 11-bit big-endian bit field starting at bit 3 of byte 3.
///
/// Covers bytes 3..6: low 5 bits of byte 3, all of byte 4, top 3 bits of
/// byte 5.
rt::export!(thiscall, rw_006f5080(this: *const u8) -> u32 {
    unsafe {
        let b3 = *this.add(3) as u32;
        let b4 = *this.add(4) as u32;
        let b5 = *this.add(5) as u32;
        ((b3 & 0x1F) << 11) | (b4 << 3) | (b5 >> 5)
    }
});
