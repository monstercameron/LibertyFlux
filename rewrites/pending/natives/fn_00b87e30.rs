// original: 0x00b87e30 SET_VIEWPORT_SHAPE
/// Script native `SET_VIEWPORT_SHAPE` (hash 0x43ED66E3).
///
/// Forwards 2 script arguments to the engine in order.
/// No return slot is written.
export!(cdecl, rw_00b87e30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
