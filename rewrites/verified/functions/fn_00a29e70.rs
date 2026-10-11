// original: 0x00a29e70 ped_task_manager_forwarder

/// Forward the caller's stack word to the task manager stored at `this+0x228`.
/// A non-null manager is adjusted by `+0x70`; a null value is forwarded as null.
/// Both branches end in a tail call whose target removes the one stack word.
lf_checker_rt::export!(thiscall, rw_a29e70(this: u32, task_index: u32) -> u32 {
    const TASK_MANAGER: u32 = 0x0228;
    let stored = unsafe { ((this + TASK_MANAGER) as *const u32).read_unaligned() };
    let manager = if stored == 0 { 0 } else { stored.wrapping_add(0x70) };
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, manager, task_index) }
});
