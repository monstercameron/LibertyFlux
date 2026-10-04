// original: 0x00b94040 CLEAR_AREA_OF_CARS
/// Script native `CLEAR_AREA_OF_CARS` (hash 0x24367E48).
///
/// Removes vehicles around a point: forwards four float bit-patterns
/// (center x/y/z and radius) to the engine. No return slot is written.
export!(cdecl, rw_00b94040(ctx: *const u8) -> u32 {
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
