// original: 0x00ba1f30 SET_DECISION_MAKER_ATTRIBUTE_TARGET_INJURED_REACTION
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_TARGET_INJURED_REACTION` (hash 0x7CAE2557).
///
/// Forwards two script arguments (a decision-maker handle and a flag) to
/// the engine. No return slot is written.
export!(cdecl, rw_00ba1f30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
