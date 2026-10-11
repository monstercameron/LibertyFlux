// original: 0x0086C7F0 Script_CallEncrypted0

/// Forwards four cdecl stack arguments to the scripted game entry with a
/// fixed relocated context pointer in ECX and a zero fifth stack argument.
/// The wrapper returns the callee's EAX value and leaves argument cleanup to
/// its caller. The encrypted callee's own behavior is outside this wrapper.
lf_checker_rt::export!(cdecl, rw_0086c7f0(arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    const CONTEXT_ADDRESS: u32 = 0x0110_c0a0;
    const CALLEE: u32 = 1;
    let context = lf_checker_rt::relocated(CONTEXT_ADDRESS);
    lf_checker_rt::callee_thiscall!(CALLEE, u32, context, arg0, arg1, arg2, arg3, 0)
});
