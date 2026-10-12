// original: 0x00E62CF0 register_network_global_pointer_initializer_d
/// Runs the configured initialization or registration calls in order, forwarding the original relocated object and callback pointers, and returns the final helper result.
lf_checker_rt::export!(cdecl, rw_00E62CF0() -> u32 {
    let call_result_0 = lf_checker_rt::callee_cdecl!(1u32, u32, lf_checker_rt::relocated(0x00E71250));
    call_result_0
});
