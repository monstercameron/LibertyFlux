// original: 0x00a357d0 vehicle_angle_degrees (proposed)

/// Solve four angle floats through the solver callee and convert to degrees.
///
/// Calls the solver (id 1, cdecl/4) with the four argument words unchanged
/// and multiplies its x87 result by 57.295776 (180/π). Cdecl/4 (float
/// bits), returns the product on x87 ST0.
lf_checker_rt::export!(cdecl, rw_00a357d0(a: u32, b: u32, c: u32, d: u32) -> f32 {
    unsafe {
        const TO_DEG: f32 = f32::from_bits(0x4265_2EE0);
        const SOLVER: u32 = 1;
        let t: f32 = lf_checker_rt::callee_cdecl!(SOLVER, f32, a, b, c, d);
        core::hint::black_box(t) * core::hint::black_box(TO_DEG)
    }
});
