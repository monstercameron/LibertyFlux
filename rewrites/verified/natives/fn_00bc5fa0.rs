// original: 0x00bc5fa0 GET_NEAREST_CABLE_CAR
/// Script native `GET_NEAREST_CABLE_CAR` (hash 0x7F3A0E22).
///
/// Forwards 5 script arguments (four float coordinates and an integer) to the engine.
///
/// No return slot is written.
///
/// Float arguments are copied as raw bit patterns, so the forward
/// is bit-exact.
///
export!(cdecl, rw_00bc5fa0(ctx: *const u8) -> u32 {
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
