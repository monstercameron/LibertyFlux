// original: 0x00E62D30 initialize_network_state_object_d
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E62D30() -> u32 {
    let call_result_0 = lf_checker_rt::callee_thiscall!(1u32, u32, 18265800u32);
    let call_result_1 = lf_checker_rt::callee_cdecl!(2u32, u32, lf_checker_rt::relocated(0x00E71280));
    call_result_1
});
