// original: 0x00bb9860 TASK_FOLLOW_NAV_MESH_AND_SLIDE_TO_COORD_HDG_RATE
/// Script native `TASK_FOLLOW_NAV_MESH_AND_SLIDE_TO_COORD_HDG_RATE` (hash 0x38824BFE).
///
/// Forwards 9 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00bb9860(ctx: *const u8) -> u32 {
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
        )
    }
});
