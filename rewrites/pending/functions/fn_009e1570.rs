// original: 0x009e1570 audio_voice_handle_resolve
/// Resolve the voice handle for the argument through the table,
/// then attach it. Returns 1 on success, 0 when there is no voice or the
/// handle is invalid. Only AL is defined. (thiscall/1)
export!(thiscall, rw_009e1570(this: *mut u8, a: u32) -> u32 {
    unsafe {
        let voice = *((this.add(0x30)) as *const u32);
        if voice == 0 {
            return 0;
        }
        let h = callee_thiscall!(1, u32, voice, a);
        if h == 0xFFFF_FFFF {
            return 0;
        }
        callee_thiscall!(2, u32, this as u32, h);
        1
    }
});
