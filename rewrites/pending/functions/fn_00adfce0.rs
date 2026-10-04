// original: 0x00adfce0 ui_slot_address_or_null
/// Address of this object's indexed slot, or null when it matches the base.
///
/// Derives an index from flag bytes at 0x26/0x28; when the scaled index
/// equals the word at 0x18 there is no slot (return 0), otherwise the slot
/// sits that many bytes past the object start.
export!(thiscall, rw_00adfce0(this: *const u8) -> u32 {
    unsafe {
        let flag_a = *(this.add(0x28)) as u32;
        let flag_b = *(this.add(0x26)) as u32;
        let base = *(this.add(0x18) as *const u16) as u32;
        let index = flag_b
            .wrapping_sub((flag_a & 1).wrapping_mul(3))
            .wrapping_add(4)
            .wrapping_shl(4);
        if index == base {
            0
        } else {
            (this as u32).wrapping_add(index)
        }
    }
});
