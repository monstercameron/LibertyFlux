// original: 0x00ba10c0 SET_CHAR_CANT_BE_DRAGGED_OUT
/// Native handler `SET_CHAR_CANT_BE_DRAGGED_OUT` (script context in, engine call out).
///
/// Locks the char while in a car; forwards handle and coerced flag.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba10c0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), coerced(ctx, *a.add(1)))
    }
});
