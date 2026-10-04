// original: 0x00bb6500 PLAYSTATS_INT_INT
/// Script native handler `PLAYSTATS_INT_INT`.
///
/// Forwards 3 script arguments to the engine function; no return slot.
/// handler function: `0x00bb6500`, engine call site: `0x00bb650f`.
export!(cdecl, rw_bb6500(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        callee_cdecl!(1, u32, a0, a1, a2)
    }
});
