// original: 0x00bc73a0 SET_CAR_ANIM_SPEED
/// Script native `SET_CAR_ANIM_SPEED` (hash 0x74CD7D1F).
///
/// Forwards four script arguments to the engine: three integers (a vehicle
/// handle, an animation name hash and flags) and one float bit-pattern
/// (the speed). The float is copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00bc73a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
