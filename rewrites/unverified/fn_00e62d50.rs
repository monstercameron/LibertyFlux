// original: 0x00E62D50 register_rgl_language_override_setting
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E62D50() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18268764u32, 0u32, lf_checker_rt::relocated(0x00E7FB20), 0u32, lf_checker_rt::relocated(0x00E7FAF4));
    call_result_0
});
