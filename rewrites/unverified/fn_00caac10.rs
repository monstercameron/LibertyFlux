// original: 0x00CAAC10 task_open_inner (proposed)

/// Open the inner task slot with this handler's current task and an event.
///
/// Forwards to the three-argument worker on the sub-object at `this+0x20`,
/// passing this handler's current-task pointer (`this+0x04`), the event
/// (the stack argument) and a zero flags word. Returns the worker's result.
///
/// Original: 0x00CAAC10 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00caac10(this: u32, event: u32) -> u32 {
    unsafe {
        const WORKER: u32 = 1;
        const INNER: u32 = 0x20;
        const CURRENT: u32 = 0x04;
        let current = (this.wrapping_add(CURRENT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(WORKER, u32, this.wrapping_add(INNER), current, event, 0)
    }
});
