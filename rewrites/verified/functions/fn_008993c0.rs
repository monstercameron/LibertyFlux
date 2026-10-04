// original: 0x008993c0 audio_header_clear
/// Clears an audio header: two words and one flag byte, returning the object.
export!(thiscall, rw_008993c0(this: u32) -> u32 {
    unsafe {
        core::ptr::write_unaligned(this.wrapping_add(0x14) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x10) as *mut u32, 0);
        core::ptr::write_unaligned(this.wrapping_add(0x1b) as *mut u8, 0);
        this
    }
});
