// original: 0x00985530 audio_voice_reset
/// Reset the voice header: clear the word at `this` and tag `this+4`.
///
/// Stores 0 at `this`, 0xFFFF at `this+4`, and returns `this`.
export!(thiscall, rw_00985530(this: u32) -> u32 {
    unsafe {
        *((this.wrapping_add(4)) as *mut u16) = 0xFFFF;
        *(this as *mut u32) = 0;
        this
    }
});
