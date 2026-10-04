// original: 0x00bb66d0 SET_STAT_FRONTEND_ALWAYS_VISIBLE
/// Script native `SET_STAT_FRONTEND_ALWAYS_VISIBLE` (hash 0x656F1A7A).
///
/// Forwards one script argument (a stat index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb66d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
