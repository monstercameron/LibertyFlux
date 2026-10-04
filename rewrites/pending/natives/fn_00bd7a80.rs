// original: 0x00bd7a80 GET_LOCAL_GAMERLEVEL_FROM_PROFILESETTINGS
/// Script native `GET_LOCAL_GAMERLEVEL_FROM_PROFILESETTINGS` (hash 0x7C5F327E).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7a80(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
