// original: 0x00A62ED0 forward_task_or_zero

/// Read the task pointer at `this + 0x25c`. Return zero when it is null;
/// otherwise tail-forward it in ECX to the task helper and return that helper's
/// EAX result. The helper is intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00a62ed0(this: u32) -> u32 {
    const TASK_POINTER: u32 = 0x25c;
    let task = unsafe { (this.wrapping_add(TASK_POINTER) as *const u32).read_unaligned() };
    if task == 0 { 0 } else { lf_checker_rt::callee_thiscall!(1, u32, task) }
});
