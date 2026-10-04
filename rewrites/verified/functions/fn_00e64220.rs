// original: 0x00e64220 audio_callback_register
/// Register the audio callback at 0xE71740 with the registrar helper.
///
/// Pushes the code pointer and calls the registrar (stubbed, cdecl/1).
/// Returns the registrar's answer.
export!(cdecl, rw_00e64220() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE71740)) }
});
