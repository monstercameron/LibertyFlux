// original: 0x009a59c0 script_audio_set_chase_flags
/// Sets the chase-audio flags for the receiver's current script thread.
///
/// A negative thread identifier or an identifier that differs from the
/// receiver's owner at offset `0x3ABC` leaves the global flags unchanged. A
/// match stores the low byte of `enabled` in two chase flags and the related
/// conversation state byte. The routine has no declared return value.
const OWNER_THREAD: u32 = 0x3ABC;
const CHASE_FLAG_A_VA: u32 = 0x0128_3449;
const CHASE_FLAG_B_VA: u32 = 0x0128_344A;
const CONVERSATION_STATE_VA: u32 = 0x0128_4A53;

export!(thiscall, rw_009a59c0(this_: u32, enabled: u32, thread_id: i32) -> () {
    if thread_id < 0 {
        return;
    }
    let current_thread = unsafe {
        (this_.wrapping_add(OWNER_THREAD) as *const u32).read()
    };
    if current_thread != thread_id as u32 {
        return;
    }
    let enabled = enabled as u8;
    unsafe {
        lf_checker_rt::global::<u8>(CHASE_FLAG_A_VA).write(enabled);
        lf_checker_rt::global::<u8>(CHASE_FLAG_B_VA).write(enabled);
        lf_checker_rt::global::<u8>(CONVERSATION_STATE_VA).write(enabled);
    }
});
