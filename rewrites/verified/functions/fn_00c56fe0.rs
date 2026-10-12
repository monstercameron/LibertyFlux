// original: 0x00C56FE0 remove_scanner_entry_and_reset_matching_index
/// Search sixteen fixed-size scanner slots for an entry pointer. On a match,
/// clear its status word, release and clear its optional owner pointer, then
/// set the signed active-index byte to -1 when it equals the matched slot.
/// Return the sign-extended active byte after a match, or the pointer just
/// past the slot array when no entry matches. The owner release is scripted.
lf_checker_rt::export!(thiscall, rw_c56fe0(this: u32, entry: u32) -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 16;
        const SLOT_STRIDE: u32 = 0x10;
        const OWNER_OFFSET: u32 = 8;
        const STATUS_OFFSET: u32 = 4;
        const ACTIVE_INDEX_OFFSET: u32 = 0x210;

        let mut slot_address = this;
        for slot_index in 0..SLOT_COUNT {
            if slot_address == entry {
                (slot_address.wrapping_add(STATUS_OFFSET) as *mut u32).write_unaligned(0);
                let owner_field = slot_address.wrapping_add(OWNER_OFFSET);
                let owner = (owner_field as *const u32).read_unaligned();
                if owner != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(1, u32, owner, owner_field);
                    (owner_field as *mut u32).write_unaligned(0);
                }
                let active = (this.wrapping_add(ACTIVE_INDEX_OFFSET) as *const i8)
                    .read_unaligned() as i32;
                if active == slot_index as i32 {
                    (this.wrapping_add(ACTIVE_INDEX_OFFSET) as *mut u8).write_unaligned(0xff);
                }
                return active as u32;
            }
            slot_address = slot_address.wrapping_add(SLOT_STRIDE);
        }
        slot_address
    }
});
