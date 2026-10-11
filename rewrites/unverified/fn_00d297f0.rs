// original: 0x00D297F0 CPedTargetting::vf8

/// Decide whether the supplied task may advance. Reject the receiver's own
/// sentinel task and tasks whose category mask is not 0xc0. Accept the two
/// ready subtypes when the status byte is set, or any task with a nonzero
/// progress word. Otherwise consult the receiver's callback and accept only
/// when it returns false. This thiscall method removes one stack pointer.
lf_checker_rt::export!(thiscall, rw_00D297F0(this: u32, task: u32) -> u8 {
    const RECEIVER_SENTINEL: u32 = 0x238;
    const TASK_CATEGORY: u32 = 0x28;
    const CATEGORY_MASK: u32 = 0x3c0;
    const REQUIRED_CATEGORY: u32 = 0xc0;
    const STATUS_READY: u32 = 0x210;
    const SUBTYPE: u32 = 0xa74;
    const PROGRESS: u32 = 0x224;
    const CALLBACK_FLAG: u32 = 0x211;
    const CALLBACK_OBJECT: u32 = 0x24c;
    const CALLBACK_CALLEE: u32 = 1;

    unsafe {
        if task == this.wrapping_add(RECEIVER_SENTINEL) {
            return 0;
        }
        if crate::read_u32(task.wrapping_add(TASK_CATEGORY)) & CATEGORY_MASK
            != REQUIRED_CATEGORY
        {
            return 0;
        }
        if crate::read_u8(task.wrapping_add(STATUS_READY)) != 0 {
            let subtype = crate::read_u32(task.wrapping_add(SUBTYPE));
            if subtype == 1 || subtype == 2 {
                return 1;
            }
        }
        if crate::read_u32(task.wrapping_add(PROGRESS)) != 0 {
            return 1;
        }
        if crate::read_u8(task.wrapping_add(CALLBACK_FLAG)) == 0 {
            return 0;
        }
        let callback = crate::read_u32(this.wrapping_add(CALLBACK_OBJECT));
        let callback_accepts = lf_checker_rt::callee_thiscall!(CALLBACK_CALLEE, u8, callback);
        if callback_accepts == 0 { 1 } else { 0 }
    }
});
