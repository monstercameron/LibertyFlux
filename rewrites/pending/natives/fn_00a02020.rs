// original: 0x00a02020 SET_ROPE_HEIGHT_FOR_OBJECT
/// Script native `SET_ROPE_HEIGHT_FOR_OBJECT` (hash 0x3D79554A).
///
/// Forwards two script arguments (an object handle and a float height bit-pattern) to the engine. The float is copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00a02020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
