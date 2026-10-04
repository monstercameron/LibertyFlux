// original: 0x00bc5c40 GET_CAR_SPEED
/// Script native `GET_CAR_SPEED` (hash 0x16DD2D00).
///
/// Forwards two script arguments (a vehicle handle and an out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bc5c40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
