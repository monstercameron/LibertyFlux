// original: 0x00b87e10 SET_VIEWPORT_PRIORITY
/// Script native `SET_VIEWPORT_PRIORITY` (hash 0x5DA1752F).
///
/// Forwards two script arguments (a viewport and a priority) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b87e10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
