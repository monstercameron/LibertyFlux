// original: 0x00b8bda0 CHANGE_BLIP_SPRITE
/// Script native `CHANGE_BLIP_SPRITE` (hash 0x6A90123D).
///
/// Forwards 2 script arguments to the engine in order.
/// No return slot is written.
export!(cdecl, rw_00b8bda0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
