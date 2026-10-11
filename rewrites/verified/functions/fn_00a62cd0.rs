// original: 0x00A62CD0 find_first_unflagged_task_slot

/// Return the first eligible task pointer from the three slots beginning at
/// `this + 0x70` (`field_44 + 0x2c`). A null slot is skipped. A non-null
/// task is eligible when bit 0 of its byte at `+0x0c` is clear. The first
/// eligible pointer is returned, or zero when none qualifies. `this` is in
/// ECX and there are no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a62cd0(this: u32) -> u32 {
    unsafe {
        const TASK_SLOTS: u32 = 0x70;
        const SLOT_COUNT: u32 = 3;
        const TASK_FLAGS: u32 = 0x0c;
        for slot in 0..SLOT_COUNT {
            let task = (this.wrapping_add(TASK_SLOTS + slot * 4) as *const u32).read_unaligned();
            if task != 0 && ((task.wrapping_add(TASK_FLAGS) as *const u8).read() & 1) == 0 {
                return task;
            }
        }
        0
    }
});
