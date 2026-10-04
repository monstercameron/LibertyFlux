// original: 0x00b9a4c0 GENERATE_DIRECTIONS
/// Script native `GENERATE_DIRECTIONS` (hash 0x203A137B).
///
/// Forwards a position (three floats passed as raw bits) and two integers
/// to the engine. The original assembles the position on its own stack
/// frame; only the pushed words are observable. No return slot is written.
export!(cdecl, rw_00b9a4c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
