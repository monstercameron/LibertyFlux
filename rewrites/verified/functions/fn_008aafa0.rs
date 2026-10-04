// original: 0x008aafa0 audio_voice_defaults
/// Reset five voice parameter blocks to `(0, 1.0f, 0)`.
///
/// Writes the triple (dword 0, float 1.0, word 0) at `this+4`, repeating
/// every 12 bytes five times, and returns `this`.
export!(thiscall, rw_008aafa0(this: *mut u8) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F800000;
        for i in 0..5usize {
            let base = this.add(4 + i * 12);
            *(base as *mut u32) = 0;
            *(base.add(4) as *mut u32) = ONE;
            *(base.add(8) as *mut u16) = 0;
        }
        this as u32
    }
});
