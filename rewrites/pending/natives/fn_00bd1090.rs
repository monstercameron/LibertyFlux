// original: 0x00bd1090 HAS_CHAR_GOT_WEAPON
/// Script native `HAS_CHAR_GOT_WEAPON` (hash 0x11F759DE).
///
/// Forwards 2 script arguments (a character handle and a weapon id) to the engine.
/// Stores the low byte of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd1090(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
