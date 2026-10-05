// original: 0x008C7E30 stream_request_pump
/// Pump one streaming request when the request state is active.
///
/// When the state global reads 2 the request callee runs: a nonzero answer
/// dispatches the pending queue with mode 3, a zero answer with a set
/// stalled flag clears the flag, and a zero answer with a clear flag runs
/// the restart callee first. The two poll callees always run; the result
/// is 2 on the stalled/restart paths and 0 otherwise.
/// Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c7e30() -> u32 {
    unsafe {
        const REQ_CALLEE: u32 = 1;
        const DISPATCH_CALLEE: u32 = 2;
        const RESTART_CALLEE: u32 = 3;
        const POLL_A_CALLEE: u32 = 4;
        const POLL_B_CALLEE: u32 = 5;
        const DISPATCH_QUEUE_FILE_VA: u32 = 0x11732E8;
        const REQ_BASE_FILE_VA: u32 = 0x1172FE0;
        if *lf_checker_rt::global::<u32>(0x11730F8) != 2 {
            lf_checker_rt::callee_cdecl!(POLL_A_CALLEE, u32,);
            lf_checker_rt::callee_cdecl!(POLL_B_CALLEE, u32,);
            return 0;
        }
        let base = lf_checker_rt::relocated(REQ_BASE_FILE_VA);
        let pending: u32 =
            lf_checker_rt::callee_cdecl!(REQ_CALLEE, u32, base, 0);
        if pending != 0 {
            let queue = lf_checker_rt::relocated(DISPATCH_QUEUE_FILE_VA);
            lf_checker_rt::callee_thiscall!(DISPATCH_CALLEE, u32, queue, 3);
        } else if *lf_checker_rt::global::<u8>(0x1173247) != 0 {
            *lf_checker_rt::global::<u8>(0x1173247) = 0;
        } else {
            lf_checker_rt::callee_cdecl!(RESTART_CALLEE, u32, 0x1D);
        }
        lf_checker_rt::callee_cdecl!(POLL_A_CALLEE, u32,);
        lf_checker_rt::callee_cdecl!(POLL_B_CALLEE, u32,);
        if pending != 0 {
            0
        } else {
            2
        }
    }
});
