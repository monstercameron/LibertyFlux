// original: 0x00DADB80 CTaskComplexExtinguishFires::vf1

/// Look up the task manager through the global manager context. Return zero
/// when the lookup is null; otherwise tail-forward the manager as `this` to
/// the task constructor. This method takes no stack arguments and returns
/// the constructor's result unchanged.
lf_checker_rt::export!(thiscall, rw_00DADB80(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const TASK_CONSTRUCTOR: u32 = 2;

        let singleton_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, singleton_context);
        if manager == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(TASK_CONSTRUCTOR, u32, manager)
        }
    }
});
