// original: 0x00bb8a20 CLEAR_EVENT_PRECEDENCE
/// Script native `CLEAR_EVENT_PRECEDENCE` (hash 0x1CC41C5E).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb8a20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
