// original: 0x00BDFA40 CTaskSimpleTriggerLookAt::~CTaskSimpleTriggerLookAt

/// Replaces the task vtable, detaches a non-null timing-node pointer through
/// the helper, and tail-calls the base simple-task destructor. The helper may
/// update the pointer field through its argument; this method leaves that write
/// intact. The thiscall receiver is in ECX and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_bdfa40(this: u32) -> u32 {
    unsafe {
        const DETACH_TIMING_NODE: u32 = 1;
        const BASE_TASK_DTOR: u32 = 2;
        const TASK_VTABLE: u32 = 0x00EB91E4;
        const TIMING_NODE: u32 = 0x30;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        let node_field = this.wrapping_add(TIMING_NODE);
        let node = (node_field as *const u32).read_unaligned();
        if node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, node, node_field);
        }
        lf_checker_rt::callee_thiscall!(BASE_TASK_DTOR, u32, this)
    }
});
