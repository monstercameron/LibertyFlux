// original: 0x00897530 aud_environment_sound_release_member

/// Release the object's embedded resource at `+0x88` through the intercepted
/// thiscall helper, then return the original object pointer. The helper's
/// return value is ignored.
export!(thiscall, rw_00897530(audio: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, audio.wrapping_add(0x88));
    audio
});
