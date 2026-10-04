// original: 0x00bd8b30 NETWORK_PLAYER_HAS_KEYBOARD
/// Script native `NETWORK_PLAYER_HAS_KEYBOARD` (hash 0x04FE5C34).
///
/// Forwards one script argument (a network player index) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8b30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
