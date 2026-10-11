// original: 0x00e62c90 initialize_network_state_object_a

/// Initialize the global network pointer state, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62c90() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x0116b250;
    const CLEANUP_CALLBACK: u32 = 0x00e71210;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0116b250));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
