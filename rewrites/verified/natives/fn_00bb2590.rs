// original: 0x00bb2590 IS_PLAYER_SCRIPT_CONTROL_ON
/// Script native `IS_PLAYER_SCRIPT_CONTROL_ON` (hash 0x38861F3A).
///
/// Forwards one script argument (a player index) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2590(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
