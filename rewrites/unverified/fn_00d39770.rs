// original: 0x00D39770 ped_task_stationary_vf0_setup

/// Install this task's vtable, run its two cleanup helpers in order with a zero mode, and forward the base-destructor result. ECX is the task pointer; there are no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d39770(this: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE3754;
        const FIRST_HELPER: u32 = 1;
        const SECOND_HELPER: u32 = 2;
        const BASE_DESTRUCTOR: u32 = 3;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(FIRST_HELPER, u32, this, 0);
        let _ = lf_checker_rt::callee_thiscall!(SECOND_HELPER, u32, this, 0);
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
