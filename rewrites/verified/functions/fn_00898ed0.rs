// original: 0x00898ed0 audio_set_gain_and_notify
/// Stores a gain level and notifies the audio pool.
///
/// Writes the level into the object's gain slot, then runs the voice setup
/// call against the pool with the level bits and the set flag, returning the
/// call's answer.
export!(thiscall, rw_00898ed0(this: u32, level: f32) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.wrapping_add(0x5c) as *mut f32, level);
        let pool = core::ptr::read_unaligned(global::<u32>(0x115F810));
        callee_thiscall!(1, u32, pool, level.to_bits(), 1)
    }
});
