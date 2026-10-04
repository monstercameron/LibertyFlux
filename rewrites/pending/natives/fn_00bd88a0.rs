// original: 0x00bd88a0 NETWORK_IS_PLAYER_TALKING
/// Script native `NETWORK_IS_PLAYER_TALKING` (hash 0x544625D9).
///
/// Forwards one script argument (a player index) to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd88a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
