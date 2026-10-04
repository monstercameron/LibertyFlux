// original: 0x00b8d7b0 SET_MULTIPLAYER_BRIEF
/// Script native handler `SET_MULTIPLAYER_BRIEF`.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// handler function: `0x00b8d7b0`, engine call site: `0x00b8d7bc`.
export!(cdecl, rw_b8d7b0(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        callee_cdecl!(1, u32, a0, a1)
    }
});
