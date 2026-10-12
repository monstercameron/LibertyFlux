// original: 0x00E63FB0 register_graphics_setting_shadow_quality
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E63FB0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, lf_checker_rt::relocated(0x01218420), 0u32, lf_checker_rt::relocated(0x00E8B224), 0u32, lf_checker_rt::relocated(0x00E8B1F4));
    call_result_0
});
