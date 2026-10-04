// original: 0x00bc7440 SET_CAR_COLOUR_COMBINATION
/// Script native handler `SET_CAR_COLOUR_COMBINATION`.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// handler function: `0x00bc7440`, engine call site: `0x00bc744c`.
export!(cdecl, rw_bc7440(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let vehicle = *args.add(0);
        let combo = *args.add(1);
        callee_cdecl!(1, u32, vehicle, combo)
    }
});
