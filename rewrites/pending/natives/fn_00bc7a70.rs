// original: 0x00bc7a70 SET_MISSION_TRAIN_COORDINATES
/// Script native `SET_MISSION_TRAIN_COORDINATES` (hash 0x2A3F654A).
///
/// Forwards 4 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00bc7a70(ctx: *const u8) -> u32 {
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
