// original: 0x00bd89c0 NETWORK_JOIN_GAME_CNC
/// Script native `NETWORK_JOIN_GAME_CNC` (hash 0x358F40E9).
///
/// Forwards arg0, arg1, arg2 to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
export!(cdecl, rw_00bd89c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
