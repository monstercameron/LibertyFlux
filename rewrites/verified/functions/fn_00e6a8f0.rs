// original: 0x00E6A8F0 reset_vehicle_request_holder_table
/// Reproduce reset_vehicle_request_holder_table at 0x00e6a8f0. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer for 5 entries from 0x016c8d6c at stride 0x8, then register callback 0x00e72a10.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a8f0() -> () {
    const START: u32 = 0x016c8d6c;
    const COUNT: usize = 5;
    const STRIDE: u32 = 0x8;
    const CLEANUP_CALLBACK: u32 = 0x00e72a10;
    for index in 0..COUNT {
        let entry = START.wrapping_add((index as u32).wrapping_mul(STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(entry));
    }
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK));
});
