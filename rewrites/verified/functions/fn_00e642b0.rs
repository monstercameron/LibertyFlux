// original: 0x00E642B0 initialize_audio_link_head_a
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E642B0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, lf_checker_rt::relocated(0x012313F0));
    let call_result_1 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E717A0));
    call_result_1
});
