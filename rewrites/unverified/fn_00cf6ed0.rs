// original: 0x00cf6ed0 climb_subtask_init_1000 (proposed)

/// Initialises the sub-object at `+0xbb0` of the given task with the constant
/// 1000, returning whatever the initialiser returns.
///
/// Original: 0x00cf6ed0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00cf6ed0(task: u32) -> u32 {
    const SUB_OFFSET: u32 = 0xbb0;
    const INIT_ARG: u32 = 0x3e8;
    const INIT_CALLEE: u32 = 1;
    lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, task.wrapping_add(SUB_OFFSET), INIT_ARG)
});
