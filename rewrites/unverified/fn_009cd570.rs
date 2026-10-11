// original: 0x009cd570 NativeImpl_REPORT_CRIME_2

/// Build a three-float position from the first three arguments and forward it with the final crime code to the crime manager. The callee's return register is forwarded.
lf_checker_rt::export!(cdecl, rw_009cd570(x: f32, y: f32, z: f32, crime_code: u32) -> eax {
    const MANAGER_VA: u32 = 0x0128AA90;
    const CALLEE_ID: u32 = 1;
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    let position_ptr = position.as_mut_ptr() as usize as u32;
    unsafe {
        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(MANAGER_VA), position_ptr, crime_code)
    }
});
