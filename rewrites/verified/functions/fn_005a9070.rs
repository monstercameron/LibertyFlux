// original: 0x005A9070 control_forward_guard (proposed)

/// Forward to the shared control handler unless the probe says to stay.
///
/// When the bypass flag is set, control jumps straight to the shared handler
/// (a tail call: no return here). Otherwise the probe callee runs with the
/// handler object in ECX and a fixed probe word on the stack; a zero low byte
/// in its answer means "handled", returned as is, and anything else falls
/// through to the same shared handler. The probe word is a plain constant
/// (no relocation entry); the handler object address is relocated.
lf_checker_rt::export!(cdecl, rw_005A9070() -> u32 {
    unsafe {
        const BYPASS: u32 = 0x01160_C3D;
        const HANDLER: u32 = 0x01981_A4C;
        const PROBE_ARG: u32 = 0x3FAA_283B;
        const PROBE: u32 = 1;
        const FORWARD: u32 = 2;
        let flag = (lf_checker_rt::relocated(BYPASS) as *const u8).read();
        if flag != 0 {
            return lf_checker_rt::callee_cdecl!(FORWARD, u32,);
        }
        let answer = lf_checker_rt::callee_thiscall!(
            PROBE,
            u32,
            lf_checker_rt::relocated(HANDLER),
            PROBE_ARG
        );
        if answer & 0xFF == 0 {
            return answer;
        }
        lf_checker_rt::callee_cdecl!(FORWARD, u32,)
    }
});
