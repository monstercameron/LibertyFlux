// original: 0x00b98650 IS_MOUSE_BUTTON_JUST_PRESSED
/// Script native `IS_MOUSE_BUTTON_JUST_PRESSED` (hash 0x27323E51).
///
/// Forwards a mouse button id to the engine. Stores the low byte of
/// the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00b98650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
