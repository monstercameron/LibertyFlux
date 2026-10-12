// original: 0x00B226A0 interior_room_entry_resolve
/// Resolve a portal entry, copy its body into a temporary 0x5c-byte record,
/// add the caller's room offset to the copied 32-bit index, sign-extend the
/// selected room-ID table word, then return the resolved room entry. Each of
/// the three direct helpers is invoked through a scripted thiscall stand-in.
/// The copied temporary's index is at byte offset 0x0c; the original calls
/// the body-copy helper with a pointer into its own stack frame.
lf_checker_rt::export!(thiscall, rw_00b226a0(this: u32, portal_index: u32, room_offset: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }

        const ROOM_ID_TABLE: u32 = 0x0165_56D8;
        const TEMP_INDEX_OFFSET: usize = 0x0c;
        const TEMP_WORDS: usize = 0x5c / 4;

        let portal_entry = lf_checker_rt::callee_thiscall!(1, u32, this, portal_index);
        let mut copied_body = [0u32; TEMP_WORDS];
        let copied_body_ptr = copied_body.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, copied_body_ptr, portal_entry);
        let portal_room_index = copied_body[TEMP_INDEX_OFFSET / 4];
        let room_table_index = portal_room_index.wrapping_add(room_offset);
        let room_id_address = (lf_checker_rt::relocated(ROOM_ID_TABLE) as u32)
            .wrapping_add(room_table_index.wrapping_mul(2));
        let room_id = (read_u16(room_id_address) as i16 as i32) as u32;
        lf_checker_rt::callee_thiscall!(3, u32, this, room_id)
    }
});
