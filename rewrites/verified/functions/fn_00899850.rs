// original: 0x00899850 audio_buffer_init
/// Initialises an audio buffer descriptor and runs its setup call.
///
/// Stores the limit, clears the counters, records the kind byte, then
/// initialises the entry through the setup call, returning the call's answer.
export!(thiscall, rw_00899850(this: u32, entry: u32, kind: u32, limit: u32) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.wrapping_add(0x38) as *mut u32, limit);
        for off in [0x3cu32, 0, 8, 0x0c, 4, 0x20, 0x1c, 0x28, 0x40, 0x34, 0x2c, 0x30] {
            core::ptr::write_unaligned(this.wrapping_add(off) as *mut u32, 0);
        }
        // Only the low byte of the kind word is significant.
        core::ptr::write_unaligned(this.wrapping_add(0x44) as *mut u8, (kind & 0xFF) as u8);
        callee_thiscall!(1, u32, this, entry)
    }
});
