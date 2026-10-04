// original: 0x00bb27b0 PLAYER_HAS_FLASHING_STAR_AFTER_OFFENCE
/// Script native `PLAYER_HAS_FLASHING_STAR_AFTER_OFFENCE` (hash 0x070F287A).
///
/// Forwards one script argument (a player index) to the engine and stores the
/// low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb27b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
