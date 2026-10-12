// original: 0x00E6A6A0 initialize_vehicle_state_entry_records
/// Reproduce initialize_vehicle_state_entry_records at 0x00e6a6a0. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer for 16 entries from 0x016c76d0 at stride 0xd0, then register callback 0x00e728a0.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a6a0() -> () {
    const START: u32 = 0x016c76d0;
    const COUNT: usize = 16;
    const STRIDE: u32 = 0xd0;
    const CLEANUP_CALLBACK: u32 = 0x00e728a0;
    for index in 0..COUNT {
        let entry = START.wrapping_add((index as u32).wrapping_mul(STRIDE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(entry));
    }
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK));
});
