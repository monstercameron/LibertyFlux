// original: 0x00bb8c20 GET_PED_PATH_MAY_USE_LADDERS
/// Script native handler `GET_PED_PATH_MAY_USE_LADDERS`.
///
/// Forwards 1 script argument to the engine function and stores the
/// low byte of its answer (zero-extended) into the return slot.
/// handler function: `0x00bb8c20`, engine call site: `0x00bb8c2a`.
export!(cdecl, rw_bb8c20(ctx: *const NativeCtx) -> u32 {
    unsafe {
        let args = (*ctx).args_ptr;
        let a0 = *args.add(0);
        let answer: u32 = callee_cdecl!(1, u32, a0);
        *(*ctx).ret_ptr = answer & 0xFF;
        (*ctx).ret_ptr as u32
    }
});
