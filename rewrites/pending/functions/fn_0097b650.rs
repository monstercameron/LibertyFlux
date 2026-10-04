// original: 0x0097B650 NativeImpl_HANDLE_AUDIO_ANIM_EVENT
/// Handle a scripted audio animation event: forward id and field to the audio core.
///
/// thiscall(obj, event_id).
export!(thiscall, rw_s103_97b650(obj: *const u8, arg0: u32) -> u32 {
    unsafe {
        let field = *(((obj as usize) + 0x120) as *const u32);
        callee_thiscall!(1, u32, relocated(0x12315D0), arg0, field)
    }
});
