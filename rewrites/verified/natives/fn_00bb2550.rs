// original: 0x00bb2550 IS_PLAYER_PRESSING_HORN
/// Script native `IS_PLAYER_PRESSING_HORN` (hash 0x583A7A8B).
///
/// Forwards one script argument (a player index) to the engine horn-state query and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2550(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
