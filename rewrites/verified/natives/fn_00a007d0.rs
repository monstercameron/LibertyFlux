// original: 0x00a007d0 CREATE_PICKUP_ROTATE
/// Script native `CREATE_PICKUP_ROTATE` (hash 0x675E5940).
///
/// Forwards ten script arguments to the engine: three integers (a pickup type, a model hash and flags), six float bit-patterns (position and rotation) and one more integer. Floats are copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00a007d0(ctx: *const u8) -> u32 {
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
            *args.add(7),
            *args.add(8),
            *args.add(9),
        )
    }
});
