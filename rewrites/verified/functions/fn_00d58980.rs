// original: 0x00d58980 CCamViewSeq::vf4
/// Decide whether this sequence may run: a ready child blocks it, otherwise
/// a positive poll count lets it through.
///
/// Calls the child-ready check (intercepted callee 1); when it answers
/// nonzero the sequence shuts itself down (intercepted callee 2, argument 0)
/// and returns 0. Otherwise it polls (intercepted callee 3, argument 1): a
/// positive answer returns 1, anything else shuts the sequence down the
/// other way (intercepted callee 4, argument 0) and returns 0.
///
/// Original: thiscall, no stack arguments; each path ends by forcing `al`.
lf_checker_rt::export!(thiscall, rw_00d58980 (this: u32) -> u32 {
    unsafe {
        const READY_CHECK: u32 = 1;
        const SHUTDOWN_A: u32 = 2;
        const POLL: u32 = 3;
        const SHUTDOWN_B: u32 = 4;
        let ready: u32 = lf_checker_rt::callee_thiscall!(READY_CHECK, u32, this);
        if ready & 0xFF != 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(SHUTDOWN_A, u32, this, 0);
            return ans & 0xFFFFFF00;
        }
        let count: u32 = lf_checker_rt::callee_thiscall!(POLL, u32, this, 1);
        if (count as i32) > 0 {
            (count & 0xFFFFFF00) | 1
        } else {
            let ans: u32 = lf_checker_rt::callee_thiscall!(SHUTDOWN_B, u32, this, 0);
            ans & 0xFFFFFF00
        }
    }
});
