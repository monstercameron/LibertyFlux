// original: 0x008d83d0 bitfield_decode
/// Decode a packed value: bit 13 of the word at +0xE selects the path.
///
/// When the selector bit is clear the dword at +0 is returned as-is;
/// otherwise bits 0..10 are shifted left by (8 + bits 11..14).
export!(thiscall, rw_008d83d0(this_: *const u8) -> u32 {
    unsafe {
        let flags = (this_.byte_add(0x0E) as *const u16).read_unaligned();
        let value = (this_ as *const u32).read_unaligned();
        if (flags >> 13) & 1 == 0 {
            value
        } else {
            let shift = ((value >> 11) & 0xF) + 8;
            (value & 0x7FF) << shift
        }
    }
});
