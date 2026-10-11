// original: 0x0097EE90 ped_task_set_first_deadline_to_current_time

/// Copies the current task-system clock into the task's `0x13c` timestamp field.
///
/// The task pointer is passed in ECX. The clock is read as one 32-bit global
/// word and stored without arithmetic, preserving every bit pattern.
lf_checker_rt::export!(thiscall, rw_0097EE90(this: u32) -> () {
    const DEADLINE_OFFSET: u32 = 0x13C;
    const CURRENT_TIME_VA: u32 = 0x011735B4;
    let current = unsafe { lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read() };
    unsafe { (this.wrapping_add(DEADLINE_OFFSET) as *mut u32).write_unaligned(current); }
});