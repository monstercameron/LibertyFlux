// original: 0x00DCD170 reload_task_set_phase_three_and_clear_link
/// If the task pointer is nonnull, requests stage 3 through the task's
/// stage helper and clears the linked-object field at offset 0x48. The
/// helper call is observable and is forwarded through checker callee 1.
lf_checker_rt::export!(cdecl, rw_00dcd170(
    _unused_first: u32,
    task: u32
) -> () {
    if task != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, task, 3u32);
        // SAFETY: the nonnull task pointer is backed by the heap contract.
        unsafe { ((task as *mut u8).add(0x48) as *mut u32).write(0); }
    }
});
