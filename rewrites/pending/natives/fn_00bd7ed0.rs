// original: 0x00bd7ed0 IS_NETWORK_PLAYER_VISIBLE
/// Script native handler `IS_NETWORK_PLAYER_VISIBLE`.
///
/// Forwards 1 script argument to the engine function and stores the
/// low byte of its answer (zero-extended) into the return slot.
/// handler function: `0x00bd7ed0`, engine call site: `0x00bd7eda`.
export!(cdecl, rw_bd7ed0(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let answer: u32 = callee_cdecl!(1, u32, a0);
        *(*ctx).ret_ptr = answer & 0xFF;
        (*ctx).ret_ptr as u32
    }
});
