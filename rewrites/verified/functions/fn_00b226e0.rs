// original: 0x00B226E0 interior_portal_entry
/// Return the portal-entry address for `this[0x18] + index`, with 96 bytes per
/// entry and the table base loaded from 0x01045798.
lf_checker_rt::export!(thiscall, rw_00b226e0(this: u32, index: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

        const ENTRY_INDEX_OFFSET: u32 = 0x18;
        const ENTRY_BYTES: u32 = 96;
        const PORTAL_TABLE_POINTER: u32 = 0x0104_5798;
        let entry_index = read_u32(this.wrapping_add(ENTRY_INDEX_OFFSET)).wrapping_add(index);
        let table = read_u32(lf_checker_rt::relocated(PORTAL_TABLE_POINTER) as u32);
        table.wrapping_add(entry_index.wrapping_mul(ENTRY_BYTES))
    }
});
