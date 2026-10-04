// original: 0x00b9e650 COPY_GROUP_COMBAT_DECISION_MAKER
/// Script native `COPY_GROUP_COMBAT_DECISION_MAKER` (hash 0x17002E03).
///
/// Forwards 2 script argument(s) to the engine: 2 integer(s).
/// No return slot is written.
export!(cdecl, rw_00b9e650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
