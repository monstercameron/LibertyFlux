// original: 0x00bb1c60 CONVERT_INT_TO_PLAYERINDEX
/// Script native `CONVERT_INT_TO_PLAYERINDEX` (hash 0x5996315E).
///
/// Forwards one script argument (an integer) to the engine and stores
/// its full 32-bit answer into the return slot.
export!(cdecl, rw_00bb1c60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
