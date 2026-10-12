// original: 0x00cfbbd0 clear_task_effect
/// Releases the task referenced at object offset 0x60. It forwards the supplied
/// float bits to the task updater, invokes the task detach method, sets the task's
/// 0x4000 state flag, and clears the object's task pointer. A null task is a no-op.
/// The float is forwarded as its original 32-bit representation.
lf_checker_rt::export!(thiscall, rw_00cfbbd0(this: *mut u8, amount_bits: u32) -> u32 {
    unsafe {
        const TASK_POINTER: usize = 0x60;
        const STATE_FLAGS: usize = 0x04;
        const DETACHED_FLAG: u32 = 0x4000;
        let task = *((this.add(TASK_POINTER)) as *const u32);
        if task != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, task, amount_bits);
            lf_checker_rt::callee_thiscall!(2, u32, task, this as u32);
            let active_task = *((this.add(TASK_POINTER)) as *const u32);
            *((active_task as *mut u8).add(STATE_FLAGS) as *mut u32) |= DETACHED_FLAG;
            *((this.add(TASK_POINTER)) as *mut u32) = 0;
        }
        0
    }
});
