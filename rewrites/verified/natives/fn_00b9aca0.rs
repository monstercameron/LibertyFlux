// original: 0x00b9aca0 MARK_ROAD_NODE_AS_DONT_WANDER
/// Script native `MARK_ROAD_NODE_AS_DONT_WANDER` (hash 0x4C2621B6).
///
/// Forwards three script arguments (float bit-patterns, a position) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b9aca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
