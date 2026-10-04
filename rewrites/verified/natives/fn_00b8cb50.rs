// original: 0x00b8cb50 GET_STRING_FROM_TEXT_FILE
/// Script native `GET_STRING_FROM_TEXT_FILE` (hash 0x332F0E9A).
///
/// Forwards one script argument (a text label hash) to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00b8cb50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
