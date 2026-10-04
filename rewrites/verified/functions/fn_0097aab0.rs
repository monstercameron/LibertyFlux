// original: 0x0097aab0 audio_listener_create
/// Initialise a freshly allocated audio listener record.
///
/// Resolves the two source ids through the id lookup into the slots at
/// +0x20, then clears the 0x20-byte header and the state flag at +0x28.
/// Returns the record.
export!(thiscall, rw_0097aab0(this: *mut u8, src: *const u32) -> u32 {
    unsafe {
        *this.add(0x28) = 0;
        let mut s = src;
        let mut d = this.add(0x20) as *mut u32;
        for _ in 0..2 {
            *d = callee_cdecl!(1, u32, *s);
            s = s.add(1);
            d = d.add(1);
        }
        core::ptr::write_bytes(this, 0, 0x20);
        this as u32
    }
});
