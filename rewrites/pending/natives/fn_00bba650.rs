// original: 0x00bba650 TASK_SET_COMBAT_DECISION_MAKER
/// Script native `TASK_SET_COMBAT_DECISION_MAKER` (hash 0x499C0C01).
///
/// Forwards two script arguments (a character handle and a decision-maker handle) to the engine. No return slot is written.
export!(cdecl, rw_00bba650(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
