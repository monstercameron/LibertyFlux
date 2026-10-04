// original: 0x00bd8880 NETWORK_IS_PLAYER_MUTED_BY_ME

/// Native handler `NETWORK_IS_PLAYER_MUTED_BY_ME`.
///
/// Report whether a network player is muted by the local player.
/// Forwards 1 argument(s) to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8880(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let _ = arg0;
        // The original keeps only the low byte of the answer.
        let answer = callee_cdecl!(1, u32, arg0);
        *(*ctx as *mut u32) = answer & 0xFF;
        0
    }
});
