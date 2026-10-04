// original: 0x00ba1f50 SET_DECISION_MAKER_ATTRIBUTE_TARGET_LOSS_RESPONSE
/// Native handler `SET_DECISION_MAKER_ATTRIBUTE_TARGET_LOSS_RESPONSE` (script context in, engine call out).
///
/// Sets a decision-maker attribute; forwards two integers.
/// `ctx.ret` points at the return slot, `ctx.args` at the argument array.
export!(cdecl, rw_00ba1f50(ctx: *mut NativeCtx) -> u32 {
    unsafe {
        let a = (*ctx).args;
        callee_cdecl!(1, u32, *a.add(0), *a.add(1))
    }
});
