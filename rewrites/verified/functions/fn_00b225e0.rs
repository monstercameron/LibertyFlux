// original: 0x00B225E0 interior_room_link_find
/// Find a room-link record by the unsigned table index `this[0] + index`.
/// Status 2 exposes a signed 16-bit room identifier through `room_id_out`,
/// then the room entry's four link indices at offset 0x50 are searched for
/// `index`; the matching slot or the 0x3f sentinel is written to `slot_out`.
/// Other statuses write 0x3f to both outputs. The one direct helper call is
/// intercepted as a thiscall and returns the room-entry pointer. Three stack
/// arguments follow `this`.
lf_checker_rt::export!(thiscall, rw_00b225e0(this: u32, index: u32, room_id_out: u32, slot_out: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        const PORTAL_INDEX_TABLE: u32 = 0x0165_7624;
        const ROOM_LINKS_OFFSET: u32 = 0x50;
        const SEARCH_WIDTH: u32 = 4;
        const VALID_STATUS: u32 = 2;
        const MISSING_INDEX: u32 = 0x3f;

        write_u32(slot_out, MISSING_INDEX);
        let base_index = read_u32(this);
        let table = read_u32(lf_checker_rt::relocated(PORTAL_INDEX_TABLE) as u32);
        let row_index = base_index.wrapping_add(index);
        let row = table.wrapping_add(row_index.wrapping_mul(8));
        if read_u32(row.wrapping_add(4)) != VALID_STATUS {
            write_u32(room_id_out, MISSING_INDEX);
            return 0;
        }

        let room_id = read_u16(row) as i16 as i32 as u32;
        write_u32(room_id_out, room_id);
        if room_id == MISSING_INDEX {
            return 0;
        }

        let room_entry = lf_checker_rt::callee_thiscall!(1, u32, this, room_id);
        for slot in 0..SEARCH_WIDTH {
            if read_u32(room_entry.wrapping_add(ROOM_LINKS_OFFSET + slot * 4)) == index {
                write_u32(slot_out, slot);
                break;
            }
        }
        0
    }
});
