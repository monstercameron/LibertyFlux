// original: 0x00ba1e50 SET_DECISION_MAKER_ATTRIBUTE_FIRE_RATE
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_FIRE_RATE` (hash 0x31FC3392).
///
/// Forwards two script arguments (a decision-maker handle and a fire-rate
/// value) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba1e50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
