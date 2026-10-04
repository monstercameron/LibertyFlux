// original: 0x00a01f60 SET_OBJECT_SCALE
/// Script native `SET_OBJECT_SCALE` (hash 0x145B13C7).
///
/// Forwards two script arguments (an object handle and a float
/// bit-pattern scale) to the engine. The float is copied as raw bits, so
/// the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00a01f60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
