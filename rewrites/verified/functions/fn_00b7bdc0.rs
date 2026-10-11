// original: 0x00b7bdc0 CTaskComplexLeaveCarAndFlee::vf20

/// Set the supplied task's leave-car flag and return this task's stored
/// result pointer. The argument points to a task whose 32-bit flags are at
/// offset `+0x2a0`; bit `0x20` is set without disturbing other bits. The
/// return value is the dword stored at this object's `+0x08` field.
///
/// Original ABI: thiscall with one pointer argument and callee stack cleanup.
lf_checker_rt::export!(thiscall, rw_00b7bdc0(this: u32, task: u32) -> u32 {
    unsafe {
        const TASK_FLAGS: u32 = 0x2a0;
        const LEAVE_CAR_FLAG: u32 = 0x20;
        const RESULT_FIELD: u32 = 0x08;
        let flags = (task + TASK_FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() | LEAVE_CAR_FLAG);
        ((this + RESULT_FIELD) as *const u32).read_unaligned()
    }
});
