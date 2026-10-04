// original: 0x00bb1f60 GET_PLAYER_MAX_ARMOUR
/// Script native `GET_PLAYER_MAX_ARMOUR` (hash 0x17265607).
///
/// Forwards two script arguments (a player index and an out-pointer) to the
/// engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bb1f60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
