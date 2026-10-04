// original: 0x008ab010 audio_voice_defaults_sub
/// Reset five voice parameter blocks, then reinit the sub-object.
///
/// Same `(0, 1.0f, 0)` pattern as `rw_008aafa0` but at `this+0xc0`, then
/// forwards `this+0xfc` to the shared helper (thiscall/0, stubbed).
/// Returns `this`.
export!(thiscall, rw_008ab010(this: *mut u8) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F800000;
        for i in 0..5usize {
            let base = this.add(0xC0 + i * 12);
            *(base as *mut u32) = 0;
            *(base.add(4) as *mut u32) = ONE;
            *(base.add(8) as *mut u16) = 0;
        }
        callee_thiscall!(1, u32, this.add(0xFC) as u32);
        this as u32
    }
});
