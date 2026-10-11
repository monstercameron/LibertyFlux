// original: 0x00e62140 initialize_network_thread_pool_lock_b

/// Initialize the second global network critical section, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62140() -> u32 {
    const LOCK_OBJECT: u32 = 0x01b498d8;
    const CLEANUP_CALLBACK: u32 = 0x00e70950;
    let _lock_result = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(0x01b498d8));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
