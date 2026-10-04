// original: 0x00bd2290 GET_SCRIPT_FIRE_COORDS
/// Script native `GET_SCRIPT_FIRE_COORDS` (hash 0x4F256F49).
///
/// Forwards 4 script arguments to the engine in order.
/// No return slot is written.
export!(cdecl, rw_00bd2290(ctx: *const u8) -> u32 {
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
