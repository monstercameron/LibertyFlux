// original: 0x00e629a0 audio_bind_named_pair
/// Bind two named audio resources through the four-argument binder.
///
/// Passes (0, second, 0, first) with this module's binder object: the two
/// zero words are unused option slots. Returns the binder's answer.
export!(cdecl, rw_00e629a0() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0116_1728;
        const FIRST: u32 = 0x00E7_D8FC;
        const SECOND: u32 = 0x00E7_D92C;
        callee_thiscall!(
            1,
            u32,
            relocated(OBJECT),
            0,
            relocated(SECOND),
            0,
            relocated(FIRST)
        )
    }
});
