// original: 0x00bb2790 PLAYER_HAS_FLASHING_STARS_ABOUT_TO_DROP
/// Script native `PLAYER_HAS_FLASHING_STARS_ABOUT_TO_DROP`
/// (hash 0x69804B35).
///
/// Forwards one script argument (a player index) to the engine, which
/// walks the player wanted state, and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00bb2790(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
