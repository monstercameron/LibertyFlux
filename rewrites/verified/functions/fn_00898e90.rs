// original: 0x00898e90 audio_notify_current
/// Notifies the audio pool of the current selector position.
///
/// Does nothing and returns the object pointer when no pool is registered;
/// otherwise passes the word at offset 0x62 to the pool and returns its answer.
export!(thiscall, rw_00898e90(this: u32) -> u32 {
    unsafe {
        let pool = core::ptr::read_unaligned(global::<u32>(0x115F810));
        if pool == 0 {
            return this;
        }
        let sel = core::ptr::read_unaligned(this.wrapping_add(0x62) as *const u16) as u32;
        callee_thiscall!(1, u32, pool, sel)
    }
});
