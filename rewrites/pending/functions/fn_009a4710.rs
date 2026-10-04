// original: 0x009a4710 audio_store_mode_if_ungated
/// Original 0x009a4710 (unnamed): conditionally store a mode byte.
///
/// When the global gate dword is zero, stores the low byte of `mode` into
/// the global mode byte; otherwise leaves it. Returns the mode byte.
export!(stdcall, rw_009a4710(mode: u32) -> u32 {
    let gate = unsafe { (relocated(0x012845C4) as *const u32).read() };
    let v = if gate == 0 {
        (mode & 0xff) as u8
    } else {
        unsafe { (relocated(0x012845C8) as *const u8).read() }
    };
    unsafe { (relocated(0x012845C8) as *mut u8).write(v) };
    mode & 0xff
});
