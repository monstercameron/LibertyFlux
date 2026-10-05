// original: 0x00a94300 stream_entry_data_address (proposed)

/// Data address of a stream entry, or 0 when the entry is empty.
///
/// An entry is empty when its size word at `+0x08` has no bits above the
/// low two set and bit 11 of its flag word at `+0x0e` is clear; the result
/// is then 0. Otherwise the low byte of the word at `+0x04` selects one of
/// 256 dword slots spaced 160 bytes apart in a global table, and the result
/// is `(value >> 8) + slot` with wrapping addition.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a94300(this: u32) -> u32 {
    unsafe {
        const ENT_SIZE: u32 = 0x08;
        const ENT_VALUE: u32 = 0x04;
        const ENT_FLAGS: u32 = 0x0e;
        const SIZE_MASK: u32 = 0xffff_fffc;
        const PRESENT_BIT: u32 = 11;
        const SLOT_STRIDE: u32 = 160;
        const DATA_TABLE: u32 = 0x012fb44c;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        if rd32(this.wrapping_add(ENT_SIZE)) & SIZE_MASK == 0
            && (rd16(this.wrapping_add(ENT_FLAGS)) >> PRESENT_BIT) & 1 == 0
        {
            return 0;
        }
        let v = rd32(this.wrapping_add(ENT_VALUE));
        let slot = lf_checker_rt::relocated(DATA_TABLE)
            .wrapping_add((v & 0xff).wrapping_mul(SLOT_STRIDE));
        (v >> 8).wrapping_add(rd32(slot))
    }
});
