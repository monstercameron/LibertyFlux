// original: 0x00bc6250 GET_RANDOM_CAR_IN_SPHERE
/// Script native `GET_RANDOM_CAR_IN_SPHERE` (hash 0x528F5EA7).
///
/// Forwards seven script arguments (four float bit-patterns for the sphere centre and radius, then three integers) to the engine.
export!(cdecl, rw_00bc6250(ctx: *const u8) -> u32 {
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
            *args.add(5),
            *args.add(6),
        )
    }
});
