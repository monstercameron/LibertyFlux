// original: 0x00985c90 audio_activate_emitter
/// Original 0x00985c90 (unnamed): activate an emitter for one source.
///
/// Same as 0x00985bb0 but takes only the position block and skips the final
/// bind step. Returns the handle, or 0 when acquisition fails.
export!(stdcall, rw_00985c90(pos: u32) -> u32 {
    let h = callee_cdecl!(1, u32,);
    if h == 0 {
        return 0;
    }
    callee_thiscall!(2, u32, h, 0, 0x41a00000, 0, 0xfa0, 0x3f000000);
    let mut m = [0u32; 16];
    m[0] = 0x3f800000;
    m[5] = 0x3f800000;
    m[10] = 0x3f800000;
    unsafe {
        m[12] = (pos as *const u32).read();
        m[13] = ((pos + 4) as *const u32).read();
        m[14] = ((pos + 8) as *const u32).read();
        m[15] = ((pos + 12) as *const u32).read();
    }
    callee_thiscall!(3, u32, h, m.as_ptr() as u32);
    h
});
