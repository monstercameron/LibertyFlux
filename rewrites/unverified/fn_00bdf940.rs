// original: 0x00BDF940 CTaskSimpleSayAudio::~CTaskSimpleSayAudio

/// Replaces the task vtable, detaches a non-null timing-node pointer through
/// the helper, clears that pointer, and tail-calls the base simple-task
/// destructor. The thiscall receiver is passed in ECX and there are no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_bdf940(this: u32) -> u32 {
    unsafe {
        const DETACH_TIMING_NODE: u32 = 1;
        const BASE_TASK_DTOR: u32 = 2;
        const AUDIO_TASK_VTABLE: u32 = 0x00EB8B8C;
        const TIMING_NODE: u32 = 0x44;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(AUDIO_TASK_VTABLE));
        let node_field = this.wrapping_add(TIMING_NODE);
        let node = (node_field as *const u32).read_unaligned();
        if node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_TIMING_NODE, u32, node, node_field);
            (node_field as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_TASK_DTOR, u32, this)
    }
});
