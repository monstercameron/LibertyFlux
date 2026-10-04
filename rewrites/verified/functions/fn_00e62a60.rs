// original: 0x00e62a60 audio_kick_pair_1165880
/// Kick the audio pump object, then register this module's tag.
///
/// Same two-call shape: pump routine on the module object, then the tag
/// sink. Returns the sink's answer.
export!(cdecl, rw_00e62a60() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0116_5880;
        const TAG: u32 = 0x00E7_1120;
        callee_thiscall!(1, u32, relocated(OBJECT));
        callee_cdecl!(2, u32, relocated(TAG))
    }
});
