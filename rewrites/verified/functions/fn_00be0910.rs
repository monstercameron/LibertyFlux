// original: 0x00be0910 CTaskComplexWaitForBus::vf1
/// Look up the global task manager and return zero when it is absent. Otherwise tail-forward the manager pointer in ECX to the shared wait-task body and return its EAX result. The contract patches the direct tail jump and scripts its thiscall answer. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be0910(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const WAIT_TASK_BODY: u32 = 2;
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(WAIT_TASK_BODY, u32, manager)
    }
});
