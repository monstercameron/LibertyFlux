// original: 0x00e62a80 audio_emit_tag_e71130
/// Register this module's tag word with the audio tag sink.
///
/// Pushes the module tag address and invokes the shared sink (cdecl/1),
/// returning the sink's answer.
export!(cdecl, rw_00e62a80() -> u32 {
    unsafe {
        const TAG: u32 = 0x00E7_1130;
        callee_cdecl!(1, u32, relocated(TAG))
    }
});
