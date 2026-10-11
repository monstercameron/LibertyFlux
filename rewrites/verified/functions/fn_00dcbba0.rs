// original: 0x00DCBBA0 CTaskComplexMove_StepAwayFromCollisionObjects::~CTaskComplexMove_StepAwayFromCollisionObjects
/// Installs the collision-avoidance task's derived tables, then forwards
/// destruction to the base task destructor through the checker call stub.
/// Only the task object is touched; the base destructor owns later cleanup.
lf_checker_rt::export!(thiscall, rw_00dcbba0(task: u32) -> () {
    const STEP_AWAY_VTABLE_VA: u32 = 0x00ef554c;
    const STEP_AWAY_EXTRA_VTABLE_VA: u32 = 0x00ef55a4;
    const EXTRA_VTABLE_OFFSET: usize = 0x14;

    // SAFETY: the caller provides a writable task object for these fields.
    unsafe {
        (task as *mut u32).write(lf_checker_rt::relocated(STEP_AWAY_VTABLE_VA));
        ((task as *mut u8).add(EXTRA_VTABLE_OFFSET) as *mut u32)
            .write(lf_checker_rt::relocated(STEP_AWAY_EXTRA_VTABLE_VA));
    }
    let _ = lf_checker_rt::callee_thiscall!(1, u32, task);
});
