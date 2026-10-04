// original: 0x006f5050 read_field_10bit
/// Read a 10-bit big-endian bit field starting at bit 2 of byte 4.
///
/// Covers bytes 4..7: low 6 bits of byte 4, all of byte 5, top 2 bits of
/// byte 6.
rt::export!(thiscall, rw_006f5050(this: *const u8) -> u32 {
    unsafe {
        let b4 = *this.add(4) as u32;
        let b5 = *this.add(5) as u32;
        let b6 = *this.add(6) as u32;
        ((b4 & 0x3F) << 10) | (b5 << 2) | (b6 >> 6)
    }
});
