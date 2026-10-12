// original: 0x00E63B90 reset_counter_state_a
/// Forwards to the configured object method with its relocated object pointer and returns the method result.
lf_checker_rt::export!(cdecl, rw_00E63B90() -> u32 {
    let result = lf_checker_rt::callee_thiscall!(1u32, u32, lf_checker_rt::relocated(0x011F6598));
    result
});
