// original: 0x00bd95c0 SET_RICH_PRESENCE_TEMPLATEMP5
/// Script native `SET_RICH_PRESENCE_TEMPLATEMP5` (hash 0x314F6DD3).
///
/// Forwards three template values to the engine. No return slot is written.
export!(cdecl, rw_00bd95c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
