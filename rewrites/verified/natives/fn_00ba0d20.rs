// original: 0x00ba0d20 REMOVE_DECISION_MAKER
/// Script native `REMOVE_DECISION_MAKER` (hash 0x47147EC5).
///
/// Forwards one script argument (a decision-maker handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba0d20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
