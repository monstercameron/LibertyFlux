// original: 0x00d26a60 ped_stat_threshold (proposed)

/// Test whether a callee-provided float reaches 0.9.
///
/// Calls the stat helper (intercepted) with `arg`; returns 1 when the result
/// is greater than or equal to 0.9, else 0. NaN compares below, so it
/// returns 0.
///
/// Original: 0x00D26A60 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00d26a60(arg: u32) -> u32 {
    unsafe {
        const LIMIT_BITS: u32 = 0x3f66_6666; // 0.9f
        let f: f32 = lf_checker_rt::callee_cdecl!(1, f32, arg);
        if f >= f32::from_bits(LIMIT_BITS) { 1 } else { 0 }
    }
});
