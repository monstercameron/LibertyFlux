// original: 0x00bd9610 SET_RICH_PRESENCE_TEMPLATESP1
/// Script native `SET_RICH_PRESENCE_TEMPLATESP1` (hash 0x00132487).
///
/// Forwards three script arguments (presence template parameters) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd9610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
