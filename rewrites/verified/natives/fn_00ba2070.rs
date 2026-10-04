// original: 0x00ba2070 SET_GROUP_COMBAT_DECISION_MAKER
/// Script native `SET_GROUP_COMBAT_DECISION_MAKER` (hash 0x58123F7A).
///
/// Forwards two script arguments (a group handle and a decision-maker id) to
/// the engine. No return slot is written.
export!(cdecl, rw_00ba2070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
