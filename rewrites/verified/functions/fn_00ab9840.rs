// original: 0x00ab9840 slot_field_select

/// Indexed slot field select over stride-64 records.
///
/// Reads the record at `this + idx * 64`. When the tag word at offset 0x14
/// is all-ones the record is empty and the header word at offset 0 is
/// returned instead; otherwise the tag word itself is returned.
export!(thiscall, rs64_ab9840(this: *const u8, idx: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 64;
        const TAG_OFF: u32 = 0x14;
        const EMPTY: u32 = 0xFFFF_FFFF;
        let slot = (this as u32).wrapping_add(idx.wrapping_mul(STRIDE));
        let tag = *((slot.wrapping_add(TAG_OFF)) as *const u32);
        if tag == EMPTY {
            *(slot as *const u32)
        } else {
            tag
        }
    }
});
