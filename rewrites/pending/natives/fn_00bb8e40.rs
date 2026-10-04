// original: 0x00bb8e40 PED_QUEUE_CONSIDER_PEDS_WITH_FLAG_TRUE
/// Script native handler `PED_QUEUE_CONSIDER_PEDS_WITH_FLAG_TRUE`.
///
/// Forwards 1 script argument to the engine function; no return slot.
/// handler function: `0x00bb8e40`, engine call site: `0x00bb8e49`.
export!(cdecl, rw_bb8e40(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
