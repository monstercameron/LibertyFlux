// original: 0x00a00f10 GET_PICKUP_COORDINATES
/// Script native `GET_PICKUP_COORDINATES` (hash 0x0F636C38).
///
/// Forwards four script arguments to the engine. No return slot is written.
export!(cdecl, rw_00a00f10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
