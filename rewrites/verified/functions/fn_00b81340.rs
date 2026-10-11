// original: 0x00B81340 ped_task_manager_update_sequence

/// Run the task manager's two preparatory methods in order, then tail-forward
/// to its final update method with the original manager pointer. The final
/// helper's EAX value is returned unchanged.
lf_checker_rt::export!(thiscall, rw_b81340(this: u32) -> u32 {
    const PREPARE_TASKS: u32 = 1;
    const REFRESH_TASKS: u32 = 2;
    const FINAL_UPDATE: u32 = 3;

    let _: u32 = lf_checker_rt::callee_thiscall!(PREPARE_TASKS, u32, this);
    let _: u32 = lf_checker_rt::callee_thiscall!(REFRESH_TASKS, u32, this);
    lf_checker_rt::callee_thiscall!(FINAL_UPDATE, u32, this)
});
