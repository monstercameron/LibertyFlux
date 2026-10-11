// original: 0x00D3A750 CDummyTask_Dead::vf11

/// When the second stack argument equals 2, invoke the task timer helper and return true in AL; otherwise return false. ECX is `this`; the callee removes all three stack arguments.
lf_checker_rt::export!(thiscall, rw_00d3a750(this: u32, first_arg: u32, state: u32, third_arg: u32) -> u32 {
    unsafe {
        const ACTIVE_STATE: u32 = 2;
        const TIMER_HELPER: u32 = 1;
        if state == ACTIVE_STATE {
            let _ = lf_checker_rt::callee_thiscall!(TIMER_HELPER, u32, this);
            1
        } else {
            0
        }
    }
});
