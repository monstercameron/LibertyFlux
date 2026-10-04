// original: 0x00ba2130 SET_GROUP_SEPARATION_RANGE
/// Script native `SET_GROUP_SEPARATION_RANGE` (hash 0x22DD329E).
///
/// Forwards two script arguments (a group handle and a float range) to the engine; the float is copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00ba2130(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
