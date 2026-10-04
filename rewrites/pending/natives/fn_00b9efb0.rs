// original: 0x00b9efb0 GET_CHAR_MOVE_ANIM_SPEED_MULTIPLIER
/// Script native `GET_CHAR_MOVE_ANIM_SPEED_MULTIPLIER` (hash 0x325B1A34).
///
/// Forwards two script arguments to the engine. The handler itself writes no
/// return slot (the engine presumably answers through the second argument).
export!(cdecl, rw_00b9efb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
