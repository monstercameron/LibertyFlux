// original: 0x00E64310 initialize_audio_link_head_c
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E64310() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, lf_checker_rt::relocated(0x01231548));
    let call_result_1 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E717D0));
    call_result_1
});
