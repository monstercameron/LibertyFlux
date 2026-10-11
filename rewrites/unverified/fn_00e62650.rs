// original: 0x00e62650 initialize_light_attribute_state

/// Initialize the global light-attribute state object, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62650() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x0115a400;
    const CLEANUP_CALLBACK: u32 = 0x00e70f90;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0115a400));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
