// original: 0x00bc74a0 SET_CAR_COORDINATES_NO_OFFSET
/// Script native `SET_CAR_COORDINATES_NO_OFFSET` (hash 0x12D64378).
///
/// Forwards a vehicle handle and three float coordinates to the engine. Floats are copied as raw bits. No return slot is written.
export!(cdecl, rw_00bc74a0(ctx: *const u8) -> u32 {
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
