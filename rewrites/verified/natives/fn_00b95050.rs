// original: 0x00b95050 SET_UP_TRIP_SKIP
/// Script native `SET_UP_TRIP_SKIP` (hash 0x27B724F1).
///
/// Forwards four float bit-patterns (destination and heading) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b95050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
