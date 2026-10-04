// original: 0x00bc7370 SET_CAR_ANIM_CURRENT_TIME
/// Script native `SET_CAR_ANIM_CURRENT_TIME` (hash 0x04485574).
///
/// Forwards four script arguments to the engine: three integers (vehicle
/// and animation identifiers) and one float bit-pattern (the time).
/// No return slot is written.
export!(cdecl, rw_00bc7370(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3),)
    }
});
