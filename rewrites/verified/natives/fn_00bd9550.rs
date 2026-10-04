// original: 0x00bd9550 SET_RICH_PRESENCE_TEMPLATEMP1
/// Script native `SET_RICH_PRESENCE_TEMPLATEMP1` (hash 0x6C236A54).
///
/// Forwards four script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd9550(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
