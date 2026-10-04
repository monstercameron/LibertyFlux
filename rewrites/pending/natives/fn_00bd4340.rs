// original: 0x00bd4340 SET_TIMECYCLE_MODIFIER
/// Script native handler `SET_TIMECYCLE_MODIFIER`.
///
/// Sets the timecycle modifier.
///
/// Forwards 1 script argument to the engine function; no return slot.
/// handler function: `0x00bd4340`, engine call site: `0x00bd4349`.
export!(cdecl, rw_bd4340(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
