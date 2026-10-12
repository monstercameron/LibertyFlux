// original: 0x00E63EB0 reset_audio_object_and_register_cleanup
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E63EB0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18909368u32);
    let call_result_1 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E71670));
    call_result_1
});
