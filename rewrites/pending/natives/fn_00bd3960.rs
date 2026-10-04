// original: 0x00bd3960 DRAW_CHECKPOINT
/// Script native `DRAW_CHECKPOINT` (hash 0x29FC3E19).
///
/// Forwards a checkpoint description to the engine: a position (three
/// floats passed as raw bits), a size word and three colour words.
/// The original builds the position on its own stack frame with SSE moves;
/// only the pushed words are observable, so they are forwarded directly.
/// No return slot is written.
export!(cdecl, rw_00bd3960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6))
    }
});
