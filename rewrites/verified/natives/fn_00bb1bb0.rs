// original: 0x00bb1bb0 CHANGE_PLAYER_PHONE_MODEL
/// Script native `CHANGE_PLAYER_PHONE_MODEL` (hash 0x7F2A71FD).
///
/// Forwards two script arguments (a player index and a phone model) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bb1bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
