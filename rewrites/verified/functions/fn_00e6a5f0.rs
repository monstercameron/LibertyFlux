// original: 0x00E6A5F0 initialize_navigation_entry_table
/// Reproduce initialize_navigation_entry_table at 0x00e6a5f0. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer with global object 0x016b9c30 in ECX, then register callback 0x00e72830.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a5f0() -> () {
    const OBJECT: u32 = 0x016b9c30;
    const CALLBACK: u32 = 0x00e72830;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT));
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK));
    
});
