// original: 0x00bd7f50 IS_OUR_PLAYER_HIGHER_PRIORITY_FOR_CAR_GENERATION
/// Script native `IS_OUR_PLAYER_HIGHER_PRIORITY_FOR_CAR_GENERATION`
/// (hash 0x504E03FC).
///
/// Forwards one script argument (a player index) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7f50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
