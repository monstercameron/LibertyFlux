// original: 0x00bc61a0 GET_RANDOM_CAR_BACK_BUMPER_IN_SPHERE
/// Script native `GET_RANDOM_CAR_BACK_BUMPER_IN_SPHERE` (hash 0x2C37408C).
///
/// Forwards seven script arguments to the engine: four float bit-patterns
/// (a sphere centre and radius) followed by three integers. Floats are
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written by the handler itself.
export!(cdecl, rw_00bc61a0(ctx: *const u8) -> u32 {
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
