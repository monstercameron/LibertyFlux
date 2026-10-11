// original: 0x00e625f0 register_network_object_cleanup

/// Register the network object cleanup callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e625f0() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70f30;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e70f30))
});
