// original: 0x00A63930 NativeImpl_IS_CHAR_SWIMMING

/// Read the ped entity from `this + 0x40`, then inspect its move-state
/// object at `entity + 0x6c`. If that pointer is null or its byte at `+0x0e`
/// is zero, return bit 11 of the entity word at `+0x2a0`. Otherwise search
/// the task chain at `this + 0x2e0` for type `0x518` and return whether it
/// occurs. Both results are one-byte booleans in AL.
lf_checker_rt::export!(thiscall, rw_00a63930(this: u32) -> u32 {
    const PED_ENTITY: u32 = 0x40;
    const MOVE_STATE: u32 = 0x6c;
    const MOVE_STATE_FLAG: u32 = 0x0e;
    const SWIM_STATE_BIT: u32 = 1 << 11;
    const TASK_HEAD: u32 = 0x2e0;
    const TASK_TYPE: u32 = 0x04;
    const TASK_NEXT: u32 = 0x0c;
    const SWIM_TASK_TYPE: u32 = 0x518;
    unsafe {
        let entity = (this.wrapping_add(PED_ENTITY) as *const u32).read_unaligned();
        let move_state = (entity.wrapping_add(MOVE_STATE) as *const u32).read_unaligned();
        if move_state == 0 || (move_state.wrapping_add(MOVE_STATE_FLAG) as *const u8).read() == 0 {
            return (((entity.wrapping_add(0x2a0) as *const u32).read_unaligned() & SWIM_STATE_BIT) != 0) as u32;
        }
        let mut task = (this.wrapping_add(TASK_HEAD) as *const u32).read_unaligned();
        while task != 0 {
            if (task.wrapping_add(TASK_TYPE) as *const u32).read_unaligned() == SWIM_TASK_TYPE { return 1; }
            task = (task.wrapping_add(TASK_NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
