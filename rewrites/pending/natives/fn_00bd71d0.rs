// original: 0x00bd71d0 GET_CURRENT_DATE
/// Native handler `GET_CURRENT_DATE` (script context in, engine call out).
///
/// Returns in-game day and month through two out-param slots passed to the engine.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00bd71d0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
    }
});
