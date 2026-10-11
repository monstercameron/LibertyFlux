// original: 0x00B80B80 CEventNewTask::~CEventNewTask

/// Allocate a `CEventNewTask` copy from the global task pool. On success,
/// preserve the source words at `+0x10` and `+0x14`, call the base event
/// constructor, install the derived vtable, clear byte `+0x0c`, and return the
/// new object. Return null if allocation fails.
lf_checker_rt::export!(thiscall, rw_b80b80(this: u32) -> u32 {
    unsafe {
        const TASK_POOL_GLOBAL: u32 = 0x016669c8;
        const EVENT_VTABLE: u32 = 0x00eb3acc;
        const EVENT_KIND: u32 = 0x10;
        const EVENT_DATA: u32 = 0x14;
        const EVENT_FLAGS: u32 = 0x0c;
        const TASK_POOL_ALLOC: u32 = 1;
        const BASE_CONSTRUCTOR: u32 = 2;

        let task_pool = (lf_checker_rt::relocated(TASK_POOL_GLOBAL) as *const u32).read_unaligned();
        let copy = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOC, u32, task_pool);
        if copy == 0 {
            return 0;
        }
        let kind = ((this + EVENT_KIND) as *const u32).read_unaligned();
        let data = ((this + EVENT_DATA) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, copy);
        ((copy + EVENT_DATA) as *mut u32).write_unaligned(data);
        ((copy + EVENT_KIND) as *mut u32).write_unaligned(kind);
        (copy as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        ((copy + EVENT_FLAGS) as *mut u8).write(0);
        copy
    }
});
