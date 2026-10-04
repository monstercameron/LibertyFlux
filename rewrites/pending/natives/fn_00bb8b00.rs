// original: 0x00bb8b00 EXTEND_PATROL_ROUTE
/// Script native `EXTEND_PATROL_ROUTE` (hash 0x0F3402B8).
///
/// Forwards five script arguments to the engine: three float bit-patterns
/// (coordinates, copied as raw bits) followed by two integers. No return
/// slot is written.
export!(cdecl, rw_00bb8b00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
