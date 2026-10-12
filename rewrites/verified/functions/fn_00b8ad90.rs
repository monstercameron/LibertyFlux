// original: 0x00B8AD90 NativeImpl_IS_POS_IN_CUTSCENE_BLOCKING_BOUNDS

/// Test a point against the cutscene blocking-bounds helper and return its
/// full status word.
lf_checker_rt::export!(cdecl, rw_00b8ad90(x_bits: u32, y_bits: u32, z_bits: u32) -> u32 {
    let point = [x_bits, y_bits, z_bits];
    lf_checker_rt::callee_cdecl!(1, u32, point.as_ptr() as u32)
});
