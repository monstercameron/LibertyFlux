// original: 0x00b984f0 IS_GAME_KEYBOARD_KEY_PRESSED
/// Script native `IS_GAME_KEYBOARD_KEY_PRESSED` (hash 0x5FA96262).
///
/// Forwards one script argument (a key code) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b984f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
