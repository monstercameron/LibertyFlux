// original: 0x009cbc60 GET_RADIO_NAME
/// Script native `GET_RADIO_NAME` (hash 0x7EC9580E).
///
/// Forwards one script argument (a radio station index) to the engine.
///
/// Stores the engine answer's full 32 bits into the return slot
/// (`mov`, not `movzx`).
///
export!(cdecl, rw_009cbc60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
