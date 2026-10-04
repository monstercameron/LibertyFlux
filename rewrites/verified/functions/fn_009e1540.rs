// original: 0x009e1540 audio_voice_install_and_bind
/// When a voice is given, install the voice/count pair and bind
/// this tracker to the voice's owner. Returns the bind answer, else 0.
/// (thiscall/2)
export!(thiscall, rw_009e1540(this: *mut u8, voice: u32, count: u32) -> u32 {
    unsafe {
        if voice == 0 {
            return 0;
        }
        callee_thiscall!(1, u32, this as u32, voice, count);
        let owner = *((this.add(0x30)) as *const u32);
        callee_thiscall!(2, u32, owner, this as u32)
    }
});
