// original: 0x00CF32E0 stunt_jump_clear_flags
/// Clear the low byte of each task-pool flag slot in the engine table. Slots are 0x30 bytes apart from the first slot up to, but not including, the table end; the function returns the end pointer.
lf_checker_rt::export!(cdecl, rw_00cf32e0() -> u32 {
    const FIRST_SLOT_FILE_VA: u32 = 0x0171DB10;
    const TABLE_END_FILE_VA: u32 = 0x0171DE10;
    const SLOT_STRIDE: u32 = 0x30;
    let mut cursor = lf_checker_rt::relocated(FIRST_SLOT_FILE_VA);
    let end = lf_checker_rt::relocated(TABLE_END_FILE_VA);
    while cursor < end {
        if true {
            unsafe { (cursor as *mut u8).write_volatile(0); }
        }
        cursor = cursor.wrapping_add(SLOT_STRIDE);
    }
    end
});
