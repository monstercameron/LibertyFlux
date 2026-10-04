// original: 0x00bc7880 SET_CAR_WATERTIGHT
/// Script native handler `SET_CAR_WATERTIGHT`.
///
/// Sets whether the vehicle is watertight.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// Argument 1 is bool-coerced (`!= 0`); the original also overwrites the low
/// byte of its own incoming stack slot with the flag, which is dead after
/// return and therefore not reproduced (see contract note).
/// handler function: `0x00bc7880`, engine call site: `0x00bc7896`.
export!(cdecl, rw_bc7880(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let vehicle = *args.add(0);
        let watertight = ((*args.add(1) != 0) as u32) | ((ctx as u32) & 0xFFFF_FF00);
        callee_cdecl!(1, u32, vehicle, watertight)
    }
});
