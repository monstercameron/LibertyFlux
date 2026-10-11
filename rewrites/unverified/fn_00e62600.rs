// original: 0x00e62600 initialize_network_state_object_a

/// Initialize the first global network state object, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62600() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x01154068;
    const CLEANUP_CALLBACK: u32 = 0x00e70f50;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x01154068));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
