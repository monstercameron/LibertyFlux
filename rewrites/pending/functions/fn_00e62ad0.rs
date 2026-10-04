// original: 0x00e62ad0 audio_kick_pair_1168b20
/// Kick the audio pump object, then register this module's tag.
///
/// Same two-call shape: pump routine on the module object, then the tag
/// sink. Returns the sink's answer.
export!(cdecl, rw_00e62ad0() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0116_8B20;
        const TAG: u32 = 0x00E7_1160;
        callee_thiscall!(1, u32, relocated(OBJECT));
        callee_cdecl!(2, u32, relocated(TAG))
    }
});
