// original: 0x00bd88c0 NETWORK_IS_PLAYER_TYPING
/// Script native `NETWORK_IS_PLAYER_TYPING` (hash 0x5AE1245E).
///
/// Forwards a network player index to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00bd88c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
