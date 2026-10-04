// original: 0x00ba1ed0 SET_DECISION_MAKER_ATTRIBUTE_RETREATING_BEHAVIOUR
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_RETREATING_BEHAVIOUR`
/// (hash 0x67890049).
///
/// Forwards two script arguments (a decision-maker handle and a flag) to
/// the engine. No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00ba1ed0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
