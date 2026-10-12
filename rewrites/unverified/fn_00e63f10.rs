// original: 0x00E63F10 register_graphics_setting_automatic_quality_adjustment
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E63F10() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18973608u32, 0u32, lf_checker_rt::relocated(0x00E8B234), 0u32, lf_checker_rt::relocated(0x00E8B240));
    call_result_0
});
