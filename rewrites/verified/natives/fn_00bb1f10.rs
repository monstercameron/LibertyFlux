// original: 0x00bb1f10 GET_PLAYER_GROUP
/// Script native `GET_PLAYER_GROUP` (hash 0x41AB3C30).
///
/// Forwards two script arguments (a player handle and a result slot for
/// the engine to fill) to the engine. No return slot is written by the
/// handler itself.
export!(cdecl, rw_00bb1f10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
