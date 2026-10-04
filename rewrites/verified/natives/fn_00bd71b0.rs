// original: 0x00bd71b0 FORWARD_TO_TIME_OF_DAY
/// Script native handler `FORWARD_TO_TIME_OF_DAY`.
///
/// Advances the clock to the given hour and minute.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// handler function: `0x00bd71b0`, engine call site: `0x00bd71bc`.
export!(cdecl, rw_bd71b0(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let hour = *args.add(0);
        let minute = *args.add(1);
        callee_cdecl!(1, u32, hour, minute)
    }
});
