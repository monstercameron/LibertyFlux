// original: 0x00bb26d0 LIMIT_TWO_PLAYER_DISTANCE
/// Script native `LIMIT_TWO_PLAYER_DISTANCE` (hash 0x50AD1F3E).
///
/// Forwards one script argument (a float bit-pattern, moved through an SSE
/// register) to the engine. No return slot is written.
export!(cdecl, rw_00bb26d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
