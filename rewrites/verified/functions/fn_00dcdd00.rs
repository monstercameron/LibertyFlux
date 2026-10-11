// original: 0x00DCDD00 aim_task_set_phase_seven_and_clear_timer
/// If the task pointer is nonnull, asks the aim-task helper to set stage
/// 7 and clears the timer field at offset 0x64. The helper call is part of
/// the observable behavior and is forwarded through checker callee 1.
lf_checker_rt::export!(cdecl, rw_00dcdd00(
    _unused_first: u32,
    task: u32
) -> () {
    if task != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, task, 7u32);
        // SAFETY: the nonnull task pointer is backed by the heap contract.
        unsafe { ((task as *mut u8).add(0x64) as *mut u32).write(0); }
    }
});
