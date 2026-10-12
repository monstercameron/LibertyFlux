// original: 0x009a5960 script_audio_set_conv_flags
/// Updates the conversation flags for the receiver's current script thread.
///
/// A negative thread identifier is ignored. Otherwise both the receiver's
/// owner identifier at offset `0x3ABC` and the supplied thread identifier
/// must match. On a match, the low bytes of the two flag arguments are stored
/// in the conversation flag word. The routine has no declared return value.
const OWNER_THREAD: u32 = 0x3ABC;
const CONVERSATION_FLAGS_VA: u32 = 0x0128_4A50;

export!(thiscall, rw_009a5960(
    this_: u32,
    conversation_enabled: u32,
    speech_enabled: u32,
    thread_id: i32,
) -> () {
    if thread_id < 0 {
        return;
    }
    let current_thread = unsafe {
        (this_.wrapping_add(OWNER_THREAD) as *const u32).read()
    };
    if current_thread != thread_id as u32 {
        return;
    }
    unsafe {
        lf_checker_rt::global::<u8>(CONVERSATION_FLAGS_VA).write(conversation_enabled as u8);
        lf_checker_rt::global::<u8>(CONVERSATION_FLAGS_VA + 1).write(speech_enabled as u8);
    }
});
