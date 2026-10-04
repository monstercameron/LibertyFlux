// original: 0x00BA0310 LOAD_COMBAT_DECISION_MAKER_EVENT_RESPONSE
/// F13 LOAD_COMBAT_DECISION_MAKER_EVENT_RESPONSE: forwards 2 args, void.
export!(cdecl, rn10_load_combat_dmer(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args, *args.add(1));
        0
    }
});
