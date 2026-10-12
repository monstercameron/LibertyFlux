// original: 0x00E6AA70 initialize_vehicle_slot_tables
/// Reproduce initialize_vehicle_slot_tables at 0x00e6aa70. This zero-argument cdecl routine has no semantic return.
///
/// Initialize 256 list heads at stride 0x44, then write 128 all-ones sentinel dwords at stride 0x30.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6aa70() -> () {
    unsafe {
        const LIST_BASE: u32 = 0x016c8ff0;
        const LIST_COUNT: usize = 256;
        const LIST_STRIDE: u32 = 0x44;
        const SENTINEL_BASE: u32 = 0x016cd3c8;
        const SENTINEL_COUNT: usize = 128;
        const SENTINEL_STRIDE: u32 = 0x30;
        for index in 0..LIST_COUNT {
            let entry = LIST_BASE.wrapping_add((index as u32).wrapping_mul(LIST_STRIDE));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(entry));
        }
        for index in 0..SENTINEL_COUNT {
            let slot = SENTINEL_BASE.wrapping_add((index as u32).wrapping_mul(SENTINEL_STRIDE));
            lf_checker_rt::global::<u32>(slot).write(u32::MAX);
        }
    }
});
