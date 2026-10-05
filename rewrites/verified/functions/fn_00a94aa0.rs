// original: 0x00a94aa0 stream_table_entry_is_active (proposed)

/// Whether entry `idx` of the table is active (1) or not (0).
///
/// Inactive when the table base at `[this]` is null or when the entry
/// matches the empty test (size word at `+0x08` clear above the low two
/// bits and bit 11 of the flag word at `+0x0e` clear). The null-table path
/// sets only `al`, so the comparison covers `al` on every path. Pure leaf.
///
/// Original: thiscall, one stack argument (index).
lf_checker_rt::export!(thiscall, rw_00a94aa0(this: u32, idx: u32) -> u8 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_SIZE: u32 = 0x08;
        const ENT_FLAGS: u32 = 0x0e;
        const SIZE_MASK: u32 = 0xffff_fffc;
        const PRESENT_BIT: u32 = 11;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        if base == 0 {
            return 0;
        }
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        if rd32(ent.wrapping_add(ENT_SIZE)) & SIZE_MASK != 0
            || (rd16(ent.wrapping_add(ENT_FLAGS)) >> PRESENT_BIT) & 1 != 0
        {
            1
        } else {
            0
        }
    }
});
