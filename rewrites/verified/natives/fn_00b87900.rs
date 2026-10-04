// original: 0x00b87900 SET_FIXED_CAM_POS
/// Script native `SET_FIXED_CAM_POS` (hash 0x511A3B01).
///
/// Forwards three script arguments (float bit-patterns, a camera position)
/// to the engine. Copied as raw bits, so the forward is bit-exact. No
/// return slot is written.
export!(cdecl, rw_00b87900(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
