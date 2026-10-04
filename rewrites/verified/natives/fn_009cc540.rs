// original: 0x009CC540 SET_MISSION_PICKUP_SOUND
/// F21 SET_MISSION_PICKUP_SOUND: forwards 2 args, void.
export!(cdecl, rn10_set_mission_pickup_sound(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args, *args.add(1));
        0
    }
});
