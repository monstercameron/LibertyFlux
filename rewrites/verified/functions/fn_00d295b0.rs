// original: 0x00D295B0 task_manager_clear_task_state (proposed)

/// Invoke the task manager's slot helper once for each of eight slot indices,
/// then clear the manager's saved selection, active-task, state-byte and
/// trailing word fields. The method takes only the receiver and returns no
/// value; its one-argument thiscall helper cleans its own stack word.
lf_checker_rt::export!(thiscall, rw_00D295B0(this: u32) -> () {
    const SLOT_HELPER: u32 = 1;
    const SLOT_COUNT: u32 = 8;
    const SAVED_SELECTION: u32 = 0x228;
    const ACTIVE_TASK: u32 = 0x224;
    const STATE_BYTE: u32 = 0x220;
    const TRAILING_STATE: u32 = 0x234;

    unsafe {
        for slot in 0..SLOT_COUNT {
            let _ = lf_checker_rt::callee_thiscall!(SLOT_HELPER, u32, this, slot);
        }
        crate::write_u16(this.wrapping_add(SAVED_SELECTION), u16::MAX);
        crate::write_u32(this.wrapping_add(ACTIVE_TASK), 0);
        crate::write_u8(this.wrapping_add(STATE_BYTE), 0);
        crate::write_u16(this.wrapping_add(TRAILING_STATE), 0);
    }
});
