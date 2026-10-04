// original: 0x00bd9690 SET_TEAM_COLOUR
/// Script native `SET_TEAM_COLOUR` (hash 0x22780707).
///
/// Forwards two script arguments (a team index and a colour) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9690(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
