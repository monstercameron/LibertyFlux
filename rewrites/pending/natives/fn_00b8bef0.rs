// original: 0x00b8bef0 CLEAR_THIS_BIG_PRINT
/// Script native `CLEAR_THIS_BIG_PRINT` (hash 0x4A4F2699).
///
/// Forwards one script argument (a print handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8bef0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
