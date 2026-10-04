// original: 0x0094b8c0 obj_slot_ptr
/// Compute the address of slot `idx` in a 200-byte-stride array rooted at
/// the object, using the alternate bank (offset by 0xC0 slots) when the
/// low byte of `alt` is nonzero.
export!(thiscall, rw_0094b8c0(obj: *const u8, idx: u32, alt: u32) -> u32 {
    let k = if (alt & 0xFF) != 0 {
        idx.wrapping_add(0xC0)
    } else {
        idx
    };
    (obj as u32).wrapping_add(k.wrapping_mul(0xC8))
});
