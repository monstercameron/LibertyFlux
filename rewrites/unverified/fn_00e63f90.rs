// original: 0x00E63F90 register_graphics_setting_shadow_density
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E63F90() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18973788u32, 0u32, lf_checker_rt::relocated(0x00E8B1E4), 0u32, lf_checker_rt::relocated(0x00E8B1A4));
    call_result_0
});
