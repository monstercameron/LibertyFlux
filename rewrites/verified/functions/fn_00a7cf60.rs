// original: 0x00a7cf60 bu_task_notify_flagged_children
/// Walks the child chain and runs the gated notification on each child whose
/// flag bit is set.
export!(thiscall, rw_00a7cf60(obj: *mut u8) -> u32 {
    unsafe {
        let mut node = *((obj as *const u8).add(0x124) as *const u32);
        let mut last: u32 = 0;
        while node != 0 {
            if *((node + 0x13c) as *const u8) & 4 != 0 {
                last = callee_thiscall!(1, u32, node);
            }
            node = *((node + 0x11c) as *const u32);
        }
        last
    }
});
