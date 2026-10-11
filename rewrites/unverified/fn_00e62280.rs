// original: 0x00e62280 initialize_network_thread_pool_lock_d

/// Initialize the fourth global network critical section, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62280() -> u32 {
    const LOCK_OBJECT: u32 = 0x01b4a8b0;
    const CLEANUP_CALLBACK: u32 = 0x00e709d0;
    let _lock_result = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(0x01b4a8b0));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
