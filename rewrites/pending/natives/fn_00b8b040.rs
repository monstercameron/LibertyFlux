// original: 0x00b8b040 GET_CONTENTS_OF_TEXT_WIDGET
/// Script native `GET_CONTENTS_OF_TEXT_WIDGET` (hash 0x742E3376).
///
/// Forwards one script argument to the engine and stores the engine's full
/// 32-bit answer into the return slot.
export!(cdecl, rw_00b8b040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
