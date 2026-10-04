// original: 0x00bb67a0 UPDATE_NETWORK_STATISTICS
/// Script native handler `UPDATE_NETWORK_STATISTICS`.
///
/// Forwards 4 script arguments to the engine function; no return slot.
/// handler function: `0x00bb67a0`, engine call site: `0x00bb67b2`.
export!(cdecl, rw_bb67a0(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        callee_cdecl!(1, u32, a0, a1, a2, a3)
    }
});
