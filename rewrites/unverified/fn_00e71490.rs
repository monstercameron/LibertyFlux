// original: 0x00E71490 drain_notification_table
/// Drain 900 four-byte table slots in descending address order, passing each slot to the notification-list helper.
lf_checker_rt::export!(cdecl, rw_fn_00E71490() -> () {
    unsafe {
        const TABLE_END: u32 = 0x011D4E38;
        const SLOT_WIDTH: u32 = 4;
        const SLOT_COUNT: u32 = 900;

        let mut slot = lf_checker_rt::relocated(TABLE_END);
        for _ in 0..SLOT_COUNT {
            slot = slot.wrapping_sub(SLOT_WIDTH);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, slot);
        }
    }
});
