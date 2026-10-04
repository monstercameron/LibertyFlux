// original: 0x00b8c950 GET_LENGTH_OF_STRING_WITH_THIS_TEXT_LABEL
/// Script native `GET_LENGTH_OF_STRING_WITH_THIS_TEXT_LABEL` (hash 0x6D795EC0).
///
/// Forwards one script argument (a text label) to the engine and stores
/// its full 32-bit answer (the resolved string length) into the return slot.
export!(cdecl, rw_00b8c950(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
