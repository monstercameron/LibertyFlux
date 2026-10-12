// original: 0x00E6AA50 vehicle_tuning_ratio_init_21
/// Reproduce vehicle_tuning_ratio_init_21 at 0x00e6aa50. This zero-argument cdecl routine has no semantic return.
///
/// Read f32 values at 0x01048400 and 0x01048404, divide numerator by denominator in that order, and store the exact result at 0x016c8f90.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6aa50() -> () {
    unsafe {
        const NUM: u32 = 0x01048400;
        const DEN: u32 = 0x01048404;
        const OUT: u32 = 0x016c8f90;
        let numerator = lf_checker_rt::global::<f32>(NUM).read();
        let denominator = lf_checker_rt::global::<f32>(DEN).read();
        let value = core::hint::black_box(numerator) / core::hint::black_box(denominator);
        lf_checker_rt::global::<f32>(OUT).write(value);
    }
});
