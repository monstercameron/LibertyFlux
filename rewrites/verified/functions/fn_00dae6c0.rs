// original: 0x00DAE6C0 CTaskComplexInvestigateDeadPed::vf19

/// Forward this task's fixed request code and its ped argument to the
/// task factory, returning the factory's scripted result unchanged.
///
/// `this` is the task object in ECX. `ped` is the single stack argument.
/// The factory receives `this`, the request code, and `ped` through a
/// thiscall with two callee-cleaned stack arguments. The entry removes
/// its own one stack argument before returning.
lf_checker_rt::export!(thiscall, rw_00DAE6C0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_FACTORY: u32 = 1;
        const REQUEST_CODE: u32 = 0xcb;
        lf_checker_rt::callee_thiscall!(TASK_FACTORY, u32, this, REQUEST_CODE, ped)
    }
});
