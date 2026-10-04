// original: 0x00a00870 CREATE_PICKUP_WITH_AMMO
/// Script native `CREATE_PICKUP_WITH_AMMO` (hash 0x1F736F00).
///
/// Forwards 7 script arguments (three integers, three float coordinates and one more integer) to the engine.
///
/// No return slot is written.
///
/// Float arguments are copied as raw bit patterns, so the forward
/// is bit-exact.
///
export!(cdecl, rw_00a00870(ctx: *const u8) -> u32 {
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
