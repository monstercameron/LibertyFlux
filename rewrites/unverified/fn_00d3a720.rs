// original: 0x00D3A720 CDummyTask_FallDownAndGetUp::vf11

/// When the second stack argument equals 2, invoke the task timer helper, set the task state word at +0x28 to 3, and return true in AL; otherwise return false. ECX is `this` and the callee removes all three stack arguments.
lf_checker_rt::export!(thiscall, rw_00d3a720(this: u32, first_arg: u32, state: u32, third_arg: u32) -> u32 {
    unsafe {
        const ACTIVE_STATE: u32 = 2;
        const COMPLETE_STATE: u32 = 3;
        const TIMER_HELPER: u32 = 1;
        if state == ACTIVE_STATE {
            let _ = lf_checker_rt::callee_thiscall!(TIMER_HELPER, u32, this);
            (this.wrapping_add(0x28) as *mut u32).write_unaligned(COMPLETE_STATE);
            1
        } else {
            0
        }
    }
});
