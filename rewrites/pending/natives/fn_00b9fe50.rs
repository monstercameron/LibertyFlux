// original: 0x00b9fe50 IS_CHAR_TOUCHING_CHAR
/// Native handler `IS_CHAR_TOUCHING_CHAR` (script context in, engine call out).
///
/// True if one char touches another; forwards both handles, stores the byte answer.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b9fe50(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let r = callee_cdecl!(1, u32, *a.add(0), *a.add(1));
        // The original keeps only AL and zero-extends it into the slot.
        *(*ctx).ret = r & 0xFF;
        (*ctx).ret as u32
    }
});
