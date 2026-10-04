// original: 0x00b9eb60 FORCE_RANDOM_PED_TYPE
/// Script native handler `FORCE_RANDOM_PED_TYPE`.
///
/// Forwards 1 script argument to the engine function; no return slot.
/// handler function: `0x00b9eb60`, engine call site: `0x00b9eb69`.
export!(cdecl, rw_b9eb60(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
