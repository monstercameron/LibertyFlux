// original: 0x00977430 audio_collision_voice_is_enabled

/// Return whether the object's collision-audio flag bit is set. The flag
/// word is at byte offset `0x120`; the object pointer arrives in ECX and the
/// result is a zero or one in EAX.
lf_checker_rt::export!(thiscall, rw_00977430(object: u32) -> u32 {
    const AUDIO_FLAGS: u32 = 0x120;
    const COLLISION_VOICE_ENABLED: u32 = 0x2;

    let flags = unsafe { (object.wrapping_add(AUDIO_FLAGS) as *const u32).read_unaligned() };
    u32::from(flags & COLLISION_VOICE_ENABLED != 0)
});
