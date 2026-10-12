// original: 0x00B22650 interior_portal_index_resolve
/// Resolve an entry in the portal index table. `this[0] + index` selects an
/// eight-byte record from the table pointer at 0x01657624. Status 1 returns
/// the record's signed 16-bit index; every other status returns 0x3f.
/// The function is thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00b22650(this: u32, index: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }

        const PORTAL_INDEX_TABLE: u32 = 0x0165_7624;
        const STATUS_VALID: u32 = 1;
        const MISSING_INDEX: u32 = 0x3f;
        let table = read_u32(lf_checker_rt::relocated(PORTAL_INDEX_TABLE) as u32);
        let row_index = read_u32(this).wrapping_add(index);
        let row = table.wrapping_add(row_index.wrapping_mul(8));
        if read_u32(row.wrapping_add(4)) == STATUS_VALID {
            (read_u16(row) as i16 as i32) as u32
        } else {
            MISSING_INDEX
        }
    }
});
