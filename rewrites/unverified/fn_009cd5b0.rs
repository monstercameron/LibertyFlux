// original: 0x009cd5b0 NativeImpl_REPORT_DISPATCH_2

/// Build a three-float position and forward the dispatch code, a zero flag word, and the position pointer to the dispatch manager. The callee's return register is forwarded.
lf_checker_rt::export!(cdecl, rw_009cd5b0(dispatch_code: u32, x: f32, y: f32, z: f32) -> eax {
    const MANAGER_VA: u32 = 0x0128AA90;
    const CALLEE_ID: u32 = 1;
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    let position_ptr = position.as_mut_ptr() as usize as u32;
    unsafe {
        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(MANAGER_VA), dispatch_code, 0, position_ptr)
    }
});
