// original: 0x00b94f60 SET_PHONE_HUD_ITEM
/// Script native `SET_PHONE_HUD_ITEM` (hash 0x43A13718).
///
/// Forwards three script arguments to the engine. No return slot is written
/// by the handler itself.
export!(cdecl, rw_00b94f60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
