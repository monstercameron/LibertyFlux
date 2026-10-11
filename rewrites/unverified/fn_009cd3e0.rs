// original: 0x009cd3e0 NativeImpl_PLAY_SOUND_FROM_POSITION

/// Forward a sound identifier, a name pointer, and a three-float position to the audio manager. The position is passed by pointer to a temporary local vector; the callee's return register is forwarded.
lf_checker_rt::export!(cdecl, rw_009cd3e0(sound_id: u32, sound_name: u32, x: f32, y: f32, z: f32) -> u32 {
    const MANAGER_VA: u32 = 0x01284A60;
    const CALLEE_ID: u32 = 1;
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    let position_ptr = position.as_mut_ptr() as usize as u32;
    unsafe {
        lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(MANAGER_VA), sound_id, sound_name, position_ptr)
    }
});
