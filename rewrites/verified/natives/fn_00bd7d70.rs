// original: 0x00bd7d70 HAS_PLAYER_COLLECTED_PICKUP
/// Script native `HAS_PLAYER_COLLECTED_PICKUP` (hash 0x025D2170).
///
/// Forwards two script arguments (a player index and a pickup handle) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bd7d70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
