// original: 0x00898aa0 audio_state_reset_defaults
/// Resets an audio state object to its default control values.
///
/// Runs the nested reset call first, then clears the three control words and
/// restores the enable flag and the unity gain. Returns the object pointer.
export!(thiscall, rw_00898aa0(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        core::ptr::write_unaligned(this.wrapping_add(0x4c) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x50) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x58) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x60) as *mut u32, 1);
        core::ptr::write_unaligned(this.wrapping_add(0x5c) as *mut f32, 1.0);
        this
    }
});
