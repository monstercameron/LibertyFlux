// original: 0x008ABD10 audio_notify_event
/// Notify the audio manager: forward the live backend handle with the
/// fixed event class and id, returning the manager's answer.
export!(cdecl, rw_008ABD10() -> u32 {
    unsafe {
        let handle = *(global::<u32>(0x0115DEA8) as *const u32);
        callee_stdcall!(1, u32, handle, 1, 0x0B)
    }
});
