// original: 0x00ba1b70 SET_CHAR_VELOCITY
/// Script native `SET_CHAR_VELOCITY` (hash 0x07C76803).
///
/// Forwards four script arguments to the engine: a character handle
/// and three float bit-patterns (the velocity vector). Floats are
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00ba1b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
