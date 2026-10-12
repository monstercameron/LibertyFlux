// original: 0x00E6A6E0 initialize_vehicle_tuning_records_four
/// Reproduce initialize_vehicle_tuning_records_four at 0x00e6a6e0. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer for 4 entries from 0x016b9d78 at stride 0x10.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a6e0() -> () {
    const START: u32 = 0x016b9d78;
    const COUNT: usize = 4;
    const STRIDE: u32 = 0x10;
    
    for index in 0..COUNT {
        let entry = START.wrapping_add((index as u32).wrapping_mul(STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(entry));
    }
    
});
