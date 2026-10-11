// original: 0x00A6D2E0 CTaskComplexPlayerPlaceCarBomb::~CTaskComplexPlayerPlaceCarBomb__deleting

/// Runs the task destructor and conditionally releases the object through
/// the task pool.
///
/// Bit zero of `flags` controls the pool call. The pool value comes from the
/// relocated task-pool global; the pool helper receives it in ECX and `this`
/// as one stack argument. This deleting destructor returns `this` and pops
/// the flags word.
lf_checker_rt::export!(thiscall, rw_00a6d2e0(this: u32, flags: u32) -> u32 {
    unsafe {
        const DESTROY_TASK: u32 = 1;
        const FREE_TASK: u32 = 2;
        const TASK_POOL_VA: u32 = 0x0167_E2A0;
        let _ = lf_checker_rt::callee_thiscall!(DESTROY_TASK, u32, this);
        if flags as u8 & 1 != 0 {
            let pool = lf_checker_rt::global::<u32>(TASK_POOL_VA).read();
            let _ = lf_checker_rt::callee_thiscall!(FREE_TASK, u32, pool, this);
        }
        this
    }
});
