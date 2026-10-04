// original: 0x00ba1e30 SET_DECISION_MAKER_ATTRIBUTE_CAUTION
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_CAUTION` (hash 0x6BAC2781).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00ba1e30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
