// original: 0x00bb6630 REGISTER_STRING_FOR_FRONTEND_STAT
/// Script native `REGISTER_STRING_FOR_FRONTEND_STAT` (hash 0x3C295451).
///
/// Forwards a string hash and a stat slot to the engine. No return slot is
/// written.
export!(cdecl, rw_00bb6630(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        )
    }
});
