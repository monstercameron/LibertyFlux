// original: 0x00bc60c0 GET_OFFSET_FROM_CAR_IN_WORLD_COORDS
/// Script native `GET_OFFSET_FROM_CAR_IN_WORLD_COORDS` (hash 0x7F8D3DD9).
///
/// Forwards seven script arguments to the engine: a vehicle handle, three
/// float bit-patterns (the local offset) and three integers. Floats are
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written by the handler itself.
export!(cdecl, rw_00bc60c0(ctx: *const u8) -> u32 {
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
