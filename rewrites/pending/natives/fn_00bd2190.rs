// original: 0x00bd2190 EXTINGUISH_FIRE_AT_POINT
/// Script native `EXTINGUISH_FIRE_AT_POINT` (hash 0x35A97B73).
///
/// Forwards 4 script arguments (three float coordinates and a float radius) to the engine.
///
/// No return slot is written.
///
/// Float arguments are copied as raw bit patterns, so the forward
/// is bit-exact.
///
export!(cdecl, rw_00bd2190(ctx: *const u8) -> u32 {
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
