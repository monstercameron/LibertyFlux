// original: 0x00bb1cf0 DISABLE_PLAYER_SPRINT
/// Script native `DISABLE_PLAYER_SPRINT` (hash 0x3A244927).
///
/// Forwards a player index and a boolean flag (`arg != 0`) to the engine.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching. No return slot is written.
export!(cdecl, rw_00bb1cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
