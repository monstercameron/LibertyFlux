// original: 0x00e629e0 audio_kick_pair_116182c
/// Kick the audio pump object, then register this module's tag.
///
/// Invokes the pump routine (thiscall/0) on the module's pump object, then
/// the shared tag sink (cdecl/1). Returns the sink's answer.
export!(cdecl, rw_00e629e0() -> u32 {
    unsafe {
        const OBJECT: u32 = 0x0116_182C;
        const TAG: u32 = 0x00E7_10E0;
        callee_thiscall!(1, u32, relocated(OBJECT));
        callee_cdecl!(2, u32, relocated(TAG))
    }
});
