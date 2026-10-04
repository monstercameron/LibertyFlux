// original: 0x00b9f800 IS_CHAR_HEALTH_GREATER
/// Script native `IS_CHAR_HEALTH_GREATER` (hash 0x7B75036E).
///
/// Reports whether a character's health exceeds a threshold. Forwards two
/// script arguments (the character handle and the threshold) to the engine
/// and stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00b9f800(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
