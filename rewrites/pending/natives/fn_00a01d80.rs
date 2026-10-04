// original: 0x00a01d80 SET_OBJECT_INITIAL_VELOCITY
/// Script native `SET_OBJECT_INITIAL_VELOCITY` (hash 0x41ED206B).
///
/// Forwards 4 script argument(s) to the engine: 3 float bit-pattern(s), 1 integer(s).
/// Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00a01d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
