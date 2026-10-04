// original: 0x00ba28c0 SET_SWIM_SPEED
/// Script native `SET_SWIM_SPEED` (hash 0x32B4293B).
///
/// Forwards two script arguments to the engine: a character handle and a
/// float bit-pattern (the speed). No return slot is written; the engine
/// answer is the exit value.
export!(cdecl, rw_00ba28c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
