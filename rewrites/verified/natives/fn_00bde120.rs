// original: 0x00bde120 GET_PLAYER_TO_PLACE_BOMB_IN_CAR
/// Script native `GET_PLAYER_TO_PLACE_BOMB_IN_CAR` (hash 0x17572318).
///
/// Forwards one script argument to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bde120(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
