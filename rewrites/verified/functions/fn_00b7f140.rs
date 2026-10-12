// original: 0x00B7F140 CTaskComplexWalkRoundCarWhileWandering::vf5

/// Asks the current child task whether it can be aborted at the requested
/// priority. The receiver's child pointer is at `+0x08`; the child flags are
/// at `+0x0c`, and its virtual method slot 5 is at vtable offset `+0x14`.
/// A null child returns false. A child with flag bit 0 already set returns
/// true without a call. Otherwise the three stack arguments are forwarded in
/// their original order; a true child result sets flag bit 1. Thiscall takes
/// the child in ECX and returns the boolean in AL.
lf_checker_rt::export!(thiscall, rw_b7f140(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CHILD_TASK: u32 = 0x08;
        const TASK_FLAGS: u32 = 0x0c;
        const CHILD_ABORTABLE: u32 = 1;
        const ABORTED_FLAG: u32 = 2;
        const ABORTABLE_CALLEE: u32 = 1;

        let child = (this.wrapping_add(CHILD_TASK) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }

        let flags_address = child.wrapping_add(TASK_FLAGS) as *mut u32;
        let flags = flags_address.read_unaligned();
        if flags & CHILD_ABORTABLE != 0 {
            return 1;
        }

        // The contract plants this stand-in in the child's vtable slot 5.
        let can_abort = lf_checker_rt::callee_thiscall!(
            ABORTABLE_CALLEE,
            u32,
            child,
            arg0,
            arg1,
            arg2,
        ) as u8;
        if can_abort != 0 {
            flags_address.write_unaligned(flags | ABORTED_FLAG);
            1
        } else {
            0
        }
    }
});
