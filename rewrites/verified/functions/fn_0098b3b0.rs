// original: 0x0098b3b0 audVehicleAudioEntity::impact_audio_allowed
/// Determines whether the vehicle impact audio path is available.
///
/// The receiver points to a voice and engine. A result of one requires both
/// objects, a clear engine status word at byte offset `0x1304`, and a voice
/// flags word at offset five whose `0x0030_0000` bits are not exactly
/// `0x0010_0000`. A missing object or a blocked state returns zero.
const VOICE_LINK: u32 = 0x9E0;
const ENGINE_LINK: u32 = 0x820;
const ENGINE_STATUS: u32 = 0x1304;
const VOICE_FLAGS: u32 = 5;
const BLOCKED_FLAGS: u32 = 0x0030_0000;
const BLOCKED_VALUE: u32 = 0x0010_0000;

export!(thiscall, rw_0098b3b0(this_: u32) -> u8 {
    let voice = unsafe {
        (this_.wrapping_add(VOICE_LINK) as *const u32).read()
    };
    if voice == 0 {
        return 0;
    }
    let engine = unsafe {
        (this_.wrapping_add(ENGINE_LINK) as *const u32).read()
    };
    if engine == 0 {
        return 0;
    }
    let status = unsafe {
        (engine.wrapping_add(ENGINE_STATUS) as *const u32).read()
    };
    if status != 0 {
        return 0;
    }
    let flags = unsafe {
        (voice.wrapping_add(VOICE_FLAGS) as *const u32).read_unaligned()
    };
    u8::from(flags & BLOCKED_FLAGS != BLOCKED_VALUE)
});
