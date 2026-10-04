// original: 0x00bb6730 SET_STAT_FRONTEND_VISIBLE_AFTER_INCREMENTED
/// Script native `SET_STAT_FRONTEND_VISIBLE_AFTER_INCREMENTED`
/// (hash 0x12D67ADA).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bb6730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
