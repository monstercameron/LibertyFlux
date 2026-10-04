// original: 0x00bb28c0 SET_DRAW_PLAYER_COMPONENT
/// Script native `SET_DRAW_PLAYER_COMPONENT` (hash 0x3EFE3DC8).
///
/// Forwards a player component id and a boolean flag to the engine. The
/// flag is coerced with `arg != 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bb28c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag1 = u32::from(*args.add(1) != 0);
        let q1 = (ctx as u32 & 0xFFFF_FF00) | flag1;
        callee_cdecl!(
            1,
            u32,
            *args,
            q1,
        )
    }
});
