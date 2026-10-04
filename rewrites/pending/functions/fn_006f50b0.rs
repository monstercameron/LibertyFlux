// original: 0x006f50b0 read_field_11bit_b1
/// Read an 11-bit big-endian bit field starting at bit 3 of byte 1.
///
/// Covers bytes 1..4: low 5 bits of byte 1, all of byte 2, top 3 bits of
/// byte 3.
rt::export!(thiscall, rw_006f50b0(this: *const u8) -> u32 {
    unsafe {
        let b1 = *this.add(1) as u32;
        let b2 = *this.add(2) as u32;
        let b3 = *this.add(3) as u32;
        ((b1 & 0x1F) << 11) | (b2 << 3) | (b3 >> 5)
    }
});
