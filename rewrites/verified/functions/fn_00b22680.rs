// original: 0x00B22680 interior_room_entry
/// Return the room-entry address for `this[0x1c] + index`, with 96 bytes per
/// entry and the table base loaded from 0x0104578c. Address arithmetic wraps
/// as 32-bit pointer arithmetic in the original image.
lf_checker_rt::export!(thiscall, rw_00b22680(this: u32, index: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

        const ENTRY_INDEX_OFFSET: u32 = 0x1c;
        const ENTRY_BYTES: u32 = 96;
        const ROOM_TABLE_POINTER: u32 = 0x0104_578C;
        let entry_index = read_u32(this.wrapping_add(ENTRY_INDEX_OFFSET)).wrapping_add(index);
        let table = read_u32(lf_checker_rt::relocated(ROOM_TABLE_POINTER) as u32);
        table.wrapping_add(entry_index.wrapping_mul(ENTRY_BYTES))
    }
});
