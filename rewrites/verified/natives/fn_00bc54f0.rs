// original: 0x00bc54f0 CREATE_RANDOM_CAR_FOR_CAR_PARK
/// Script native `CREATE_RANDOM_CAR_FOR_CAR_PARK` (hash 0x36DA42AF).
///
/// Forwards four float script arguments (a position and a heading) as raw
/// bits to the engine. No return slot is written.
export!(cdecl, rw_00bc54f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
