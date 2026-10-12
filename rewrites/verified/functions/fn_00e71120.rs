// original: 0x00E71120 timing_buffer_release

/// Tail-forwards to the timing release routine with the fixed object
/// address in ECX. The target's EAX result is returned unchanged.
lf_checker_rt::export!(cdecl, rw_00e71120() -> u32 {
    const RECEIVER_VA: u32 = 0x01165880;
    let receiver = lf_checker_rt::relocated(RECEIVER_VA);
    lf_checker_rt::callee_thiscall!(1, u32, receiver)
});
