// original: 0x00bd8170 NETWORK_CHANGE_GAME_MODE
// Rewrite of the NETWORK_CHANGE_GAME_MODE native handler.

/// Script native `NETWORK_CHANGE_GAME_MODE(...)` (four integers).
///
/// Forwards the four script arguments to the engine game-mode routine and
/// stores the zero-extended low byte of its answer in the context's return
/// slot.
export!(cdecl, rw_bd8170(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3)
        );
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
