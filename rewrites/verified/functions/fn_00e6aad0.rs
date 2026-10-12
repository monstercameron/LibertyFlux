// original: 0x00E6AAD0 initialize_cable_car_name_hash
/// Reproduce initialize_cable_car_name_hash at 0x00e6aad0. This zero-argument cdecl routine has no semantic return.
///
/// Hash the read-only string at 0x00ec6558 with the direct helper and store its return value at 0x016cf3bc.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6aad0() -> () {
    unsafe {
        const NAME: u32 = 0x00ec6558;
        const OUT: u32 = 0x016cf3bc;
        let hash = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(NAME), 0);
        lf_checker_rt::global::<u32>(OUT).write(hash);
    }
});
