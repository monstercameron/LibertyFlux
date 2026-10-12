// original: 0x00B20BA0 room_id_table_append
/// Append one room identifier to a table whose word base and count are stored
/// at offsets 0x10 and 0x14 of `this`. The selected 16-bit table entry is
/// `base + count`; the count advances with wrapping 32-bit arithmetic.
/// The method is thiscall with one stack argument. Only its low 16 bits are
/// stored; the function has no declared return value.
lf_checker_rt::export!(thiscall, rw_00b20ba0(this: u32, room_id: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }

        const ROOM_TABLE: u32 = 0x0165_56D8;
        const TABLE_BASE_OFFSET: u32 = 0x10;
        const TABLE_COUNT_OFFSET: u32 = 0x14;
        let base_index = read_u32(this.wrapping_add(TABLE_BASE_OFFSET));
        let count_address = this.wrapping_add(TABLE_COUNT_OFFSET);
        let count = read_u32(count_address);
        let entry_index = base_index.wrapping_add(count);
        let entry_address = ROOM_TABLE.wrapping_add(entry_index.wrapping_mul(2));
        write_u16(lf_checker_rt::relocated(entry_address) as u32, room_id as u16);
        write_u32(count_address, count.wrapping_add(1));
        0
    }
});
