// original: 0x00cceef0 task_timer_commit
/// Commit the task's pending timer value into the ped and clear the slot.
///
/// `this` is the task object, `ped` the ped. If the pending slot at
/// `this+0x1c` holds anything other than -1 (empty), that value is stored to
/// `ped+0xb94` and the slot is reset to -1. Otherwise nothing is written.
/// Thiscall with one stack argument; the original leaves EAX holding the ped
/// pointer on the store path and entry EAX otherwise, so no return channel
/// is compared (see contract).
export!(thiscall, rw_00cceef0(this: u32, ped: u32) -> u32 {
    unsafe {
        const PENDING_OFF: u32 = 0x1c;
        const PED_TIMER_OFF: u32 = 0xb94;
        const EMPTY: u32 = 0xffff_ffff;
        let pending = (this.wrapping_add(PENDING_OFF) as *const u32).read_unaligned();
        if pending != EMPTY {
            (ped.wrapping_add(PED_TIMER_OFF) as *mut u32).write_unaligned(pending);
            (this.wrapping_add(PENDING_OFF) as *mut u32).write_unaligned(EMPTY);
        }
        0
    }
});
