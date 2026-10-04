// original: 0x00b86cc0 GET_SCREEN_FADE_ALPHA
/// Script native `GET_SCREEN_FADE_ALPHA` (hash 0x04161E66).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b86cc0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
