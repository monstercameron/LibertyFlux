// original: 0x00ba1eb0 SET_DECISION_MAKER_ATTRIBUTE_NAVIGATION_STYLE
/// Script native `SET_DECISION_MAKER_ATTRIBUTE_NAVIGATION_STYLE` (hash 0x26A1722C).
///
/// Forwards two script arguments (a decision-maker handle and a style value) to the engine. No return slot is written.
export!(cdecl, rw_00ba1eb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
