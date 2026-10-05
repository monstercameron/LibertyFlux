// original: 0x00a358d0 vehicle_angle_index (proposed)

/// Map two angle floats to a table index through two callees.
///
/// Calls the angle solver (id 1, cdecl/4) with `(a, b, 0.0, 0.0)`, parks
/// its x87 result in a scratch slot, and passes it as the single argument
/// to the indexer (id 2, cdecl/1). Cdecl/2 (float bits), returns the
/// indexer's answer.
lf_checker_rt::export!(cdecl, rw_00a358d0(a_bits: u32, b_bits: u32) -> u32 {
    unsafe {
        const SOLVER: u32 = 1;
        const INDEXER: u32 = 2;
        let t: f32 = lf_checker_rt::callee_cdecl!(SOLVER, f32, a_bits, b_bits, 0, 0);
        lf_checker_rt::callee_cdecl!(INDEXER, u32, t.to_bits())
    }
});
