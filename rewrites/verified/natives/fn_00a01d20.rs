// original: 0x00a01d20 SET_OBJECT_HEALTH
/// Script native `SET_OBJECT_HEALTH` (hash 0x46C41EA8).
///
/// Forwards two script arguments to the engine: an object handle and a
/// health value whose float bits are copied unchanged. No return slot is
/// written.
export!(cdecl, rw_00a01d20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
