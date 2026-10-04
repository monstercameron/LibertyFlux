// original: 0x00b985f0 IS_KEYBOARD_KEY_JUST_PRESSED
/// Script native `IS_KEYBOARD_KEY_JUST_PRESSED` (hash 0x75C9772B).
///
/// Forwards one script argument (a key code) to the engine input query and
/// stores the low byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_00b985f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32, *args);
        *slot = answer & 0xFF;
        slot as u32
    }
});
