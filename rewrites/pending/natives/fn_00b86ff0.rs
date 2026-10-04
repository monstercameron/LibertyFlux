// original: 0x00b86ff0 POINT_CAM_AT_COORD
/// Script native `POINT_CAM_AT_COORD` (hash 0x4496175C).
///
/// Forwards four script arguments (a camera handle and three float bit-patterns for a position) to the engine. No return slot is written.
export!(cdecl, rw_00b86ff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
