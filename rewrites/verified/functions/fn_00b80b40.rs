// original: 0x00B80B40 CEventFlushTasks::~CEventFlushTasks

/// Allocate a `CEventFlushTasks` object from the global task pool. On success,
/// call the base event constructor, copy this object's field at `+0x0c`, install
/// the derived vtable, and return the new object. Return null if allocation fails.
lf_checker_rt::export!(thiscall, rw_b80b40(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x016669c8;
        const EVENT_VTABLE: u32 = 0x00eb3b24;
        const EVENT_PAYLOAD: u32 = 0x0c;
        const TASK_POOL_ALLOC: u32 = 1;
        const BASE_CONSTRUCTOR: u32 = 2;

        let task_pool = (lf_checker_rt::relocated(TASK_POOL_GLOBAL) as *const u32).read_unaligned();
        let copy = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, task_pool);
        if copy == 0 {
            return 0;
        }
        let payload = ((this + EVENT_PAYLOAD) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, copy);
        ((copy + EVENT_PAYLOAD) as *mut u32).write_unaligned(payload);
        (copy as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        copy
    }
});
