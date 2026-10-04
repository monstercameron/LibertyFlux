// original: 0x008b81b0 control_axis_code_lookup
/// Control-table axis code lookup.
///
/// Indexes the control descriptor tables by `group` (valid range 0..=0x48)
/// and `slot`. Returns the signed 16-bit code stored at offset 0x12 of the
/// selected 0x16-byte entry, or 0x7fffffff when either index is out of range.
export!(cdecl, rw_008b81b0(group: u32, slot: u32) -> u32 {
    unsafe {
        const COUNT_TABLE: u32 = 0x019D33A4;
        const BASE_TABLE: u32 = 0x019D33A0;
        const ENTRY_LEN: u32 = 0x16;
        const CODE_OFF: u32 = 0x12;
        const GROUPS: u32 = 0x48;
        const NONE: u32 = 0x7FFF_FFFF;
        if group > GROUPS {
            return NONE;
        }
        if (slot as i32) < 0 {
            return NONE;
        }
        let row = group.wrapping_mul(3);
        let count_addr = relocated(COUNT_TABLE).wrapping_add(row.wrapping_mul(8));
        let count = (count_addr as *const u16).read() as u32;
        if (slot as i32) >= (count as i32) {
            return NONE;
        }
        let base_addr = relocated(BASE_TABLE).wrapping_add(row.wrapping_mul(8));
        let base = (base_addr as *const u32).read();
        let code_addr = base
            .wrapping_add(slot.wrapping_mul(ENTRY_LEN))
            .wrapping_add(CODE_OFF);
        (code_addr as *const i16).read() as i32 as u32
    }
});
