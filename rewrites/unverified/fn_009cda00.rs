// original: 0x009cda00 NativeImpl_SET_SCRIPT_MIC_LOOK_AT

/// Pack three float arguments into a local vector and pass its address to the script microphone manager's look-at operation. Forward the callee's return register.
lf_checker_rt::export!(cdecl, rw_009cda00(x: f32, y: f32, z: f32) -> eax {
    const MANAGER_VA: u32 = 0x01165880;
    const CALLEE_ID: u32 = 1;
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    let position_ptr = position.as_mut_ptr() as usize as u32;
    unsafe {
        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(MANAGER_VA), position_ptr)
    }
});
