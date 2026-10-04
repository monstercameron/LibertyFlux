// original: 0x00b947a0 GET_WIDTH_OF_LITERAL_STRING
/// Native handler `GET_WIDTH_OF_LITERAL_STRING` (script context in, engine call out).
///
/// Measures a literal string with the current font; engine returns width in ST0, stored to the return slot.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00b947a0(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        let w: f32 = callee_cdecl!(1, f32, *a.add(0));
        *((*ctx).ret as *mut f32) = w;
        (*ctx).ret as u32
    }
});
