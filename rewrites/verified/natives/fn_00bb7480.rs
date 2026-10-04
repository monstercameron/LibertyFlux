// original: 0x00bb7480 LOAD_SCENE
/// Script native `LOAD_SCENE` (hash 0x39F62BFB).
///
/// Forwards three script arguments (scene coordinates, as float
/// bit-patterns) to the engine. Floats are copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bb7480(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
