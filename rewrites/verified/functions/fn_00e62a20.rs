// original: 0x00e62a20 audio_kick_pair_11625ec
/// Kick the audio pump object, then register this module's tag.
///
/// Same two-call shape: pump routine on the module object, then the tag
/// sink. Returns the sink's answer.
export!(cdecl, rw_00e62a20() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0116_25EC;
        const TAG: u32 = 0x00E7_1100;
        callee_thiscall!(1, u32, relocated(OBJECT));
        callee_cdecl!(2, u32, relocated(TAG))
    }
});
