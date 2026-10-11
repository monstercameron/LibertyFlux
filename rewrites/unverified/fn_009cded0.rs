// original: 0x009cded0 NativeImpl_TRIGGER_VIGILANTE_CRIME

/// Pack the location into a local three-float vector and forward the event identifier and vector to the crime scanner manager. Forward the callee's return register.
lf_checker_rt::export!(cdecl, rw_009cded0(event_id: u32, x: f32, y: f32, z: f32) -> eax {
    const MANAGER_VA: u32 = 0x0128AA90;
    const CALLEE_ID: u32 = 1;
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    let position_ptr = position.as_mut_ptr() as usize as u32;
    unsafe {
        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(MANAGER_VA), event_id, position_ptr)
    }
});
