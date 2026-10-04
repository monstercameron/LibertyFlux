// original: 0x00bb6360 GET_GAMER_NETWORK_SCORE
/// Script native `GET_GAMER_NETWORK_SCORE`.
///
/// Forwards three script arguments to the engine and stores the full 32-bit
/// answer into the return slot. Returns the engine answer.
export!(cdecl, rw_00bb6360(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

