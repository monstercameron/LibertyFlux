// original: 0x00B809B0 CEventFlushTasks::~CEventFlushTasks__deleting

/// Install the `CEventFlushTasks` vtable, invoke the base event destructor,
/// and conditionally release this object when flag bit zero is set. The
/// optional release uses the global task pool and receives `this` as its one
/// stack argument; this function always returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_b809b0(this: u32, flags: u32) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00eb3b24;
        const TASK_POOL_GLOBAL: u32 = 0x016669c8;
        const BASE_DESTRUCTOR: u32 = 1;
        const OPERATOR_DELETE: u32 = 2;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this);
        if flags & 1 != 0 {
            let task_pool = (lf_checker_rt::relocated(TASK_POOL_GLOBAL) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(OPERATOR_DELETE, u32, task_pool, this);
        }
        this
    }
});
