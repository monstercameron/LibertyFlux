// original: 0x00b9a220 ADD_AREA_TO_NETWORK_RESTART_NODE_GROUP_MAPPING
/// Script native handler `ADD_AREA_TO_NETWORK_RESTART_NODE_GROUP_MAPPING`.
///
/// Forwards 2 script arguments to the engine function; no return slot.
/// handler function: `0x00b9a220`, engine call site: `0x00b9a22c`.
export!(cdecl, rw_b9a220(ctx: *const NativeCtx03) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        callee_cdecl!(1, u32, a0, a1)
    }
});
