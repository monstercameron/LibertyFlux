// original: 0x00b8bdc0 CHANGE_BLIP_TEAM_RELEVANCE
/// Script native `CHANGE_BLIP_TEAM_RELEVANCE` (hash 0x4B2625BE).
///
/// Blip handle + team flag.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00b8bdc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
