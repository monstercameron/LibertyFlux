// original: 0x00CD89E0 CTaskComplexFollowLeaderAnyMeans::vf19

/// Request the class's fixed follow-leader subtask and forward the supplied
/// argument to the shared task helper. The helper's return value is returned
/// unchanged to the caller.
///
/// Calling convention: thiscall with one 32-bit stack argument. The outgoing
/// thiscall passes the task pointer in ECX, the fixed subtask code first on
/// the stack, then the caller's argument.
lf_checker_rt::export!(thiscall, rw_00cd89e0(this: u32, task_arg: u32) -> u32 {
    const FOLLOW_LEADER_SUBTASK: u32 = 0xCA;
    const TASK_HELPER: u32 = 1;

    lf_checker_rt::callee_thiscall!(TASK_HELPER, u32, this, FOLLOW_LEADER_SUBTASK, task_arg)
});
