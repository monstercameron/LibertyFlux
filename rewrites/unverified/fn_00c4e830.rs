// original: 0x00C4E830 mark_task_resource_flag

/// Mark a task's resource state when its streaming resource is available.
///
/// The task pointer is the first stack argument and the owner is in ECX.
/// The resource lookup receives `task + 0x2b0`. A nonzero lookup result sets
/// bit 4 of the task word at `+0x2a0` and returns the owner's word at `+8`;
/// a zero result leaves the task untouched and is returned unchanged.
lf_checker_rt::export!(thiscall, rw_00c4e830(owner: u32, task: u32) -> u32 {
    const RESOURCE_LOOKUP: u32 = 0;
    let resource = unsafe {
        lf_checker_rt::callee_thiscall!(RESOURCE_LOOKUP, u32, task.wrapping_add(0x2b0))
    };
    if resource == 0 {
        return resource;
    }
    unsafe {
        let flags = task.wrapping_add(0x2a0) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() | 0x10);
        (owner.wrapping_add(8) as *const u32).read_unaligned()
    }
});
