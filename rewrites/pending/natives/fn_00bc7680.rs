// original: 0x00bc7680 SET_CAR_LIGHT_MULTIPLIER
/// Script native `SET_CAR_LIGHT_MULTIPLIER` (hash 0x74824ADA).
///
/// Forwards two script arguments to the engine: a vehicle handle and a float bit-pattern (light multiplier). No return slot is written.
export!(cdecl, rw_00bc7680(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
