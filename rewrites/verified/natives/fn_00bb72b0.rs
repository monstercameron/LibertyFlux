// original: 0x00bb72b0 ALLOW_GAME_TO_PAUSE_FOR_STREAMING
/// Script native `ALLOW_GAME_TO_PAUSE_FOR_STREAMING` (hash 0x085E559E).
///
/// Forwards one boolean script argument (coerced with `arg != 0`) to the
/// engine. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00bb72b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
