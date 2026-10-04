// original: 0x00ba0fc0 SET_CHAR_ANIM_CURRENT_TIME
/// Script native `SET_CHAR_ANIM_CURRENT_TIME` (hash 0x245F424F).
///
/// Forwards four script arguments to the engine: three integers and a float
/// bit-pattern (the animation time). The float is copied as raw bits, so
/// the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba0fc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
