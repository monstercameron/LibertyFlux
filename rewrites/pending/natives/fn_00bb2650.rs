// original: 0x00bb2650 IS_SCORE_GREATER
/// Script native `IS_SCORE_GREATER` (hash 0x517B7068).
///
/// Forwards two integer script arguments (a player handle and a score)
/// to the engine and stores the low byte of its answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bb2650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
