// original: 0x00B7F1F0 CTaskComplexScreamInCarThenLeave::vf5

/// Handles the abort-priority value 2 for the current child task. Any other
/// priority returns false without reading the child. The child pointer is at
/// receiver offset `+0x08`, with flags at child offset `+0x0c`; flag bit 0
/// means the child is already abortable. Otherwise virtual slot 5 receives
/// the first and third original arguments with the fixed middle value 2. A
/// successful callback sets child flag bit 1. The method is thiscall with
/// three stack arguments and returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_b7f1f0(this: u32, arg0: u32, priority: u32, arg2: u32) -> u32 {
    unsafe {
        const CHILD_TASK: u32 = 0x08;
        const TASK_FLAGS: u32 = 0x0c;
        const SUPPORTED_PRIORITY: u32 = 2;
        const CHILD_ABORTABLE: u32 = 1;
        const CHILD_ABORT_ACCEPTED: u32 = 2;
        const ABORTABLE_CALLEE: u32 = 1;

        if priority != SUPPORTED_PRIORITY {
            return 0;
        }

        let child = (this.wrapping_add(CHILD_TASK) as *const u32).read_unaligned();
        let flags_address = child.wrapping_add(TASK_FLAGS) as *mut u32;
        let flags = flags_address.read_unaligned();
        if flags & CHILD_ABORTABLE == 0 {
            let can_abort = lf_checker_rt::callee_thiscall!(
                ABORTABLE_CALLEE,
                u32,
                child,
                arg0,
                SUPPORTED_PRIORITY,
                arg2,
            ) as u8;
            if can_abort == 0 {
                return 0;
            }
            let flags_after_callback = flags_address.read_unaligned();
            flags_address.write_unaligned(flags_after_callback | CHILD_ABORT_ACCEPTED);
        }
        1
    }
});
