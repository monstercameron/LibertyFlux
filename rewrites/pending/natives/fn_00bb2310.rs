// original: 0x00bb2310 IS_2PLAYER_GAME_GOING_ON
/// Script native `IS_2PLAYER_GAME_GOING_ON` (hash 0x604E1C46).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2310(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
